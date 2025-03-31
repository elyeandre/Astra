local import_lua_dir = AstraIO.get_script_path():match("(.*[/\\])") or ""
local normal_require = require

---@diagnostic disable-next-line: duplicate-doc-alias
---@alias importFun fun(moduleName: string): any

---@type importFun
---@diagnostic disable-next-line: assign-type-mismatch
local import = require

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
---@param moduleName string
---@return any
---@diagnostic disable-next-line: redefined-local, duplicate-set-field
function _G.require(moduleName)
---@diagnostic disable-next-line: undefined-global
	local ok, result = pcall(astra_internal__require, moduleName)
	if not ok then
		return normal_require(moduleName)
	end
	return result
end

---The lua-import module provides a function,
---the function takes single single string argument which is a glob pattern.
---The return value is the module refered by the glob pattern.
---@param moduleName string
---@return any
---@diagnostic disable-next-line: redefined-local
function import(moduleName)
	local resolved_path = resolve_relative(moduleName, import_lua_dir)
	---@diagnostic disable-next-line: param-type-mismatch
	local ok, result = pcall(require, resolved_path)
	if not ok then
		error("Failed to load module at path: " .. resolved_path .. "\nError: " .. result)
	end
	return result
end

return import
