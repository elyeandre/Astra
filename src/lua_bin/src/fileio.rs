use crate::startup::SCRIPT_PATH;
use mlua::UserData;

pub async fn register_fileio_functions(lua: &mlua::Lua) -> mlua::Result<()> {
    let astra_io = lua.create_table()?;

    astra_io.set(
        "read_dir",
        lua.create_function(|_, path: String| match std::fs::read_dir(path) {
            Ok(result) => Ok(result
                .filter_map(|entry| {
                    if let Ok(entry) = entry {
                        Some(AstraDirEntry(entry))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()),
            Err(e) => Err(mlua::Error::runtime(e)),
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

struct AstraFileType(std::fs::FileType);
impl UserData for AstraFileType {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("is_file", |_, this, ()| Ok(this.0.is_file()));
        methods.add_method("is_dir", |_, this, ()| Ok(this.0.is_dir()));
        methods.add_method("is_symlink", |_, this, ()| Ok(this.0.is_symlink()));
    }
}
struct AstraDirEntry(std::fs::DirEntry);
impl UserData for AstraDirEntry {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("file_name", |_, this, ()| {
            match this.0.file_name().into_string() {
                Ok(file_name) => Ok(file_name),
                Err(e) => Err(mlua::Error::runtime(format!("{e:?}"))),
            }
        });
        methods.add_method("file_type", |_, this, ()| match this.0.file_type() {
            Ok(file_type) => Ok(AstraFileType(file_type)),
            Err(e) => Err(mlua::Error::runtime(format!("{e:?}"))),
        });
        methods.add_method("path", |_, this, ()| match this.0.path().to_str() {
            Some(path) => Ok(path.to_string()),
            None => Err(mlua::Error::runtime("Could not get the path")),
        });
    }
}
