---@meta

---@class NetworkManager
---@field set_link_up fun(self: NetworkManager, interface_name: string): nil
---@field set_link_down fun(self: NetworkManager, interface_name: string): nil
---@field set_link_mtu fun(self: NetworkManager, interface_name: string, mtu: number): nil
---@field add_route fun(self: NetworkManager, route_spec: string): nil
---@field delete_route fun(self: NetworkManager, route_spec: string): nil
---@field get_link_status fun(self: NetworkManager, interface_name: string): NetworkInterfaceStatus
---@field list_interfaces fun(self: NetworkManager): NetworkInterfaceStatus[]
---@field add_address fun(self: NetworkManager, interface_name: string, address_spec: string): nil
---@field remove_address fun(self: NetworkManager, interface_name: string, address_spec: string): nil
---@field list_addresses fun(self: NetworkManager, interface_name: string): NetworkAddress[]

---@class NetworkInterfaceStatus
---@field name string Interface name (e.g., "wg0", "eth0")
---@field index number Interface index
---@field is_up boolean Whether the interface is administratively up
---@field is_running boolean Whether the interface is operationally running
---@field mtu number|nil Maximum Transmission Unit (if available)

---@class NetworkAddress
---@field address string IP address with CIDR notation (e.g., "192.168.1.1/24")
---@field family string Address family ("inet" for IPv4, "inet6" for IPv6)
---@field prefix_len number Prefix length (subnet mask length)
---@field scope string Address scope ("global", "site", "link", "host")
---@field label string|nil Optional address label

---@class _AstraNetwork
---@field new fun(): NetworkManager Create a new NetworkManager instance

---@class Astra
---@field Network _AstraNetwork