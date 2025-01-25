use mlua::LuaSerdeExt;

pub static LUA: std::sync::LazyLock<mlua::Lua> = std::sync::LazyLock::new(mlua::Lua::new);

use clap::{command, crate_authors, crate_name, crate_version, Parser};

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
    #[command(
        arg_required_else_help = true,
        about = "Exports the packages lua bundle for import for intellisense"
    )]
    ExportBundle,
}

pub async fn init() {
    let lua = &LUA;
    let lib = include_str!("./lua/astra_bundle.lua");

    #[allow(clippy::expect_used)]
    lua.load(lib)
        .exec_async()
        .await
        .expect("Couldn't add prelude");

    if let Err(e) = crate::utils::register_utils(lua).await {
        println!("Error setting the util functions: {e}");
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

    // commands
    match AstraCLI::parse() {
        AstraCLI::Run { file_path } => {
            let updated_content = prepare_script(&file_path);
            #[allow(clippy::expect_used)]
            if let Err(e) = lua.load(updated_content).exec_async().await {
                eprintln!("Error loading lua file: {}", e);
            }
        }
        AstraCLI::ExportBundle => {
            #[allow(clippy::expect_used)]
            std::fs::write("./astra_bundle.lua", lib)
                .expect("Could not export the bundled library");

            println!("🚀 Successfully exported the bundled library!");
            std::process::exit(0);
        }
    }

    register_run_function().await;
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

async fn register_run_function() {
    // Register function for running the server
    if let Ok(function) = LUA.create_async_function(|lua, ()| async move {
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
        if let Err(e) = LUA.globals().set("astra_internal__start_server", function) {
            println!("Could not insert the function for astra_internal__start_server: {e}");
        }
    }
}

#[derive(Debug, Clone)]
pub struct BodyLua {
    #[allow(unused)]
    pub body: bytes::Bytes,
    pub body_string: String,
}
impl BodyLua {
    pub fn new(bytes: bytes::Bytes) -> Self {
        let body_string = String::from_utf8_lossy(&bytes).to_string();

        Self {
            body: bytes,
            body_string,
        }
    }
}
impl mlua::UserData for BodyLua {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("text", |_, this, ()| Ok(this.body_string.clone()));

        methods.add_method("json", |_, this, ()| {
            match serde_json::from_str::<serde_json::Value>(&this.body_string) {
                Ok(body_json) => Ok(LUA.to_value(&body_json)?),
                Err(e) => Err(mlua::Error::runtime(format!(
                    "Could not parse the body as JSON: {e:#?}"
                ))),
            }
        });
    }
}
