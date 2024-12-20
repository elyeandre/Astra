local utils = { _version = "0.1.0" }

-- Recursively converts a Lua table into a pretty-formatted JSON string.
-- @param tbl The input table.
-- @return A pretty-formatted JSON string representation of the input table.
function pretty_table(table)
    local json_str = ""

    -- Iterate over each key-value pair in the table
    for k, v in pairs(table) do
        if type(k) ~= 'number' then k = '"' .. k .. '"' end

        -- Recursively convert nested tables to JSON strings
        if type(v) == "table" then
            json_str = json_str .. k .. ": " .. pretty_table(v) .. ", "
        else
            -- Format string values with quotation marks
            if type(v) == 'string' then
                v = '"' .. v .. '"'
            end
            json_str = json_str .. k .. ": " .. tostring(v) .. ", "
        end
    end

    -- Remove the trailing comma and space, and wrap in curly braces for JSON format
    return "{ " .. string.sub(json_str, 1, -3) .. " }"
end

string.trim = function(str)
    local trimmed_str = str:match("^%s*(.-)%s*$")
    return trimmed_str
end

return utils