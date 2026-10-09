use std::{cmp::Ordering, convert::TryInto, sync::Arc};

use crate::{
    ExecutionError,
    context::Context,
    magic::{Arguments, This},
    objects::Value,
    parser::Expression,
    resolvers::Resolver,
};

type Result<T> = std::result::Result<T, ExecutionError>;

/// `FunctionContext` is a context object passed to functions when they are called.
///
/// It contains references to the target object (if the function is called as
/// a method), the program context ([`Context`]) which gives functions access
/// to variables, and the arguments to the function call.
#[derive(Clone)]
pub struct FunctionContext<'context> {
    pub name: Arc<String>,
    pub this: Option<Value>,
    pub ptx: &'context Context<'context>,
    pub args: Vec<Expression>,
    pub arg_idx: usize,
}

impl<'context> FunctionContext<'context> {
    pub fn new(
        name: Arc<String>,
        this: Option<Value>,
        ptx: &'context Context<'context>,
        args: Vec<Expression>,
    ) -> Self {
        Self {
            name,
            this,
            ptx,
            args,
            arg_idx: 0,
        }
    }

    /// Resolves the given expression using the program's [`Context`].
    pub fn resolve<R>(&self, resolver: R) -> Result<Value>
    where
        R: Resolver,
    {
        resolver.resolve(self)
    }

    /// Returns an execution error for the currently execution function.
    pub fn error<M: ToString>(&self, message: M) -> ExecutionError {
        ExecutionError::function_error(self.name.as_str(), message)
    }
}

/// Calculates the size of either the target, or the provided args depending on how
/// the function is called.
///
/// If called as a method, the target will be used. If called as a function, the
/// first argument will be used.
///
/// The following [`Value`] variants are supported:
/// * [`Value::List`]
/// * [`Value::Map`]
/// * [`Value::String`]
/// * [`Value::Bytes`]
///
/// # Examples
/// ```skip
/// length([1, 2, 3]) == 3
/// ```
/// ```skip
/// 'foobar'.length() == 6
/// ```
pub fn length(ftx: &FunctionContext, This(this): This<Value>) -> Result<i64> {
    let length = match this {
        Value::List(l) => l.len(),
        Value::Map(m) => m.map.len(),
        Value::String(s) => s.len(),
        Value::Bytes(b) => b.len(),
        value => return Err(ftx.error(format!("cannot determine the length of {value:?}"))),
    };
    Ok(length as i64)
}

/// Returns true if the target contains the provided argument. The actual behavior
/// depends mainly on the type of the target.
///
/// The following [`Value`] variants are supported:
/// * [`Value::List`] - Returns true if the list contains the provided value.
/// * [`Value::Map`] - Returns true if the map contains the provided key.
/// * [`Value::String`] - Returns true if the string contains the provided substring.
/// * [`Value::Bytes`] - Returns true if the bytes contain the provided byte.
///
/// # Example
///
/// ## List
/// ```cel
/// [1, 2, 3].contains(1) == true
/// ```
///
/// ## Map
/// ```cel
/// {"a": 1, "b": 2, "c": 3}.contains("a") == true
/// ```
///
/// ## String
/// ```cel
/// "abc".contains("b") == true
/// ```
///
/// ## Bytes
/// ```cel
/// b"abc".contains(b"c") == true
/// ```
pub fn contains(This(this): This<Value>, arg: Value) -> Result<Value> {
    Ok(match this {
        Value::List(v) => v.contains(&arg),
        Value::Map(v) => v
            .map
            .contains_key(&arg.try_into().map_err(ExecutionError::UnsupportedKeyType)?),
        Value::String(s) => {
            if let Value::String(arg) = arg {
                s.contains(arg.as_str())
            } else {
                false
            }
        }
        Value::Bytes(b) => {
            if let Value::Bytes(arg) = arg {
                let s = arg.as_slice();
                b.windows(arg.len()).any(|w| w == s)
            } else {
                false
            }
        }
        #[cfg(feature = "ip")]
        Value::Ip(v) => {
            if let Value::Ip(arg) = arg {
                let is_arg_single_ip = match arg {
                    ipnetwork::IpNetwork::V4(v4) => v4.prefix() == 32,
                    ipnetwork::IpNetwork::V6(v6) => v6.prefix() == 128,
                };
                is_arg_single_ip && v.contains(arg.ip())
            } else {
                false
            }
        }
        _ => false,
    }
    .into())
}

