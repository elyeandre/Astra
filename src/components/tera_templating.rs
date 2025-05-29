use mlua::{LuaSerdeExt, UserData};

#[derive(Debug, Clone)]
pub struct TeraTemplating {
    env: tera::Tera,
    context: tera::Context,
}
impl super::AstraComponent for TeraTemplating {
    fn register_to_lua(lua: &mlua::Lua) -> mlua::Result<()> {
        lua.globals().set(
            "astra_internal__new_tera",
            lua.create_function(|_, dir: String| match tera::Tera::new(&dir) {
                Ok(env) => Ok(Self {
                    env,
                    context: tera::Context::new(),
                }),
                Err(e) => Err(mlua::Error::runtime(format!(
                    "Could not start the Tera templating engine: {e}"
                ))),
            })?,
        )?;

        Ok(())
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

        methods.add_method_mut("render", |_, this, name: String| {
            println!("{:?}", this.env.get_template_names().collect::<Vec<_>>());
            match this.env.render(&name, &this.context) {
                Ok(result) => Ok(result),
                Err(e) => Err(mlua::Error::runtime(format!(
                    "ERROR TERRA - Could not add a template: {e}"
                ))),
            }
        });
    }
}
