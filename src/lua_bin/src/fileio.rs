use crate::startup::SCRIPT_PATH;

pub async fn register_fileio_functions(lua: &mlua::Lua) -> mlua::Result<()> {
    let astra_io = lua.create_table()?;

    astra_io.set(
        "read",
        lua.create_function(|_, path: String| match std::fs::read_to_string(path) {
            Ok(result) => Ok(result),
            Err(e) => Err(mlua::Error::runtime(e)),
        })?,
    )?;

    astra_io.set(
        "write",
        lua.create_function(|_, (path, content): (String, String)| {
            match std::fs::write(path, content) {
                Ok(_) => Ok(()),
                Err(e) => Err(mlua::Error::runtime(e)),
            }
        })?,
    )?;

    astra_io.set(
        "get_current_dir",
        lua.create_function(|_, ()| match std::env::current_dir() {
            Ok(result) => Ok(result),
            Err(e) => Err(mlua::Error::runtime(e)),
        })?,
    )?;

    astra_io.set(
        "get_script_path",
        lua.create_function(|_, ()| Ok(SCRIPT_PATH.get().cloned()))?,
    )?;

    lua.globals().set("AstraIO", astra_io)?;

    Ok(())
}
