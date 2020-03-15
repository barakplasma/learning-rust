const assert = require('assert');

function moreZeros(s){
  let arr = s.split('').map(c => c.charCodeAt(0)).map(n => n.toString(2)); /*?*/
  let chars = arr.filter(s => s.split('0').length > s.split('1').length).map(s => parseInt(s, 2)); /*?*/
  let uniq = new Set(chars);
  return String.fromCharCode(...Array.from(uniq)).split('')
}

assert.deepEqual(moreZeros('abcde'),['a','b','d'])
assert.deepEqual(moreZeros('Great job!'),['a', ' ', 'b', '!'])
assert.deepEqual(moreZeros('DIGEST'),['D','I','E','T'])
assert.deepEqual(moreZeros('abcdeabcde'),['a','b','d'])