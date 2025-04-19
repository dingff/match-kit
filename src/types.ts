export type MatchValue = string | number | boolean | null | undefined

export type PatternHandler<R> = () => R
export type PatternMap<R> = {
  /**
   * The special key '_' is used for the default handler.
   */
  [pattern: string]: PatternHandler<R>
}

export type Options = {
  caseSensitive?: boolean
}
