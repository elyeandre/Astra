use axum_extra::extract::cookie::Cookie;
use mlua::{FromLua, UserData};

#[derive(Debug, Clone, FromLua)]
pub struct LuaCookie<'a> {
    pub cookie: Cookie<'a>,
}
impl UserData for LuaCookie<'_> {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("set_domain", |_, this, domain: String| {
            this.cookie.set_domain(domain);

            Ok(())
        });

        methods.add_method_mut("set_name", |_, this, name: String| {
            this.cookie.set_name(name);

            Ok(())
        });

        methods.add_method_mut("set_path", |_, this, path: String| {
            this.cookie.set_path(path);

            Ok(())
        });

        methods.add_method_mut("set_value", |_, this, value: String| {
            this.cookie.set_value(value);

            Ok(())
        });
    }
}
