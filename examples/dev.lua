--[[
    THIS IS PURELY FOR TESTING, DO NOT USE AS EXAMPLE!
]]

require("../src/lua/astra")

-- local db = database_connect("postgres://astra_postgres:password@localhost/astr_database")
-- db:execute("CREATE TABLE IF NOT EXISTS test (id SERIAL PRIMARY KEY, name TEXT)", {});

-- http_request("https://myip.wtf/json"):execute_task(function (response)
--     pretty_print(response:status_code())
--     pretty_print(response:headers())
--     pretty_print(response:remote_address())
--     pretty_print(response:body():json())
-- end)

-- local task_id = spawn_interval(function ()
--     print("HOOOOO");
-- end, 2000)
-- print("YEE HAW");

-- spawn_timeout(function ()
--     task_id:abort()
--     task_id:abort()
-- end, 5000)

-- print(ENV["SOMETHING"])


Astra.get("/", function(req, res)
    res:set_status_code(300)
    res:set_header("test", "VALLLL")

    pretty_print(req:body():text())
    -- local result = db:query_one("SELECT * FROM test;", {});
    -- print(utils.pretty_table(result))

    return "hello from default Astra instance! " .. Astra.version
end)

Astra.post("/", function(req)
    req:multipart():save_file()

    -- local result = db:query_one("SELECT * FROM test;", {});
    -- print(utils.pretty_table(result))

    return "hello from default Astra instance! " .. Astra.version
end)

Astra.get("/test/{name}/{id}", function(request, response)
    pretty_print(request:uri())
end)

-- Astra.get("/insert", function(req)
--     -- local queries = utils.parseurl(req:uri())
--     -- local result = db:query_all("INSERT INTO test (name) VALUES ($1)", {queries.name});
--     -- print(utils.pretty_table(result))

--     return "Successfully inserted name: " .. "queries.name"
-- end)

Astra.static_file("/asd", "examples")

Astra.run()
