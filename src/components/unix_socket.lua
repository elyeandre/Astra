---@meta

---@class UnixSocket
---@field send fun(socket: UnixSocket, data: string)
---@field receive fun(socket: UnixSocket, buffer_size: integer): string
---@field close fun(socket: UnixSocket)
---@field is_connected fun(socket: UnixSocket): boolean

---@class _AstraUnix
---@field connect fun(path: string): UnixSocket|nil, string|nil
