# Cookies

Cookies allow you to store data on each HTTP request, if supported. Astra does not currently support signed and private cookies. You can create a new cookie by getting it from a request:

```lua
Astra:get("/", function(request)
    local cookie = request:new_cookie("key", "value")

    return "HEY"
end)
```

You can also get a previously set cookie:

```lua
local cookie = request:get_cookie("key")
```

After modification or creation, they will have no effect unless you set them to the response

```lua
response:set_cookie("key", cookie)
```

And similary, remove them with

```lua
response:removoe_cookie("key")
```

Each cookie contains extra details and functions which are as follows:

```lua
set_name(cookie: Cookie, name: string)
set_value(cookie: Cookie, value: string)
set_domain(cookie: Cookie, domain: string)
set_path(cookie: Cookie, path: string)
set_expiration(cookie: Cookie, expiration: number)
set_http_only(cookie: Cookie, http_only: boolean)
set_max_age(cookie: Cookie, max_age: number)
set_permanent(cookie: Cookie)
get_name(cookie: Cookie): string?
get_value(cookie: Cookie): string?
get_domain(cookie: Cookie): string?
get_path(cookie: Cookie): string?
get_expiration(cookie: Cookie): number?
get_http_only(cookie: Cookie): boolean?
get_max_age(cookie: Cookie): number?
```
