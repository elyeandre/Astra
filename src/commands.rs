use crate::{LUA, SCRIPT_PATH};
use clap::crate_version;
use std::str::FromStr;

/// Runs a Lua script.
pub async fn run_command(file_path: String, extra_args: Option<Vec<String>>) {
    let lua = &LUA;

    // ! Move VM preparation into a separate function
    // ! To enable VM creation on route request

    // Set the script path.
    #[allow(clippy::expect_used)]
    let path =
        std::path::PathBuf::from_str(&file_path).expect("Could not turn path into a path buffer");
    #[allow(clippy::expect_used)]
    SCRIPT_PATH
        .set(path)
        .expect("Could not set the script path to OnceCell");

    // Register Lua components.
    registration(lua).await;

    // Handle extra arguments.
    if let Some(extra_args) = extra_args {
        if let Ok(args) = lua.create_table() {
            if let Err(e) = args.set(0, file_path.clone()) {
                eprintln!("Error adding arg to the args list: {e:?}");
            }

            for (index, value) in extra_args.into_iter().enumerate() {
                if let Err(e) = args.set((index + 1) as i32, value) {
                    eprintln!("Error adding arg to the args list: {e:?}");
                }
            }

            if let Err(e) = lua.globals().set("arg", args) {
                eprintln!("Error setting the global variable ARGS: {e:?}");
            }
        }
    }

    // Load and execute the Lua script.
    #[allow(clippy::expect_used)]
    let user_file = std::fs::read_to_string(&file_path).expect("Couldn't read file");
    if let Err(e) = lua.load(user_file).set_name(file_path).exec_async().await {
        eprintln!("{e}");
    }

    // Wait for all Tokio tasks to finish.
    let metrics = tokio::runtime::Handle::current().metrics();
    loop {
        let alive_tasks = metrics.num_alive_tasks();
        if alive_tasks == 0 {
            break;
        }
    }
}

/// Exports the Lua bundle.
pub async fn export_bundle_command(folder_path: Option<String>) {
    let mut lua_lib = prepare_prelude();
    #[allow(clippy::expect_used)]
    let std_lib = crate::components::register_components(&LUA)
        .await
        .expect("Error setting up the standard library");
    lua_lib.extend(std_lib);

    let folder_path = std::path::Path::new(&folder_path.unwrap_or(".".to_string()));

    // Write the bundled library to the file.
    #[allow(clippy::expect_used)]
    std::fs::write(file_path, lib).expect("Could not export the bundled library");
    println!("🚀 Successfully exported the bundled library!");
    std::process::exit(0);
}

/// Upgrades to the latest version.
pub async fn upgrade_command(user_agent: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let user_agent = user_agent.unwrap_or(
        "Mozilla/5.0 (X11; \
            Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) \
            Chrome/51.0.2704.103 Safari/537.36"
            .to_string(),
    );
    let latest_tag = reqwest::Client::new()
        .get("https://api.github.com/repos/ArkForgeLabs/Astra/tags")
        .header(reqwest::header::USER_AGENT, user_agent)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    // Get the latest tag.
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

    // Compare the latest tag with the current version.
    if version_compare::compare_to(latest_tag, crate_version!(), version_compare::Cmp::Gt)
        .is_ok_and(|compared| compared)
    {
        println!("Updating from {} to {latest_tag}...", crate_version!());

        let runtime = if cfg!(feature = "lua54") {
            "lua54"
        } else if cfg!(feature = "luajit52") {
            "luajit52"
        } else if cfg!(feature = "luau") {
            "luau"
        } else if cfg!(feature = "lua51") {
            "lua51"
        } else if cfg!(feature = "lua52") {
            "lua52"
        } else if cfg!(feature = "lua53") {
            "lua53"
        } else {
            "luajit"
        };

        let architecture = if cfg!(windows) {
            "windows-amd64.exe"
        } else {
            "linux-amd64"
        };

        let file_name = format!("astra-{runtime}-{architecture}");
        let url =
            format!("https://github.com/ArkForgeLabs/Astra/releases/latest/download/{file_name}");

        // Download the latest release.
        let content = reqwest::get(url).await?.bytes().await?;
        let current_file_name = std::env::current_exe()?.to_string_lossy().to_string();

        std::fs::write(format!("{file_name}-{latest_tag}"), content)?;
        std::fs::rename(
            current_file_name.clone(),
            format!("{current_file_name}_old"),
        )?;
        std::fs::rename(
            format!("{file_name}-{latest_tag}"),
            current_file_name.clone(),
        )?;
        std::fs::remove_file(format!("{current_file_name}_old"))?;

        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("chmod")
                .arg("+x")
                .arg(current_file_name)
                .spawn();
        }

        println!("Done! Enjoy!");
    } else {
        println!("Already up to date!");
    }

    Ok(())
}

/// Registers Lua components.
async fn registration(lua: &mlua::Lua) {
    let mut lua_lib = prepare_prelude();

    #[allow(clippy::expect_used)]
    let std_lib = crate::components::register_components(lua)
        .await
        .expect("Error setting up the standard library");

    // Set Astra version in Lua globals.
    if let Err(e) = lua
        .globals()
        .set("astra_internal__version", crate_version!())
    {
        eprintln!("Error adding version to Astra: {e:#?}");
    }

    lua_lib.extend(std_lib);

    for (file_name, content) in lua_lib {
        if let Err(e) = lua
            .load(content.as_str())
            .set_name(file_name)
            .exec_async()
            .await
        {
            eprintln!("Couldn't add prelude:\n{e}");
        }
    }
}

fn prepare_prelude() -> Vec<(String, String)> {
    let mut lua_lib = include_dir::include_dir!("./src/lua/libs")
        .files()
        .filter_map(|file| {
            if let Some(name) = file
                .path()
                .file_name()
                .map(|name| name.to_str().and_then(|name| Some(name.replace("@", ""))))
                && let Some(name) = name
                && let Some(content) = file.contents_utf8()
            {
                Some((name, content.replace("@ASTRA_VERSION", crate_version!())))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    lua_lib.sort_by(|itema, itemb| itema.0.cmp(&itemb.0));

    lua_lib
}
