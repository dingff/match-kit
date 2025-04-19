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

type MatchValue = string | number | boolean | null | undefined

const Some: string = _some()
const None: string = _none()
function not(...values: MatchValue[]): string {
  return _not(values)
}
function any(...values: MatchValue[]): string {
  return _any(values)
}

function regex(pattern: string, flags?: string): string {
  return _regex(pattern, flags)
}

type PatternHandler<R> = () => R
type PatternMap<R> = {
  /**
   * The special key '_' is used for the default handler.
   */
  [pattern: string]: PatternHandler<R>
}

/**
 * The patterns for match must be exhaustive. If you only care about one case, ifLet is more suitable.
 */
function match<R>(
  value: MatchValue,
  patterns: PatternMap<R>,
  options?: {
    caseSensitive?: boolean
  },
): R {
  return _match(value, patterns, options)
}

/**
 * If value matches pattern, execute handler and return its result, otherwise return undefined.
 */
function ifLet<R>(
  value: MatchValue,
  pattern: MatchValue,
  handler: PatternHandler<R>,
): R | undefined {
  return _ifLet(value, pattern, handler)
}

/**
 * Determine whether value matches the given pattern, returns true or false.
 */
function matches(
  value: MatchValue,
  pattern: MatchValue,
  options?: {
    caseSensitive?: boolean
  },
): boolean {
  return _matches(value, pattern, options)
}

export { match, ifLet, matches, Some, None, any, not, regex }
