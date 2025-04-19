# 🎯 Match-Kit

A lightweight, flexible and powerful pattern matching library for JavaScript/TypeScript.

## 📦 Installation

```bash
npm install match-kit
```

## 🚀 Quick Start

```typescript
import { match, Some, None, any, not, regex } from 'match-kit';

const greeting = 'hello';

const result = match(greeting, {
  hello: () => 'Exact match! 👌',
  [any('hi', 'hey')]: () => 'Multiple values match! 🔄',
  [not('bye', 'goodbye')]: () => 'Not "bye" or "goodbye" 🚫',
  [regex('^h.*o$')]: () => 'Regular expression match! 🔍',
  'h*o': () => 'Wildcard match! ✨',
  [Some]: () => 'Has some value! ✅',
  [None]: () => 'Has no value! ❌',
  _: () => 'Default case 🤷‍♂️',
});

console.log(result);
```

## 🧩 Features

- 💯 **Exact Value Matching**: Match exact string, number, boolean values
- 🔢 **Multiple Value Matching**: Match against multiple possible values
- 🚫 **Negation Matching**: Match when value is NOT one of specified values
- 🔍 **Regular Expression Matching**: Match using regex patterns
- ✨ **Wildcard Matching**: Simple wildcard patterns with `*` and `?`
- ✅ **Some/None Matching**: Check for presence or absence of a value
- 🔤 **Case Sensitivity Control**: Configure case sensitivity of string matching

## 📖 API Reference

### Core Functions

#### `match<R>(value, patterns, options?): R`

Main pattern matching function that evaluates a value against multiple patterns and returns the result of the matching pattern handler.

**Parameters:**
- `value`: The value to match (string, number, boolean, null, undefined)
- `patterns`: Object mapping patterns to handler functions
- `options`: Optional configuration
  - `caseSensitive`: Boolean (default: true)

**Returns:**
- The result of the first matching pattern handler

#### `ifLet<R>(value, pattern, handler): R | undefined`

Conditionally execute a handler if the value matches the pattern.

**Parameters:**
- `value`: The value to match
- `pattern`: The pattern to match against
- `handler`: Function to execute if match is successful

**Returns:**
- Result of handler if matched, otherwise undefined

#### `matches(value, pattern, options?): boolean`

Check if a value matches a pattern without executing a handler.

**Parameters:**
- `value`: The value to match
- `pattern`: The pattern to match against
- `options`: Optional configuration
  - `caseSensitive`: Boolean (default: true)

**Returns:**
- Boolean indicating whether the value matches the pattern

### Pattern Helpers

#### `any(...values): string`

Create a pattern that matches if the value equals any of the provided values.

```typescript
match(value, {
  [any('apple', 'banana', 'cherry')]: () => 'This is a fruit!'
})
```

#### `not(...values): string`

Create a pattern that matches if the value does NOT equal any of the provided values.

```typescript
match(value, {
  [not('red', 'blue', 'green')]: () => 'This is not a primary color!'
})
```

#### `regex(pattern, flags?): string`

Create a pattern that matches if the value matches the given regular expression.

```typescript
match(value, {
  [regex('^[0-9]+$')]: () => 'This is a number!',
  [regex('^[A-Z]+$', 'i')]: () => 'This contains only letters!'
})
```

### Special Patterns

#### `Some`

Matches any value that is not null or undefined.

```typescript
match(value, {
  [Some]: () => 'Value exists!'
})
```

#### `None`

Matches null or undefined values.

```typescript
match(value, {
  [None]: () => 'No value provided!'
})
```

#### `_` (underscore)

Default case that matches if no other pattern matches.

```typescript
match(value, {
  // other patterns...
  _: () => 'Default fallback'
})
```

## 🌟 Examples

### Basic Value Matching

```typescript
// String matching
const fruit = 'apple';
const fruitResult = match(fruit, {
  apple: () => '🍎 This is an apple',
  banana: () => '🍌 This is a banana',
  orange: () => '🍊 This is an orange',
  _: () => '❓ Unknown fruit'
});

// Number matching
const score = 85;
const grade = match(score, {
  [any(90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100)]: () => 'A 🎉',
  [any(80, 81, 82, 83, 84, 85, 86, 87, 88, 89)]: () => 'B 👍',
  [any(70, 71, 72, 73, 74, 75, 76, 77, 78, 79)]: () => 'C 😐',
  [any(60, 61, 62, 63, 64, 65, 66, 67, 68, 69)]: () => 'D 😕',
  _: () => 'F 😢'
});
```

### Regex and Wildcard Matching

```typescript
const input = 'hello123';

const result = match(input, {
  [regex('^[a-z]+$')]: () => 'Only letters',
  [regex('^[0-9]+$')]: () => 'Only numbers',
  [regex('^[a-z]+[0-9]+$')]: () => 'Letters followed by numbers',
  'hello*': () => 'Starts with hello',
  _: () => 'No pattern matched'
});
// Result: 'Letters followed by numbers'
```

### Option Objects and Case-Insensitive Matching

```typescript
const command = 'HELP';

const result = match(command, {
  help: () => 'Displaying help information',
  exit: () => 'Exiting the program',
  version: () => 'Current version: 1.0.0',
  _: () => 'Unknown command'
}, { caseSensitive: false });
// Result: 'Displaying help information' (despite case difference)
```

### Using ifLet for Conditional Logic

```typescript
const username = getUserInput(); // Could be a string or undefined

const welcomeMessage = ifLet(username, Some, () => `Welcome back, ${username}!`) || 
                       'Welcome, guest!';
```

### Handling Optional Values

```typescript
const data = fetchData(); // Could return null if fetch failed

const displayResult = match(data, {
  [Some]: () => `Data loaded: ${processData(data)}`,
  [None]: () => 'Failed to load data. Please try again.'
});
```

## 🎭 Pattern Matching Priority

When multiple patterns could match a value, the following priority rules apply:

1. Exact matches take highest priority
2. `Some` and `None` special patterns
3. `any` and `not` patterns
4. Regular expression patterns
5. Wildcard patterns (with patterns having fewer wildcards taking precedence)
6. Default case (`_`) has lowest priority

## ⚠️ Error Handling

If no pattern matches and no default case (`_`) is provided, an error will be thrown:

```typescript
// This will throw an error if value doesn't match any pattern
const result = match(value, {
  pattern1: () => 'Result 1',
  pattern2: () => 'Result 2'
  // No default case!
});
```

## 🤔 When to Use

Pattern matching is particularly useful for:

- 🌳 Handling complex conditional logic more elegantly than if/else chains
- 🎮 Processing user input and commands
- 🧠 Implementing state machines
- 🔄 Transforming data based on various conditions
- ⚙️ Processing configuration options

## 📄 License

MIT