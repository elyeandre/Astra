# HTTP Client

Sometimes your server needs to access other servers and make an HTTP Request, for these cases Astra provides a HTTP Client function inspired by JavaScript's Fetch api. However unlike JS Fetch, this http client awaits by default. You would need to wrap them in [Async Tasks](./async_tasks.md) if you wish to not interrupt the execution flow.

```lua
-- By default its always a GET request
http_request("https://myip.wtf/json", nil, function(response)
    pretty_print(response:status_code())
    pretty_print(response:headers())
    pretty_print(response:remote_address())
    pretty_print(response:body():json())
end)
```

The second argument is optional lua table that can change the way your HTTP Request is performed. The available options are:

- Method. You can pick between one of these:

  - GET,
  - POST,
  - PUT,
  - PATCH,
  - DELETE,
  - HEAD,

- Headers which are a key-value table that are set as request headers
- Body which can be any value that you wish to be sent as the request body
- Form which are key-value table that is then converted into an HTTP form
