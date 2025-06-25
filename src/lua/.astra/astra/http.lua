---@meta

Astra.http = {}

---Represents an HTTP body.
---@class Body
---@field text fun(): string Returns the body as text
---@field json fun(): table Returns the body parsed as JSON -> Lua Table

---Represents a multipart.
---@class Multipart
---@field save_file fun(multipart: Multipart, file_path: string | nil): string | nil Saves the multipart into disk

---Represents an HTTP request.
---@class Request
---@field method     fun(request: Request): string Returns the HTTP method (e.g., "GET", "POST").
---@field uri        fun(request: Request): string Returns the URI of the request.
---@field queries    fun(request: Request): table Returns the query list.
---@field headers    fun(request: Request): table Returns a table containing the headers of the request.
---@field body       fun(request: Request): Body|nil Returns the body of the request, which can be a table or a string.
---@field multipart  fun(request: Request): Multipart|nil Returns a multipart if available.
---@field get_cookie fun(request: Request, name: string): Cookie Returns a cookie
---@field new_cookie fun(request: Request, name: string, value: string): Cookie Returns a cookie

---Represents an HTTP response.
---@class Response
---@field set_status_code fun(response: Response, new_status_code: number) Sets the HTTP status code of the response
---@field set_header      fun(response: Response, key: string, value: string) Sets a header
---@field get_headers     fun(response: Response): table|nil Returns the entire headers list that so far has been set for the response
---@field remove_header   fun(response: Response, key: string) Removes a header from the headers list
---@field set_cookie      fun(response: Response, cookie: Cookie) Sets a cookie
---@field remove_cookie   fun(response: Response, cookie: Cookie) Removes a cookie from the list


---Represents an HTTP client request.
---@class HTTPClientRequest
---@field set_method   fun(http_request: HTTPClientRequest, method: string): HTTPClientRequest Sets the HTTP method
---@field set_header   fun(http_request: HTTPClientRequest, key: string, value: string): HTTPClientRequest Sets a header
---@field set_headers  fun(http_request: HTTPClientRequest, headers: table): HTTPClientRequest Sets all of the headers
---@field set_form     fun(http_request: HTTPClientRequest, key: string, value: string): HTTPClientRequest Sets a form
---@field set_forms    fun(http_request: HTTPClientRequest, headers: table): HTTPClientRequest Sets all of the forms
---@field set_body     fun(http_request: HTTPClientRequest, body: string): HTTPClientRequest Sets the HTTP body
---@field set_json     fun(http_request: HTTPClientRequest, json: table): HTTPClientRequest Sets the HTTP json
---@field set_file     fun(http_request: HTTPClientRequest, file_path: string): HTTPClientRequest Sets the for-upload file path
---@field execute      fun(): HTTPClientResponse Executes the request and returns the response
---@field execute_task fun(http_request: HTTPClientRequest, callback: http_client_callback) Executes the request as an async task

---@alias http_client_callback fun(response: HTTPClientResponse)

---Represents an HTTP client response.
---@class HTTPClientResponse
---@field status_code    fun(): table Gets the response HTTP Status code
---@field body           fun(): Body Gets the response HTTP Body which further can be parsed
---@field headers        fun(): table|nil Returns the entire headers list from the HTTP response
---@field remote_address fun(): string|nil Gets the remote address of the HTTP response server

---Opens a new async HTTP Request. The request is running as a task in parallel
---@param url string
---@return HTTPClientRequest
---@nodiscard
function Astra.http.request(url) end

---@type HTTPServer
Astra.http.server = nil

---@class HTTPServer
---@field version string 0.23.0
---@field hostname string "127.0.0.1"
---@field compression boolean false
---@field port number 8080
---@field routes Route[]
---@field __index HTTPServer
---@field new         fun(server: HTTPServer): HTTPServer
---@field get         fun(server: HTTPServer, path: string, callback: callback, config: RouteConfiguration?)
---@field post        fun(server: HTTPServer, path: string, callback: callback, config: RouteConfiguration?)
---@field put         fun(server: HTTPServer, path: string, callback: callback, config: RouteConfiguration?)
---@field delete      fun(server: HTTPServer, path: string, callback: callback, config: RouteConfiguration?)
---@field options     fun(server: HTTPServer, path: string, callback: callback, config: RouteConfiguration?)
---@field patch       fun(server: HTTPServer, path: string, callback: callback, config: RouteConfiguration?)
---@field trace       fun(server: HTTPServer, path: string, callback: callback, config: RouteConfiguration?)
---@field static_dir  fun(server: HTTPServer, path: string, serve_path: string, config: RouteConfiguration?)
---@field static_file fun(server: HTTPServer, path: string, serve_path: string, config: RouteConfiguration?)
---@field run         fun(server: HTTPServer) Runs the server

---@alias callback fun(request: Request, response: Response): any

---@class RouteConfiguration
---@field body_limit? number

---@class Route
---@field path string
---@field method string
---@field func function
---@field static_dir string?
---@field static_file string?
---@field config RouteConfiguration?

---@class Cookie
---@field set_name       fun(cookie: Cookie, name: string)
---@field set_value      fun(cookie: Cookie, value: string)
---@field set_domain     fun(cookie: Cookie, domain: string)
---@field set_path       fun(cookie: Cookie, path: string)
---@field set_expiration fun(cookie: Cookie, expiration: number)
---@field set_http_only  fun(cookie: Cookie, http_only: boolean)
---@field set_max_age    fun(cookie: Cookie, max_age: number)
---@field set_permanent  fun(cookie: Cookie)
---@field get_name       fun(cookie: Cookie): string?
---@field get_value      fun(cookie: Cookie): string?
---@field get_domain     fun(cookie: Cookie): string?
---@field get_path       fun(cookie: Cookie): string?
---@field get_expiration fun(cookie: Cookie): number?
---@field get_http_only  fun(cookie: Cookie): boolean?
---@field get_max_age    fun(cookie: Cookie): number?
