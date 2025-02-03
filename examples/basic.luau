-- import for intellisense. This line is replaced
-- by the bundle code during runtime
local Astra = require("../src/lua/luau/astra_bundle.luau").Astra

print(Astra)
local app = Astra.new()
print(app.version)

-- A simple GET index route with text return
app:get("/", function()
	return "hello from default Astra instance! " .. app.version
end)

-- You can also use the local variables within routes
local counter = 0
app:get("/count", function()
	counter = counter + 1
	-- and also can return JSON
	return { counter }
end)

-- The request parameter is optional but contains useful information
app:get("/headers", function(req)
	return req:headers()
end)

-- Run the server
app:run()
