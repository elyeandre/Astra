---@diagnostic disable: duplicate-set-field

_G.utils = require "./libs/utils.lua"
_G.json = require "./libs/json.lua"
_G.validate_table = require "./libs/table_schema.lua"

-- TODO: include http status codes as enum

-- Load envs
local dotenv = require "./libs/dotenv.lua"
dotenv:load(".env")
dotenv:load(".env.production")
dotenv:load(".env.development")
dotenv:load(".env.test")
dotenv:load(".env.local")

-- MARK: Astra

_G.Astra = {
    version = "0.0.0",
    hostname = "127.0.0.1",
    compression = true,
    port = 20001
}

---@diagnostic disable-next-line: duplicate-doc-alias
---@alias callback fun(request: Request, response: Response): any

---@param path string The URL path for the request.
---@param callback callback A function that will be called when the request is made.
function Astra.get(path, callback)
    table.insert(Astra, { path = path, method = "get", func = callback })
end

---@param path string The URL path for the request.
---@param callback callback A function that will be called when the request is made.
function Astra.post(path, callback)
    table.insert(Astra, { path = path, method = "post", func = callback })
end

---@param path string The URL path for the request.
---@param callback callback A function that will be called when the request is made.
function Astra.put(path, callback)
    table.insert(Astra, { path = path, method = "put", func = callback })
end

---@param path string The URL path for the request.
---@param callback callback A function that will be called when the request is made.
function Astra.delete(path, callback)
    table.insert(Astra, { path = path, method = "delete", func = callback })
end

---@param path string The URL path for the request.
---@param callback callback A function that will be called when the request is made.
function Astra.options(path, callback)
    table.insert(Astra, { path = path, method = "options", func = callback })
end

---@param path string The URL path for the request.
---@param callback callback A function that will be called when the request is made.
function Astra.patch(path, callback)
    table.insert(Astra, { path = path, method = "patch", func = callback })
end

---@param path string The URL path for the request.
---@param callback callback A function that will be called when the request is made.
function Astra.trace(path, callback)
    table.insert(Astra, { path = path, method = "trace", func = callback })
end

---
---Registers a static folder to serve
---@param path string The URL path for the request.
---@param serve_path string The directory path relatively
function Astra.static_dir(path, serve_path)
    table.insert(Astra, { path = path, method = "static_dir", func = function() end, static_dir = serve_path })
end

---
---Registers a static file to serve
---@param path string The URL path for the request.
---@param serve_path string The directory path relatively
function Astra.static_file(path, serve_path)
    table.insert(Astra, { path = path, method = "static_file", func = function() end, static_file = serve_path })
end

---
---Runs the Astra server
function Astra.run()
    ---@diagnostic disable-next-line: undefined-global
    astra_internal__start_server()
end

-- MARK: Internal

---
--- Represents an HTTP body.
---@class Body
---@field text fun(): string Returns the body as text
---@field json fun(): table Returns the body parsed as JSON -> Lua Table

---
--- Represents a multipart.
---@class Multipart
_G.Multipart = {}
---
---Saves the multipart into disk
---@param file_path string
function Multipart:save_file(file_path) end

---
--- Represents an HTTP request.
---@class Request
---@field method fun(): string Returns the HTTP method (e.g., "GET", "POST").
---@field uri fun(): string Returns the URI of the request.
---@field headers fun(): table Returns a table containing the headers of the request.
---@field body fun(): Body|nil Returns the body of the request, which can be a table or a string.
---@field multipart fun(): Multipart|nil Returns a multipart if available.

---
--- Represents an HTTP response.
---@class Response
---@field set_status_code fun(response: Response, new_status_code: number) Sets the HTTP status code of the response
---@field set_header fun(response: Response, key: string, value: string) Sets a header
---@field get_headers fun(): table|nil Returns the entire headers list that so far has been set for the response
---@field remove_header fun(response: Response, key: string) Removes a header from the headers list

---
--- SQLx driver for PostgreSQL
---@class Database
_G.Database = {}

---@param sql string The SQL query to execute.
---@param parameters table Optional table containing the parameters to bind to the query.
function Database:execute(sql, parameters) end

---
---@param sql string The SQL query to execute that returns one row.
---@param parameters table Optional table containing the parameters to bind to the query.
---@return table|nil row a table representing the result row if successful, or `nil` on failure.
function Database:query_one(sql, parameters) end

---
---@param sql string The SQL query to execute that returns multiple rows.
---@param parameters table Optional table containing the parameters to bind to the query.
---@return table|nil rows a table containing all result rows if successful, or `nil` on failure.
function Database:query_all(sql, parameters) end

---
---Opens a new PostgreSQL connection using the provided URL and returns a table representing the connection.
---@param url string The URL of the PostgreSQL database to connect to.
---@return Database Database that represents the PostgreSQL connection.
---@nodiscard
---@diagnostic disable-next-line: missing-return, lowercase-global
function database_connect(url) end

---
--- Represents an HTTP client response.
---@class HTTPClientResponse
---@field status_code fun(): table Gets the response HTTP Status code
---@field body fun(): Body Gets the response HTTP Body which further can be parsed
---@field headers fun(): table|nil Returns the entire headers list from the HTTP response
---@field remote_address fun(): string|nil Gets the remote address of the HTTP response server

---@diagnostic disable-next-line: duplicate-doc-alias
---@alias http_client_callback fun(response: HTTPClientResponse)

---
--- Represents an HTTP client request.
---@class HTTPClientRequest
---@field set_method fun(http_request: HTTPClientRequest, method: string): HTTPClientRequest Sets the HTTP method
---@field set_header fun(http_request: HTTPClientRequest, key: string, value: string): HTTPClientRequest Sets a header
---@field set_headers fun(http_request: HTTPClientRequest, headers: table): HTTPClientRequest Sets all of the headers
---@field set_form fun(http_request: HTTPClientRequest, key: string, value: string): HTTPClientRequest Sets a form
---@field set_forms fun(http_request: HTTPClientRequest, headers: table): HTTPClientRequest Sets all of the forms
---@field set_body fun(http_request: HTTPClientRequest, body: string): HTTPClientRequest Sets the HTTP body
---@field set_json fun(http_request: HTTPClientRequest, json: table): HTTPClientRequest Sets the HTTP json
---@field execute fun(): HTTPClientResponse Executes the request and returns the response
---@field execute_task fun(http_request: HTTPClientRequest, callback: http_client_callback) Executes the request as an async task

---
---Opens a new async HTTP Request. The request is running as a task in parallel
---@param url string
---@return HTTPClientRequest
---@nodiscard
---@diagnostic disable-next-line: missing-return, lowercase-global
function http_request(url) end

---
--- Represents an async task
---@class TaskHandler
---@field abort fun() Aborts the running task

---
---Starts a new async task
---@param callback fun() | nil The callback to run the content of the async task
---@return TaskHandler
---@diagnostic disable-next-line: missing-return, lowercase-global
function spawn_task(callback) end

---
---Starts a new async task with a delay in milliseconds
---@param callback fun() | nil The callback to run the content of the async task
---@param timeout number The delay in milliseconds
---@return TaskHandler
---@diagnostic disable-next-line: missing-return, lowercase-global
function spawn_timeout(callback, timeout) end

---
---Starts a new async task that runs infinitely in a loop but with a delay in milliseconds
---@param callback fun() | nil The callback to run the content of the async task
---@param timeout number The delay in milliseconds
---@return TaskHandler
---@diagnostic disable-next-line: missing-return, lowercase-global
function spawn_interval(callback, timeout) end
