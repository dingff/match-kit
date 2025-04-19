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

export const Some: string = _some()

export const None: string = _none()

export function not(...values: MatchValue[]): string {
  return _not(values)
}

export function any(...values: MatchValue[]): string {
  return _any(values)
}

export function regex(pattern: string, flags?: string): string {
  return _regex(pattern, flags)
}

/**
 * The patterns for match must be exhaustive. If you only care about one case, ifLet is more suitable.
 */
export function match<R>(value: MatchValue, patterns: PatternMap<R>, options?: Options): R {
  return _match(value, patterns, options)
}

/**
 * If value matches pattern, execute handler and return its result, otherwise return undefined.
 */
export function ifLet<R>(
  value: MatchValue,
  pattern: MatchValue,
  handler: PatternHandler<R>,
): R | undefined {
  return _ifLet(value, pattern, handler)
}

/**
 * Determine whether value matches the given pattern, returns true or false.
 */
export function matches(value: MatchValue, pattern: MatchValue, options?: Options): boolean {
  return _matches(value, pattern, options)
}
