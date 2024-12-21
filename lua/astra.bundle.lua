---@diagnostic disable: duplicate-set-field, lowercase-global

__luapack_modules__ = {
    (function()
        local b={_version="0.1.0"}
        function pretty_table(c)local d=""
        for _a,aa in pairs(c)do if type(_a)~='number'then
        _a='"'.._a..'"'end
        if type(aa)=="table"then d=d.._a..
        ": "..pretty_table(aa)..", "else if type(aa)=='string'then aa=
        '"'..aa..'"'end;d=d.._a..
        ": "..tostring(aa)..", "end end;return"{ "..string.sub(d,1,-3).." }"end
    string.trim=function(c)local d=c:match("^%s*(.-)%s*$")return d end;return b
    end),

}
__luapack_cache__ = {}
__luapack_require__ = function(idx)
    local cache = __luapack_cache__[idx]
    if cache then
        return cache
    end
    local module = __luapack_modules__[idx]()
    __luapack_cache__[idx] = module
    return module
end

---@diagnostic disable: duplicate-set-field

__luapack_require__(1)
_G.Astra = {}

Astra.get_request = function(path, callback)
    table.insert(_G.Astra, { path = path, method = "get", func = callback })
end

Astra.post_request = function(path, callback)
    table.insert(_G.Astra, { path = path, method = "post", func = callback })
end

Astra.put_request = function(path, callback)
    table.insert(_G.Astra, { path = path, method = "put", func = callback })
end

Astra.delete_request = function(path, callback)
    table.insert(_G.Astra, { path = path, method = "delete", func = callback })
end
