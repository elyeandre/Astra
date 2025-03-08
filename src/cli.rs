use clap::{Parser, command, crate_authors, crate_name, crate_version};
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
    Run {
        file_path: String,
        #[arg(long, short = 'c', action = clap::ArgAction::SetTrue, default_value = "true")]
        core: bool,
        #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
        extra_args: Option<Vec<String>>,
    },
    #[command(about = "Exports the packages lua bundle for import for intellisense")]
    ExportBundle {
        file_path: Option<String>,
        #[arg(long, short = 'c', action = clap::ArgAction::SetTrue)]
        core: bool,
    },
    #[command(about = "Updates to the latest version", alias = "update")]
    Upgrade,
}

pub async fn init() {
    // commands
    match AstraCLI::parse() {
        AstraCLI::Run {
            file_path,
            core,
            extra_args,
        } => {
            let lua = &LUA;

            #[allow(clippy::expect_used)]
            SCRIPT_PATH
                .set(file_path.clone())
                .expect("Could not set the script path to OnceCell");

            let _ = registration(lua, core).await;

            // args
            if let Some(extra_args) = extra_args {
                if let Ok(args) = lua.create_table() {
                    // file path
                    if let Err(e) = args.set(0, file_path.clone()) {
                        eprintln!("Error adding arg to the args list: {e:?}");
                    }

                    for (args_length, value) in extra_args.into_iter().enumerate() {
                        if let Err(e) = args.set((args_length + 1) as i32, value) {
                            eprintln!("Error adding arg to the args list: {e:?}");
                        }
                    }

                    if let Err(e) = lua.globals().set("arg", args) {
                        eprintln!("Error setting the global variable ARGS: {e:?}");
                    }
                }
            }

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
        AstraCLI::ExportBundle { file_path, core } => {
            let (lib, _) = prepare_prelude(core);

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

async fn registration(lua: &mlua::Lua, include_utils: bool) -> String {
    let (lib, cleaned_lib) = prepare_prelude(include_utils);

    // register required global functions
    crate::components::global_functions::essential_global_functions(lua);

    if include_utils {
        if let Err(e) = crate::components::register_components(lua).await {
            eprintln!("Error setting the util functions:\n{e}");
        }
    }

    if let Err(e) = lua.load(cleaned_lib.as_str()).exec_async().await {
        eprintln!("Couldn't add prelude:\n{e}");
    }

    lib
}

fn prepare_prelude(include_utils: bool) -> (String, String) {
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
        let lib = include_str!("./lua/astra_bundle.lua").to_string();

        if include_utils {
            let utils_lib = include_str!("./lua/astra_utils.lua");
            format!("{utils_lib}\n{lib}")
        } else {
            lib
        }
    };

    let lib = filter(lib, "--- @START_REMOVING_PACK", "--- @END_REMOVING_PACK");

    let cleaned_lib = filter(
        lib.clone(),
        "--- @START_REMOVING_RUNTIME",
        "--- @END_REMOVING_RUNTIME",
    );

    (lib, cleaned_lib)
}

pub async fn self_update_cli() -> Result<(), Box<dyn ::std::error::Error>> {
    let latest_tag = reqwest::Client::new()
        .get("https://api.github.com/repos/ArkForgeLabs/Astra/tags")
        .header(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36"
        )
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

            let architecture = if cfg!(windows) {
                "windows-amd64.exe"
            } else {
                "linux-amd64"
            };

            let file_name = format!("astra-{architecture}");
            let url = format!(
                "https://github.com/ArkForgeLabs/Astra/releases/latest/download/{file_name}"
            );

            let content = reqwest::get(url).await?.bytes().await?;
            let current_file_name = std::env::current_exe()?.to_string_lossy().to_string();

            std::fs::File::create(format!("{file_name}-{}", latest_tag))?.write_all(&content)?;

            std::fs::rename(
                current_file_name.clone(),
                format!("{current_file_name}_old"),
            )?;
            std::fs::rename(
                format!("{file_name}-{}", latest_tag),
                current_file_name.clone(),
            )?;

            std::fs::remove_file(format!("{current_file_name}_old"))?;

            #[cfg(target_os = "linux")]
            let _ = std::process::Command::new("chmod")
                .arg("+x")
                .arg(current_file_name)
                .spawn();

            println!("Done! Enjoy!")
        } else {
            println!("Already up to date!");
        }
    }
    Ok(())
}
