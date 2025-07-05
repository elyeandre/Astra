---@meta

_G.import = require

---@param modName string
function _G.import(modName)
	---@diagnostic disable-next-line: param-type-mismatch, undefined-global
	local ok, result = pcall(astra_internal__import, modName)
	if not ok then
		ok, result = require(modName)
		if not ok then
			error("Failed to load module.\nError: " .. result)
		end
		return result
	end
	return result
end
