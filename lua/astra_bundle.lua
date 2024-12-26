---@diagnostic disable: duplicate-set-field, lowercase-global

__luapack_modules__ = {
    (function()
        local b={_version="0.1.0"}
        function b.pretty_table(c)local d=""
        for _a,aa in pairs(c)do if type(_a)~='number'then
        _a='"'.._a..'"'end
        if type(aa)=="table"then d=d.._a..": "..
        b.pretty_table(aa)..", "else if
        type(aa)=='string'then aa='"'..aa..'"'end;d=d.._a..": "..
        tostring(aa)..", "end end;return"{ "..string.sub(d,1,-3).." }"end;function b.urldecode(c)
        c=c:gsub('+',' '):gsub('%%(%x%x)',function(d)
        return string.char(tonumber(d,16))end)return c end;function b.parseurl(c)
        local d={}
    for _a,aa in c:gmatch('([^&=?]-)=([^&=?]+)')do d[_a]=b.urldecode(aa)end;return d end;return b
    end),
    (function()
        local c_a={_version="0.1.2"}local d_a
        local _aa={["\\"]="\\",["\""]="\"",["\b"]="b",["\f"]="f",["\n"]="n",["\r"]="r",["\t"]="t"}local aaa={["/"]="/"}for _ab,aab in pairs(_aa)do aaa[aab]=_ab end;local function baa(_ab)
        return"\\".. (_aa[_ab]or
        string.format("u%04x",_ab:byte()))end;local function caa(_ab)return"null"end
        local function daa(_ab,aab)
        local bab={}aab=aab or{}
        if aab[_ab]then error("circular reference")end;aab[_ab]=true
        if rawget(_ab,1)~=nil or next(_ab)==nil then
        local cab=0
        for dab in pairs(_ab)do if type(dab)~="number"then
        error("invalid table: mixed or invalid key types")end;cab=cab+1 end
        if cab~=#_ab then error("invalid table: sparse array")end
        for dab,_bb in ipairs(_ab)do table.insert(bab,d_a(_bb,aab))end;aab[_ab]=nil
        return"["..table.concat(bab,",").."]"else
        for cab,dab in pairs(_ab)do if type(cab)~="string"then
        error("invalid table: mixed or invalid key types")end;table.insert(bab,d_a(cab,aab)..
        ":"..d_a(dab,aab))end;aab[_ab]=nil
        return"{"..table.concat(bab,",").."}"end end;local function _ba(_ab)
        return'"'.._ab:gsub('[%z\1-\31\\"]',baa)..'"'end
        local function aba(_ab)if
        _ab~=_ab or _ab<=-math.huge or _ab>=math.huge then
        error("unexpected number value '"..
        tostring(_ab).."'")end;return
        string.format("%.14g",_ab)end
        local bba={["nil"]=caa,["table"]=daa,["string"]=_ba,["number"]=aba,["boolean"]=tostring}
        d_a=function(_ab,aab)local bab=type(_ab)local cab=bba[bab]if cab then return cab(_ab,aab)end;error(
        "unexpected type '"..bab.."'")end;function c_a.encode(_ab)return(d_a(_ab))end;local cba
        local function dba(...)local _ab={}for i=1,select("#",...)do
        _ab[select(i,...)]=true end;return _ab end;local _ca=dba(" ","\t","\r","\n")
        local aca=dba(" ","\t","\r","\n","]","}",",")local bca=dba("\\","/",'"',"b","f","n","r","t","u")
        local cca=dba("true","false","null")local dca={["true"]=true,["false"]=false,["null"]=nil}local function _da(_ab,aab,bab,cab)for i=aab,#_ab do if
        bab[_ab:sub(i,i)]~=cab then return i end end;return#_ab+
        1 end
        local function ada(_ab,aab,bab)local cab=1
        local dab=1;for i=1,aab-1 do dab=dab+1
        if _ab:sub(i,i)=="\n"then cab=cab+1;dab=1 end end
        error(string.format("%s at line %d col %d",bab,cab,dab))end
        local function bda(_ab)local aab=math.floor
        if _ab<=0x7f then return string.char(_ab)elseif _ab<=0x7ff then
        return string.char(aab(_ab/
        64)+192,_ab%64 +128)elseif _ab<=0xffff then
        return string.char(aab(_ab/4096)+224,aab(_ab%4096 /64)+128,
        _ab%64 +128)elseif _ab<=0x10ffff then return
        string.char(aab(_ab/262144)+240,
        aab(_ab%262144 /4096)+128,aab(_ab%4096 /64)+128,_ab%64 +128)end
        error(string.format("invalid unicode codepoint '%x'",_ab))end
        local function cda(_ab)local aab=tonumber(_ab:sub(1,4),16)
        local bab=tonumber(_ab:sub(7,10),16)
        if bab then return
        bda((aab-0xd800)*0x400 + (bab-0xdc00)+0x10000)else return bda(aab)end end
        local function dda(_ab,aab)local bab=""local cab=aab+1;local dab=cab
        while cab<=#_ab do local _bb=_ab:byte(cab)
        if _bb<32 then
        ada(_ab,cab,"control character in string")elseif _bb==92 then bab=bab.._ab:sub(dab,cab-1)cab=cab+1
        local abb=_ab:sub(cab,cab)
        if abb=="u"then
        local bbb=_ab:match("^[dD][89aAbB]%x%x\\u%x%x%x%x",cab+1)or
        _ab:match("^%x%x%x%x",cab+1)or
        ada(_ab,cab-1,"invalid unicode escape in string")bab=bab..cda(bbb)cab=cab+#bbb else if not bca[abb]then
        ada(_ab,cab-1,"invalid escape char '"..abb..
        "' in string")end;bab=bab..aaa[abb]end;dab=cab+1 elseif _bb==34 then bab=bab.._ab:sub(dab,cab-1)
        return bab,cab+1 end;cab=cab+1 end;ada(_ab,aab,"expected closing quote for string")end
        local function __b(_ab,aab)local bab=_da(_ab,aab,aca)local cab=_ab:sub(aab,bab-1)
        local dab=tonumber(cab)if not dab then
        ada(_ab,aab,"invalid number '"..cab.."'")end;return dab,bab end
        local function a_b(_ab,aab)local bab=_da(_ab,aab,aca)local cab=_ab:sub(aab,bab-1)
        if not cca[cab]then ada(_ab,aab,
        "invalid literal '"..cab.."'")end;return dca[cab],bab end
        local function b_b(_ab,aab)local bab={}local cab=1;aab=aab+1
        while 1 do local dab;aab=_da(_ab,aab,_ca,true)if
        _ab:sub(aab,aab)=="]"then aab=aab+1;break end;dab,aab=cba(_ab,aab)
        bab[cab]=dab;cab=cab+1;aab=_da(_ab,aab,_ca,true)local _bb=_ab:sub(aab,aab)aab=aab+
        1;if _bb=="]"then break end;if _bb~=","then
        ada(_ab,aab,"expected ']' or ','")end end;return bab,aab end
        local function c_b(_ab,aab)local bab={}aab=aab+1
        while 1 do local cab,dab;aab=_da(_ab,aab,_ca,true)if
        _ab:sub(aab,aab)=="}"then aab=aab+1;break end;if _ab:sub(aab,aab)~='"'then
        ada(_ab,aab,"expected string for key")end;cab,aab=cba(_ab,aab)
        aab=_da(_ab,aab,_ca,true)if _ab:sub(aab,aab)~=":"then
        ada(_ab,aab,"expected ':' after key")end;aab=_da(_ab,aab+1,_ca,true)
        dab,aab=cba(_ab,aab)bab[cab]=dab;aab=_da(_ab,aab,_ca,true)
        local _bb=_ab:sub(aab,aab)aab=aab+1;if _bb=="}"then break end;if _bb~=","then
        ada(_ab,aab,"expected '}' or ','")end end;return bab,aab end
        local d_b={['"']=dda,["0"]=__b,["1"]=__b,["2"]=__b,["3"]=__b,["4"]=__b,["5"]=__b,["6"]=__b,["7"]=__b,["8"]=__b,["9"]=__b,["-"]=__b,["t"]=a_b,["f"]=a_b,["n"]=a_b,["["]=b_b,["{"]=c_b}
        cba=function(_ab,aab)local bab=_ab:sub(aab,aab)local cab=d_b[bab]
        if cab then return cab(_ab,aab)end
        ada(_ab,aab,"unexpected character '"..bab.."'")end
        function c_a.decode(_ab)if type(_ab)~="string"then
        error("expected argument of type string, got "..type(_ab))end
        local aab,bab=cba(_ab,_da(_ab,1,_ca,true))bab=_da(_ab,bab,_ca,true)if bab<=#_ab then
    ada(_ab,bab,"trailing garbage")end;return aab end;return c_a
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

_G.utils = __luapack_require__(1)
_G.json = __luapack_require__(2)
-- MARK: Astra

_G.Astra = {
    version = "0.0.0",
    hostname = "127.0.0.1",
    port = 20001
}

---@param path string The URL path for the request.
---@param callback fun(request: Request): any A function that will be called when the request is made.
function Astra.get(path, callback)
    table.insert(Astra, { path = path, method = "get", func = callback })
end

---@param path string The URL path for the request.
---@param callback fun(request: Request): any A function that will be called when the request is made.
function Astra.post(path, callback)
    table.insert(Astra, { path = path, method = "post", func = callback })
end

---@param path string The URL path for the request.
---@param callback fun(request: Request): any A function that will be called when the request is made.
function Astra.put(path, callback)
    table.insert(Astra, { path = path, method = "put", func = callback })
end

---@param path string The URL path for the request.
---@param callback fun(request: Request): any A function that will be called when the request is made.
function Astra.delete(path, callback)
    table.insert(Astra, { path = path, method = "delete", func = callback })
end

---@param path string The URL path for the request.
---@param callback fun(request: Request): any A function that will be called when the request is made.
function Astra.options(path, callback)
    table.insert(Astra, { path = path, method = "options", func = callback })
end

---@param path string The URL path for the request.
---@param callback fun(request: Request): any A function that will be called when the request is made.
function Astra.patch(path, callback)
    table.insert(Astra, { path = path, method = "patch", func = callback })
end

---@param path string The URL path for the request.
---@param callback fun(request: Request): any A function that will be called when the request is made.
function Astra.trace(path, callback)
    table.insert(Astra, { path = path, method = "trace", func = callback })
end

---
---Registers a static folder to serve
---@param path string The URL path for the request.
---@param serve_path string The directory path relatively
function Astra.static_dir(path, serve_path)
    table.insert(Astra, { path = path, method = "static_dir", func = function() end, static_dir = serve_path })
end

---
---Registers a static file to serve
---@param path string The URL path for the request.
---@param serve_path string The directory path relatively
function Astra.static_file(path, serve_path)
    table.insert(Astra, { path = path, method = "static_file", func = function() end, static_file = serve_path })
end

-- MARK: Internal

---
--- Represents an HTTP request.
---@class Request
---@field method fun(): string Returns the HTTP method (e.g., "GET", "POST").
---@field uri fun(): string Returns the URI of the request.
---@field headers fun(): table Returns a table containing the headers of the request.
---@field body fun(): string|nil Returns the body of the request, which can be a table or a string.

---
--- SQLx driver for PostgreSQL
---@class Database
_G.Database = {}

---@param sql string The SQL query to execute.
---@param parameters table Optional table containing the parameters to bind to the query.
function Database:execute(sql, parameters) end

---
---@param sql string The SQL query to execute that returns one row.
---@param parameters table Optional table containing the parameters to bind to the query.
---@return table|nil row a table representing the result row if successful, or `nil` on failure.
function Database:query_one(sql, parameters) end

---
---@param sql string The SQL query to execute that returns multiple rows.
---@param parameters table Optional table containing the parameters to bind to the query.
---@return table|nil rows a table containing all result rows if successful, or `nil` on failure.
function Database:query_all(sql, parameters) end

---
---Opens a new PostgreSQL connection using the provided URL and returns a table representing the connection.
---@param url string The URL of the PostgreSQL database to connect to.
---@return Database Database that represents the PostgreSQL connection.
---@nodiscard
---@diagnostic disable-next-line: missing-return, lowercase-global
function database_connect(url) end
