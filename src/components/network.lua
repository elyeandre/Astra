---@meta

---@class _AstraNet
---@field set_link_up fun(interface: string): boolean, string|nil
---@field set_link_down fun(interface: string): boolean, string|nil

---Sets a network interface up
---@param interface string Interface name (e.g., "wg0")
---@return boolean success
---@return string|nil error_message
function Astra.net.set_link_up(interface) end

---Sets a network interface down
---@param interface string Interface name (e.g., "wg0")
---@return boolean success
---@return string|nil error_message
function Astra.net.set_link_down(interface) end
