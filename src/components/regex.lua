---@meta

---@class Regex
---@field captures fun(regex: Regex, content: string): string[][]
---@field replace fun(regex: Regex, content: string, replacement: string, limit: number?): string
---@field is_match fun(regex: Regex, content: string): boolean

---@param expression string
---@return Regex
function Astra.regex(expression)
	---@diagnostic disable-next-line: undefined-global
	return astra_internal__regex(expression)
end
