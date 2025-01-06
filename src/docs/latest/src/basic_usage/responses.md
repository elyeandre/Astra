# Responses

Responses are the second argument provided in the route callback. They allow you to modify the response to the way you want. Each response has the default 200 OK status along content header based on your response. The following methods are available:

* `set_status_code(status_code: number)`
* `set_header(key: string, value: string)`
* `remove_header(key: string)`
* `get_headers()`: `table<string, string>`

Example:

```lua
Astra.get("/", function(req, res)
    -- set header code
    res:set_status_code(300)
    -- set headers
    res:set_header("header-key", "header-value")

    return "Responding with Code 300 cuz why not"
end)
```

The headers, as stated, will include content type when sending to user, but can be changed while setting the type yourself.
