-- import for intellisense. This line is replaced
-- by the bundle code during runtime
import("../src/lua/astra_bundle")

local server = Astra.http.server:new()

-- A simple GET index route with text return
server:get("/", function()
	return "hello from default Astra instance! " .. Astra.version
end)

-- You can also use the local variables within routes
local counter = 0
server:get("/count", function()
	counter = counter + 1
	-- and also can return JSON
	return { counter }
end)

-- The request parameter is optional but contains useful information
server:get("/headers", function(req)
	return req:headers()
end)

-- Run the server
server:run()
