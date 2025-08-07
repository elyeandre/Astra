---@meta

---@class NetworkManager
---@field set_link_up fun(self: NetworkManager, interface_name: string): nil
---@field set_link_down fun(self: NetworkManager, interface_name: string): nil
---@field get_link_status fun(self: NetworkManager, interface_name: string): NetworkInterfaceStatus
---@field list_interfaces fun(self: NetworkManager): NetworkInterfaceStatus[]

---@class NetworkInterfaceStatus
---@field name string Interface name (e.g., "wg0", "eth0")
---@field index number Interface index
---@field is_up boolean Whether the interface is administratively up
---@field is_running boolean Whether the interface is operationally running

---@class _AstraNetwork
---@field new fun(): NetworkManager Create a new NetworkManager instance

---@class Astra
---@field Network _AstraNetwork
