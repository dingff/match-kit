use js_sys::{Function, Object, Reflect, JSON};
use regex::Regex;
use wasm_bindgen::prelude::*;

static SOME_VALUE: &str = "__SOME__";
static NONE_VALUE: &str = "__NONE__";
static DEFAULT_HANDLER: &str = "_";

static PREFIX_ANY: &str = "any::";
static PREFIX_NOT: &str = "not::";
static PREFIX_REGEX: &str = "regex::";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Function")]
    pub type PatternHandler;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
}

#[wasm_bindgen]
pub fn some() -> String {
    SOME_VALUE.to_string()
}

#[wasm_bindgen]
pub fn none() -> String {
    NONE_VALUE.to_string()
}

#[inline]
fn js_typeof(value: &JsValue) -> &'static str {
    if value.is_string() {
        "string"
    } else if value.as_f64().is_some() {
        "number"
    } else if value.as_bool().is_some() {
        "boolean"
    } else if value.is_null() {
        "null"
    } else if value.is_undefined() {
        "undefined"
    } else {
        "object"
    }
}

#[inline]
fn encode_value(value: &JsValue) -> Result<String, JsValue> {
    const SEP: char = '\x1F';
    let value_type = js_typeof(value);
    let encoded = match value_type {
        "undefined" => format!("undefined{}", SEP),
        "null" => format!("null{}", SEP),
        "string" => format!("string{}{}", SEP, value.as_string().unwrap_or_default()),
        "number" => format!("number{}{}", SEP, value.as_f64().unwrap_or(0.0)),
        "boolean" => format!("boolean{}{}", SEP, value.as_bool().unwrap_or(false)),
        _ => {
            // fallback to JSON for object
            match JSON::stringify(value) {
                Ok(json_value) => format!("object{}{}", SEP, json_value.as_string().unwrap_or_default()),
                Err(e) => return Err(e),
            }
        }
    };
    Ok(encoded)
}

fn compare_encoded_value(encoded: &str, value: &JsValue, case_sensitive: bool) -> bool {
    const SEP: char = '\x1F';
    if !encoded.contains(SEP) {
        return false;
    }
    let mut parts = encoded.splitn(2, SEP);
    let val_type = parts.next().unwrap_or("");
    let val_str = parts.next().unwrap_or("");

    if val_type == "undefined" && value.is_undefined() {
        return true;
    }
    if val_type == "null" && value.is_null() {
        return true;
    }

    let value_type = js_typeof(value);

    if val_type != value_type {
        return false;
    }

    match value_type {
        "string" => {
            let value_str = value.as_string().unwrap_or_default();
            if case_sensitive {
                val_str == value_str
            } else {
                val_str.eq_ignore_ascii_case(&value_str)
            }
        }
        "number" => {
            let value_num = value.as_f64().unwrap_or(0.0);
            val_str.parse::<f64>().map(|v| v == value_num).unwrap_or(false)
        }
        "boolean" => {
            let value_bool = value.as_bool().unwrap_or(false);
            val_str.parse::<bool>().map(|v| v == value_bool).unwrap_or(false)
        }
        "object" => {
            // fallback to JSON string compare
            match JSON::stringify(value) {
                Ok(json_value) => val_str == json_value.as_string().unwrap_or_default(),
                Err(_) => false,
            }
        }
        _ => false,
    }
}

#[wasm_bindgen]
pub fn not(args: &js_sys::Array) -> Result<String, JsValue> {
    let length = args.length();

    if length == 0 {
        return Err(JsValue::from_str("not() requires at least one value"));
    }

    let mut encoded_values = Vec::with_capacity(length as usize);
    for i in 0..length {
        let value = args.get(i);
        match encode_value(&value) {
            Ok(encoded) => encoded_values.push(encoded),
            Err(e) => return Err(e),
        }
    }

    Ok(format!("{}{}", PREFIX_NOT, encoded_values.join("|")))
}

#[wasm_bindgen]
pub fn any(args: &js_sys::Array) -> Result<String, JsValue> {
    let length = args.length();

    if length == 0 {
        return Err(JsValue::from_str("any() requires at least one value"));
    }

    let mut encoded_values = Vec::with_capacity(length as usize);
    for i in 0..length {
        let value = args.get(i);
        match encode_value(&value) {
            Ok(encoded) => encoded_values.push(encoded),
            Err(e) => return Err(e),
        }
    }

    Ok(format!("{}{}", PREFIX_ANY, encoded_values.join("|")))
}

