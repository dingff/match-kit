import type { MatchValue, Options, PatternHandler, PatternMap } from '../types'

export declare function some(): string
export declare function none(): string
export declare function when(condition: ((value: any) => boolean) | boolean): string
export declare function any(args: MatchValue[]): string
export declare function not(args: MatchValue[]): string
export declare function regex(pattern: string, flags?: string): string
export declare function match<R>(value: MatchValue, patterns: PatternMap<R>, options?: Options): R
export declare function ifLet<R>(
  value: MatchValue,
  pattern: MatchValue,
  handler: PatternHandler<R>,
): R | undefined
export declare function matches(value: MatchValue, pattern: MatchValue, options?: Options): boolean
