use js_sys::{Function, Object, Reflect};
use regex_lite::Regex;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use wasm_bindgen::prelude::*;

const SOME_VALUE: &str = "__SOME__";
const NONE_VALUE: &str = "__NONE__";
const DEFAULT_HANDLER: &str = "_";

const PREFIX_WHEN: &str = "when::";
const PREFIX_ANY: &str = "any::";
const PREFIX_NOT: &str = "not::";
const PREFIX_REGEX: &str = "regex::";
const SEP: char = '\x1F';

#[wasm_bindgen]
pub fn some() -> String {
  SOME_VALUE.to_string()
}

#[wasm_bindgen]
pub fn none() -> String {
  NONE_VALUE.to_string()
}

pub enum JsType {
  String,
  Number,
  Boolean,
  Null,
  Undefined,
  Unknown,
}

#[inline]
fn js_typeof(value: &JsValue) -> JsType {
  if value.is_string() {
    JsType::String
  } else if value.as_f64().is_some() {
    JsType::Number
  } else if value.as_bool().is_some() {
    JsType::Boolean
  } else if value.is_null() {
    JsType::Null
  } else if value.is_undefined() {
    JsType::Undefined
  } else {
    JsType::Unknown
  }
}

fn get_global_storage() -> Result<Object, JsValue> {
  let global = js_sys::global();

  let storage_key = JsValue::from_str("__whenPredicates");
  let storage = match Reflect::get(&global, &storage_key) {
    Ok(storage) if !storage.is_undefined() => storage
      .dyn_into::<Object>()
      .map_err(|_| JsValue::from_str("Invalid storage object"))?,
    _ => {
      let new_storage = Object::new();
      Reflect::set(&global, &storage_key, &new_storage)?;
      new_storage
    }
  };

  Ok(storage)
}

fn calculate_hash(s: &str) -> String {
  let mut hasher = DefaultHasher::new();
  s.hash(&mut hasher);
  hasher.finish().to_string()
}

#[wasm_bindgen]
pub fn when(condition: &JsValue) -> Result<String, JsValue> {
  if let Some(predicate_fn) = condition.dyn_ref::<Function>() {
    let func_str = predicate_fn
      .to_string()
      .as_string()
      .ok_or_else(|| JsValue::from_str("Failed to convert function to string"))?;
    let hash = calculate_hash(&func_str);
    let storage = get_global_storage()?;
    Reflect::set(&storage, &JsValue::from_str(&hash), predicate_fn)?;
    Ok(format!("{}{}", PREFIX_WHEN, hash))
  } else if let Some(bool_val) = condition.as_bool() {
    Ok(format!("{}{}", PREFIX_WHEN, bool_val))
  } else {
    Err(JsValue::from_str(
      "when() requires a function or boolean as argument",
    ))
  }
}

fn get_predicate_function(function_hash: &str) -> Result<Option<Function>, JsValue> {
  let storage = get_global_storage()?;

  match Reflect::get(&storage, &JsValue::from_str(function_hash)) {
    Ok(value) if !value.is_undefined() => Ok(value.dyn_into::<Function>().ok()),
    Ok(_) => Ok(None),
    Err(e) => Err(e),
  }
}

#[inline]
fn encode_value(value: &JsValue) -> Result<String, JsValue> {
  let value_type = js_typeof(value);
  let encoded = match value_type {
    JsType::Undefined => format!("undefined{}", SEP),
    JsType::Null => format!("null{}", SEP),
    JsType::String => format!("string{}{}", SEP, value.as_string().unwrap_or_default()),
    JsType::Number => format!("number{}{}", SEP, value.as_f64().unwrap_or(0.0)),
    JsType::Boolean => format!("boolean{}{}", SEP, value.as_bool().unwrap_or(false)),
    JsType::Unknown => return Err(JsValue::from_str("Unsupported value type")),
  };
  Ok(encoded)
}

fn compare_encoded_value(encoded: &str, value: &JsValue, case_sensitive: bool) -> bool {
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

  match (val_type, value_type) {
    ("string", JsType::String) => {
      let value_str = value.as_string().unwrap_or_default();
      if case_sensitive {
        val_str == value_str
      } else {
        val_str.eq_ignore_ascii_case(&value_str)
      }
    }
    ("number", JsType::Number) => {
      let value_num = value.as_f64().unwrap_or(0.0);
      val_str.parse::<f64>().map_or(false, |v| v == value_num)
    }
    ("boolean", JsType::Boolean) => {
      let value_bool = value.as_bool().unwrap_or(false);
      val_str.parse::<bool>().map_or(false, |v| v == value_bool)
    }
    _ => false,
  }
}

