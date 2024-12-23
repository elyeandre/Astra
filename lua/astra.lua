---@diagnostic disable: duplicate-set-field

require "./libs/utils.lua"
require "./libs/json.lua"

-- MARK: Astra

_G.Astra = {
    version = "0.0.0",
    hostname = "127.0.0.1",
    port = 20001
}

---
---Registers a GET request to the specified path with the provided callback function.
---@param path string The URL path for the request.
---@param callback fun(request: Request): any A function that will be called when the request is made.
function Astra.get(path, callback)
    table.insert(Astra, { path = path, method = "get", func = callback })
end

---
---Registers a POST request to the specified path with the provided callback function.
---@param path string The URL path for the request.
---@param callback fun(request: Request): any A function that will be called when the request is made.
function Astra.post(path, callback)
    table.insert(Astra, { path = path, method = "post", func = callback })
end

---
---Registers a PUT request to the specified path with the provided callback function.
---@param path string The URL path for the request.
---@param callback fun(request: Request): any A function that will be called when the request is made.
function Astra.put(path, callback)
    table.insert(Astra, { path = path, method = "put", func = callback })
end

---
---Registers a DELETE request to the specified path with the provided callback function.
---@param path string The URL path for the request.
---@param callback fun(request: Request): any A function that will be called when the request is made.
function Astra.delete(path, callback)
    table.insert(Astra, { path = path, method = "delete", func = callback })
end

-- MARK: Internal

---
--- Represents an HTTP request.
---@class Request
---@field method fun(): string Returns the HTTP method (e.g., "GET", "POST").
---@field uri fun(): string Returns the URI of the request.
---@field headers fun(): table Returns a table containing the headers of the request.
---@field body fun(): string|nil Returns the body of the request, which can be a table or a string.

---
---Opens a new PostgreSQL connection using the provided URL and returns a table representing the connection.
---@param url string The URL of the PostgreSQL database to connect to.
---@return table Database that represents the PostgreSQL connection.
---@nodiscard
---@diagnostic disable-next-line: missing-return, lowercase-global
function database_connect(url) end

_G.Database = {}