#[wasm_bindgen]
pub fn regex(pattern: &str, flags: Option<String>) -> Result<String, JsValue> {
    let flags_str = flags.unwrap_or_default();

    let regex_str = if flags_str.is_empty() {
        pattern.to_string()
    } else {
        format!("(?{}){}", flags_str, pattern)
    };

    match Regex::new(&regex_str) {
        Ok(_) => Ok(format!("{}{}::{}", PREFIX_REGEX, pattern, flags_str)),
        Err(e) => Err(JsValue::from_str(&format!(
            "Invalid regex pattern: {} with flags: {} - {}",
            pattern, flags_str, e
        ))),
    }
}

fn wildcard_to_regex(pattern: &str, case_sensitive: bool) -> Regex {
    let mut regex_str = String::with_capacity(pattern.len() * 2);
    for c in pattern.chars() {
        match c {
            '*' => regex_str.push_str(".*"),
            '?' => regex_str.push('.'),
            '.' | '+' | '^' | '$' | '{' | '}' | '(' | ')' | '|' | '[' | ']' | '\\' => {
                regex_str.push('\\');
                regex_str.push(c);
            }
            _ => regex_str.push(c),
        }
    }

    regex_str = format!("^{}$", regex_str);

    let regex_str = if !case_sensitive {
        format!("(?i){}", regex_str)
    } else {
        regex_str
    };

    Regex::new(&regex_str).unwrap()
}

fn create_regex(pattern: &str, flags: &str) -> Regex {
    let regex_str = if flags.contains('i') {
        format!("(?i){}", pattern)
    } else {
        pattern.to_string()
    };

    Regex::new(&regex_str).unwrap()
}

fn try_composite_pattern(
    value: &JsValue,
    entries: &[(String, JsValue)],
    prefix: &str,
    case_sensitive: bool,
    match_fn: impl Fn(bool) -> bool,
) -> Option<JsValue> {
    for (pattern, handler) in entries {
        if pattern.starts_with(prefix) {
            let values_part = &pattern[prefix.len()..];
            let values: Vec<&str> = values_part.split('|').collect();

            let mut matched = false;
            for val_str in values {
                if compare_encoded_value(val_str, value, case_sensitive) {
                    matched = true;
                    break;
                }
            }

            if match_fn(matched) {
                if let Some(func) = handler.dyn_ref::<Function>() {
                    return func.call0(&JsValue::NULL).ok();
                }
            }
        }
    }

    None
}

