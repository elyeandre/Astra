# Astra

[![Release](https://github.com/ArkForgeLabs/Astra/actions/workflows/release.yml/badge.svg)](https://github.com/ArkForgeLabs/Astra/actions/workflows/release.yml)
[![Static Badge](https://img.shields.io/badge/Join-The_Discord-blue?style=flat&logo=discord&color=blue)](https://discord.com/invite/6PMjUx8x3b)
[![Static Badge](https://img.shields.io/badge/Read_The_Docs-blue?style=flat&logo=docsdotrs&color=%23000000)](https://astra.arkforge.net/docs/latest)

Experimentational web framework for Lua 5.1 running on Axum for potential use at ArkForge. The goal is to get as much performance as possible and write web server logic in lua instead for faster iteration, fault-tolerance and no-build requirements.

> [!Note]
> The performance has not been thoroughly tested, but it should be expected to be close to Rust as this is a thin wrapper. However the fault-tolerance can be ensured through [error handling policies](https://github.com/ArkForgeLabs/Astra/blob/main/src/main.rs#L1-L2) set within the codebase.

## Example

Typically in lua you can register routes like below, binded to a method

```lua
Astra.get("/", function()
    return "hello from default Astra instance!"
end)
```

You can also use the local variables within routes

```lua
local counter = 0
Astra.get("/count", function()
    counter = counter + 1
    -- and also can return JSON
    return { counter }
end)
```

Requests and Responses and their configuration are provided when needed

```lua
Astra.get("/", function(req, res)
    -- set header code
    res:set_status_code(300)
    -- set headers
    res:set_header("header-key", "header-value")

    -- consume the request body
    print(req:body():text())

    return "Responding with Code 300 cuz why not"
end)
```

There are also utilities provided such as a PostgreSQL, http client requests, lua extra utils, and async tasks.

```lua
-- spawn an async task that does not block the running thread
new_task(function ()
    -- HTTP Request to check your IP address
    http_request("https://myip.wtf/json", nil, function(response)
        pretty_print(response:status_code())
        pretty_print(response:remote_address())
        pretty_print(response:body():json())
    end)
end)
```

## Note

This project is NOT recommended for production. It is very much experimental and at the very early staged. Contributions, however, are welcome.

Some features that are missing but are planned:

* [ ] Websockets
* [ ] TLS/SSL support
* [ ] Middleware
* [ ] SQLite support
* [ ] Templating
