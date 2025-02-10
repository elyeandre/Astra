--!nocheck
---@diagnostic disable: duplicate-set-field

_G.utils = require("./libs/utils.lua")
_G.json = require("./libs/json.lua")
_G.validate_table = require("./libs/table_schema.lua")
_G.import = require("./libs/import.lua")

-- MARK: Load envs

---@type fun(file_path: string)
---@diagnostic disable-next-line: undefined-global
_G.dotenv_load = dotenv_load

_G.ENV = {}
dotenv_load(".env")
dotenv_load(".env.production")
dotenv_load(".env.prod")
dotenv_load(".env.development")
dotenv_load(".env.dev")
dotenv_load(".env.test")
dotenv_load(".env.local")

-- MARK: Astra

_G.Astra = {
	version = "0.0.0",
	hostname = "127.0.0.1",
	compression = true,
	port = 20001,
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
---@param file_path string | nil
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

---@class FileType
---@field is_file fun(file_type: FileType): boolean
---@field is_dir fun(file_type: FileType): boolean
---@field is_symlink fun(file_type: FileType): boolean

---@class DirEntry
---@field file_name fun(dir_entry: DirEntry): string Returns the file_name of the entry
---@field file_type fun(dir_entry: DirEntry): FileType
---@field path fun(dir_entry: DirEntry): string Returns the path of each entry in the list

--- @START_REMOVING_RUNTIME
_G.AstraIO = {
	---Returns the content of the directory
	---@param path string Path to the file
	---@return DirEntry[]
	read_dir = function(path)
		return {}
	end,

	---Returns the path of the current directory
	---@return string
	get_current_dir = function()
		return ""
	end,

	---Returns the path of the current running script
	---@return string
	get_script_path = function()
		return ""
	end,
}
--- @END_REMOVING_RUNTIME
