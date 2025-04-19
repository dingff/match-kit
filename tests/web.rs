//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use js_sys::{Array, Function, Object, Reflect};
use match_kit::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test_some_none() {
    assert_eq!(some(), "__SOME__");
    assert_eq!(none(), "__NONE__");
}

#[wasm_bindgen_test]
fn test_any_not() {
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from("a"));
    let any_pattern = any(&arr).unwrap();
    assert!(any_pattern.starts_with("any::"));
    let not_pattern = not(&arr).unwrap();
    assert!(not_pattern.starts_with("not::"));
}

#[wasm_bindgen_test]
fn test_regex() {
    let pat = regex("abc", None).unwrap();
    assert_eq!(pat, "regex::abc::");
    let pat2 = regex("abc", Some("i".to_string())).unwrap();
    assert_eq!(pat2, "regex::abc::i");
    assert!(regex("[", None).is_err());
}

#[wasm_bindgen_test]
fn test_match_pattern_exact() {
    let patterns = Object::new();
    let f = Function::new_no_args("return 42;");
    Reflect::set(&patterns, &JsValue::from_str("foo"), &f).unwrap();
    let v = JsValue::from_str("foo");
    let result = match_pattern(&v, &patterns, None).unwrap();
    assert_eq!(result.as_f64().unwrap(), 42.0);
}

#[wasm_bindgen_test]
fn test_match_pattern_any() {
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from(2));
    let any_pat = any(&arr.into()).unwrap();
    let patterns = Object::new();
    let f = Function::new_no_args("return 'ok';");
    Reflect::set(&patterns, &JsValue::from_str(&any_pat), &f).unwrap();
    let v = JsValue::from(2);
    let result = match_pattern(&v, &patterns, None).unwrap();
    assert_eq!(result.as_string().unwrap(), "ok");
}

#[wasm_bindgen_test]
fn test_match_pattern_not() {
    let arr = Array::new();
    arr.push(&JsValue::from("x"));
    let not_pat = not(&arr.into()).unwrap();
    let patterns = Object::new();
    let f = Function::new_no_args("return 'not-x';");
    Reflect::set(&patterns, &JsValue::from_str(&not_pat), &f).unwrap();
    let v = JsValue::from_str("y");
    let result = match_pattern(&v, &patterns, None).unwrap();
    assert_eq!(result.as_string().unwrap(), "not-x");
}

#[wasm_bindgen_test]
fn test_match_pattern_regex() {
    let pat = regex("^foo.*", None).unwrap();
    let patterns = Object::new();
    let f = Function::new_no_args("return 1;");
    Reflect::set(&patterns, &JsValue::from_str(&pat), &f).unwrap();
    let v = JsValue::from_str("foobar");
    let result = match_pattern(&v, &patterns, None).unwrap();
    assert_eq!(result.as_f64().unwrap(), 1.0);
}

#[wasm_bindgen_test]
fn test_match_pattern_wildcard() {
    let patterns = Object::new();
    let f = Function::new_no_args("return 'wild';");
    Reflect::set(&patterns, &JsValue::from_str("foo*bar"), &f).unwrap();
    let v = JsValue::from_str("foozzzbar");
    let result = match_pattern(&v, &patterns, None).unwrap();
    assert_eq!(result.as_string().unwrap(), "wild");
}

