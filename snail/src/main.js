const assert = require('assert');

const snail = (array) => {
  console.log(array)
  const dimensionSize = array[0].length;
  if (dimensionSize == 0) return [];
  if (dimensionSize == 1) return [array[0][0]];
  let stepCount = 0;
  let direction = 'r';
  let position = [0, 0];
  let listOfLimits = limits(dimensionSize);
  let limit = listOfLimits.shift();
  const turnRight = (previousDirection = direction) => {
    direction = {
      'r': 'd',
      'd': 'l',
      'l': 'u',
      'u': 'r'
    }[previousDirection];
    limit = listOfLimits.shift();
  }
  const output = [];
  const add = function () {
    const found = array[position[1]][position[0]];
    output.push(found)
  }
  const walk = function () {
    const move = {
      r: [position[0] + 1, position[1]],
      d: [position[0], position[1] + 1],
      l: [position[0] - 1, position[1]],
      u: [position[0], position[1] - 1]
    }[direction]
    position = move;
    add()
    limit--;
  }
  add()
  while (limit >= 0 && stepCount <= (dimensionSize * dimensionSize) + 1) {
    stepCount++;
    walk();
    if (limit == 0) {
      turnRight();
    }
  }
  return output;
}
function limits(d) {
  const output = [];
  if (d == 0) return [];
  if (d == 1) return [1];
  if (d == 2) return [1,1,1];
  output.push(...Array(3).fill(d - 1))
  const middle = Array(d - 3).fill(d - 1).map((x, i) => x - i - 1);
  output.push(...middle.reduce((acc, cur) => { acc.push(cur, cur); return acc }, []))
  output.push(1, 1)
  return output
}
assert.deepStrictEqual(limits(2), [1,1,1])
assert.deepStrictEqual(limits(3), [2, 2, 2, 1, 1])
assert.deepStrictEqual(limits(5), [4, 4, 4, 3, 3, 2, 2, 1, 1])
assert.deepStrictEqual(limits(7), [6, 6, 6, 5, 5, 4, 4, 3, 3, 2, 2, 1, 1])
assert.deepStrictEqual(snail([[]]), []);
assert.deepStrictEqual(snail([[1]]), [1]);
assert.deepStrictEqual(snail([[1, 2, 3], [4, 5, 6], [7, 8, 9]]), [1, 2, 3, 6, 9, 8, 7, 4, 5]);
assert.deepStrictEqual(snail([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15], [16, 17, 18, 19, 20], [21, 22, 23, 24, 25]]), [1, 2, 3, 4, 5, 10, 15, 20, 25, 24, 23, 22, 21, 16, 11, 6, 7, 8, 9, 14, 19, 18, 17, 12, 13]);
assert.deepStrictEqual(snail([[1, 2, 3, 4, 5, 6], [20, 21, 22, 23, 24, 7], [19, 32, 33, 34, 25, 8], [18, 31, 36, 35, 26, 9], [17, 30, 29, 28, 27, 10], [16, 15, 14, 13, 12, 11]]), [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36]);
assert.deepStrictEqual(snail([[172,833],[5,480]]), [172,833,480,5])
assert.deepStrictEqual(snail([ [ 633 ] ]), [633])