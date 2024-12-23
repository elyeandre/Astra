--[[
    THIS IS PURELY FOR TESTING, DO NOT USE AS EXAMPLE!
]]

require("../lua/astra_bundle")

local db = database_connect("postgres://username:password@localhost/database_name")
--print(db:name())

Astra.get("/", function(req)
    return "hello from default Astra instance! " .. Astra.version
end)