Astra.http.server = {}

---@diagnostic disable-next-line: duplicate-doc-alias
---@alias callback fun(request: HTTPServerRequest, response: HTTPServerResponse): any

---@class HTTPRouteConfiguration
---@field body_limit? number

---Represents an HTTP body.
---@class Body
---@field text fun(): string Returns the body as text
---@field json fun(): table Returns the body parsed as JSON -> Lua Table

---Represents an HTTP request.
---@class Request
---@field method     fun(request: Request): string Returns the HTTP method (e.g., "GET", "POST").
---@field uri        fun(request: Request): string Returns the URI of the request.
---@field queries    fun(request: Request): table Returns the query list.
---@field headers    fun(request: Request): table Returns a table containing the headers of the request.
---@field body       fun(request: Request): Body|nil Returns the body of the request, which can be a table or a string.
---@field multipart  fun(request: Request): HTTPMultipart|nil Returns a multipart if available.
---@field get_cookie fun(request: Request, name: string): Cookie Returns a cookie
---@field new_cookie fun(request: Request, name: string, value: string): Cookie Returns a cookie

---Represents an HTTP response.
---@class Response
---@field set_status_code fun(response: Response, new_status_code: number) Sets the HTTP status code of the response
---@field set_header      fun(response: Response, key: string, value: string) Sets a header
---@field get_headers     fun(response: Response): table|nil Returns the entire headers list that so far has been set for the response
---@field remove_header   fun(response: Response, key: string) Removes a header from the headers list
---@field set_cookie      fun(response: Response, cookie: Cookie) Sets a cookie
---@field remove_cookie   fun(response: Response, cookie: Cookie) Removes a cookie from the list


---@class HTTPRoute
---@field path string
---@field method string
---@field func function
---@field static_dir string?
---@field static_file string?
---@field config HTTPRouteConfiguration?

---@class HTTPMultipart
---@field save_file fun(multipart: HTTPMultipart, file_path: string | nil): string | nil Saves the multipart into disk

---@class HTTPServerRequest
---@field method fun(request: HTTPServerRequest): string Returns the HTTP method (e.g., "GET", "POST").
---@field uri fun(request: HTTPServerRequest): string Returns the URI of the request.
---@field queries fun(request: HTTPServerRequest): table Returns the query list.
---@field headers fun(request: HTTPServerRequest): table Returns a table containing the headers of the request.
---@field body fun(request: HTTPServerRequest): HTTPBody|nil Returns the body of the request, which can be a table or a string.
---@field multipart fun(request: HTTPServerRequest): HTTPMultipart|nil Returns a multipart if available.
---@field get_cookie fun(request: HTTPServerRequest, name: string): Cookie Returns a cookie
---@field new_cookie fun(request: HTTPServerRequest, name: string, value: string): Cookie Returns a cookie

---@class HTTPServerResponse
---@field set_status_code fun(response: HTTPServerResponse, new_status_code: number) Sets the HTTP status code of the response
---@field set_header fun(response: HTTPServerResponse, key: string, value: string) Sets a header
---@field get_headers fun(response: HTTPServerResponse): table|nil Returns the entire headers list that so far has been set for the response
---@field remove_header fun(response: HTTPServerResponse, key: string) Removes a header from the headers list
---@field set_cookie fun(response: HTTPServerResponse, cookie: Cookie) Sets a cookie
---@field remove_cookie fun(response: HTTPServerResponse, cookie: Cookie) Removes a cookie from the list


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
