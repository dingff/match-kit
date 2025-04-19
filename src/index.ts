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
   * 特殊键 '_' 表示默认处理
   */
  [pattern: string]: PatternHandler<R>
}

/**
 * match 的模式必须是穷尽的，如果你只关心一种情况，ifLet 更适合
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
 * 如果 value 匹配 pattern，则执行 handler 并返回其结果，否则返回 undefined
 */
function ifLet<R>(
  value: MatchValue,
  pattern: MatchValue,
  handler: PatternHandler<R>,
): R | undefined {
  return _ifLet(value, pattern, handler)
}

/**
 * 判断 value 是否匹配给定的 pattern，返回 true 或 false
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
