local server = Astra.http.server:new()
local middleware = Astra.http.middleware

local context = middleware.context
local console_logger = middleware.console_logger
local file_logger = middleware.file_logger
local chain = middleware.chain
local html = middleware.html


local function homepage()
    return "Hi there!"
end

--- `on Entry:`
--- Inserts `Astra.datetime.new_utc_now()` into `ctx.datetime`
---
--- `Depends on:`
--- `context`
local function insert_datetime(next_handler)
    ---@param request HTTPServerRequest
    ---@param response HTTPServerResponse
    return function(request, response, ctx)
        ctx.datetime = Astra.datetime.new_utc_now()
        local result = next_handler(request, response, ctx)
        return result
    end
end

---@param ctx { datetime: DateTime }
local function favourite_day(_, _, ctx)
    local today = string.format("%d/%d/%d", ctx.datetime:get_day(), ctx.datetime:get_month(), ctx.datetime:get_year())
    return "My favourite day is " .. today
end

local file_handler, err = io.open("logs.txt", "a")
if not file_handler then
    error(err)
end

local long_chain = chain { context, file_logger(file_handler), insert_datetime, html }

server:get("/", chain { console_logger, html } (homepage))
server:get("/fn", long_chain(favourite_day))

server:run()
