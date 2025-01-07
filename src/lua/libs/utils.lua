local utils = { _version = "0.1.0" }

---Pretty prints any table or value
---@diagnostic disable-next-line: duplicate-set-field
function _G.pretty_print(inner_table)
    local function pretty_print_table(table)
        local str = ""

        -- Iterate over each key-value pair in the table
        for k, v in pairs(table) do
            k = '[' .. k .. ']'

            -- Recursively convert nested tables to JSON strings
            if type(v) == "table" then
                str = str .. k .. ": " .. pretty_print_table(v) .. ", "
            else
                -- Format string values with quotation marks
                if type(v) == 'string' then
                    v = '"' .. v .. '"'
                end
                str = str .. k .. ": " .. tostring(v) .. ", "
            end
        end

        return "{ " .. string.sub(str, 1, -3) .. " }"
    end

    if type(inner_table) == 'table' then
        print(pretty_print_table(inner_table))
    else
        print(tostring(inner_table))
    end
end

---
---Recursively converts a Lua table into a pretty-formatted JSON string.
---@param table table The input table.
---@diagnostic disable-next-line: duplicate-set-field
function _G.pretty_json_table(table)
    local json_str = ""

    -- Iterate over each key-value pair in the table
    for k, v in pairs(table) do
        if type(k) ~= 'number' then k = '"' .. k .. '"' end

        -- Recursively convert nested tables to JSON strings
        if type(v) == "table" then
            json_str = json_str .. k .. ": " .. _G.pretty_json_table(v) .. ", "
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

-- function string.trim(str)
--     local trimmed_str = str:match("^%s*(.-)%s*$")
--     return trimmed_str
-- end

function utils.parse_query(str)
    local function unescape(s)
        s = string.gsub(s, "+", " ")
        s = string.gsub(s, "%%(%x%x)", function(h)
            return string.char(tonumber(h, 16))
        end)
        return s
    end

    local result_table = {}
    for k, v in string.gmatch(str, "([^&=?]+)=([^&=?]+)") do
        --t[k] = v
        result_table[k] = unescape(v)
    end

    return result_table
end

---
---Splits a word into an array given the separator
---@param str string The input string
---@param separator string The input string
---@return table array
---@nodiscard
---@diagnostic disable-next-line: duplicate-set-field
function string.split(str, separator)
    local result_table = {}
    for word in str:gmatch("([^" .. separator .. "]+)") do
        table.insert(result_table, word)
    end
    return result_table
end

return utils
