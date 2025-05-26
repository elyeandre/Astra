--!nocheck

local utils = {}

---Pretty prints any table or value
---@param value any
---@diagnostic disable-next-line: duplicate-set-field
function _G.pprint(value)
	---@diagnostic disable-next-line: undefined-global
	astra_internal__pretty_print(value)
end

utils.json = {
	---Encodes the value into a valid JSON string
	---@param value any
	---@return string
	---@diagnostic disable-next-line: duplicate-set-field
	encode = function(value)
		---@diagnostic disable-next-line: undefined-global
		return astra_internal__json_encode(value)
	end,

	---Decodes the JSON string into a valid lua value
	---@param value string
	---@return any
	---@diagnostic disable-next-line: duplicate-set-field
	decode = function(value)
		---@diagnostic disable-next-line: undefined-global
		return astra_internal__json_decode(value)
	end,
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