fn get_string_value(value: &JsValue) -> String {
  if value.is_null() {
    return "null".to_string();
  } else if value.is_undefined() {
    return "undefined".to_string();
  } else if let Some(str_val) = value.as_string() {
    return str_val;
  } else if let Some(num_val) = value.as_f64() {
    return num_val.to_string();
  } else if let Some(bool_val) = value.as_bool() {
    return bool_val.to_string();
  }
  "unknown".to_string()
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

  if flags_str.chars().any(|c| !"ims".contains(c)) {
    return Err(JsValue::from_str(
      "Only 'i', 'm', 's' flags are supported in regex",
    ));
  }

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
  let mut prefix = String::from("(?");
  let mut has_flag = false;
  for ch in flags.chars() {
    match ch {
      'i' | 'm' | 's' => {
        prefix.push(ch);
        has_flag = true;
      }
      _ => {}
    }
  }
  if has_flag {
    prefix.push(')');
    Regex::new(&format!("{}{}", prefix, pattern)).unwrap()
  } else {
    Regex::new(pattern).unwrap()
  }
}

fn try_composite_pattern(
  value: &JsValue,
  entries: &[(String, JsValue)],
  prefix: &str,
  case_sensitive: bool,
  match_fn: impl Fn(bool) -> bool,
) -> Option<JsValue> {
  for (pattern, handler) in entries {
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

  None
}

struct PatternGroups {
  when: Vec<(String, JsValue)>,
  any: Vec<(String, JsValue)>,
  not: Vec<(String, JsValue)>,
  regex: Vec<(String, JsValue)>,
  wildcard: Vec<(String, JsValue)>,
}
impl PatternGroups {
  fn from_object(obj: &Object) -> Self {
    let keys = Object::keys(obj);
    let length = keys.length();

    let mut when = Vec::with_capacity(length as usize);
    let mut any = Vec::with_capacity(length as usize);
    let mut not = Vec::with_capacity(length as usize);
    let mut regex = Vec::with_capacity(length as usize);
    let mut wildcard = Vec::with_capacity(length as usize);

    for i in 0..length {
      let key = keys.get(i);
      if let Some(key_str) = key.as_string() {
        if key_str == SOME_VALUE || key_str == NONE_VALUE || key_str == DEFAULT_HANDLER {
          continue;
        }
        if let Ok(value) = Reflect::get(obj, &key) {
          if key_str.starts_with(PREFIX_WHEN) {
            when.push((key_str, value));
          } else if key_str.starts_with(PREFIX_ANY) {
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
      when,
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
        return func.call0(&JsValue::NULL);
      }
    }
  } else if let Ok(none_handler) = Reflect::get(patterns, &JsValue::from_str(NONE_VALUE)) {
    if let Some(func) = none_handler.dyn_ref::<Function>() {
      return func.call0(&JsValue::NULL);
    }
  }

  let string_value = get_string_value(value);

  if let Ok(handler) = Reflect::get(patterns, &JsValue::from_str(&string_value)) {
    if let Some(func) = handler.dyn_ref::<Function>() {
      return func.call0(&JsValue::NULL);
    }
  }

  let pattern_groups = PatternGroups::from_object(patterns);

  for (pattern, handler) in &pattern_groups.when {
    if pattern == &format!("{}{}", PREFIX_WHEN, true) {
      if let Some(func) = handler.dyn_ref::<Function>() {
        return func.call0(&JsValue::NULL);
      }
    }
    let function_hash = &pattern[PREFIX_WHEN.len()..];
    if let Ok(Some(predicate)) = get_predicate_function(function_hash) {
      if let Ok(result) = predicate.call1(&JsValue::NULL, value) {
        if result.as_bool() == Some(true) {
          if let Some(func) = handler.dyn_ref::<Function>() {
            return func.call0(&JsValue::NULL);
          }
        }
      }
    }
  }

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
          return func.call0(&JsValue::NULL);
        }
      }
    }
  }

  if value.is_string() && !pattern_groups.wildcard.is_empty() {
    if let Some(value_str) = value.as_string() {
      for (pattern, handler) in &pattern_groups.wildcard {
        let regex = wildcard_to_regex(pattern, case_sensitive);
        if regex.is_match(&value_str) {
          if let Some(func) = handler.dyn_ref::<Function>() {
            return func.call0(&JsValue::NULL);
          }
        }
      }
    }
  }

  let default_handler = Reflect::get(patterns, &JsValue::from_str(DEFAULT_HANDLER)).ok();
  if let Some(default_handler) = default_handler {
    if let Some(func) = default_handler.dyn_ref::<Function>() {
      return func.call0(&JsValue::NULL);
    }
  }

  let keys = Object::keys(patterns);
  let attempted_patterns: Vec<String> = keys
    .to_vec()
    .into_iter()
    .filter_map(|v| v.as_string())
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
    Ok(result) => result.as_bool().unwrap(),
    Err(_) => false,
  }
}
