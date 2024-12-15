_G.Astra = {}

function get_request(path, returns, func)
    table.insert(Astra, { path = path, method = "get", returns = returns or "plain", func = func })
end

get_request("/", "plain", function()
    print("hello from default Astra instance!")
end)


get_request("/test", "plain", function()
    print("YEEE HAW")
end)