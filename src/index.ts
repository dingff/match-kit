import type { MatchValue, Options, PatternHandler, PatternMap } from './types'
import {
  any as _any,
  ifLet as _ifLet,
  match as _match,
  matches as _matches,
  none as _none,
  not as _not,
  regex as _regex,
  some as _some,
  when as _when,
} from './wasm/match_kit'

/**
 * A predefined pattern that matches any value except null or undefined.
 * Used in pattern matching to capture values that are present and defined.
 *
 * @example
 * ```typescript
 * match(value, {
 *   [Some]: () => `Value exists`,
 *   [None]: () => 'No value'
 * })
 * ```
 */
export const Some: string = _some()

/**
 * A predefined pattern that matches null or undefined values.
 * Used in pattern matching to handle absent or undefined values.
 *
 * @example
 * ```typescript
 * match(value, {
 *   [Some]: () => `Has value`,
 *   [None]: () => 'Value is null or undefined'
 * })
 * ```
 */
export const None: string = _none()

/**
 * Creates a conditional pattern that matches when the specified condition evaluates to true.
 * Enables custom matching logic through predicate functions or direct boolean values.
 *
 * @template T - The type of value being evaluated
 * @param condition - A predicate function that receives the value and returns a boolean,
 *                   or a boolean value for direct conditional matching
 * @returns A pattern string that can be used in match expressions
 *
 * @example
 * ```typescript
 * match(number, {
 *   [when((n) => n > 0)]: () => 'Positive',
 *   [when((n) => n < 0)]: () => 'Negative',
 *   [when(false)]: () => 'Never matches'
 * })
 * ```
 */
export function when<T>(condition: ((value: T) => boolean) | boolean): string {
  return _when<T>(condition)
}

/**
 * Creates a pattern that matches if the value equals any of the provided values.
 * Performs equality comparison using strict equality (===) semantics.
 *
 * @param values - Variable number of values to match against
 * @returns A pattern string that matches any of the specified values
 *
 * @example
 * ```typescript
 * match(status, {
 *   [any('success', 'completed', 'done')]: () => 'Operation successful',
 *   [any('error', 'failed')]: () => 'Operation failed'
 * })
 * ```
 */
export function any(...values: MatchValue[]): string {
  return _any(values)
}

/**
 * Creates a pattern that matches if the value does NOT equal any of the provided values.
 * Performs negated equality comparison using strict inequality (!==) semantics.
 *
 * @param values - Variable number of values to exclude from matching
 * @returns A pattern string that matches values not in the exclusion list
 *
 * @example
 * ```typescript
 * match(userRole, {
 *   [not('admin', 'moderator')]: () => 'Regular user permissions',
 *   [any('admin', 'moderator')]: () => 'Elevated permissions'
 * })
 * ```
 */
export function not(...values: MatchValue[]): string {
  return _not(values)
}

/**
 * Creates a pattern that matches string values against a regular expression.
 * Supports standard JavaScript regex patterns with optional flags.
 *
 * @param pattern - Regular expression pattern string
 * @param flags - Optional regex flags (e.g., 'i' for case-insensitive, 'g' for global)
 * @returns A pattern string that performs regex matching
 *
 * @example
 * ```typescript
 * match(email, {
 *   [regex('^[\\w-\\.]+@([\\w-]+\\.)+[\\w-]{2,4}$')]: () => 'Valid email',
 *   [regex('\\d+', 'g')]: () => 'Contains numbers'
 * })
 * ```
 */
export function regex(pattern: string, flags?: string): string {
  return _regex(pattern, flags)
}

/**
 * Performs exhaustive pattern matching against a value and returns the result of the first matching pattern handler.
 * All possible cases must be covered in the pattern map to ensure exhaustiveness.
 * For single-case matching or when you only care about specific patterns, use `ifLet` instead.
 *
 * @template R - The return type of the pattern handlers
 * @param value - The value to evaluate against patterns
 * @param patterns - A mapping of patterns to their corresponding handler functions
 * @param options - Optional configuration object for matching behavior
 * @param options.caseSensitive - Whether string comparisons should be case-sensitive (default: true)
 * @returns The result returned by the matching pattern handler
 * @throws {Error} If no pattern matches and patterns are not exhaustive
 *
 * @example
 * ```typescript
 * const result = match(httpStatus, {
 *   [any(200, 201, 204)]: () => 'Success',
 *   [when((code) => code >= 400 && code < 500)]: () => 'Client Error',
 *   [when((code) => code >= 500)]: () => 'Server Error'
 * })
 * ```
 */
export function match<R>(value: MatchValue, patterns: PatternMap<R>, options?: Options): R {
  return _match(value, patterns, options)
}

/**
 * Conditionally executes a handler if the value matches the specified pattern.
 * Returns the handler result if matched, otherwise returns undefined.
 * Ideal for single-case pattern matching without exhaustiveness requirements.
 *
 * @template R - The return type of the handler function
 * @param value - The value to evaluate against the pattern
 * @param pattern - The pattern to match against
 * @param handler - Function to execute if the pattern matches
 * @returns The handler result if matched, undefined otherwise
 *
 * @example
 * ```typescript
 * const greeting = ifLet(user.role, 'admin', () =>
 *   `Welcome, administrator!`
 * ) ?? 'Welcome, user!'
 * ```
 */
export function ifLet<R>(
  value: MatchValue,
  pattern: MatchValue,
  handler: PatternHandler<R>,
): R | undefined {
  return _ifLet(value, pattern, handler)
}

/**
 * Tests whether a value matches the specified pattern without executing any handlers.
 * Returns a boolean indicating match success or failure.
 *
 * @param value - The value to test against the pattern
 * @param pattern - The pattern to evaluate
 * @param options - Optional configuration object for matching behavior
 * @param options.caseSensitive - Whether string comparisons should be case-sensitive (default: true)
 * @returns true if the value matches the pattern, false otherwise
 *
 * @example
 * ```typescript
 * if (matches(userInput, regex('^\\d+$'))) {
 *   console.log('Input is numeric')
 * }
 *
 * const isValidStatus = matches(response.status, any(200, 201, 204))
 * ```
 */
export function matches(value: MatchValue, pattern: MatchValue, options?: Options): boolean {
  return _matches(value, pattern, options)
}
