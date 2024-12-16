_G.Astra = {}

function get_request(path, func)
    table.insert(Astra, { path = path, method = "get", func = func })
end

get_request("/", function()
    return "hello from default Astra instance!"
end)


get_request("/test", function()
    return { key = 123 }
end)
