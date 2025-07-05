
Astra.http.server = {}

---@class HTTPServer
---@field __index HTTPServer
local HTTPServer = {
    version = "0.0.0",
    hostname = "127.0.0.1",
    compression = false,
    port = 8080,
    routes = {}
}

---@return HTTPServer
function Astra.http.server:new()
	local server = {
		version = "0.0.0",
		hostname = "127.0.0.1",
		--- Enable or disable compression
		compression = false,
		port = 8080,
		--- Contains all of the route details
		routes = {},
	}

	setmetatable(server, self)
	self.__index = self
	server:register_methods()
	return server
end