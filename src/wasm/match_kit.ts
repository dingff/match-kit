export function some(): string {
  return ''
}
export function none(): string {
  return ''
}
export function not(args: any[]): any {
  console.log(args)
}
export function any(args: any[]): any {
  console.log(args)
}
export function regex(pattern: any, flags?: any): any {
  console.log(pattern, flags)
}
export function match(
  value: any,
  patterns: any,
  options?: {
    caseSensitive?: boolean
  },
): any {
  console.log(value, patterns, options)
}
export function ifLet(value: any, pattern: any, handler: any): any {
  console.log(value, pattern, handler)
}
export function matches(
  value: any,
  pattern: any,
  options?: {
    caseSensitive?: boolean
  },
): any {
  console.log(value, pattern, options)
}
