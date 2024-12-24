--[[
    THIS IS PURELY FOR TESTING, DO NOT USE AS EXAMPLE!
]]

require("../lua/astra_bundle")

local db = database_connect("postgres://astra_postgres:password@localhost/astr_database")
print("ASDASDAS")
print(db)
db:execute("CREATE TABLE IF NOT EXISTS test (id SERIAL PRIMARY KEY, name TEXT)", {});

Astra.get("/", function(req)
    local result = db:query_one("SELECT * FROM test;", {});
    print(utils.pretty_table(result))

    return "hello from default Astra instance! " .. Astra.version
end)

Astra.get("/insert", function(req)
    local queries = utils.parseurl(req:uri())
    local result = db:query_all("INSERT INTO test (name) VALUES ($1)", {queries.name});
    print(result)
    print(utils.pretty_table(result))

    return "hello from default Astra instance! " .. Astra.version
end)