// Performs a type conversion on the target. The following conversions are currently
// supported:
// * `string` - Returns a copy of the target string.
// * `timestamp` - Returns the timestamp in RFC3339 format.
// * `duration` - Returns the duration in a string formatted like "72h3m0.5s".
// * `int` - Returns the integer value of the target.
// * `uint` - Returns the unsigned integer value of the target.
// * `float` - Returns the float value of the target.
// * `bytes` - Converts bytes to string using from_utf8_lossy.
pub fn string(ftx: &FunctionContext, value: Value) -> Result<Value> {
    Ok(match value {
        Value::String(v) => Value::String(v.clone()),
        #[cfg(feature = "time")]
        Value::Timestamp(t) => Value::String(t.to_string().into()),
        #[cfg(feature = "time")]
        Value::Duration(v) => Value::String(crate::duration::format_duration(&v).into()),
        Value::Int(v) => Value::String(v.to_string().into()),
        // Value::UInt(v) => Value::String(v.to_string().into()),
        Value::Float(v) => Value::String(v.to_string().into()),
        Value::Bytes(v) => Value::String(Arc::new(String::from_utf8_lossy(v.as_slice()).into())),
        #[cfg(feature = "regex")]
        Value::Regex(regex) => Value::String(Arc::new(regex.to_string())),
        #[cfg(feature = "ip")]
        Value::Ip(ip) => Value::String(Arc::new(ip.to_string())),
        v => return Err(ftx.error(format!("cannot convert {v:?} to string"))),
    })
}

pub fn bytes(value: Arc<String>) -> Result<Value> {
    Ok(Value::Bytes(value.as_bytes().to_vec().into()))
}

// Performs a type conversion on the target.
pub fn float(ftx: &FunctionContext, value: Value) -> Result<Value> {
    Ok(match value {
        Value::String(v) => v
            .parse::<f64>()
            .map(Value::Float)
            .map_err(|e| ftx.error(format!("string parse error: {e}")))?,
        Value::Float(v) => Value::Float(v),
        Value::Int(v) => Value::Float(v as f64),
        // Value::UInt(v) => Value::Float(v as f64),
        v => return Err(ftx.error(format!("cannot convert {v:?} to Float"))),
    })
}

// Performs a type conversion on the target.
// pub fn uint(ftx: &FunctionContext, value: Value) -> Result<Value> {
//     Ok(match value {
//         Value::String(v) => v
//             .parse::<u64>()
//             .map(Value::UInt)
//             .map_err(|e| ftx.error(format!("string parse error: {e}")))?,
//         Value::Float(v) => {
//             if v > u64::MAX as f64 || v < u64::MIN as f64 {
//                 return Err(ftx.error("unsigned integer overflow"));
//             }
//             Value::UInt(v as u64)
//         }
//         Value::Int(v) => Value::UInt(
//             v.try_into()
//                 .map_err(|_| ftx.error("unsigned integer overflow"))?,
//         ),
//         Value::UInt(v) => Value::UInt(v),
//         v => return Err(ftx.error(format!("cannot convert {v:?} to uint"))),
//     })
// }

// Performs a type conversion on the target.
pub fn int(ftx: &FunctionContext, value: Value) -> Result<Value> {
    Ok(match value {
        Value::String(v) => v
            .parse::<i64>()
            .map(Value::Int)
            .map_err(|e| ftx.error(format!("string parse error: {e}")))?,
        Value::Float(v) => {
            if v > i64::MAX as f64 || v < i64::MIN as f64 {
                return Err(ftx.error("integer overflow"));
            }
            Value::Int(v as i64)
        }
        Value::Int(v) => Value::Int(v),
        // Value::UInt(v) => Value::Int(v.try_into().map_err(|_| ftx.error("integer overflow"))?),
        v => return Err(ftx.error(format!("cannot convert {v:?} to int"))),
    })
}

/// Returns true if a string starts with another string.
///
/// # Example
/// ```cel
/// "abc".starts_with("a") == true
/// ```
pub fn starts_with(This(this): This<Arc<String>>, prefix: Arc<String>) -> bool {
    this.starts_with(prefix.as_str())
}

