require("../lua/astra.bundle.lua")

get_request("/", function(req)
    print(pretty_table(req))
    return "hello from default Astra instance!"
end)


get_request("/test", function()
    return { key = 123 }
end)
