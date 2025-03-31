---@diagnostic disable-next-line: duplicate-doc-alias
---@alias importFun fun(moduleName: string): any

---@type importFun
---@diagnostic disable-next-line: assign-type-mismatch
local import = require

---The import function is similar to the lua's require functions.
---With the exception of the async import capability required for the
---Astra's utilities.
---@param moduleName string
---@return any
---@diagnostic disable-next-line: redefined-local
function import(moduleName)
	---@diagnostic disable-next-line: param-type-mismatch, undefined-global
	local ok, result = pcall(astra_internal__require, moduleName)
	if not ok then
		ok, result = require(moduleName)
		if not ok then
			error("Failed to load module.\nError: " .. result)
		end
		return result
	end
	return result
end

return import