/// Returns true if a string ends with another string.
///
/// # Example
/// ```cel
/// "abc".ends_with("c") == true
/// ```
pub fn ends_with(This(this): This<Arc<String>>, suffix: Arc<String>) -> bool {
    this.ends_with(suffix.as_str())
}

/// Returns true if a string matches the regular expression.
///
/// # Example
/// ```cel
/// "abc".matches("^[a-z]*$") == true
/// ```
#[cfg(feature = "regex")]
pub fn matches(
    // ftx: &FunctionContext,
    This(this): This<Arc<String>>,
    regex: regex::Regex,
) -> bool {
    regex.is_match(&this)
    // match &regex {
    //     Value::Regex(regex) => Ok(regex.is_match(&this)),
    //     _ => Err(ftx.error(format!("matches mut be used with a regex"))),
    // }
    // match regex::Regex::new(&regex) {
    //     Ok(re) => Ok(re.is_match(&this)),
    //     Err(err) => Err(ftx.error(format!("'{regex}' not a valid regex:\n{err}"))),
    // }
}

#[cfg(feature = "regex")]
pub fn regex(ftx: &FunctionContext, This(this): This<Value>) -> Result<Value> {
    Ok(match this {
        Value::String(v) => Value::Regex(regex::Regex::new(v.as_str()).map_err(|e| ftx.error(e.to_string()))?),
        v => return Err(ftx.error(format!("cannot convert {v:?} to Regex"))),
    })
}

#[cfg(feature = "time")]
pub use time::duration;
#[cfg(feature = "time")]
pub use time::timestamp;

// Performs a type conversion on the target.
#[cfg(feature = "ip")]
pub fn ip(ftx: &FunctionContext, value: Value) -> Result<Value> {
    match value {
        Value::String(v) => {
            use ipnetwork::IpNetwork;
            let ip: IpNetwork = v
                .parse()
                .map_err(|err| ftx.error(format!("error converting {v:?} to Ip: {err}")))?;
            Ok(Value::Ip(ip))
        }
        v => Err(ftx.error(format!("cannot convert {v:?} to String"))),
    }
}

#[cfg(feature = "time")]
pub mod time {
    use std::sync::Arc;

    use time::{DateTime, TimeZone};

    use super::Result;
    use crate::{ExecutionError, Value, magic::This};

    /// Duration parses the provided argument into a [`Value::Duration`] value.
    ///
    /// The argument must be string, and must be in the format of a duration. See
    /// the [`parse_duration`] documentation for more information on the supported
    /// formats.
    ///
    /// # Examples
    /// - `1h` parses as 1 hour
    /// - `1.5h` parses as 1 hour and 30 minutes
    /// - `1h30m` parses as 1 hour and 30 minutes
    /// - `1h30m1s` parses as 1 hour, 30 minutes, and 1 second
    /// - `1ms` parses as 1 millisecond
    /// - `1.5ms` parses as 1 millisecond and 500 microseconds
    /// - `1ns` parses as 1 nanosecond
    /// - `1.5ns` parses as 1 nanosecond (sub-nanosecond durations not supported)
    pub fn duration(value: Arc<String>) -> crate::functions::Result<Value> {
        Ok(Value::Duration(_duration(value.as_str())?))
    }

    /// Timestamp parses the provided argument into a [`Value::Timestamp`] value.
    pub fn timestamp(value: Arc<String>) -> Result<Value> {
        Ok(Value::Timestamp(_timestamp(value.as_str())?))
    }

    /// A wrapper around [`parse_duration`] that converts errors into [`ExecutionError`].
    /// and only returns the duration, rather than returning the remaining input.
    fn _duration(i: &str) -> Result<crate::duration::Duration> {
        let (_, duration) = crate::duration::parse_duration(i)
            .map_err(|e| ExecutionError::function_error("duration", e.to_string()))?;
        Ok(duration)
    }

    fn _timestamp(i: &str) -> Result<DateTime> {
        i.parse::<DateTime>()
            .map_err(|e| ExecutionError::function_error("timestamp", e.to_string()))
    }

    pub fn timestamp_year(This(this): This<DateTime>) -> Result<Value> {
        Ok(this.year().into())
    }

    pub fn timestamp_month(This(this): This<DateTime>) -> Result<Value> {
        Ok(((this.month() as i32) - 1).into())
    }

