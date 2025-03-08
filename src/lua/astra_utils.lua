--!nocheck
-- MARK: Utils

---
--- SQLx driver
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
---Opens a new SQL connection using the provided URL and returns a table representing the connection.
---@param database_type "sqlite"|"postgres" The type of database to connect to.
---@param url string The URL of the SQL database to connect to.
---@param max_connections number? Max number of connections to the database pool
---@return Database Database that represents the SQL connection.
---@nodiscard
---@diagnostic disable-next-line: missing-return, lowercase-global
function database_connect(database_type, url, max_connections) end

---
--- Represents an HTTP client response.
---@class HTTPClientResponse
---@field status_code fun(): table Gets the response HTTP Status code
---@field body fun(): Body Gets the response HTTP Body which further can be parsed
---@field headers fun(): table|nil Returns the entire headers list from the HTTP response
---@field remote_address fun(): string|nil Gets the remote address of the HTTP response server

---@diagnostic disable-next-line: duplicate-doc-alias
---@alias http_client_callback fun(response: HTTPClientResponse)

---
--- Represents an HTTP client request.
---@class HTTPClientRequest
---@field set_method fun(http_request: HTTPClientRequest, method: string): HTTPClientRequest Sets the HTTP method
---@field set_header fun(http_request: HTTPClientRequest, key: string, value: string): HTTPClientRequest Sets a header
---@field set_headers fun(http_request: HTTPClientRequest, headers: table): HTTPClientRequest Sets all of the headers
---@field set_form fun(http_request: HTTPClientRequest, key: string, value: string): HTTPClientRequest Sets a form
---@field set_forms fun(http_request: HTTPClientRequest, headers: table): HTTPClientRequest Sets all of the forms
---@field set_body fun(http_request: HTTPClientRequest, body: string): HTTPClientRequest Sets the HTTP body
---@field set_json fun(http_request: HTTPClientRequest, json: table): HTTPClientRequest Sets the HTTP json
---@field set_file fun(http_request: HTTPClientRequest, file_path: string): HTTPClientRequest Sets the for-upload file path
---@field execute fun(): HTTPClientResponse Executes the request and returns the response
---@field execute_task fun(http_request: HTTPClientRequest, callback: http_client_callback) Executes the request as an async task

---
---Opens a new async HTTP Request. The request is running as a task in parallel
---@param url string
---@return HTTPClientRequest
---@nodiscard
---@diagnostic disable-next-line: missing-return, lowercase-global
function http_request(url) end

---
--- Represents an async task
---@class TaskHandler
---@field abort fun() Aborts the running task

---
---Starts a new async task
---@param callback fun() | nil The callback to run the content of the async task
---@return TaskHandler
---@diagnostic disable-next-line: missing-return, lowercase-global
function spawn_task(callback) end

---
---Starts a new async task with a delay in milliseconds
---@param callback fun() | nil The callback to run the content of the async task
---@param timeout number The delay in milliseconds
---@return TaskHandler
---@diagnostic disable-next-line: missing-return, lowercase-global
function spawn_timeout(callback, timeout) end

---
---Starts a new async task that runs infinitely in a loop but with a delay in milliseconds
---@param callback fun() | nil The callback to run the content of the async task
---@param timeout number The delay in milliseconds
---@return TaskHandler
---@diagnostic disable-next-line: missing-return, lowercase-global
function spawn_interval(callback, timeout) end

-- MARK: Crypto

---
---Hashes a given string according to the provided hash type.
---@param hash_type string Current available types: sha2_256, sha2_512, sha3_256, sha3_512
---@param input string The input to be hashed
---@return string
---@diagnostic disable-next-line: missing-return, lowercase-global
function hash(hash_type, input) end

---
---Encodes the given input as Base64
---@param input string The input to be encoded
---@return string
---@diagnostic disable-next-line: missing-return, lowercase-global
function base64_encode(input) end

---
---Encodes the given input as Base64 but URL safe
---@param input string The input to be encoded
---@return string
---@diagnostic disable-next-line: missing-return, lowercase-global
function base64_encode_urlsafe(input) end

---
---Decodes the given input as Base64
---@param input string The input to be decoded
---@return string
---@diagnostic disable-next-line: missing-return, lowercase-global
function base64_decode(input) end

---
---Decodes the given input as Base64 but URL safe
---@param input string The input to be decoded
---@return string
---@diagnostic disable-next-line: missing-return, lowercase-global
function base64_decode_urlsafe(input) end

