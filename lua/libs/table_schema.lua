---
---Schema validation function with support for nested tables and arrays of tables
---@param tbl table
---@param schema table
---@return boolean, string | nil
local function validate_table(tbl, schema)
    -- Helper function to check if a value is of the expected type
    local function check_type(value, expectedType)
        local TypeMap = {
            number = "number",
            string = "string",
            boolean = "boolean",
            table = "table",
            ["function"] = "function",
            ["nil"] = "nil",
            table_array = "table"
        }
        return type(value) == TypeMap[expectedType]
    end

    -- Helper function to check if a value is within a range (if applicable)
    local function check_range(value, min, max)
        return not (min and value < min) and not (max and value > max)
    end

    -- Helper function to validate nested tables
    local function validate_nested_table(value, nestedSchema, path)
        local isValid, err = validate_table(value, nestedSchema)
        if not isValid then
            return false, path .. ": " .. err
        end
        return true
    end

    -- Helper function to validate arrays of tables
    local function validate_array_of_tables(value, arraySchema, path)
        if type(value) ~= "table" then
            return false, path .. ": Expected an array of tables, got " .. type(value)
        end
        for i, item in ipairs(value) do
            local isValid, err = validate_nested_table(item, arraySchema, path .. "[" .. i .. "]")
            if not isValid then
                return false, err
            end
        end
        return true
    end

    -- Iterate over the schema
    for key, constraints in pairs(schema) do
        local value = tbl[key]
        local expectedType = constraints.type
        local required = constraints.required or false
        local min = constraints.min
        local max = constraints.max
        local nestedSchema = constraints.schema -- Schema for nested tables
        local defaultValue = constraints.default
        local path = key

        -- Check if the key exists in the table and is required
        if required and value == nil then
            return false, "Missing required key: " .. path
        end

        -- If the key exists, check its type
        if value ~= nil and not check_type(value, expectedType) then
            return false, "Incorrect type for key: " .. path .. ". Expected " .. expectedType .. ", got " .. type(value)
        end

        -- If the value is a nested table, validate it recursively
        if nestedSchema and type(value) == "table" and expectedType == "table" then
            local isValid, err = validate_nested_table(value, nestedSchema, path)
            if not isValid then
                return false, "Error in nested table for key: " .. path .. ". " .. err
            end
        end

        -- If the value is an array of tables, validate each element
        if expectedType == "table_array" and type(value) == "table" then
            local isValid, err = validate_array_of_tables(value, nestedSchema, path)
            if not isValid then
                return false, "Error in array of tables for key: " .. path .. ". " .. err
            end
        end

        -- Check range constraints (if applicable)
        if value ~= nil and not check_range(value, min, max) then
            return false, "Value for key " .. path .. " is out of range."
        end

        -- Apply default values if the key is missing and a default is provided
        if value == nil and defaultValue ~= nil then
            tbl[key] = defaultValue
        end
    end

    -- Check if the table has any unexpected keys
    for key in pairs(tbl) do
        if not schema[key] then
            return false, "Unexpected key found: " .. key
        end
    end

    return true
end

return validate_table
