# DateTime

Astra provides date & time functionality as an extension to the standard library. It is loosely inspired by the [JavaScript Date](https://www.w3schools.com/jsref/jsref_obj_date.asp) implementation. It contains 4 constructors and 25 methods as of now.

## Quick View
- [Constructors](#constructors)
  - [`new_now()`](#new_now)
  - [`new_from()`](#new_from)
  - [`new_utc_now()`](#new_utc_now)
  - [`new_utc_from()`](#new_utc_from)
- [Functions](#functions)
  - Getters
    - [`get_year()`](#get_year)
    - [`get_month()`](#get_month)
    - [`get_day()`](#get_day)
    - [`get_weekday()`](#get_weekday)
    - [`get_hour()`](#get_hour)
    - [`get_minute()`](#get_minute)
    - [`get_second()`](#get_second)
    - [`get_millisecond()`](#get_millisecond)
    - [`get_epoch_milliseconds()`](#get_epoch_milliseconds)
    - [`get_timezone_offset()`](#get_timezone_offset)
  - Setters
    - [`set_year()`](#set_yearyear-number)
    - [`set_month()`](#set_monthmonth-number)
    - [`set_day()`](#set_dayday-number)
    - [`set_hour()`](#set_hourhour-number)
    - [`set_minute()`](#set_minutemin-number)
    - [`set_second()`](#set_secondsec-number)
    - [`set_millisecond()`](#set_millisecondmilli-number)
    - [`set_epoch_milliseconds()`](#set_epoch_millisecondsmilli-number)
  - Formats
    - [`to_date_string()`](#to_date_string)
    - [`to_time_string()`](#to_time_string)
    - [`to_datetime_string()`](#to_datetime_string)
    - [`to_iso_string()`](#to_iso_string)
    - [`to_locale_date_string()`](#to_locale_date_string)
    - [`to_locale_time_string()`](#to_locale_time_string)
    - [`to_locale_datetime_string()`](#to_locale_datetime_string)

## Constructors

### `new_now()`
Returns a DateTime object which corresponds to the current date, time & local offset from UTC.
```lua
local dt = Astra.datetime.new_now()
```

### `new_from()`
Returns a DateTime object which corresponds to the provided date and/or time arguments & local offset from UTC.
```lua
-- You can provide the following arguments:
--     year: number
--     month: number?
--     day: number?
--     hour: number?
--     min: number?
--     sec: number?
--     milli: number?
--
-- Providing the year is mandatory.
-- The other values, if not provided, will default to the following in respective order: 1, 1, 0, 0, 0, 0

local dt = Astra.datetime.new_from(2001, 7, 8, 0, 24, 48, 241)
```

### `new_utc_now()`
Returns a DateTime object which corresponds to the current date & time in UTC.
```lua
local dt = Astra.datetime.new_utc_now()
```

### `new_utc_from()`
Returns a DateTime object which corresponds to the provided date and/or time arguments in UTC.
```lua
-- You can provide the following arguments:
--     year: number
--     month: number?
--     day: number?
--     hour: number?
--     min: number?
--     sec: number?
--     milli: number?
-- Providing the year is mandatory.
-- The other values, if not provided, will default to the following in respective order: 1, 1, 0, 0, 0, 0

local dt = Astra.datetime.new_utc_from(2001, 7, 8, 0, 24, 48, 241)
```

## Functions

### `get_year()`
Returns the year number in the calendar date.

### `get_month()`
Returns the month number starting from 1.

### `get_day()`
Returns the day of the month starting from 1.

### `get_weekday()`
Returns a day-of-the-week number starting from Sunday = 0

### `get_hour()`
Returns the hour number from 0 to 23.

### `get_minute()`
Returns the minute number from 0 to 59.

### `get_second()`
Returns the second number from 0 to 59.

### `get_millisecond()`
Returns the number of milliseconds since the last second boundary.
In the event of a leap second, this may exceed 999.

### `get_epoch_milliseconds()`
Returns the number of non-leap-milliseconds since January 1, 1970 UTC.

### `get_timezone_offset()`
Returns the number of minutes to add to convert from UTC to the local time.

### `set_year(year: number)`
Changes the year number of the DateTime object.

### `set_month(month: number)`
Changes the month number of the DateTime object. 

### `set_day(day: number)`
Changes the day-of-the-month number of the DateTime object.

### `set_hour(hour: number)`
Changes the hour number of the DateTime object.

### `set_minute(min: number)`
Changes the minute number of the DateTime object.

### `set_second(sec: number)`
Changes the second number of the DateTime object.

### `set_millisecond(milli: number)`
Changes the millisecond number since the last second boundary of the DateTime object.

### `set_epoch_milliseconds(milli: number)`
Changes the number of non-leap milliseconds since January 1, 1970 UTC of the DateTime object.

### `to_date_string()`
Returns the date stored in the DateTime object as a string in the format `Sun Jul 08 2001`.

### `to_time_string()`
Returns the time stored in the DateTime object as a string in the format `00:34:48 ACST+0930`.

### `to_datetime_string()`
Returns the date & time stored in the DateTime object as a string in the format `Sun Jul 08 2001 00:34:48 ACST+0930`.

### `to_iso_string()`
Returns the DateTime object as a string in the ISO8601 format with millisecond precision.

### `to_locale_date_string()`
Returns the date stored in the DateTime object as a string in your locale's format.

### `to_locale_time_string()`
Returns the time stored in the DateTime object as a string in your locale's format.

### `to_locale_datetime_string()`
Returns the date & time stored in the DateTime object as a string in your locale's format.
