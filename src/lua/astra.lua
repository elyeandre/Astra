---@diagnostic disable: duplicate-doc-field

---============================ TYPES ============================---

-- MARK: HTTPServer

---@class HTTPServer
---@field version string
---@field hostname string
---@field compression boolean
---@field port number
---@field routes Route[]
---@field __index HTTPServer
--- methods
---@field new fun(server: HTTPServer): HTTPServer
---@field get fun(server: HTTPServer, path: string, callback: callback, config: RouteConfiguration?)
---@field post fun(server: HTTPServer, path: string, callback: callback, config: RouteConfiguration?)
---@field put fun(server: HTTPServer, path: string, callback: callback, config: RouteConfiguration?)
---@field delete fun(server: HTTPServer, path: string, callback: callback, config: RouteConfiguration?)
---@field options fun(server: HTTPServer, path: string, callback: callback, config: RouteConfiguration?)
---@field patch fun(server: HTTPServer, path: string, callback: callback, config: RouteConfiguration?)
---@field trace fun(server: HTTPServer, path: string, callback: callback, config: RouteConfiguration?)
---@field static_dir fun(server: HTTPServer, serve_path: string, callback: callback, config: RouteConfiguration?)
---@field static_file fun(server: HTTPServer, serve_path: string, callback: callback, config: RouteConfiguration?)
---@field run fun(server: HTTPServer) Runs the server

---@diagnostic disable-next-line: duplicate-doc-alias
---@alias callback fun(request: Request, response: Response): any

---@class RouteConfiguration
---@field body_limit? number

---@class Route
---@field path string
---@field method string
---@field func function
---@field static_dir string?
---@field static_file string?
---@field config RouteConfiguration?

-- MARK: Common HTTP
---
--- Represents an HTTP body.
---@class Body
---@field text fun(): string Returns the body as text
---@field json fun(): table Returns the body parsed as JSON -> Lua Table

---
--- Represents a multipart.
---@class Multipart
---@field save_file fun(multipart: Multipart, file_path: string | nil): string | nil Saves the multipart into disk

---
--- Represents an HTTP request.
---@class Request
---@field method fun(request: Request): string Returns the HTTP method (e.g., "GET", "POST").
---@field uri fun(request: Request): string Returns the URI of the request.
---@field queries fun(request: Request): table Returns the query list.
---@field headers fun(request: Request): table Returns a table containing the headers of the request.
---@field body fun(request: Request): Body|nil Returns the body of the request, which can be a table or a string.
---@field multipart fun(request: Request): Multipart|nil Returns a multipart if available.
---@field get_cookie fun(request: Request, name: string): Cookie Returns a cookie
---@field new_cookie fun(request: Request, name: string, value: string): Cookie Returns a cookie

---
--- Represents an HTTP response.
---@class Response
---@field set_status_code fun(response: Response, new_status_code: number) Sets the HTTP status code of the response
---@field set_header fun(response: Response, key: string, value: string) Sets a header
---@field get_headers fun(response: Response): table|nil Returns the entire headers list that so far has been set for the response
---@field remove_header fun(response: Response, key: string) Removes a header from the headers list
---@field set_cookie fun(response: Response, cookie: Cookie) Sets a cookie
---@field remove_cookie fun(response: Response, cookie: Cookie) Removes a cookie from the list

-- MARK: FileIO

---@class AstraIO
---@field get_metadata fun(path: string): FileMetadata Returns the metadata of a file or directory
---@field read_dir fun(path: string): DirEntry[] Returns the content of the directory
---@field get_current_dir fun(): string Returns the path of the current directory
---@field get_script_path fun(): string Returns the path of the current running script
---@field change_dir fun(path: string) Changes the current directory
---@field exists fun(path: string): boolean Checks if a path exists
---@field create_dir fun(path: string) Creates a directory
---@field create_dir_all fun(path: string) Creates all directories in the path if they do not exist
---@field remove fun(path: string) Removes a file
---@field remove_dir fun(path: string) Removes a directory and all its contents
---@field remove_dir_all fun(path: string) Removes all directories in the path

---@class FileType
---@field is_file fun(file_type: FileType): boolean
---@field is_dir fun(file_type: FileType): boolean
---@field is_symlink fun(file_type: FileType): boolean

---@class DirEntry
---@field file_name fun(dir_entry: DirEntry): string Returns the file_name of the entry
---@field file_type fun(dir_entry: DirEntry): FileType
---@field path fun(dir_entry: DirEntry): string Returns the path of each entry in the list

---@class FileMetadata
---@field last_accessed fun(file_metadata: FileMetadata): number
---@field created_at fun(file_metadata: FileMetadata): number
---@field last_modified fun(file_metadata: FileMetadata): number
---@field file_type fun(file_metadata: FileMetadata): FileType
---@field file_permissions fun(file_metadata: FileMetadata): FileIOPermissions

---@class FileIOPermissions
---@field is_readonly fun(file_io_permissions: FileIOPermissions): boolean
---@field set_readonly fun(file_io_permissions: FileIOPermissions, value: boolean)

-- MARK: Cookie

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

---============================ DEFINITIONS ============================---

-- The main global
_G.Astra = {
	http = {},
	utils = {},
}

-- Imports
_G.Astra.utils = require("./libs/utils.lua")
_G.Astra.validate_table = require("./libs/table_schema.lua")
_G.import = require("./libs/import.lua")

---@type fun(file_path: string)
---@diagnostic disable-next-line: undefined-global
_G.Astra.dotenv_load = astra_internal__dotenv_load
_G.Astra.dotenv_load(".env")
_G.Astra.dotenv_load(".env.production")
_G.Astra.dotenv_load(".env.prod")
_G.Astra.dotenv_load(".env.development")
_G.Astra.dotenv_load(".env.dev")
_G.Astra.dotenv_load(".env.test")
_G.Astra.dotenv_load(".env.local")

---@diagnostic disable-next-line: undefined-global
os.getenv = astra_internal__getenv
---Sets the environment variable.
---
---NOT SAFE WHEN USED IN MULTITHREADING ENVIRONMENT
---@diagnostic disable-next-line: undefined-global
os.setenv = astra_internal__setenv

---@type HTTPServer
---@diagnostic disable-next-line: missing-fields
local Server = {}
_G.Astra.http.server = Server

function Server:new()
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
	server:register_methods()
	return server
end

---@diagnostic disable-next-line: inject-field
function Server:register_methods()
	local http_methods = { "get", "post", "put", "delete", "options", "patch", "trace" }

	for _, method in ipairs(http_methods) do
		self[method] = function(_, path, callback, config)
			table.insert(self.routes, {
				path = path,
				method = method,
				func = callback,
				config = config or {},
			})
		end
	end

	self.static_dir = function(_, path, serve_path, config)
		table.insert(self.routes, {
			path = path,
			method = "static_dir",
			func = function() end,
			static_dir = serve_path,
			config = config or {},
		})
	end

	self.static_file = function(_, path, serve_path, config)
		table.insert(self.routes, {
			path = path,
			method = "static_file",
			func = function() end,
			static_file = serve_path,
			config = config or {},
		})
	end

	self.run = function (_)
		---@diagnostic disable-next-line: undefined-global
		astra_internal__start_server(self)
	end
end

_G.Astra.io = {}

-- This is to prevent a small undefined behavior in Lua
---@diagnostic disable-next-line: redundant-parameter
setmetatable(_G, {
	---@diagnostic disable-next-line: redundant-parameter, unused-local
	__index = function(T, k, v)
		error("Called non-existing variable")
	end,
})
