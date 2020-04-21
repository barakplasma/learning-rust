export function evenNumbers(array: number[], n: number): number[] {
  return array.filter(n => n % 2 == 0).slice(-n);
}