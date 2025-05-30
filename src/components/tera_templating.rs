use std::sync::Arc;

use mlua::{FromLua, LuaSerdeExt, UserData};

#[derive(Debug, Clone, FromLua)]
pub struct TeraTemplating {
    pub env: tera::Tera,
    pub context: tera::Context,
    pub exclusions: Vec<Arc<str>>,
}
impl TeraTemplating {
    pub fn register_to_lua(lua: &mlua::Lua) -> mlua::Result<()> {
        lua.globals().set(
            "astra_internal__new_tera",
            lua.create_function(|_, dir: String| match tera::Tera::new(&dir) {
                Ok(env) => Ok(Self {
                    env,
                    context: tera::Context::new(),
                    exclusions: Vec::new(),
                }),
                Err(e) => Err(mlua::Error::runtime(format!(
                    "Could not start the Tera templating engine: {e}"
                ))),
            })?,
        )?;

        Ok(())
    }

    pub fn get_template_names(&self) -> impl Iterator<Item = &str> {
        self.env
            .get_template_names()
            .filter(|name| !self.exclusions.contains(&(*name).into()))
    }
}
impl UserData for TeraTemplating {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut(
            "add_template",
            |_, this, (name, template): (String, String)| match this
                .env
                .add_raw_template(&name, &template)
            {
                Ok(()) => Ok(()),
                Err(e) => Err(mlua::Error::runtime(format!(
                    "ERROR TERRA - Could not add a template: {e}"
                ))),
            },
        );
        methods.add_method_mut(
            "add_template_file",
            |_, this, (name, path): (String, String)| match this
                .env
                .add_template_file(&path, Some(&name))
            {
                Ok(()) => Ok(()),
                Err(e) => Err(mlua::Error::runtime(format!(
                    "ERROR TERRA - Could not add a template: {e}"
                ))),
            },
        );
        methods.add_method_mut("exclude_templates", |_, this, names: Vec<String>| {
            for i in names {
                this.exclusions.push(i.into());
            }

            Ok(())
        });

        methods.add_method_mut(
            "context_add",
            |_, this, (key, value): (String, mlua::Value)| {
                this.context.insert(&key, &value);
                Ok(())
            },
        );
        methods.add_method_mut("context_remove", |_, this, key: String| {
            this.context.remove(&key);
            Ok(())
        });
        methods.add_method("context_get", |lua, this, key: String| {
            match this.context.get(&key) {
                Some(value) => lua.to_value(value),
                None => Ok(mlua::Nil),
            }
        });

        methods.add_method("render", |_, this, name: String| {
            match this.env.render(&name, &this.context) {
                Ok(result) => Ok(result),
                Err(e) => Err(mlua::Error::runtime(format!(
                    "ERROR TERRA - Could not add a template: {e}"
                ))),
            }
        });
    }
}