    pub fn timestamp_year_day(This(this): This<DateTime>) -> Result<Value> {
        Ok(((this.ordinal() as i32) - 1).into())
    }

    pub fn timestamp_month_day(This(this): This<DateTime>) -> Result<Value> {
        Ok(((this.day() as i32) - 1).into())
    }

    pub fn timestamp_date(This(this): This<DateTime>) -> Result<Value> {
        Ok((this.day() as i32).into())
    }

    pub fn timestamp_weekday(This(this): This<DateTime>) -> Result<Value> {
        Ok((this.weekday().number_from_sunday() as i32).into())
    }

    pub fn timestamp_hours(This(this): This<DateTime>) -> Result<Value> {
        Ok((this.hour() as i32).into())
    }

    pub fn timestamp_minutes(This(this): This<DateTime>) -> Result<Value> {
        Ok((this.minute() as i32).into())
    }

    pub fn timestamp_seconds(This(this): This<DateTime>) -> Result<Value> {
        Ok((this.second() as i32).into())
    }

    pub fn timestamp_millis(This(this): This<DateTime>) -> Result<Value> {
        Ok((this.millisecond() as i32).into())
    }

    pub fn now() -> Result<Value> {
        Ok(Value::Timestamp(DateTime::now_in(TimeZone::UTC)))
    }

    pub fn unix(This(this): This<DateTime>) -> Result<Value> {
        Ok((this.unix()).into())
    }
}

pub fn max(Arguments(args): Arguments) -> Result<Value> {
    // If items is a list of values, then operate on the list
    let items = if args.len() == 1 {
        match &args[0] {
            Value::List(values) => values,
            _ => return Ok(args[0].clone()),
        }
    } else {
        &args
    };

    items
        .iter()
        .skip(1)
        .try_fold(items.first().unwrap_or(&Value::Null), |acc, x| match acc.partial_cmp(x) {
            Some(Ordering::Greater) => Ok(acc),
            Some(_) => Ok(x),
            None => Err(ExecutionError::ValuesNotComparable(acc.clone(), x.clone())),
        })
        .cloned()
}

pub fn min(Arguments(args): Arguments) -> Result<Value> {
    // If items is a list of values, then operate on the list
    let items = if args.len() == 1 {
        match &args[0] {
            Value::List(values) => values,
            _ => return Ok(args[0].clone()),
        }
    } else {
        &args
    };

    items
        .iter()
        .skip(1)
        .try_fold(items.first().unwrap_or(&Value::Null), |acc, x| match acc.partial_cmp(x) {
            Some(Ordering::Less) => Ok(acc),
            Some(_) => Ok(x),
            None => Err(ExecutionError::ValuesNotComparable(acc.clone(), x.clone())),
        })
        .cloned()
}

#[cfg(test)]
mod tests {
    use crate::{context::Context, tests::test_script};

    fn assert_script(input: &(&str, &str)) {
        assert_eq!(test_script(input.1, None), Ok(true.into()), "{}", input.0);
    }

    fn assert_error(input: &(&str, &str, &str)) {
        assert_eq!(
            test_script(input.1, None).expect_err("expected error").to_string(),
            input.2,
            "{}",
            input.0
        );
    }

