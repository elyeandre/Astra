use clap::{command, crate_authors, crate_name, crate_version, Parser};
use std::{io::Write, sync::LazyLock};
use tokio::sync::OnceCell;

pub static LUA: LazyLock<mlua::Lua> = LazyLock::new(mlua::Lua::new);
pub static SCRIPT_PATH: OnceCell<String> = OnceCell::const_new();

#[derive(Parser)] // requires `derive` feature
#[command(name = "Astra")]
#[command(
    bin_name = crate_name!(),
    author = crate_authors!(),
    version(crate_version!()),
    about = r#"
    _    ____ _____ ____      _    
   / \  / ___|_   _|  _ \    / \   
  / _ \ \___ \ | | | |_) |  / _ \  
 / ___ \ ___) || | |  _ <  / ___ \ 
/_/   \_\____/ |_| |_| \_\/_/   \_\

🔥 Blazingly Fast 🔥 web server runtime for Lua"#
)]
enum AstraCLI {
    #[command(arg_required_else_help = true, about = "Runs a lua script")]
    Run { file_path: String },
    #[command(about = "Exports the packages lua bundle for import for intellisense")]
    ExportBundle { file_path: Option<String> },
    #[command(about = "Updates to the latest version", alias = "update")]
    Upgrade,
}

pub async fn init() {
    let lua = &LUA;

    cli(lua).await;
}

async fn cli(lua: &mlua::Lua) {
    // commands
    match AstraCLI::parse() {
        AstraCLI::Run { file_path } => {
            #[allow(clippy::expect_used)]
            SCRIPT_PATH
                .set(file_path.clone())
                .expect("Could not set the script path to OnceCell");

            let _ = registration(lua).await;

            // settings
            if let Ok(settings) = lua.globals().get::<mlua::Table>("Astra") {
                // set the version
                if settings.set("version", crate_version!()).is_ok() {
                    if let Err(e) = lua.globals().set("Astra", settings) {
                        println!("Error adding setting back to Astra: {e:#?}");
                    }
                }
            }

            #[allow(clippy::expect_used)]
            let user_file = std::fs::read_to_string(file_path).expect("Couldn't read file");
            #[allow(clippy::expect_used)]
            if let Err(e) = lua.load(user_file).exec_async().await {
                eprintln!("{}", e);
            }

            // get the metrics for current tokio tasks
            let metrics = tokio::runtime::Handle::current().metrics();
            loop {
                // wait for them to finish
                let alive_tasks = metrics.num_alive_tasks();
                if alive_tasks == 0 {
                    break;
                }
            }
        }
        AstraCLI::ExportBundle { file_path } => {
            let (lib, _) = prepare_prelude();

            let file_path = if let Some(file_path) = file_path {
                file_path
            } else {
                "astra_bundle.lua".to_string()
            };

            #[allow(clippy::expect_used)]
            std::fs::write(file_path, lib).expect("Could not export the bundled library");

            println!("🚀 Successfully exported the bundled library!");
            std::process::exit(0);
        }
        AstraCLI::Upgrade => {
            #[allow(clippy::expect_used)]
            self_update_cli()
                .await
                .expect("Could not update to the latest version.");
        }
    }
}

async fn registration(lua: &mlua::Lua) -> String {
    let (lib, cleaned_lib) = prepare_prelude();

    // register required global functions
    crate::essential_utils::essential_utils_registration(lua);
    register_run_function(lua).await;
    if let Err(e) = crate::fileio::register_fileio_functions(lua).await {
        eprintln!("Could not register File IO functions:\n{e}");
    }

    if let Err(e) = lua.load(cleaned_lib.as_str()).exec_async().await {
        eprintln!("Couldn't add prelude:\n{e}");
    }
    #[cfg(any(feature = "utils_luajit", feature = "utils_luau"))]
    if let Err(e) = utils::register_utils(lua).await {
        eprintln!("Error setting the util functions:\n{e}");
    }

    lib
}

