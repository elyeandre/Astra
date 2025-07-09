---@meta

---@class HTTPBody
---@field text fun(): string
---@field json fun(): table Returns the body parsed as JSON -> Lua Table

Astra.http = {}

Astra.http.server = {}


---@class Cookie
---@field set_name fun(cookie: Cookie, name: string)
---@field set_value fun(cookie: Cookie, value: string)
---@field set_domain fun(cookie: Cookie, domain: string)
---@field set_path fun(cookie: Cookie, path: string)
---@field set_expiration fun(cookie: Cookie, expiration: number)
---@field set_http_only fun(cookie: Cookie, http_only: boolean)
---@field set_max_age fun(cookie: Cookie, max_age: number)
---@field set_permanent fun(cookie: Cookie)
---@field get_name fun(cookie: Cookie): string?
---@field get_value fun(cookie: Cookie): string?
---@field get_domain fun(cookie: Cookie): string?
---@field get_path fun(cookie: Cookie): string?
---@field get_expiration fun(cookie: Cookie): number?
---@field get_http_only fun(cookie: Cookie): boolean?
---@field get_max_age fun(cookie: Cookie): number?

----------------------------------------------------------------------------------------
----------------------------------------------------------------------------------------
----------------------------------------------------------------------------------------

---@class HTTPServer
---@diagnostic disable-next-line: missing-fields
local HTTPServer = {
	version = "0.0.0",
	hostname = "127.0.0.1",
	--- Enable or disable compression
	compression = false,
	port = 8080,
	--- Contains all of the route details
	routes = {},
}
function HTTPServer:new()
	local server = {
		version = "0.0.0",
		hostname = "127.0.0.1",
		--- Enable or disable compression
		compression = false,
		port = 8080,
		--- Contains all of the route details
		routes = {},
	}

	setmetatable(server, self)
	self.__index = self
	return server
end

---@return HTTPServer
function Astra.http.server.new()
	return HTTPServer:new()
end

local function add_to_routes(server, method, path, callback, config)
	local index = (path == "/") and 1 or #server.routes + 1
	table.insert(server.routes, index, {
		path = path,
		method = method,
		func = callback,
		config = config or {},
	})
end

---@param path string
---@param callback callback
---@param config HTTPRouteConfiguration?
function HTTPServer:get(path, callback, config)
	add_to_routes(self, "get", path, callback, config)
end

---@param path string
---@param callback callback
---@param config HTTPRouteConfiguration?
function HTTPServer:post(path, callback, config)
	add_to_routes(self, "post", path, callback, config)
end

---@param path string
---@param callback callback
---@param config HTTPRouteConfiguration?
function HTTPServer:put(path, callback, config)
	add_to_routes(self, "put", path, callback, config)
end

---@param path string
---@param callback callback
---@param config HTTPRouteConfiguration?
function HTTPServer:delete(path, callback, config)
	add_to_routes(self, "delete", path, callback, config)
end

---@param path string
---@param callback callback
---@param config HTTPRouteConfiguration?
function HTTPServer:options(path, callback, config)
	add_to_routes(self, "options", path, callback, config)
end

---@param path string
---@param callback callback
---@param config HTTPRouteConfiguration?
function HTTPServer:patch(path, callback, config)
	add_to_routes(self, "patch", path, callback, config)
end

---@param path string
---@param callback callback
---@param config HTTPRouteConfiguration?
function HTTPServer:trace(path, callback, config)
	add_to_routes(self, "trace", path, callback, config)
end

---@param path string
---@param serve_path string
---@param config HTTPRouteConfiguration?
function HTTPServer:static_dir(path, serve_path, config)
	table.insert(self.routes, {
		path = path,
		method = "static_dir",
		func = function() end,
		static_dir = serve_path,
		config = config or {},
	})
end

---@param path string
---@param serve_path string
---@param config HTTPRouteConfiguration?
function HTTPServer:static_file(path, serve_path, config)
	table.insert(self.routes, {
		path = path,
		method = "static_file",
		func = function() end,
		static_file = serve_path,
		config = config or {},
	})
end

---Runs the server
function HTTPServer:run()
	---@diagnostic disable-next-line: undefined-global
	astra_internal__start_server(self)
end

---Opens a new async HTTP Request. The request is running as a task in parallel
---@param url string
---@return HTTPClientRequest
---@nodiscard
---@diagnostic disable-next-line: missing-return, lowercase-global
function Astra.http.request(url)
	---@diagnostic disable-next-line: undefined-global
	return astra_internal__http_request(url)
end