    #[test]
    fn test_length() {
        [
            ("length of list", "length([1, 2, 3]) == 3"),
            ("length of map", r#"length({"a": 1, "b": 2, "c": 3}) == 3"#),
            ("length of string", r#"length("foo") == 3"#),
            ("length of bytes", r#"length(b"foo") == 3"#),
            ("length as a list method", "[1, 2, 3].length() == 3"),
            ("length as a string method", r#""foobar".length() == 6"#),
        ]
        .iter()
        .for_each(assert_script);
    }

    #[test]
    fn test_has() {
        let tests = vec![
            ("map has", "has(foo.bar) == true"),
            ("map not has", "has(foo.baz) == false"),
        ];

        for (name, script) in tests {
            let mut ctx = Context::default();
            ctx.add_variable_from_value("foo", std::collections::HashMap::from([("bar", 1)]));
            assert_eq!(test_script(script, Some(ctx)), Ok(true.into()), "{name}");
        }
    }

    #[test]
    fn test_map() {
        [
            ("map list", "[1, 2, 3].map(x, x * 2) == [2, 4, 6]"),
            ("map list 2", "[1, 2, 3].map(y, y + 1) == [2, 3, 4]"),
            ("map list filter", "[1, 2, 3].map(y, y + 1) == [2, 3, 4]"),
            ("nested map", "[[1, 2], [2, 3]].map(x, x.map(x, x * 2)) == [[2, 4], [4, 6]]"),
            ("map to list", r#"{"John": "smart"}.map(key, key) == ["John"]"#),
        ]
        .iter()
        .for_each(assert_script);
    }

    #[test]
    fn test_filter() {
        [("filter list", "[1, 2, 3].filter(x, x > 2) == [3]")]
            .iter()
            .for_each(assert_script);
    }

    #[test]
    fn test_all() {
        [
            ("all list #1", "[0, 1, 2].all(x, x >= 0)"),
            ("all list #2", "[0, 1, 2].all(x, x > 0) == false"),
            ("all map", "{0: 0, 1:1, 2:2}.all(x, x >= 0) == true"),
        ]
        .iter()
        .for_each(assert_script);
    }

    #[test]
    fn test_any() {
        [
            ("exist list #1", "[0, 1, 2].any(x, x > 0)"),
            ("exist list #2", "[0, 1, 2].any(x, x == 3) == false"),
            ("exist list #3", "[0, 1, 2, 2].any(x, x == 2)"),
            ("exist map", "{0: 0, 1:1, 2:2}.any(x, x > 0)"),
        ]
        .iter()
        .for_each(assert_script);
    }

    // #[test]
    // fn test_exists_one() {
    //     [
    //         ("exist list #1", "[0, 1, 2].exists_one(x, x > 0) == false"),
    //         ("exist list #2", "[0, 1, 2].exists_one(x, x == 0)"),
    //         ("exist map", "{0: 0, 1:1, 2:2}.exists_one(x, x == 2)"),
    //     ]
    //     .iter()
    //     .for_each(assert_script);
    // }

    #[test]
    fn test_max() {
        [
            ("max single", "max(1) == 1"),
            ("max multiple", "max(1, 2, 3) == 3"),
            ("max negative", "max(-1, 0) == 0"),
            ("max float", "max(-1.0, 0.0) == 0.0"),
            ("max list", "max([1, 2, 3]) == 3"),
            ("max empty list", "max([]) == null"),
            ("max no args", "max() == null"),
        ]
        .iter()
        .for_each(assert_script);
    }

    #[test]
    fn test_min() {
        [
            ("min single", "min(1) == 1"),
            ("min multiple", "min(1, 2, 3) == 1"),
            ("min negative", "min(-1, 0) == -1"),
            ("min float", "min(-1.0, 0.0) == -1.0"),
            ("min float multiple", "min(1.61803, 3.1415, 2.71828, 1.41421) == 1.41421"),
            ("min list", "min([1, 2, 3]) == 1"),
            ("min empty list", "min([]) == null"),
            ("min no args", "min() == null"),
        ]
        .iter()
        .for_each(assert_script);
    }

    #[test]
    fn test_starts_with() {
        [
            ("starts with true", r#""foobar".starts_with("foo") == true"#),
            ("starts with false", r#""foobar".starts_with("bar") == false"#),
        ]
        .iter()
        .for_each(assert_script);
    }

    #[test]
    fn test_ends_with() {
        [
            ("ends with true", r#""foobar".ends_with("bar") == true"#),
            ("ends with false", r#""foobar".ends_with("foo") == false"#),
        ]
        .iter()
        .for_each(assert_script);
    }

    #[cfg(feature = "time")]
    #[test]
    fn test_timestamp() {
        [
            (
                "comparison",
                r#"Timestamp("2023-05-29T00:00:00Z") > Timestamp("2023-05-28T00:00:00Z")"#,
            ),
            (
                "comparison",
                r#"Timestamp("2023-05-29T00:00:00Z") < Timestamp("2023-05-30T00:00:00Z")"#,
            ),
            (
                "subtracting duration",
                r#"Timestamp("2023-05-29T00:00:00Z") - Duration("24h") == Timestamp("2023-05-28T00:00:00Z")"#,
            ),
            (
                "subtracting date",
                r#"Timestamp("2023-05-29T00:00:00Z") - Timestamp("2023-05-28T00:00:00Z") == Duration("24h")"#,
            ),
            (
                "adding duration",
                r#"Timestamp("2023-05-28T00:00:00Z") + Duration("24h") == Timestamp("2023-05-29T00:00:00Z")"#,
            ),
            (
                "timestamp string",
                r#"String(Timestamp("2023-05-28T00:00:00Z")) == "2023-05-28T00:00:00Z""#,
            ),
            ("timestamp year", r#"Timestamp("2023-05-28T00:00:00Z").year() == 2023"#),
            ("timestamp month", r#"Timestamp("2023-05-28T00:00:00Z").month() == 4"#),
            (
                "timestamp getDayOfMonth",
                r#"Timestamp("2023-05-28T00:00:00Z").getDayOfMonth() == 27"#,
            ),
            (
                "timestamp getDayOfYear",
                r#"Timestamp("2023-05-28T00:00:00Z").getDayOfYear() == 147"#,
            ),
            ("timestamp getDate", r#"Timestamp("2023-05-28T00:00:00Z").getDate() == 28"#),
            (
                "timestamp getDayOfWeek",
                r#"Timestamp("2023-05-28T00:00:00Z").getDayOfWeek() == 0"#,
            ),
            ("timestamp getHours", r#"Timestamp("2023-05-28T02:00:00Z").getHours() == 2"#),
            (
                "timestamp getMinutes",
                r#" Timestamp("2023-05-28T00:05:00Z").getMinutes() == 5"#,
            ),
            ("timestamp seconds", r#"Timestamp("2023-05-28T00:00:06Z").seconds() == 6"#),
            (
                "timestamp milliseconds",
                r#"Timestamp("2023-05-28T00:00:42.123Z").milliseconds() == 123"#,
            ),
        ]
        .iter()
        .for_each(assert_script);

        [
            (
                "timestamp out of range",
                r#"Timestamp("0000-01-00T00:00:00Z")"#,
                "Error executing function 'timestamp': day is not valid for the given month",
            ),
            (
                "timestamp out of range",
                r#"Timestamp("9999-12-32T23:59:59.999999999Z")"#,
                "Error executing function 'timestamp': day is not valid for the given month",
            ),
            (
                "timestamp overflow",
                r#"Timestamp("9999-12-31T23:59:59Z") + Duration("1s")"#,
                "Overflow from binary operator 'add': Timestamp(9999-12-31T23:59:59Z), Duration(Duration { nanos: 1000000000 })",
            ),
            (
                "timestamp underflow",
                r#"Timestamp("0001-01-01T00:00:00Z") - Duration("1s")"#,
                "Overflow from binary operator 'sub': Timestamp(0001-01-01T00:00:00Z), Duration(Duration { nanos: 1000000000 })",
            ),
            (
                "timestamp underflow",
                r#"Timestamp("0001-01-01T00:00:00Z") + Duration("-1s")"#,
                "Overflow from binary operator 'add': Timestamp(0001-01-01T00:00:00Z), Duration(Duration { nanos: -1000000000 })",
            ),
        ]
        .iter()
        .for_each(assert_error)
    }

    #[cfg(feature = "time")]
    #[test]
    fn test_duration() {
        [
            ("duration equal 1", r#"Duration("1s") == Duration("1000ms")"#),
            ("duration equal 2", r#"Duration("1m") == Duration("60s")"#),
            ("duration equal 3", r#"Duration("1h") == Duration("60m")"#),
            ("duration comparison 1", r#"Duration("1m") > Duration("1s")"#),
            ("duration comparison 2", r#"Duration("1m") < Duration("1h")"#),
            ("duration subtraction", r#"Duration("1h") - Duration("1m") == Duration("59m")"#),
            ("duration addition", r#"Duration("1h") + Duration("1m") == Duration("1h1m")"#),
        ]
        .iter()
        .for_each(assert_script);
    }

    #[cfg(feature = "time")]
    #[test]
    fn test_timestamp_variable() {
        let mut context = Context::default();
        let ts: time::DateTime = "2023-05-29T00:00:00Z".parse().unwrap();
        context.add_variable("ts", crate::Value::Timestamp(ts)).unwrap();

        let program = crate::Program::compile(r#"ts == Timestamp("2023-05-29T00:00:00Z")"#).unwrap();
        let result = program.execute(&context).unwrap();
        assert_eq!(result, true.into());
    }

    #[cfg(feature = "time")]
    #[test]
    fn test_time_string() {
        [
            ("duration", r#"String(Duration("1h30m")) == "1h30m0s""#),
            (
                "timestamp",
                r#"String(Timestamp("2023-05-29T00:00:00Z")) == "2023-05-29T00:00:00Z""#,
            ),
        ]
        .iter()
        .for_each(assert_script);
    }

    #[test]
    fn test_contains() {
        let tests = vec![
            ("list", "[1, 2, 3].contains(3) == true"),
            ("map", "{1: true, 2: true, 3: true}.contains(3) == true"),
            ("string", r#""foobar".contains("bar") == true"#),
            ("bytes", r#"b"foobar".contains(b"o") == true"#),
            #[cfg(feature = "ip")]
            ("ip", r#"Ip("0.0.0.0/0").contains(Ip("127.0.0.1"))"#),
            #[cfg(feature = "ip")]
            ("ip does not contain", r#"!Ip("0.0.0.0/32").contains(Ip("127.0.0.1"))"#),
        ];

        for (name, script) in tests {
            assert_eq!(test_script(script, None), Ok(true.into()), "{name}");
        }
    }

    #[cfg(feature = "regex")]
    #[test]
    fn test_matches() {
        let tests = vec![
            ("string", r#""foobar".matches(Regex("^[a-zA-Z]*$")) == true"#),
            (
                "map",
                r#"{"1": "abc", "2": "def", "3": "ghi"}.all(key, key.matches(Regex("^[a-zA-Z]*$"))) == false"#,
            ),
        ];

        for (name, script) in tests {
            assert_eq!(test_script(script, None), Ok(true.into()), ".matches failed for '{name}'");
        }
    }

    #[cfg(feature = "regex")]
    #[test]
    fn test_regex_err() {
        assert_eq!(
            test_script(r#""foobar".matches(Regex("(foo")) == true"#, None),
            Err(crate::ExecutionError::FunctionError {
                function: "Regex".to_string(),
                // message: "'(foo' not a valid regex:\nregex parse error:\n    (foo\n    ^\nerror: unclosed group".to_string()
                message: "regex parse error:\n    (foo\n    ^\nerror: unclosed group".to_string()
            })
        );
    }

    #[test]
    fn test_string() {
        [
            ("String", r#"String("foo") == "foo""#),
            ("Int", r#"String(10) == "10""#),
            ("Float", r#"String(10.5) == "10.5""#),
            ("Bytes", r#"String(b"foo") == "foo""#),
        ]
        .iter()
        .for_each(assert_script);
    }

    #[test]
    fn test_bytes() {
        [
            ("String", r#"Bytes("abc") == b"abc""#),
            ("Bytes", r#"Bytes("abc") == b"\x61b\x63""#),
        ]
        .iter()
        .for_each(assert_script);
    }

    #[test]
    fn test_float() {
        [
            ("String", r#"Float("10") == 10.0"#),
            ("Int", "Float(10)== 10.0"),
            ("Float", "Float(10) == 10.0"),
        ]
        .iter()
        .for_each(assert_script);
    }

    // #[test]
    // fn test_uint() {
    //     [
    //         ("String", r#"Uint("10") == Uint(10)"#),
    //         ("Float", "Uint(10.5) == Uint(10)"),
    //     ]
    //     .iter()
    //     .for_each(assert_script);
    // }

    #[test]
    fn test_int() {
        [
            ("String", r#"Int("10") == 10"#),
            ("Int", "Int(10) == 10"),
            // ("Uint", "10.uint().int() == 10"),
            ("Float", "Int(10.5) == 10"),
        ]
        .iter()
        .for_each(assert_script);
    }

    #[test]
    fn no_bool_coercion() {
        [
            ("String || bool", r#""" || false"#, "No such overload"),
            ("Int || bool", "1 || false", "No such overload"),
            // ("UInt || bool", "1u || false", "No such overload"),
            ("Float || bool", "0.1|| false", "No such overload"),
            ("List || bool", "[] || false", "No such overload"),
            ("Map || bool", "{} || false", "No such overload"),
            ("null || bool", "null || false", "No such overload"),
        ]
        .iter()
        .for_each(assert_error)
    }
}
