use mlua::UserData;

pub struct Database {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}
impl Database {
    pub async fn register_to_lua(lua: &mlua::Lua) -> mlua::Result<()> {
        let database_constructor = lua.create_async_function(|_, url: String| async move {
            if let Ok(pool) = sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(url.as_str())
                .await
            {
                Ok(Database { pool })
            } else {
                Err(mlua::Error::runtime("Could not open a connection"))
            }
        })?;
        lua.globals().set("database", database_constructor)?;

        Ok(())
    }
}
impl UserData for Database {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("name", |_, this, ()| Ok("YAYA".to_string()));
    }
}
