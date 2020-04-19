function rot13oneLetter(letter) {
  let isUpper = letter.toLowerCase() !== letter;
  let alphabet = 'abcdefghijklmnopqrstuvwxyz';
  let index = alphabet.indexOf(letter.toLowerCase());
  if (index === -1) {
    return letter;
  }
  let newLetterIndex = (index + 13) % 26;
  let newLetter = alphabet[newLetterIndex];
  return isUpper ? newLetter.toUpperCase() : newLetter ;
}

function rot13(message){
  return message.split('').map(rot13oneLetter).join('');
}