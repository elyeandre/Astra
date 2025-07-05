---@meta

---@class Astra
Astra = {
    version = "@ASTRA_VERSION",
}

-- This is to prevent a small undefined behavior in Lua
---@diagnostic disable-next-line: redundant-parameter
setmetatable(_G, {
	---@diagnostic disable-next-line: redundant-parameter, unused-local
	__index = function(T, k, v)
		error("Called non-existing variable")
	end,
})