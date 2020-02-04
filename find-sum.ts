const {assert} = require('chai');

const range = n => new Array(n).fill(0).map((_,i)=>i+1);
// https://www.codewars.com/kata/57f36495c0bb25ecf50000e7/javascript 
function findSum(n) {
  return range(n).reduce((acc, cur) => {
    if (cur % 3 == 0 || cur % 5 == 0) {
      return acc+cur
    } else {
      return acc
    }
  },0)
}

assert.equal(findSum(5), 8);
assert.equal(findSum(10), 33);