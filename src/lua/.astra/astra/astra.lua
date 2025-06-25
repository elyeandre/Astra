---@meta

---@class Astra
---@field version string 
Astra = {
    version = "0.23.1", -- Default
}

---Schema validation function with support for nested tables and arrays of tables
---@param input_table table
---@param schema table
---@return boolean, string | nil
function Astra.validate_table(input_table, schema) end

---@class Regex
---@field captures fun(regex: Regex, content: string): string[][]
---@field replace  fun(regex: Regex, content: string, replacement: string, limit: number?): string
---@field is_match fun(regex: Regex, content: string): boolean

---@param expression string
---@return Regex
function Astra.regex(expression) end

---Load your own file into env
---@param file_path string
function Astra.dotenv_load(file_path) end


---Jinja2 templating engine
---@class TemplateEngine
---@field add_template fun(templates: TemplateEngine, name: string, template: string)
---@field add_template_file fun(templates: TemplateEngine, name: string, path: string)
---@field get_template_names fun(template: TemplateEngine): string[]
---Excludes template files from being added to the server for rendering
---@field exclude_templates fun(templates: TemplateEngine, names: string[])
---@field reload_templates fun(templates: TemplateEngine) Refreshes the template code from the glob given at the start
---@field add_function fun(templates: TemplateEngine, name: string, function: template_function): any Add a function to the templates
---Renders the given template into a string with the available context
---@field render fun(templates: TemplateEngine, name: string, context?: table): string
---@field add_to_server fun(templates: TemplateEngine, server: HTTPServer, context?: table) Adds the templates to the server
---Adds the templates to the server in debugging manner, where the content refreshes on each request
---@field add_to_server_debug fun(templates: TemplateEngine, server: HTTPServer, context?: table)

---@alias template_function fun(args: table): any

---Returns a new templating engine
---@param dir? string path to the directory, for example: `"templates/**/[!exclude.html]*.html"`
---@return TemplateEngine
---@nodiscard
function Astra.new_templating_engine(dir) end


---SQLx driver
---@class Database
---@field execute fun(database: Database, sql: string, parameters: table | nil)
---@field query_one fun(database: Database, sql: string, parameters: table | nil): table | nil
---@field query_all fun(database: Database, sql: string, parameters: table | nil): table | nil
---@field close fun(database: Database)

---Opens a new SQL connection using the provided URL and returns a table representing the connection.
---@param database_type "sqlite"|"postgres" The type of database to connect to.
---@param url string The URL of the SQL database to connect to.
---@param max_connections number? Max number of connections to the database pool
---@return Database Database that represents the SQL connection.
---@nodiscard
function Astra.database_connect(database_type, url, max_connections) end

