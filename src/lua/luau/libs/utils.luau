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
