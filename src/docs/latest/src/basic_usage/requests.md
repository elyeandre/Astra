# Requests

Requests are provided as the first argument of the route callbacks as a table (not deseralized). Each request in the route callbacks can be accessed through its methods. The following methods are available:

- body: `Body`
- headers: `table<string, string>`
- uri: `string`
- queries: `table<any, any>`
- method: `string`
- multipart: `Multipart`

where Body has:

- text: `string`
- json: `table`

and where Multipart has:

- `save_file(file_path: string | nil)`

Example:

```lua
Astra:get("/", function(req)
    -- access the headers
    pretty_print(req:headers())

    -- print the body as text
    print(req:body():text())
end)
```
