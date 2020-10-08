export function decodeMorse(morseCode: string): string {
  return morseCode
  .split(' ')
  .map(l =>{return l == '' ? ' ' : MORSE_CODE[l] })
  .join('')
  .replace(/\s+/g, ' ')
  .trim();
}