use clap::{command, crate_authors, crate_name, crate_version, Parser};
use std::{io::Write, sync::LazyLock};

pub static LUA: LazyLock<mlua::Lua> = LazyLock::new(mlua::Lua::new);

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
    ExportBundle,
    #[command(about = "Updates to the latest version", alias = "update")]
    Upgrade,
}

pub async fn init() {
    let lua = &LUA;

    let lib = registration(lua).await;

    cli(lua, lib).await;
}

async fn cli(lua: &mlua::Lua, lib: String) {
    // commands
    match AstraCLI::parse() {
        AstraCLI::Run { file_path } => {
            // settings
            if let Ok(settings) = lua.globals().get::<mlua::Table>("Astra") {
                // set the version
                if settings.set("version", crate_version!()).is_ok() {
                    if let Err(e) = lua.globals().set("Astra", settings) {
                        println!("Error adding setting back to Astra: {e:#?}");
                    }
                }
            }

            let updated_content = prepare_script(&file_path);
            #[allow(clippy::expect_used)]
            if let Err(e) = lua.load(updated_content).exec_async().await {
                eprintln!("Error loading lua file: {}", e);
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
        AstraCLI::ExportBundle => {
            #[allow(clippy::expect_used)]
            std::fs::write("./astra_bundle.lua", lib)
                .expect("Could not export the bundled library");

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
    let lib = include_str!("../../lua/astra_bundle.lua").to_string();
    #[cfg(any(feature = "utils_luajit", feature = "utils_luau"))]
    let lib = {
        let utils_lib = include_str!("../../lua/astra_utils.lua");
        format!("{lib}\n{utils_lib}")
    };

    // register required global functions
    dotenv_function(lua);
    register_run_function(lua).await;

    #[allow(clippy::expect_used)]
    lua.load(lib.as_str())
        .exec_async()
        .await
        .expect("Couldn't add prelude");

    #[cfg(any(feature = "utils_luajit", feature = "utils_luau"))]
    if let Err(e) = utils::register_utils(lua).await {
        println!("Error setting the util functions: {e}");
    }

    lib
}

fn prepare_script(path: &str) -> String {
    // Filter out lines that start with "require" and contain "astra.lua" or "astra.bundle.lua"
    #[allow(clippy::expect_used)]
    let user_file = std::fs::read_to_string(path).expect("Couldn't read file");

    let lines: Vec<&str> = user_file.lines().collect();

    // Filter out lines that start with "require" and contain "astra.lua" or "astra.bundle.lua"
    let filtered_lines: Vec<String> = lines
        .into_iter()
        .filter(|line| {
            !(line.starts_with("require")
                && (line.contains("astra") || line.contains("astra_bundle")))
        })
        .map(|line| line.to_string()) // Convert to String
        .collect();

    // Join the filtered lines back into a single string
    filtered_lines.join("\n")
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

fn dotenv_function(lua: &mlua::Lua) {
    if let Ok(function) = lua.create_function(|lua, file_name: String| {
        let env_table = lua.globals().get::<mlua::Table>("ENV")?;

        // if the file exists
        match dotenvy::from_filename_iter(file_name) {
            Ok(file) => {
                // filter the available and parsed items
                for (key, value) in file.filter_map(|item| match item {
                    Ok(item) => Some(item),
                    Err(_) => None,
                }) {
                    env_table.set(key, value)?;
                }
            }
            Err(_) => {
                // eprintln!("Error loading a dotenv file: {e}");
            }
        }

        Ok(())
    }) {
        if let Err(e) = lua.globals().set("dotenv_load", function) {
            println!("Could not insert the function for dotenv_load: {e}");
        }
    }
}

async fn self_update_cli() -> Result<(), Box<dyn ::std::error::Error>> {
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
