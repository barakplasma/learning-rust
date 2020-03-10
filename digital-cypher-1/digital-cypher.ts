const assert = require('assert');
// https://www.codewars.com/kata/592e830e043b99888600002d/train/javascript
function encode(str: string,  n: number) {
  // Digital Cypher assigns to each letter of the alphabet unique number.
  const alphabet = new Map(
    "abcdefghijklmnopqrstuvwxyz"
      .split('')
      .map((l, i) => [l,i+1])
  )
  // Instead of letters in encrypted word we write the corresponding number
  const str_to_vec = str.split('').map(l => alphabet.get(l));
  //Then we add to each obtained digit consecutive digits from the key.
  const key = n.toString().split('').map(Number);
  const encoded_vec = str_to_vec.map((vector_element, i) => vector_element+key[i%key.length]);
  return encoded_vec;
}

assert.deepEqual(encode("scout",1939) , [ 20, 12, 18, 30, 21] );
assert.deepEqual(encode("masterpiece",1939) , [ 14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]);