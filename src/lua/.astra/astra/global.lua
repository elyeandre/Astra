---@meta

---Pretty prints any table or value
---@param value any
function pprint(value) end

---The import function is similar to the lua's require functions.
---With the exception of the async import capability required for the
---Astra's utilities.
---@param moduleName string
---@return any
function import(moduleName) end

---Represents an async task
---@class TaskHandler
local TaskHandler = {}

---Aborts the running task
function TaskHandler:abort() end

---Awaits the task
function TaskHandler:await() end

---Starts a new async task
---@param callback fun() The callback to run the content of the async task
---@return TaskHandler
function spawn_task(callback) end

---Starts a new async task with a delay in milliseconds
---@param callback fun() The callback to run the content of the async task
---@param timeout number The delay in milliseconds
---@return TaskHandler
function spawn_timeout(callback, timeout) end

---Starts a new async task that runs infinitely in a loop but with a delay in milliseconds
---@param callback fun() The callback to run the content of the async task
---@param timeout number The delay in milliseconds
---@return TaskHandler
function spawn_interval(callback, timeout) end
