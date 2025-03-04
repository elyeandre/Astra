---@diagnostic disable: duplicate-set-field, lowercase-global

__luapack_modules__ = {
    (function()
        --!nocheck
        
        local utils = { _version = "0.1.0" }
        
        ---Pretty prints any table or value
        ---@param value any
        ---@diagnostic disable-next-line: duplicate-set-field
        function _G.pretty_print(value)
            ---@diagnostic disable-next-line: undefined-global
            astra_internal__pretty_print(value)
        end
        
        _G.json = {
            ---Encodes the value into a valid JSON string
            ---@param value any
            ---@return string
            ---@diagnostic disable-next-line: duplicate-set-field
            encode = function (value)
                ---@diagnostic disable-next-line: undefined-global
                return astra_internal__json_encode(value)
            end,
        
            ---Decodes the JSON string into a valid lua value
            ---@param value string
            ---@return any
            ---@diagnostic disable-next-line: duplicate-set-field
            decode = function (value)
                ---@diagnostic disable-next-line: undefined-global
                return astra_internal__json_decode(value)
            end
        }
        
        ---
        ---Splits a sentence into an array given the separator
        ---@param input_str string The input string
        ---@param separator_str string The input string
        ---@return table array
        ---@nodiscard
        ---@diagnostic disable-next-line: duplicate-set-field
        function string.split(input_str, separator_str)
            local result_table = {}
            for word in input_str:gmatch("([^" .. separator_str .. "]+)") do
                table.insert(result_table, word)
            end
            return result_table
        end
        
        return utils
    
    end),
    (function()
        ---
        ---Schema validation function with support for nested tables and arrays of tables
        ---@param input_table table
        ---@param schema table
        ---@return boolean, string | nil
        local function validate_table(input_table, schema)
            -- Helper function to check if a value is of the expected type
            local function check_type(value, expected_type)
                local type_map = {
                    number = "number",
                    string = "string",
                    boolean = "boolean",
                    table = "table",
                    ["function"] = "function",
                    ["nil"] = "nil",
                    array = "table"
                }
                return type(value) == type_map[expected_type]
            end
        
            -- Helper function to check if a value is within a range (if applicable)
            local function check_range(value, min, max)
                return not (min and value < min) and not (max and value > max)
            end
        
            -- Helper function to validate nested tables
            local function validate_nested_table(value, nested_schema, path)
                local is_valid, err = validate_table(value, nested_schema)
                if not is_valid then
                    return false, path .. ": " .. err
                end
                return true
            end
        
            -- Helper function to validate arrays of tables
            local function validate_array_of_tables(value, array_schema, path)
                if type(value) ~= "table" then
                    return false, path .. ": Expected an array of tables, got " .. type(value)
                end
                for i, item in ipairs(value) do
                    local is_valid, err = validate_nested_table(item, array_schema, path .. "[" .. i .. "]")
                    if not is_valid then
                        return false, err
                    end
                end
                return true
            end
        
            -- Helper function to validate arrays of primitive types
            local function validate_array_of_primitives(value, array_item_type, path)
                if type(value) ~= "table" then
                    return false, path .. ": Expected an array, got " .. type(value)
                end
                for i, item in ipairs(value) do
                    if not check_type(item, array_item_type) then
                        return false, path .. "[" .. i .. "]: Expected " .. array_item_type .. ", got " .. type(item)
                    end
                end
                return true
            end
        
            -- Iterate over the schema
            for key, constraints in pairs(schema) do
                local value = input_table[key]
                local expected_type = constraints.type
                local required = constraints.required or false
                local min = constraints.min
                local max = constraints.max
                local nested_schema = constraints.schema -- Schema for nested tables
                local default_value = constraints.default
                local path = key
        
                -- Check if the key exists in the table and is required
                if required and value == nil then
                    return false, "Missing required key: " .. path
                end
        
                -- If the key exists, check its type
                if value ~= nil and not check_type(value, expected_type) then
                    return false, "Incorrect type for key: " .. path .. ". Expected " .. expected_type .. ", got " .. type(value)
                end
        
                -- If the value is a nested table, validate it recursively
                if nested_schema and type(value) == "table" and expected_type == "table" then
                    local is_valid, err = validate_nested_table(value, nested_schema, path)
                    if not is_valid then
                        return false, "Error in nested table for key: " .. path .. ". " .. err
                    end
                end
        
                -- If the value is an array of tables, validate each element
                if expected_type == "array" and type(value) == "table" and nested_schema then
                    local is_valid, err = validate_array_of_tables(value, nested_schema, path)
                    if not is_valid then
                        return false, "Error in array of tables for key: " .. path .. ". " .. err
                    end
                end
        
                -- If the value is an array of primitive types, validate each element
                if expected_type == "array" and type(value) == "table" and not nested_schema then
                    local is_valid, err = validate_array_of_primitives(value, constraints.array_item_type, path)
                    if not is_valid then
                        return false, "Error in array of primitives for key: " .. path .. ". " .. err
                    end
                end
        
                -- Check range constraints (if applicable)
                if value ~= nil and not check_range(value, min, max) then
                    return false, "Value for key " .. path .. " is out of range."
                end
        
                -- Apply default values if the key is missing and a default is provided
                if value == nil and default_value ~= nil then
                    input_table[key] = default_value
                end
            end
        
            -- Check if the table has any unexpected keys
            for key in pairs(input_table) do
                if not schema[key] then
                    return false, "Unexpected key found: " .. key
                end
            end
        
            return true
        end
        
        return validate_table
    
    end),
    (function()
        local import_lua_dir = AstraIO.get_script_path():match("(.*[/\\])") or ""
        local old_require = require
        
        ---Converts a path relative to the current directory to realpath relative to root.
        --
        ---@param path string to be resolved
        ---@param __dirname string of calling script
        ---@return string
        local function resolve_relative(path, __dirname)
        	-- Split the path into parts
        	local function split_path(p)
        		local result = {}
        		for part in p:gmatch("[^/\\]+") do
        			table.insert(result, part)
        		end
        		return result
        	end
        
        	local segments = split_path(__dirname)
        	local parts = split_path(path)
        
        	for i, part in ipairs(parts) do
        		if part == ".." then
        			table.remove(segments)
        		elseif part == "." or part == "" then
        		-- Ignore current directory and empty segments
        		else
        			table.insert(segments, part)
        		end
        	end
        
        	return table.concat(segments, "/")
        end
        
        ---The lua-import module provides a function,
        ---the function takes single single string argument which is a glob pattern.
        ---The return value is the module refered by the glob pattern.
        --
        ---@param path any
        local function import(path)
        	local resolved_path = resolve_relative(path, import_lua_dir)
        	---@diagnostic disable-next-line: param-type-mismatch
        	local ok, result = pcall(old_require, resolved_path)
        	if not ok then
        		error("Failed to load module at path: " .. resolved_path .. "\nError: " .. result)
        	end
        	return result
        end
        
        ---
        ---Loads the given module, returns any value returned by the given module(`true` when `nil`).
        ---
        ---@param modname string
        ---@return unknown
        return function (modname)
        	if modname:find("astra_bundle") then
        		return {}
        	else
        		return import(modname)
        	end
    end
    end),
}
__luapack_cache__ = {}
__luapack_require__ = function(idx)
    local cache = __luapack_cache__[idx]
    if cache then
        return cache
    end
    local module = __luapack_modules__[idx]()
    __luapack_cache__[idx] = module
    return module
