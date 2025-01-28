mod chrono;
mod database;
mod http_client_request;
mod tasks;

pub trait LuaUtils {
    fn register_to_lua(lua: &mlua::Lua) -> impl std::future::Future<Output = mlua::Result<()>>;
}

pub async fn register_utils(lua: &mlua::Lua) -> mlua::Result<()> {
    database::Database::register_to_lua(lua).await?;
    http_client_request::HTTPClientRequest::register_to_lua(lua).await?;
    tasks::LuaTask::register_to_lua(lua).await?;
    tasks::LuaTimeout::register_to_lua(lua).await?;
    tasks::LuaInterval::register_to_lua(lua).await?;

    Ok(())
}
