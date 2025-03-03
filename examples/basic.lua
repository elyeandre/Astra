-- import for intellisense. This line is replaced
-- by the bundle code during runtime
require("../src/lua/astra_bundle")


-- A simple GET index route with text return
Astra:get("/", function()
    return "hello from default Astra instance! " .. Astra.version
end)

-- You can also use the local variables within routes
local counter = 0
Astra:get("/count", function()
    counter = counter + 1
    -- and also can return JSON
    return { counter }
end)

-- The request parameter is optional but contains useful information
Astra:get("/headers", function(req)
    return req:headers()
end)

-- Run the server
Astra:run()
