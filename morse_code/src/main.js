const decodeMorse = function(morseCode){
  return morseCode.split(' ').map(l=>{return l == '' ? ' ' : MORSE_CODE[l] }).join('').replace(/\s+/g, ' ').trim();
}