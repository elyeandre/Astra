--!strict

-- local utils = require("libs/utils.luau")
-- local json = require("libs/json.luau")
-- local table_schema = require("libs/table_schema.luau")

_G.Astra = {}
_G.Astra.__index = _G.Astra
function _G.Astra.new()
	print("HASDKNASD")
	local self = setmetatable({}, _G.Astra)

	self.version = "0.0.0"
	self.hostname = "127.0.0.1"
	self.compression = true
	self.port = 20001
	self.routes = {}

	return self
end

--- @param path string The URL path for the request.
--- @param callback callback A function that will be called when the request is made.
function _G.Astra:get(path: string, callback: Callback)
	print("HEHE")
	table.insert(self.routes, { path = path, method = "get", func = callback })
end

--- @param path string The URL path for the request.
--- @param callback callback A function that will be called when the request is made.
function _G.Astra:post(path: string, callback: Callback)
	table.insert(self.routes, { path = path, method = "post", func = callback })
end

--- @param path string The URL path for the request.
--- @param callback callback A function that will be called when the request is made.
function _G.Astra:put(path: string, callback: Callback)
	table.insert(self.routes, { path = path, method = "put", func = callback })
end

--- @param path string The URL path for the request.
--- @param callback callback A function that will be called when the request is made.
function _G.Astra:delete(path: string, callback: Callback)
	table.insert(self.routes, { path = path, method = "delete", func = callback })
end

--- @param path string The URL path for the request.
--- @param callback callback A function that will be called when the request is made.
function _G.Astra:options(path: string, callback: Callback)
	table.insert(self.routes, { path = path, method = "options", func = callback })
end

--- @param path string The URL path for the request.
--- @param callback callback A function that will be called when the request is made.
function _G.Astra:patch(path: string, callback: Callback)
	table.insert(self.routes, { path = path, method = "patch", func = callback })
end

--- @param path string The URL path for the request.
--- @param callback callback A function that will be called when the request is made.
function _G.Astra:trace(path: string, callback: Callback)
	table.insert(self.routes, { path = path, method = "trace", func = callback })
end

--- Registers a static folder to serve
--- @param serve_path string The directory path relatively
--- @param path string The URL path for the request.
function _G.Astra:static_dir(path: string, serve_path: string)
	table.insert(self.routes, {
		path = path,
		method = "static_dir",
		func = function()
			return nil
		end,
		static_dir = serve_path,
	})
end

--- Registers a static file to serve
--- @param serve_path string The file path relatively
--- @param path string The URL path for the request.
function _G.Astra:static_file(path: string, serve_path: string)
	table.insert(self.routes, {
		path = path,
		method = "static_file",
		func = function()
			return nil
		end,
		static_file = serve_path,
	})
end

--- @START_REMOVING_RUNTIME
local astra_internal__start_server = function()
	print("Function exists and is called!")
end
--- @END_REMOVING_RUNTIME

--- Runs the _G.Astra server
function _G.Astra:run()
	astra_internal__start_server()
end

type RouteDetail = {
	path: string,
	method: string,
	func: Callback,
	static_dir: string?,
	static_file: string?,
}

type LogicRouteFunction = (string, Callback) -> ()
type StaticRouteFunction = (string, string) -> ()

type Callback = (request: Request, response: Response) -> any

--- Represents an HTTP body.
type Body = {
	--- Returns the body as text
	text: (body: Body) -> string,
	--- Returns the body parsed as JSON -> Lua Table
	json: (body: Body) -> { [any]: any },
}

--- Represents a multipart.
type Multipart = {
	save_file: (multipart: Multipart, file_path: string?) -> nil,
}

type Headers = { [string]: string }

--- Represents an HTTP request.
type Request = {
	--- Returns the HTTP method (e.g., "GET", "POST").
	method: (request: Request) -> string,
	--- Returns the URI of the request.
	uri: (request: Request) -> string,
	--- Returns a table containing the headers of the request.
	headers: (request: Request) -> Headers,
	--- Returns the body of the request, which can be a table or a string.
	body: (request: Request) -> Body?,
	--- Returns a multipart if available.
	multipart: (request: Request) -> Multipart?,
}

type Response = {
	--- Sets the HTTP status code of the response
	set_status_code: (response: Response, new_status_code: number) -> nil,
	--- Sets a header
	set_header: (response: Response, key: string, value: string) -> nil,
	--- Returns the entire headers list that so far has been set for the response
	get_headers: (response: Response) -> Headers?,
	--- Removes a header from the headers list
	remove_header: (response: Response, key: string) -> nil,
}

--- @START_REMOVING_RUNTIME
return {}
--- @END_REMOVING_RUNTIME