fn prepare_prelude() -> (String, String) {
    fn filter(input: String, start: &str, end: &str) -> String {
        let mut new_lines = Vec::new();
        let mut removing = false;
        for i in input.lines() {
            if i.contains(start) {
                removing = true;
                continue;
            } else if i.contains(end) {
                removing = false;
                continue;
            }

            if !removing {
                new_lines.push(i);
            }
        }
        new_lines.join("\n")
    }

    let lib = {
        let lib = include_str!("../../lua/astra_bundle.lua").to_string();
        #[cfg(any(feature = "utils_luajit", feature = "utils_luau"))]
        let lib = {
            let utils_lib = include_str!("../../lua/astra_utils.lua");
            format!("{utils_lib}\n{lib}")
        };

        lib
    };

    let lib = filter(lib, "--- @START_REMOVING_PACK", "--- @END_REMOVING_PACK");

    let cleaned_lib = filter(
        lib.clone(),
        "--- @START_REMOVING_RUNTIME",
        "--- @END_REMOVING_RUNTIME",
    );

    (lib, cleaned_lib)
}

async fn register_run_function(lua: &mlua::Lua) {
    // Register function for running the server
    if let Ok(function) = lua.create_async_function(|lua, ()| async move {
        // default address
        let mut listener_address = "127.0.0.1:8080".to_string();

        if let Ok(settings) = lua.globals().get::<mlua::Table>("Astra") {
            let mut hostname = "127.0.0.1".to_string();
            if let Ok(new_hostname) = settings.get::<String>("hostname") {
                hostname = new_hostname;
            }

            let mut port = 8080;
            if let Ok(new_port) = settings.get::<u16>("port") {
                port = new_port;
            }

            listener_address = format!("{hostname}:{port}");
        }

        #[allow(clippy::unwrap_used)]
        let listener = tokio::net::TcpListener::bind(listener_address.clone())
            .await
            .unwrap();

        println!("🚀 Listening at: http://{listener_address}");

        #[allow(clippy::unwrap_used)]
        axum::serve(listener, crate::routes::load_routes())
            .await
            .unwrap();

        Ok(())
    }) {
        if let Err(e) = lua.globals().set("astra_internal__start_server", function) {
            println!("Could not insert the function for astra_internal__start_server: {e}");
        }
    }
}

pub async fn self_update_cli() -> Result<(), Box<dyn ::std::error::Error>> {
    let latest_tag = reqwest::Client::new()
        .get("https://api.github.com/repos/ArkForgeLabs/Astra/tags")
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36")
        .send()
        .await?.json::<serde_json::Value>().await?;
    #[allow(clippy::expect_used)]
    let latest_tag = latest_tag
        .as_array()
        .expect("Could not obtain a list of releases")
        .first()
        .expect("Could not get the first available release")
        .as_object()
        .expect("Could not get the release details")
        .get("name")
        .expect("Could not get the tag")
        .as_str()
        .expect("Tag content is not in correct format");

    if let Ok(is_new_version_available) =
        version_compare::compare_to(latest_tag, crate_version!(), version_compare::Cmp::Gt)
    {
        if is_new_version_available {
            println!("Updating from {} to {latest_tag}...", crate_version!());
            #[cfg(any(feature = "utils_luajit", feature = "utils_luau"))]
            let edition = "astra-full";
            #[cfg(not(any(feature = "utils_luajit", feature = "utils_luau")))]
            let edition = "astra-core";

            #[cfg(feature = "luajit")]
            let language = "luajit";
            #[cfg(feature = "luau")]
            let language = "luau";

            let architecture = if cfg!(windows) {
                "windows-amd64.exe"
            } else {
                "linux-amd64"
            };

            let file_name = format!("{edition}-{language}-{architecture}");
            let url = format!(
                "https://github.com/ArkForgeLabs/Astra/releases/latest/download/{file_name}"
            );

            let content = reqwest::get(url).await?.bytes().await?;

            let file_name = if let Some(bin_name) = std::env::args().collect::<Vec<_>>().first() {
                let path = std::path::PathBuf::from(bin_name);
                if let Some(file_name_inner) = path.file_name() {
                    if let Some(file_name) = file_name_inner.to_str() {
                        file_name.to_string()
                    } else {
                        file_name
                    }
                } else {
                    file_name
                }
            } else {
                file_name
            };

            std::fs::File::create(file_name.clone())?.write_all(&content)?;

            #[cfg(target_os = "linux")]
            let _ = std::process::Command::new("chmod")
                .arg("+x")
                .arg(file_name)
                .spawn();

            println!("Done!")
        } else {
            println!("Already up to date!");
        }
    }
    Ok(())
}
