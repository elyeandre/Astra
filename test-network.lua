-- Example usage of the Network API in your Lua script

local function test_network_interface()
    print("Testing Network Interface Management")
    
    -- Create a new network manager
    local net = Astra.Network.new()
    
    -- Test interface name (replace with your actual interface)
    local interface_name = "wg0"  -- or "eth0", "lo", etc.
    
    -- Get current status
    print("\n--- Getting interface status ---")
    local status = net:get_link_status(interface_name)
    print(string.format("Interface: %s", status.name))
    print(string.format("Index: %d", status.index))
    print(string.format("Is Up: %s", status.is_up and "yes" or "no"))
    print(string.format("Is Running: %s", status.is_running and "yes" or "no"))
    
    -- Try to bring interface up
    print(string.format("\n--- Bringing %s up ---", interface_name))
    local success, err = pcall(function()
        net:set_link_up(interface_name)
    end)
    
    if success then
        print(string.format("Successfully brought %s up", interface_name))
        
        -- Check status again
        status = net:get_link_status(interface_name)
        print(string.format("New status - Is Up: %s", status.is_up and "yes" or "no"))
        
        -- Wait a moment
        os.execute("sleep 2")
        
        -- Try to bring interface down
        print(string.format("\n--- Bringing %s down ---", interface_name))
        local success2, err2 = pcall(function()
            net:set_link_down(interface_name)
        end)
        
        if success2 then
            print(string.format("Successfully brought %s down", interface_name))
            
            -- Check status again
            status = net:get_link_status(interface_name)
            print(string.format("New status - Is Up: %s", status.is_up and "yes" or "no"))
        else
            print(string.format("Failed to bring %s down: %s", interface_name, err2))
        end
    else
        print(string.format("Failed to bring %s up: %s", interface_name, err))
    end
end

local function list_all_interfaces()
    print("\n--- Listing all network interfaces ---")
    
    local net = Astra.Network.new()
    local interfaces = net:list_interfaces()
    
    for i, interface in ipairs(interfaces) do
        print(string.format("%d. %s (index: %d, up: %s, running: %s)", 
            i, 
            interface.name or "unknown", 
            interface.index,
            interface.is_up and "yes" or "no",
            interface.is_running and "yes" or "no"
        ))
    end
end

-- Run the tests
list_all_interfaces()
test_network_interface()
