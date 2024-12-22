require("../lua/astra_bundle")

local counter = 0

Astra.get_request("/", function(req)
    print(Astra.version)

    return "hello from default Astra instance!"
end)

Astra.get_request("/count", function()
    counter = counter + 1
    return { counter }
end)

Astra.post_request("/test", function()
    return { key = 123 }
end)
