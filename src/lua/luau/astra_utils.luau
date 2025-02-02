--!strict

-- MARK: Database

type AnyTable = { [any]: any }

--- Represents an HTTP body.
type Body = {
	--- Returns the body as text
	text: (body: Body) -> string,
	--- Returns the body parsed as JSON -> Lua Table
	json: (body: Body) -> { [any]: any },
}

--- SQLx driver for PostgreSQL
type Database = {
	--- @param sql string The SQL query to execute.
	--- @param parameters table Optional table containing the parameters to bind to the query.
	execute: (database: Database, sql: string, parameters: AnyTable) -> (),
	--- @param sql string The SQL query to execute that returns one row.
	--- @param parameters table Optional table containing the parameters to bind to the query.
	query_one: (database: Database, sql: string, parameters: AnyTable) -> AnyTable,
	--- @param sql string The SQL query to execute that returns multiple rows.
	--- @param parameters table Optional table containing the parameters to bind to the query.
	query_all: (database: Database, sql: string, parameters: { AnyTable }) -> AnyTable,
}

--- Opens a new PostgreSQL connection using the provided URL and returns a table representing the connection.
--- @param url string The URL of the PostgreSQL database to connect to.
local function database_connect(url: string): Database
	return {} :: Database
end
_G.database_connect = database_connect

-- MARK: HTTP Client

---
--- Represents an HTTP client response.
type HTTPClientResponse = {
	--- Gets the response HTTP Status code
	status_code: (http_client_response: HTTPClientResponse) -> AnyTable,
	--- Gets the response HTTP Body which further can be parsed
	body: (http_client_response: HTTPClientResponse) -> Body,
	--- Returns the entire headers list from the HTTP response
	headers: (http_client_response: HTTPClientResponse) -> { [string]: string }?,
	--- Gets the remote address of the HTTP response server
	remote_address: (http_client_response: HTTPClientResponse) -> string?,
}

--- Represents an HTTP client request.
type HTTPClientRequest = {
	--- Sets the HTTP method
	set_method: (http_request: HTTPClientRequest, method: string) -> HTTPClientRequest,
	--- Sets a header
	set_header: (http_request: HTTPClientRequest, key: string, value: string) -> HTTPClientRequest,
	--- Sets all of the headers
	set_headers: (http_request: HTTPClientRequest, headers: { [string]: string }) -> HTTPClientRequest,
	--- Sets a form
	set_form: (http_request: HTTPClientRequest, key: string, value: string) -> HTTPClientRequest,
	--- Sets all of the forms
	set_forms: (http_request: HTTPClientRequest, headers: { [string]: string }) -> HTTPClientRequest,
	--- Sets the HTTP body
	set_body: (http_request: HTTPClientRequest, body: string) -> HTTPClientRequest,
	--- Sets the HTTP json
	set_json: (http_request: HTTPClientRequest, json: AnyTable) -> HTTPClientRequest,
	--- Sets the for-upload file path
	set_file: (http_request: HTTPClientRequest, file_path: string) -> HTTPClientRequest,
	--- Executes the request and returns the response
	execute: (http_request: HTTPClientRequest) -> HTTPClientResponse,
	--- Executes the request as an async task
	execute_task: (http_request: HTTPClientRequest, callback: http_client_callback) -> (),
}

type http_client_callback = (response: HTTPClientResponse) -> ()

--- Opens a new async HTTP Request. The request is running as a task in parallel
--- @param url string
local function http_request(url): HTTPClientRequest
	return {} :: HTTPClientRequest
end
_G.http_request = http_request

-- MARK: Async Tasks

---
--- Represents an async task
type TaskHandler = {
	--- Aborts the running task
	abort: () -> (),
}

type TaskCallback = () -> ()

--- Starts a new async task
--- @param callback The callback to run the content of the async task
local function spawn_task(callback: TaskCallback): TaskHandler
	return {} :: TaskHandler
end
_G.spawn_task = spawn_task

--- Starts a new async task with a delay in milliseconds
--- @param callback The callback to run the content of the async task
--- @param timeout The delay in milliseconds
function spawn_timeout(callback: TaskHandler, timeout: number): TaskHandler
	return {} :: TaskHandler
end
_G.spawn_timeout = spawn_timeout

--- Starts a new async task that runs infinitely in a loop but with a delay in milliseconds
--- @param callback The callback to run the content of the async task
--- @param timeout The delay in milliseconds
function spawn_interval(callback: TaskHandler, timeout: number): TaskHandler
	return {} :: TaskHandler
end
_G.spawn_interval = spawn_interval

-- MARK: Crypto

--- Hashes a given string according to the provided hash type.
--- @param hash_type Current available types: sha2_256, sha2_512, sha3_256, sha3_512
--- @param input The input to be hashed
function hash(hash_type: string, input: string): string
	return ""
end
_G.hash = hash

--- Encodes the given input as Base64
--- @param input The input to be encoded
function base64_encode(input: string): string
	return ""
end
_G.base64_encode = base64_encode

--- Encodes the given input as Base64 but URL safe
--- @param input The input to be encoded
function base64_encode_urlsafe(input: string): string
	return ""
end
_G.base64_encode_urlsafe = base64_encode_urlsafe

--- Decodes the given input as Base64
--- @param input The input to be decoded
function base64_decode(input: string): string
	return ""
end
_G.base64_decode = base64_decode

--- Decodes the given input as Base64 but URL safe
--- @param input The input to be decoded
function base64_decode_urlsafe(input: string): string
	return ""
end
_G.base64_decode_urlsafe = base64_decode_urlsafe
