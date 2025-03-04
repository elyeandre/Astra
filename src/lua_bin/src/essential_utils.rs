use mlua::LuaSerdeExt;

pub fn essential_utils_registration(lua: &mlua::Lua) {
    dotenv_function(lua);
    pretty_print(lua);
    // json
    json_encode(lua);
    json_decode(lua);
}

pub fn dotenv_function(lua: &mlua::Lua) {
    if let Ok(function) = lua.create_function(|lua, file_name: String| {
        let env_table = lua.globals().get::<mlua::Table>("ENV")?;

        // if the file exists
        match dotenvy::from_filename_iter(file_name) {
            Ok(file) => {
                // filter the available and parsed items
                for (key, value) in file.filter_map(|item| match item {
                    Ok(item) => Some(item),
                    Err(_) => None,
                }) {
                    env_table.set(key, value)?;
                }
            }
            Err(_) => {
                // eprintln!("Error loading a dotenv file: {e}");
            }
        }

        Ok(())
    }) {
        if let Err(e) = lua.globals().set("dotenv_load", function) {
            println!("Could not insert the function for dotenv_load: {e}");
        }
    }
}

pub fn pretty_print(lua: &mlua::Lua) {
    if let Ok(function) = lua.create_function(|_, input: mlua::Value| {
        println!("{input:#?}");

        Ok(())
    }) {
        if let Err(e) = lua.globals().set("astra_internal__pretty_print", function) {
            println!("Could not insert the function for pretty printing: {e}");
        }
    }
}

pub fn json_encode(lua: &mlua::Lua) {
    if let Ok(function) = lua.create_function(|lua, input: mlua::Value| {
        // removing functions
        let input = if let Some(input) = input.as_table() {
            let new_input = lua.create_table()?;

            for pair in input.pairs::<mlua::Value, mlua::Value>() {
                let (key, value) = pair?;
                if !value.is_function()
                    && !value.is_light_userdata()
                    && !value.is_userdata()
                    && !value.is_error()
                    && !value.is_thread()
                {
                    new_input.set(key, value)?;
                }
            }

            lua.to_value(&new_input)?
        } else {
            input
        };

        let json_value = lua.from_value::<serde_json::Value>(input)?;
        match serde_json::to_string(&json_value) {
            Ok(serialized) => Ok(serialized),
            Err(e) => Err(mlua::Error::runtime(format!(
                "Could not serialize the input into a valid JSON string: {e:?}"
            ))),
        }
    }) {
        if let Err(e) = lua.globals().set("astra_internal__json_encode", function) {
            println!("Could not insert the function for JSON encoding: {e}");
        }
    }
}

pub fn json_decode(lua: &mlua::Lua) {
    if let Ok(function) = lua.create_function(|lua, input: String| {
        match serde_json::from_str::<serde_json::Value>(&input) {
            Ok(deserialized) => Ok(lua.to_value(&deserialized)),
            Err(e) => Err(mlua::Error::runtime(format!(
                "Could not deserialize the input into a valid Lua value: {e:?}"
            ))),
        }
    }) {
        if let Err(e) = lua.globals().set("astra_internal__json_decode", function) {
            println!("Could not insert the function for JSON decoding: {e}");
        }
    }
}
