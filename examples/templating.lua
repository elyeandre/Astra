-- Create a templating engine, read more about template syntax at: https://keats.github.io/tera/docs
local template_engine = Astra.new_templating_engine("examples/templates/**/*.html")
-- Exclude base files from rendering
template_engine:exclude_templates({ "base.html" })
-- Add some data for the templates
template_engine:context_add("count", 5)

-- Create an HTTP Server
local server = Astra.http.server:new()

-- Serve the templates
server:templates(template_engine)

-- Run
server:run()
