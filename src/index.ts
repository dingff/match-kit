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
} from './wasm/match_kit'

/**
 * Matches any value that is not null or undefined.
 */
export const Some: string = _some()

/**
 * Matches null or undefined values.
 */
export const None: string = _none()

/**
 * Create a pattern that matches if the value does NOT equal any of the provided values.
 * @param values Values to exclude from matching.
 */
export function not(...values: MatchValue[]): string {
  return _not(values)
}

/**
 * Create a pattern that matches if the value equals any of the provided values.
 * @param values Values to match.
 */
export function any(...values: MatchValue[]): string {
  return _any(values)
}

/**
 * Create a pattern that matches if the value matches the given regular expression.
 * @param pattern Regex pattern string.
 * @param flags Optional regex flags.
 */
export function regex(pattern: string, flags?: string): string {
  return _regex(pattern, flags)
}

/**
 * Evaluates a value against multiple patterns and returns the result of the matching pattern handler.
 * Note that the patterns for match must be exhaustive. If you only care about one case, ifLet is more suitable.
 * @param value The value to match.
 * @param patterns Pattern map, where each key is a pattern and value is a handler function.
 * @param options Optional settings (caseSensitive).
 */
export function match<R>(value: MatchValue, patterns: PatternMap<R>, options?: Options): R {
  return _match(value, patterns, options)
}

/**
 * If value matches pattern, execute handler and return its result, otherwise return undefined.
 * @param value The value to match.
 * @param pattern The pattern to match against.
 * @param handler Handler function to execute if matched.
 */
export function ifLet<R>(
  value: MatchValue,
  pattern: MatchValue,
  handler: PatternHandler<R>,
): R | undefined {
  return _ifLet(value, pattern, handler)
}

/**
 * Returns true if value matches the given pattern, otherwise false.
 * @param value The value to match.
 * @param pattern The pattern to match against.
 * @param options Optional settings (caseSensitive).
 */
export function matches(value: MatchValue, pattern: MatchValue, options?: Options): boolean {
  return _matches(value, pattern, options)
}
