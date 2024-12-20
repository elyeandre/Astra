require "./libs/utils.lua"

_G.Astra = {}

function get_request(path, callback)
    table.insert(_G.Astra, { path = path, method = "get", func = callback })
end
