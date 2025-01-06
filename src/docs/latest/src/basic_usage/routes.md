# Routes

The `Astra` global table holds all of the route details. The routes are loaded at the start of the runtime and cannot be dynamically modified later on. There are also methods within the `Astra` table that makes it easy to add new routes. For example:

```lua
-- A simple GET index route with text return
Astra.get("/", function()
    return "hello from default Astra instance! " .. Astra.version
end)
```

The syntax are as follows:

```lua
Astra.ROUTE_TYPE(ROUTE_PATH, CALLBACK);

-- Where callback is:
function(request?, response?);
```

The following route types are supported as of now:

* GET
* POST
* PUT
* PATCH
* PARSE
* DELETE
* OPTIONS
* TRACE

All lowercase and snake_case when calling with astra of course. There are two additional ones available:

* STATIC_DIR
* STATIC_FILE

Which does as expected, serves a file or directory over a route.

## Route Logic

Each route function needs a callback which contains a route's logic. This callback function optionally can have two arguments: `request` and `response` respectively, and may optionally have a return.

Interally requests and responses are each a struct in Rust initialized but not parsed/deserialized beforehand. This is to save performance overhead of serialization. However its content and be modified or accessed through their methods. We will discuss them later on.

Return types of the callback can optionally be either empty, string, or a table. The table responses are parsed in Rust and serialized to JSON, and then returned. Empty responses does not include any content. Responses, or lack of them, are by default sent with status code of 200.
