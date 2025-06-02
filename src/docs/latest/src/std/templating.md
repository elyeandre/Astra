# Templating

Astra supports jinja-like templating through [Tera](https://keats.github.io/tera/docs). It is incredibly performent, feature rich and easy to use.

```lua
-- can also pass no arguments to make an empty templating engine
local templates = Astra.new_templating_engine("examples/templates/**/*.html")
-- Exclude files
templates:exclude_templates({ "base.html" })
-- Add some data for the templates
templates:context_add("count", 5)

-- You can also add functions to be used within the templates
-- Example within templates: { test(key="value") }
template_engine:add_function("test", function (args)
    pprint(args)
    return "YEE HAW"
end)
```

There are two ways of templating in Astra:

## Static serve

This is where your templates are compiled and ran at the start of your server. The data for these templates do not change once compiled.

```lua
templates:add_to_server(server)
-- Or for debugging that have reload capabilities
templates:add_to_server_debug(server)
```

## Partial Hydration

This method allows you to include dynamic data and render them yourself.

```lua
local count = 0
server:get("/hydrate", function(request, response)
    -- your dynamic data
    count = count + 1
    template_engine:context_add("count", count)
    -- response type
    response:set_header("Content-Type", "text/html")
    -- render the template
    return template_engine:render("index.html")
end)
```
