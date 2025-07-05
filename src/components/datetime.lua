---@meta

---@class DateTime
---@field new_now fun(): DateTime
---@field new_from fun(year: number, month: number?, day: number?, hour: number?, min: number?, sec: number?, milli: number?): DateTime
---@field new_utc_now fun(): DateTime
---@field new_utc_from fun(year: number, month: number?, day: number?, hour: number?, min: number?, sec: number?, milli: number?): DateTime
---@field get_year fun(datetime: DateTime): number
---@field get_month fun(datetime: DateTime): number
---@field get_day fun(datetime: DateTime): number
---@field get_weekday fun(datetime: DateTime): number
---@field get_hour fun(datetime: DateTime): number
---@field get_minute fun(datetime: DateTime): number
---@field get_second fun(datetime: DateTime): number
---@field get_millisecond fun(datetime: DateTime): number
---@field get_epoch_milliseconds fun(datetime: DateTime): number
---@field get_timezone_offset fun(datetime: DateTime): number
---@field set_year fun(datetime:DateTime, year: number)
---@field set_month fun(datetime:DateTime, month: number)
---@field set_day fun(datetime:DateTime, day: number)
---@field set_hour fun(datetime:DateTime, hour: number)
---@field set_minute fun(datetime:DateTime, min: number)
---@field set_second fun(datetime:DateTime, sec: number)
---@field set_millisecond fun(datetime:DateTime, milli: number)
---@field set_epoch_milliseconds fun(datetime: DateTime, milli: number)
---@field to_date_string fun(datetime: DateTime): string?
---@field to_time_string fun(datetime: DateTime): string?
---@field to_datetime_string fun(datetime: DateTime): string?
---@field to_iso_string fun(datetime: DateTime): string?
---@field to_locale_date_string fun(datetime: DateTime): string?
---@field to_locale_time_string fun(datetime: DateTime): string?
---@field to_locale_datetime_string fun(datetime: DateTime): string?

Astra.datetime = {}

---@return DateTime
function Astra.datetime.new_now()
	---@diagnostic disable-next-line: undefined-global
	return astra_internal__datetime_new_now()
end

---@return DateTime
function Astra.datetime.new_utc_now()
	---@diagnostic disable-next-line: undefined-global
	return astra_internal__datetime_new_now(true)
end

---@type fun(year: number, month: number?, day: number?, hour: number?, min: number?, sec: number?, milli: number?): DateTime
function Astra.datetime.new_from(year, month, day, hour, min, sec, milli)
	---@diagnostic disable-next-line: undefined-global
	return astra_internal__datetime_new_from(year, month, date, hour, min, sec, milli, false)
end

---@type fun(year: number, month: number?, day: number?, hour: number?, min: number?, sec: number?, milli: number?): DateTime
function Astra.datetime.new_utc_from(year, month, day, hour, min, sec, milli)
	---@diagnostic disable-next-line: undefined-global
	return astra_internal__datetime_new_from(year, month, date, hour, min, sec, milli, true)
end
