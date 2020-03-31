// range extraction
// https://www.codewars.com/kata/51ba717bb08c1cd60f00002f/train/javascript
const assert = require('assert');
const range = (s, e) => Array.from({length: e-s}, (v,i)=>i+s);

function solution(list){
  let uniqInts = Array.from(new Set(list)).sort((a,b) => a-b);
  let output = uniqInts.reduce((acc, cur, i) => {
    let last3 = acc.slice(-3);
    last3;
    if (last3.length === 3 && last3.join() === range(cur-3, cur).join()) {
      acc.pop();acc.pop();acc.pop();
      acc.push(`${last3[0]}-${cur}`)
    } else {
      acc.push(`${cur}`);
    }
    return acc;
  }, [''])
  return output.join()
}

// assert.equal(solution([-6, -3, -2]), "-6,-3,-2")

assert.equal(solution([-6, -3, -2, -1,0,1]), "-6,-3-1")
assert.equal(solution([-6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20]), "-6,-3-1,3-5,7-11,14,15,17-20")