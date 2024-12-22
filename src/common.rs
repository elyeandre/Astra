use std::sync::LazyLock;

pub static LUA: LazyLock<mlua::Lua> = LazyLock::new(|| {
    let lua = mlua::Lua::new();

    #[allow(clippy::expect_used)]
    lua.load(include_str!("../lua/astra_bundle.lua"))
        .exec()
        .expect("Couldn't add prelude");

    lua
});
pub static LUA_FILE_PATH: LazyLock<String> = LazyLock::new(|| {
    let lua_file = std::env::args().collect::<Vec<_>>();
    #[allow(clippy::expect_used)]
    lua_file.get(1).expect("Couldn't open the lua file").clone()
});

pub fn get_package_version() -> String {
    let project = include_str!("../Cargo.toml");
    if let Ok(toml_parse) = toml::from_str::<toml::Value>(project) {
        let get_version = move || -> Option<String> {
            let version = toml_parse
                .get("package")?
                .as_table()?
                .get("version")?
                .as_str()?;

            Some(version.to_string())
        };

        match get_version() {
            Some(version) => version,
            None => "v0.0.0".to_string(),
        }
    } else {
        "v0.0.0".to_string()
    }
}
