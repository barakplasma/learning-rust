const assert = require('assert');

function isPangram(string: string){
  const make_str_set = (str: string) => new Set(str.split(''))
  const alphabet = make_str_set("thequickbrownfoxjumpsoverthelazydog")
  const test_str = make_str_set(string.toLowerCase());
  let flag = true;
  alphabet.forEach(each_letter_in_alphabet => {
    if (!test_str.has(each_letter_in_alphabet)) {
      flag = false;
    }
  })
  return flag;
}
// https://www.codewars.com/kata/545cedaa9943f7fe7b000048/train/javascript

let string1 = "The quick brown fox jumps over the lazy dog."
assert.equal(isPangram(string1), true)
var string2 = "This is not a pangram."
assert.equal(isPangram(string2), false)
let string3 = 'ABCD45EFGH,IJK,LMNOPQR56STUVW3XYZ'
assert.equal(isPangram(string3), true)