end

---@diagnostic disable: duplicate-set-field

_G.utils = __luapack_require__(1)

_G.validate_table = __luapack_require__(2)

_G.require = __luapack_require__(3)


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
function Astra:get(path, callback)
	table.insert(self, { path = path, method = "get", func = callback })
end

---@param path string The URL path for the request.
---@param callback callback A function that will be called when the request is made.
function Astra:post(path, callback)
	table.insert(self, { path = path, method = "post", func = callback })
end

---@param path string The URL path for the request.
---@param callback callback A function that will be called when the request is made.
function Astra:put(path, callback)
	table.insert(self, { path = path, method = "put", func = callback })
end

---@param path string The URL path for the request.
---@param callback callback A function that will be called when the request is made.
function Astra:delete(path, callback)
	table.insert(self, { path = path, method = "delete", func = callback })
end

---@param path string The URL path for the request.
---@param callback callback A function that will be called when the request is made.
function Astra:options(path, callback)
	table.insert(self, { path = path, method = "options", func = callback })
end

---@param path string The URL path for the request.
---@param callback callback A function that will be called when the request is made.
function Astra:patch(path, callback)
	table.insert(self, { path = path, method = "patch", func = callback })
end

---@param path string The URL path for the request.
---@param callback callback A function that will be called when the request is made.
function Astra:trace(path, callback)
	table.insert(self, { path = path, method = "trace", func = callback })
end

---
---Registers a static folder to serve
---@param path string The URL path for the request.
---@param serve_path string The directory path relatively
function Astra:static_dir(path, serve_path)
	table.insert(self, { path = path, method = "static_dir", func = function() end, static_dir = serve_path })
end

---
---Registers a static file to serve
---@param path string The URL path for the request.
---@param serve_path string The directory path relatively
function Astra:static_file(path, serve_path)
	table.insert(self, { path = path, method = "static_file", func = function() end, static_file = serve_path })
end

---
---Runs the Astra server
function Astra:run()
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
---@field queries fun(): table Returns the query list.
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

-- MARK: FileIO

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

--- @START_REMOVING_RUNTIME
_G.AstraIO = {
	---Returns the metadata of a file or directory
	---@param path string
	---@return FileMetadata
	get_metadata = function(path)
		return {}
	end,

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

	---Changes the current directory
	---@param path string Path to the directory
	change_dir = function(path) end,

	---Checks if a path exists
	---@param path string Path to the file or directory
	---@return boolean
	exists = function(path) return false end,

	---Creates a directory
	---@param path string Path to the directory
	create_dir = function(path) end,

	---Creates a directory recursively
	---@param path string Path to the directory
	create_dir_all = function(path) end,

	---Removes a file
	---@param path string Path to the file
	remove = function(path) end,

	---Removes a directory
	---@param path string Path to the directory
	remove_dir = function(path) end,

	---Removes a directory recursively
	---@param path string Path to the directory
	remove_dir_all = function(path) end,
}
--- @END_REMOVING_RUNTIME
