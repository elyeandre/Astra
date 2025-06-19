use chrono::prelude::*;
use mlua::UserData;

pub struct LuaDateTime {
    dt: DateTime<FixedOffset>,
}

impl super::AstraComponent for LuaDateTime {
    fn register_to_lua(lua: &mlua::Lua) -> mlua::Result<()> {
        let table = lua.create_table()?;

        table.set(
            "astra_internal__new_now",
            lua.create_function(|_, ()| {
                let dt = Local::now().fixed_offset();
                Ok(Self { dt })
            })?,
        )?;

        table.set(
            "astra_internal__new_from",
            lua.create_function(|_, (
                year,
                month,
                day,
                hour,
                min,
                sec,
                milli
            ): (i32, u32, u32, u32, u32, u32, u32)| {
                let dt = Local
                    .with_ymd_and_hms(year, month, day, hour, min, sec)
                    .unwrap()
                    .with_nanosecond(milli * 1000000)
                    .unwrap()
                    .fixed_offset();

                Ok(Self{dt})
            })?
        )?;

        table.set(
            "astra_internal__new_utc_now",
            lua.create_function(|_, ()| {
                let dt = Utc::now().fixed_offset();
                Ok(Self { dt })
            })?,
        )?;

        table.set(
            "astra_internal__new_utc_from",
            lua.create_function(|_, (
                year,
                month,
                day,
                hour,
                min,
                sec,
                milli
            ): (i32, u32, u32, u32, u32, u32, u32)| {
                let dt = Utc
                    .with_ymd_and_hms(year, month, day, hour, min, sec)
                    .unwrap()
                    .with_nanosecond(milli * 1000000)
                    .unwrap()
                    .fixed_offset();

                Ok(Self{dt})
            })?
        )?;

        Ok(())
    }
}

impl UserData for LuaDateTime {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get_year", |_, this, ()| Ok(this.dt.year()));

        methods.add_method("get_month", |_, this, ()| Ok(this.dt.month()));

        methods.add_method("get_date", |_, this, ()| Ok(this.dt.day()));

        methods.add_method("get_weekday", |_, this, ()| {
            Ok(this.dt.weekday().num_days_from_sunday())
        });

        methods.add_method("get_hour", |_, this, ()| Ok(this.dt.hour()));

        methods.add_method("get_minute", |_, this, ()| Ok(this.dt.minute()));

        methods.add_method("get_second", |_, this, ()| Ok(this.dt.second()));

        methods.add_method("get_epoch_millis", |_, this, ()| {
            Ok(this.dt.timestamp_millis())
        });

        methods.add_method("get_timezone_offset", |_, this, ()| {
            Ok(this.dt.offset().local_minus_utc() / 60)
        });

        methods.add_method_mut("set_year", |_, this, year: i32| {
            this.dt = this.dt.with_year(year).expect("Invalid year!");

            Ok(())
        });

        methods.add_method_mut("set_month", |_, this, month: u32| {
            this.dt = this.dt.with_month(month).expect("Invalid month!");

            Ok(())
        });

        methods.add_method_mut("set_date", |_, this, date: u32| {
            this.dt = this.dt.with_day(date).expect("Invalid date!");

            Ok(())
        });

        methods.add_method_mut("set_hour", |_, this, hour: u32| {
            this.dt = this.dt.with_hour(hour).expect("Invalid hour!");

            Ok(())
        });

        methods.add_method_mut("set_minute", |_, this, min: u32| {
            this.dt = this.dt.with_minute(min).expect("Invalid minute!");
            Ok(())
        });

        methods.add_method_mut("set_second", |_, this, sec: u32| {
            this.dt = this.dt.with_second(sec).expect("Invalid second!");

            Ok(())
        });

        methods.add_method_mut("set_millisecond", |_, this, milli: u32| {
            this.dt = this
                .dt
                .with_nanosecond(milli * 1000000)
                .expect("Invalid nanosecond!");

            Ok(())
        });

        methods.add_method_mut("set_epoch_milliseconds", |_, this, milli: i64| {
            this.dt = DateTime::from_timestamp_millis(milli)
                .expect("Invalid millisecond!")
                .with_timezone(&this.dt.timezone().fix());

            Ok(())
        });

        methods.add_method("to_date_string", |_, this, ()| {
            Ok(format!("{}", this.dt.format("%a %b %d %Y")))
        });

        methods.add_method("to_time_string", |_, this, ()| {
            Ok(format!("{}", this.dt.format("%T %Z%z")))
        });

        methods.add_method("to_datetime_string", |_, this, ()| {
            Ok(format!("{}", this.dt.format("%a %b %d Y %T %Z%z")))
        });

        methods.add_method("to_iso_string", |_, this, ()| {
            Ok(this.dt.to_rfc3339_opts(SecondsFormat::Millis, false))
        });

        methods.add_method("to_locale_date_string", |_, this, ()| {
            Ok(format!("{}", this.dt.format("%x")))
        });

        methods.add_method("to_locale_time_string", |_, this, ()| {
            Ok(format!("{}", this.dt.format("%X")))
        });

        methods.add_method("to_locale_datetime_string", |_, this, ()| {
            Ok(format!("{}", this.dt.format("%c")))
        });
    }
}