#[wasm_bindgen_test]
fn test_match_pattern_some_none_default() {
    let patterns = Object::new();
    let f_some = Function::new_no_args("return 'some';");
    let f_none = Function::new_no_args("return 'none';");
    let f_default = Function::new_no_args("return 'def';");
    Reflect::set(&patterns, &JsValue::from_str("__SOME__"), &f_some).unwrap();
    Reflect::set(&patterns, &JsValue::from_str("__NONE__"), &f_none).unwrap();
    Reflect::set(&patterns, &JsValue::from_str("_"), &f_default).unwrap();
    let v_some = JsValue::from(123);
    let v_none = JsValue::NULL;
    let v_other = JsValue::from_str("zzz");
    assert_eq!(
        match_pattern(&v_some, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "some"
    );
    assert_eq!(
        match_pattern(&v_none, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "none"
    );
    assert_eq!(
        match_pattern(&v_other, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "some"
    );
}

#[wasm_bindgen_test]
fn test_match_pattern_no_match() {
    let patterns = Object::new();
    let v = JsValue::from_str("notfound");
    let err = match_pattern(&v, &patterns, None).unwrap_err();
    assert!(err.as_string().unwrap().contains("No pattern matched"));
}

#[wasm_bindgen_test]
fn test_iflet_and_matches() {
    let handler = Function::new_no_args("return 99;");
    let v = JsValue::from_str("abc");
    let pat = JsValue::from_str("abc");
    let result = if_let(&v, &pat, &handler);
    assert_eq!(result.as_f64().unwrap(), 99.0);
    let pat2 = JsValue::from_str("def");
    let result2 = if_let(&v, &pat2, &handler);
    assert!(result2.is_undefined());
    assert!(matches(&v, &pat, None));
    assert!(!matches(&v, &pat2, None));
}

#[wasm_bindgen_test]
fn test_any_not_regex_wildcard_no_match() {
    let patterns = Object::new();
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    let any_pat = any(&arr.clone().into()).unwrap();
    let regex_pat = regex("^foo$", None).unwrap();
    let f_any = Function::new_no_args("return 'any';");
    let f_regex = Function::new_no_args("return 'regex';");
    let f_wild = Function::new_no_args("return 'wild';");
    let f_def = Function::new_no_args("return 'def';");
    Reflect::set(&patterns, &JsValue::from_str(&any_pat), &f_any).unwrap();
    Reflect::set(&patterns, &JsValue::from_str(&regex_pat), &f_regex).unwrap();
    Reflect::set(&patterns, &JsValue::from_str("foo*bar"), &f_wild).unwrap();
    Reflect::set(&patterns, &JsValue::from_str("_"), &f_def).unwrap();
    let v = JsValue::from(999);
    let v_str = JsValue::from_str("bazzzz");
    assert_eq!(
        match_pattern(&v, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "def"
    );
    assert_eq!(
        match_pattern(&v_str, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "def"
    );
}

#[wasm_bindgen_test]
fn test_case_sensitive_option() {
    let patterns = Object::new();
    let f1 = Function::new_no_args("return 'upper';");
    let f2 = Function::new_no_args("return 'lower';");
    Reflect::set(&patterns, &JsValue::from_str("FOO"), &f1).unwrap();
    Reflect::set(&patterns, &JsValue::from_str("foo"), &f2).unwrap();
    let v = JsValue::from_str("foo");
    let mut options = Object::new();
    Reflect::set(
        &mut options,
        &JsValue::from_str("caseSensitive"),
        &JsValue::from_bool(true),
    )
    .unwrap();
    assert_eq!(
        match_pattern(&v, &patterns, Some(options.clone()))
            .unwrap()
            .as_string()
            .unwrap(),
        "lower"
    );
    Reflect::set(
        &mut options,
        &JsValue::from_str("caseSensitive"),
        &JsValue::from_bool(false),
    )
    .unwrap();
    let result = match_pattern(&v, &patterns, Some(options));
    let s = result.unwrap().as_string().unwrap();
    assert!(s == "upper" || s == "lower");
}

#[wasm_bindgen_test]
fn test_any_not_regex_empty_or_invalid() {
    let arr = Array::new();
    let any_pat = any(&arr.clone().into());
    assert!(any_pat.is_err());
}

#[wasm_bindgen_test]
fn test_wildcard_non_string_value() {
    let patterns = Object::new();
    let f_wild = Function::new_no_args("return 'wild';");
    Reflect::set(&patterns, &JsValue::from_str("foo*bar"), &f_wild).unwrap();
    let v = JsValue::from(123);
    let f_def = Function::new_no_args("return 'def';");
    Reflect::set(&patterns, &JsValue::from_str("_"), &f_def).unwrap();
    assert_eq!(
        match_pattern(&v, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "def"
    );
}

#[wasm_bindgen_test]
fn test_only_default_branch() {
    let patterns = Object::new();
    let f_def = Function::new_no_args("return 'def';");
    Reflect::set(&patterns, &JsValue::from_str("_"), &f_def).unwrap();
    let v = JsValue::from_str("anything");
    assert_eq!(
        match_pattern(&v, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "def"
    );
}

#[wasm_bindgen_test]
fn test_priority_order() {
    let patterns = Object::new();
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    let any_pat = any(&arr.clone().into()).unwrap();
    let regex_pat = regex("^foo.*", None).unwrap();
    let f_exact = Function::new_no_args("return 'exact';");
    let f_any = Function::new_no_args("return 'any';");
    let f_regex = Function::new_no_args("return 'regex';");
    let f_def = Function::new_no_args("return 'def';");
    Reflect::set(&patterns, &JsValue::from_str("foo"), &f_exact).unwrap();
    Reflect::set(&patterns, &JsValue::from_str(&any_pat), &f_any).unwrap();
    Reflect::set(&patterns, &JsValue::from_str(&regex_pat), &f_regex).unwrap();
    Reflect::set(&patterns, &JsValue::from_str("_"), &f_def).unwrap();
    let v = JsValue::from_str("foo");
    assert_eq!(
        match_pattern(&v, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "exact"
    );
    let v2 = JsValue::from(1);
    assert_eq!(
        match_pattern(&v2, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "any"
    );
    let v3 = JsValue::from_str("foobar");
    assert_eq!(
        match_pattern(&v3, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "regex"
    );
    let v4 = JsValue::from(999);
    assert_eq!(
        match_pattern(&v4, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "def"
    );
}

#[wasm_bindgen_test]
fn test_undefined_some_none() {
    let patterns = Object::new();
    let f_some = Function::new_no_args("return 'some';");
    let f_none = Function::new_no_args("return 'none';");
    let f_def = Function::new_no_args("return 'def';");
    Reflect::set(&patterns, &JsValue::from_str("__SOME__"), &f_some).unwrap();
    Reflect::set(&patterns, &JsValue::from_str("__NONE__"), &f_none).unwrap();
    Reflect::set(&patterns, &JsValue::from_str("_"), &f_def).unwrap();
    let v_some = JsValue::from_str("abc");
    let v_none = JsValue::NULL;
    let v_undef = JsValue::UNDEFINED;
    assert_eq!(
        match_pattern(&v_some, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "some"
    );
    assert_eq!(
        match_pattern(&v_none, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "none"
    );
    assert_eq!(
        match_pattern(&v_undef, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "none"
    );
}

#[wasm_bindgen_test]
fn test_wildcard_case_insensitive() {
    let patterns = Object::new();
    let f = Function::new_no_args("return 'wild';");
    Reflect::set(&patterns, &JsValue::from_str("foo*bar"), &f).unwrap();
    let v = JsValue::from_str("FOOzzzBAR");
    let mut options = Object::new();
    Reflect::set(
        &mut options,
        &JsValue::from_str("caseSensitive"),
        &JsValue::from_bool(false),
    )
    .unwrap();
    let result = match_pattern(&v, &patterns, Some(options));
    assert_eq!(result.unwrap().as_string().unwrap(), "wild");
}

#[wasm_bindgen_test]
fn test_regex_case_insensitive_option() {
    let pat = regex("^foo$", None).unwrap();
    let patterns = Object::new();
    let f = Function::new_no_args("return 'regex';");
    Reflect::set(&patterns, &JsValue::from_str(&pat), &f).unwrap();
    let v = JsValue::from_str("FOO");
    let mut options = Object::new();
    Reflect::set(
        &mut options,
        &JsValue::from_str("caseSensitive"),
        &JsValue::from_bool(false),
    )
    .unwrap();
    let result = match_pattern(&v, &patterns, Some(options));
    assert_eq!(result.unwrap().as_string().unwrap(), "regex");
}

#[wasm_bindgen_test]
fn test_any_not_mixed_types() {
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from_str("a"));
    let any_pat = any(&arr.clone().into()).unwrap();
    let not_pat = not(&arr.clone().into()).unwrap();
    let patterns = Object::new();
    let f_any = Function::new_no_args("return 'any';");
    let f_not = Function::new_no_args("return 'not';");
    Reflect::set(&patterns, &JsValue::from_str(&any_pat), &f_any).unwrap();
    Reflect::set(&patterns, &JsValue::from_str(&not_pat), &f_not).unwrap();
    let v1 = JsValue::from(1);
    let v2 = JsValue::from_str("a");
    let v3 = JsValue::from(true);
    assert_eq!(
        match_pattern(&v1, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "any"
    );
    assert_eq!(
        match_pattern(&v2, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "any"
    );
    assert_eq!(
        match_pattern(&v3, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "not"
    );
}

#[wasm_bindgen_test]
fn test_wildcard_match_empty_string() {
    let patterns = Object::new();
    let f = Function::new_no_args("return 'wild';");
    Reflect::set(&patterns, &JsValue::from_str("*"), &f).unwrap();
    let v = JsValue::from_str("");
    let result = match_pattern(&v, &patterns, None);
    assert_eq!(result.unwrap().as_string().unwrap(), "wild");
}

#[wasm_bindgen_test]
fn test_pattern_handler_non_function() {
    let patterns = Object::new();
    Reflect::set(
        &patterns,
        &JsValue::from_str("foo"),
        &JsValue::from_str("not_fn"),
    )
    .unwrap();
    let v = JsValue::from_str("foo");
    let err = match_pattern(&v, &patterns, None).unwrap_err();
    assert!(err.as_string().unwrap().contains("No pattern matched"));
}

#[wasm_bindgen_test]
fn test_pattern_key_undefined_null() {
    let patterns = Object::new();
    let f_undef = Function::new_no_args("return 'undef';");
    let f_null = Function::new_no_args("return 'null';");
    Reflect::set(&patterns, &JsValue::from_str("undefined"), &f_undef).unwrap();
    Reflect::set(&patterns, &JsValue::from_str("null"), &f_null).unwrap();
    let v_undef = JsValue::UNDEFINED;
    let v_null = JsValue::NULL;
    assert_eq!(
        match_pattern(&v_undef, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "undef"
    );
    assert_eq!(
        match_pattern(&v_null, &patterns, None)
            .unwrap()
            .as_string()
            .unwrap(),
        "null"
    );
}
