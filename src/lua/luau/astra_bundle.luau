---@diagnostic disable: duplicate-set-field, lowercase-global

__luapack_modules__ = {
	function()
		--!strict

		local utils = { _version = "0.1.0" }

		---Pretty prints any table or value
		---@diagnostic disable-next-line: duplicate-set-field
		function _G.pretty_print(inner_table)
			local function pretty_print_table(table_to_print)
				local str = ""

				-- Iterate over each key-value pair in the table
				for key: string, value in pairs(table_to_print) do
					key = "[" .. key .. "]"

					-- Recursively convert nested tables to JSON strings
					if type(value) == "table" then
						str = str .. key .. ": " .. pretty_print_table(value) :: string .. ", "
					else
						-- Format string values with quotation marks
						if type(value) == "string" then
							value = '"' .. value .. '"'
						end
						str = str .. key .. ": " .. tostring(value) .. ", "
					end
				end

				return "{ " .. string.sub(str, 1, -3) .. " }"
			end

			if type(inner_table) == "table" then
				print(pretty_print_table(inner_table))
			else
				print(tostring(inner_table))
			end
		end

		---
		---Recursively converts a Lua table into a pretty-formatted JSON string.
		---@param table_to_convert table The input table.
		---@diagnostic disable-next-line: duplicate-set-field
		function _G.pretty_json_table(table_to_convert)
			local json_str = ""

			-- Iterate over each key-value pair in the table
			for key, value in pairs(table_to_convert) do
				if type(key) ~= "number" then
					key = '"' .. key .. '"'
				end

				-- Recursively convert nested tables to JSON strings
				if type(value) == "table" then
					json_str = json_str .. key .. ": " .. _G.pretty_json_table(value) .. ", "
				else
					-- Format string values with quotation marks
					if type(value) == "string" then
						value = '"' .. value .. '"'
					end
					json_str = json_str .. key .. ": " .. tostring(value) .. ", "
				end
			end

			-- Remove the trailing comma and space, and wrap in curly braces for JSON format
			return "{ " .. string.sub(json_str, 1, -3) .. " }"
		end

		-- function string.trim(str)
		--     local trimmed_str = str:match("^%s*(.-)%s*$")
		--     return trimmed_str
		-- end

		function utils.parse_query(query_str)
			local function unescape(escaped_str)
				escaped_str = string.gsub(escaped_str, "+", " ")
				escaped_str = string.gsub(escaped_str, "%%(%x%x)", function(hex_val)
					return string.char(tonumber(hex_val, 16) :: number)
				end)
				return escaped_str
			end

			local result_table = {}
			for key, value in string.gmatch(query_str, "([^&=?]+)=([^&=?]+)") do
				--t[k] = v
				result_table[key] = unescape(value :: string)
			end

			return result_table
		end

		---
		---Splits a sentence into an array given the separator
		---@param input_str string The input string
		---@param separator_str string The input string
		---@return table array
		---@nodiscard
		---@diagnostic disable-next-line: duplicate-set-field
		function string.split(input_str: string, separator_str: string?)
			local result_table = {}
			for word in input_str:gmatch("([^" .. separator_str :: string .. "]+)") do
				table.insert(result_table, word)
			end
			return result_table
		end

		return utils
	end,
	function()
		--
		-- json.lua
		--
		-- Copyright (c) 2020 rxi
		--
		-- Permission is hereby granted, free of charge, to any person obtaining a copy of
		-- this software and associated documentation files (the "Software"), to deal in
		-- the Software without restriction, including without limitation the rights to
		-- use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
		-- of the Software, and to permit persons to whom the Software is furnished to do
		-- so, subject to the following conditions:
		--
		-- The above copyright notice and this permission notice shall be included in all
		-- copies or substantial portions of the Software.
		--
		-- THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
		-- IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
		-- FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
		-- AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
		-- LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
		-- OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
		-- SOFTWARE.
		--

		local json = { _version = "0.1.2" }

		-------------------------------------------------------------------------------
		-- Encode
		-------------------------------------------------------------------------------

		local encode

		local escape_char_map = {
			["\\"] = "\\",
			['"'] = '"',
			["\b"] = "b",
			["\f"] = "f",
			["\n"] = "n",
			["\r"] = "r",
			["\t"] = "t",
		}

		local escape_char_map_inv = { ["/"] = "/" }
		for k, v in pairs(escape_char_map) do
			escape_char_map_inv[v] = k
		end

		local function escape_char(c)
			return "\\" .. (escape_char_map[c] or string.format("u%04x", c:byte()))
		end

		local function encode_nil(val)
			return "null"
		end

		local function encode_table(val, stack)
			local res = {}
			stack = stack or {}

			-- Circular reference?
			if stack[val] then
				error("circular reference")
			end

			stack[val] = true

			if rawget(val, 1) ~= nil or next(val) == nil then
				-- Treat as array -- check keys are valid and it is not sparse
				local n = 0
				for k in pairs(val) do
					if type(k) ~= "number" then
						error("invalid table: mixed or invalid key types")
					end
					n = n + 1
				end
				if n ~= #val then
					error("invalid table: sparse array")
				end
				-- Encode
				for i, v in ipairs(val) do
					table.insert(res, encode(v, stack))
				end
				stack[val] = nil
				return "[" .. table.concat(res, ",") .. "]"
			else
				-- Treat as an object
				for k, v in pairs(val) do
					if type(k) ~= "string" then
						error("invalid table: mixed or invalid key types")
					end
					table.insert(res, encode(k, stack) .. ":" .. encode(v, stack))
				end
				stack[val] = nil
				return "{" .. table.concat(res, ",") .. "}"
			end
		end

		local function encode_string(val)
			return '"' .. val:gsub('[%z\1-\31\\"]', escape_char) .. '"'
		end

		local function encode_number(val)
			-- Check for NaN, -inf and inf
			if val ~= val or val <= -math.huge or val >= math.huge then
				error("unexpected number value '" .. tostring(val) .. "'")
			end
			return string.format("%.14g", val)
		end

		local type_func_map = {
			["nil"] = encode_nil,
			["table"] = encode_table,
			["string"] = encode_string,
			["number"] = encode_number,
			["boolean"] = tostring,
		}

		encode = function(val, stack)
			local t = type(val)
			local f = type_func_map[t]
			if f then
				return f(val, stack)
			end
			error("unexpected type '" .. t .. "'")
		end

		function json.encode(val)
			return (encode(val))
		end

		-------------------------------------------------------------------------------
		-- Decode
		-------------------------------------------------------------------------------

		local parse

		local function create_set(...)
			local res = {}
			for i = 1, select("#", ...) do
				res[select(i, ...)] = true
			end
			return res
		end

		local space_chars = create_set(" ", "\t", "\r", "\n")
		local delim_chars = create_set(" ", "\t", "\r", "\n", "]", "}", ",")
		local escape_chars = create_set("\\", "/", '"', "b", "f", "n", "r", "t", "u")
		local literals = create_set("true", "false", "null")

		local literal_map = {
			["true"] = true,
			["false"] = false,
			["null"] = nil,
		}

		local function next_char(str, idx, set, negate)
			for i = idx, #str do
				if set[str:sub(i, i)] ~= negate then
					return i
				end
			end
			return #str + 1
		end

		local function decode_error(str, idx, msg)
			local line_count = 1
			local col_count = 1
			for i = 1, idx - 1 do
				col_count = col_count + 1
				if str:sub(i, i) == "\n" then
					line_count = line_count + 1
					col_count = 1
				end
			end
			error(string.format("%s at line %d col %d", msg, line_count, col_count))
		end

		local function codepoint_to_utf8(n)
			-- http://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=iws-appendixa
			local f = math.floor
			if n <= 0x7f then
				return string.char(n)
			elseif n <= 0x7ff then
				return string.char(f(n / 64) + 192, n % 64 + 128)
			elseif n <= 0xffff then
				return string.char(f(n / 4096) + 224, f(n % 4096 / 64) + 128, n % 64 + 128)
			elseif n <= 0x10ffff then
				return string.char(
					f(n / 262144) + 240,
					f(n % 262144 / 4096) + 128,
					f(n % 4096 / 64) + 128,
					n % 64 + 128
				)
			end
			error(string.format("invalid unicode codepoint '%x'", n))
		end

		local function parse_unicode_escape(s)
			local n1 = tonumber(s:sub(1, 4), 16)
			local n2 = tonumber(s:sub(7, 10), 16)
			-- Surrogate pair?
			if n2 then
				return codepoint_to_utf8((n1 - 0xd800) * 0x400 + (n2 - 0xdc00) + 0x10000)
			else
				return codepoint_to_utf8(n1)
			end
		end

		local function parse_string(str, i)
			local res = ""
			local j = i + 1
			local k = j

			while j <= #str do
				local x = str:byte(j)

				if x < 32 then
					decode_error(str, j, "control character in string")
				elseif x == 92 then -- `\`: Escape
					res = res .. str:sub(k, j - 1)
					j = j + 1
					local c = str:sub(j, j)
					if c == "u" then
						local hex = str:match("^[dD][89aAbB]%x%x\\u%x%x%x%x", j + 1)
							or str:match("^%x%x%x%x", j + 1)
							or decode_error(str, j - 1, "invalid unicode escape in string")
						res = res .. parse_unicode_escape(hex)
						j = j + #hex
					else
						if not escape_chars[c] then
							decode_error(str, j - 1, "invalid escape char '" .. c .. "' in string")
						end
						res = res .. escape_char_map_inv[c]
					end
					k = j + 1
				elseif x == 34 then -- `"`: End of string
					res = res .. str:sub(k, j - 1)
					return res, j + 1
				end

				j = j + 1
			end

			decode_error(str, i, "expected closing quote for string")
		end

		local function parse_number(str, i)
			local x = next_char(str, i, delim_chars)
			local s = str:sub(i, x - 1)
			local n = tonumber(s)
			if not n then
				decode_error(str, i, "invalid number '" .. s .. "'")
			end
			return n, x
		end

		local function parse_literal(str, i)
			local x = next_char(str, i, delim_chars)
			local word = str:sub(i, x - 1)
			if not literals[word] then
				decode_error(str, i, "invalid literal '" .. word .. "'")
			end
			return literal_map[word], x
		end

		local function parse_array(str, i)
			local res = {}
			local n = 1
			i = i + 1
			while 1 do
				local x
				i = next_char(str, i, space_chars, true)
				-- Empty / end of array?
				if str:sub(i, i) == "]" then
					i = i + 1
					break
				end
				-- Read token
				x, i = parse(str, i)
				res[n] = x
				n = n + 1
				-- Next token
				i = next_char(str, i, space_chars, true)
				local chr = str:sub(i, i)
				i = i + 1
				if chr == "]" then
					break
				end
				if chr ~= "," then
					decode_error(str, i, "expected ']' or ','")
				end
			end
			return res, i
		end

		local function parse_object(str, i)
			local res = {}
			i = i + 1
			while 1 do
				local key, val
				i = next_char(str, i, space_chars, true)
				-- Empty / end of object?
				if str:sub(i, i) == "}" then
					i = i + 1
					break
				end
				-- Read key
				if str:sub(i, i) ~= '"' then
					decode_error(str, i, "expected string for key")
				end
				key, i = parse(str, i)
				-- Read ':' delimiter
				i = next_char(str, i, space_chars, true)
				if str:sub(i, i) ~= ":" then
					decode_error(str, i, "expected ':' after key")
				end
				i = next_char(str, i + 1, space_chars, true)
				-- Read value
				val, i = parse(str, i)
				-- Set
				res[key] = val
				-- Next token
				i = next_char(str, i, space_chars, true)
				local chr = str:sub(i, i)
				i = i + 1
				if chr == "}" then
					break
				end
				if chr ~= "," then
					decode_error(str, i, "expected '}' or ','")
				end
			end
			return res, i
		end

		local char_func_map = {
			['"'] = parse_string,
			["0"] = parse_number,
			["1"] = parse_number,
			["2"] = parse_number,
			["3"] = parse_number,
			["4"] = parse_number,
			["5"] = parse_number,
			["6"] = parse_number,
			["7"] = parse_number,
			["8"] = parse_number,
			["9"] = parse_number,
			["-"] = parse_number,
			["t"] = parse_literal,
			["f"] = parse_literal,
			["n"] = parse_literal,
			["["] = parse_array,
			["{"] = parse_object,
		}

		parse = function(str, idx)
			local chr = str:sub(idx, idx)
			local f = char_func_map[chr]
			if f then
				return f(str, idx)
			end
			decode_error(str, idx, "unexpected character '" .. chr .. "'")
		end

		function json.decode(str)
			if type(str) ~= "string" then
				error("expected argument of type string, got " .. type(str))
			end
			local res, idx = parse(str, next_char(str, 1, space_chars, true))
			idx = next_char(str, idx, space_chars, true)
			if idx <= #str then
				decode_error(str, idx, "trailing garbage")
			end
			return res
		end

		return json
	end,
	function()
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
					array = "table",
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
					return false,
						"Incorrect type for key: " .. path .. ". Expected " .. expected_type .. ", got " .. type(value)
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
	end,
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

--!strict

-- local utils = __luapack_require__(1)

-- local json = __luapack_require__(2)

-- local table_schema = __luapack_require__(3)

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
return { Astra = _G.Astra }
--- @END_REMOVING_RUNTIME
