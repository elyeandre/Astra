-- Create a templating engine, read more about template syntax at: https://keats.github.io/tera/docs
local template_engine = Astra.new_templating_engine("examples/templates/**/*.html")
-- Exclude base files from rendering
template_engine:exclude_templates({ "base.html" })
-- Add some data for the templates
template_engine:context_add("count", 5)

-- You can also add functions to be used within the templates
template_engine:add_function("test", function (args)
	return args.name
end)

-- Create an HTTP Server
local server = Astra.http.server:new()

-- Partial hydration
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

-- Serve the templates
server:templates(template_engine)

-- Or for debugging
-- server:templates_debug(template_engine)

-- Run
server:run()
