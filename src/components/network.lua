---@meta

---@class NetworkManager
---@field set_link_up fun(self: NetworkManager, interface_name: string): nil
---@field set_link_down fun(self: NetworkManager, interface_name: string): nil
---@field get_link_status fun(self: NetworkManager, interface_name: string): NetworkInterfaceStatus
---@field list_interfaces fun(self: NetworkManager): NetworkInterfaceStatus[]
---@field add_address fun(self: NetworkManager, interface_name: string, address_cidr: string): nil
---@field delete_address fun(self: NetworkManager, interface_name: string, address_cidr: string): nil
---@field add_route fun(self: NetworkManager, route_config: RouteConfig): nil
---@field delete_route fun(self: NetworkManager, route_config: RouteConfig): nil

---@class NetworkInterfaceStatus
---@field name string Interface name (e.g., "wg0", "eth0")
---@field index number Interface index
---@field is_up boolean Whether the interface is administratively up
---@field is_running boolean Whether the interface is operationally running

---@class RouteConfig
---@field destination string Destination network in CIDR format (e.g., "192.168.1.0/24") or "default"
---@field device? string Interface name (optional, e.g., "wg0", "eth0")
---@field gateway? string Gateway IP address (optional, e.g., "192.168.1.1")

---@class _AstraNetwork
---@field new fun(): NetworkManager Create a new NetworkManager instance

---@class Astra
---@field Network _AstraNetwork
