use crate::startup::SCRIPT_PATH;

pub async fn register_fileio_functions(lua: &mlua::Lua) -> mlua::Result<()> {
    lua.globals().set(
        "read_file",
        lua.create_function(|_, path: String| match std::fs::read_to_string(path) {
            Ok(result) => Ok(result),
            Err(e) => Err(mlua::Error::runtime(e)),
        })?,
    )?;

    lua.globals().set(
        "current_dir",
        lua.create_function(|_, ()| match std::env::current_dir() {
            Ok(result) => Ok(result),
            Err(e) => Err(mlua::Error::runtime(e)),
        })?,
    )?;

    lua.globals().set(
        "get_script_file",
        lua.create_function(|_, ()| Ok(SCRIPT_PATH.get().cloned()))?,
    )?;

    Ok(())
}