fn get_string_value(value: &JsValue) -> String {
    if value.is_null() {
        return "null".to_string();
    } else if value.is_undefined() {
        return "undefined".to_string();
    } else if let Some(str_val) = value.as_string() {
        return str_val;
    } else if let Ok(json_val) = JSON::stringify(value) {
        if let Some(str_val) = json_val.as_string() {
            return str_val;
        }
    }

    "[object Object]".to_string()
}
struct PatternGroups {
    any: Vec<(String, JsValue)>,
    not: Vec<(String, JsValue)>,
    regex: Vec<(String, JsValue)>,
    wildcard: Vec<(String, JsValue)>,
}
impl PatternGroups {
    fn from_object(obj: &Object) -> Self {
        let keys = Object::keys(obj);
        let length = keys.length();

        let mut any = Vec::new();
        let mut not = Vec::new();
        let mut regex = Vec::new();
        let mut wildcard = Vec::new();

        for i in 0..length {
            let key = keys.get(i);
            if let Some(key_str) = key.as_string() {
                if key_str == SOME_VALUE || key_str == NONE_VALUE || key_str == DEFAULT_HANDLER {
                    continue;
                }
                if let Ok(value) = Reflect::get(obj, &key) {
                    if key_str.starts_with(PREFIX_ANY) {
                        any.push((key_str, value));
                    } else if key_str.starts_with(PREFIX_NOT) {
                        not.push((key_str, value));
                    } else if key_str.starts_with(PREFIX_REGEX) {
                        regex.push((key_str, value));
                    } else if key_str.contains('*') || key_str.contains('?') {
                        wildcard.push((key_str, value));
                    }
                }
            }
        }

        wildcard.sort_by(|(pat_a, _), (pat_b, _)| {
            let wildcard_count_a = pat_a.chars().filter(|&c| c == '*' || c == '?').count();
            let wildcard_count_b = pat_b.chars().filter(|&c| c == '*' || c == '?').count();
            wildcard_count_a.cmp(&wildcard_count_b)
        });

        Self {
            any,
            not,
            regex,
            wildcard,
        }
    }
}
#[wasm_bindgen(js_name = "match")]
pub fn match_pattern(
    value: &JsValue,
    patterns: &Object,
    options: Option<Object>,
) -> Result<JsValue, JsValue> {
    let case_sensitive = if let Some(opts) = options {
        match Reflect::get(&opts, &JsValue::from_str("caseSensitive")) {
            Ok(val) => !val.is_falsy(),
            _ => true,
        }
    } else {
        true
    };

    if !value.is_null() && !value.is_undefined() {
        if let Ok(some_handler) = Reflect::get(patterns, &JsValue::from_str(SOME_VALUE)) {
            if let Some(func) = some_handler.dyn_ref::<Function>() {
                return func.call0(&JsValue::NULL).map_err(|e| e);
            }
        }
    } else if let Ok(none_handler) = Reflect::get(patterns, &JsValue::from_str(NONE_VALUE)) {
        if let Some(func) = none_handler.dyn_ref::<Function>() {
            return func.call0(&JsValue::NULL).map_err(|e| e);
        }
    }

    let string_value = get_string_value(value);

    if let Ok(handler) = Reflect::get(patterns, &JsValue::from_str(&string_value)) {
        if let Some(func) = handler.dyn_ref::<Function>() {
            return func.call0(&JsValue::NULL).map_err(|e| e);
        }
    }

    let pattern_groups = PatternGroups::from_object(patterns);

    if let Some(result) = try_composite_pattern(
        value,
        &pattern_groups.any,
        PREFIX_ANY,
        case_sensitive,
        |matched| matched,
    ) {
        return Ok(result);
    }

    if let Some(result) = try_composite_pattern(
        value,
        &pattern_groups.not,
        PREFIX_NOT,
        case_sensitive,
        |matched| !matched,
    ) {
        return Ok(result);
    }

    for (pattern, handler) in &pattern_groups.regex {
        if pattern.starts_with(PREFIX_REGEX) {
            let parts: Vec<&str> = pattern.splitn(3, "::").collect();
            if parts.len() >= 2 {
                let regex_pattern = parts[1];
                let flags = if parts.len() > 2 { parts[2] } else { "" };

                let effective_flags = if !case_sensitive && !flags.contains('i') {
                    format!("{}i", flags)
                } else {
                    flags.to_string()
                };

                let regex = create_regex(regex_pattern, &effective_flags);
                if regex.is_match(&string_value) {
                    if let Some(func) = handler.dyn_ref::<Function>() {
                        return func.call0(&JsValue::NULL).map_err(|e| e);
                    }
                }
            }
        }
    }
    if value.is_string() {
        let value_str = value.as_string().unwrap();

        for (pattern, handler) in &pattern_groups.wildcard {
            let regex = wildcard_to_regex(pattern, case_sensitive);
            if regex.is_match(&value_str) {
                if let Some(func) = handler.dyn_ref::<Function>() {
                    return func.call0(&JsValue::NULL).map_err(|e| e);
                }
            }
        }
    }

    let default_handler = Reflect::get(patterns, &JsValue::from_str(DEFAULT_HANDLER)).ok();
    if let Some(default_handler) = default_handler {
        if let Some(func) = default_handler.dyn_ref::<Function>() {
            return func.call0(&JsValue::NULL).map_err(|e| e);
        }
    }

    let keys = Object::keys(patterns);
    let attempted_patterns: Vec<String> = (0..keys.length())
        .filter_map(|i| keys.get(i).as_string())
        .collect();

    let error_msg = format!(
        "No pattern matched for: {}. Attempted patterns: {}",
        string_value,
        attempted_patterns.join(", ")
    );

    Err(JsValue::from_str(&error_msg))
}

#[wasm_bindgen(js_name = "ifLet")]
pub fn if_let(value: &JsValue, pattern: &JsValue, handler: &Function) -> JsValue {
    let pattern_str = get_string_value(pattern);

    let patterns = Object::new();
    let _ = Reflect::set(&patterns, &JsValue::from_str(&pattern_str), handler);
    let _ = Reflect::set(
        &patterns,
        &JsValue::from_str(DEFAULT_HANDLER),
        &Function::new_no_args("return undefined;"),
    );

    match match_pattern(value, &patterns, None) {
        Ok(result) => result,
        Err(_) => JsValue::undefined(),
    }
}

#[wasm_bindgen]
pub fn matches(value: &JsValue, pattern: &JsValue, options: Option<Object>) -> bool {
    let pattern_str = get_string_value(pattern);

    let patterns = Object::new();
    let _ = Reflect::set(
        &patterns,
        &JsValue::from_str(&pattern_str),
        &Function::new_no_args("return true;"),
    );
    let _ = Reflect::set(
        &patterns,
        &JsValue::from_str(DEFAULT_HANDLER),
        &Function::new_no_args("return false;"),
    );

    match match_pattern(value, &patterns, options) {
        Ok(result) => !result.is_falsy(),
        Err(_) => false,
    }
}
