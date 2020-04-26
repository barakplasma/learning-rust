const { Test } = require("../../codewarsTestFramework");

function makeRow(rowDim, offset) {
  if (rowDim === 0 && offset === 0) return [];
  let row = [];
  for (let col = 1; col <= rowDim; col++) {
    row.push([col, col + offset + 1])
  }
  return row;
}

function makeBoard(nDim) {
  let board = [];
  for (let row = 0; row < nDim; row++) {
    let rowToPush = makeRow(nDim, row);
    board.push(...rowToPush)
  }
  return board;
}

function simplifyFraction([numerator, denominator]) {
  var gcd = function gcd(a, b) {
    return b ? gcd(b, a % b) : a;
  };
  let gcdVal = gcd(numerator, denominator);
  let fraction = [numerator / gcdVal, denominator / gcdVal];
  if (Number.isInteger(fraction[0] / fraction[1])) {
    return [fraction[0] / fraction[1]]
  }
  return fraction;
}

function sumBoard(board) {
  let sum = board.reduce((acc, row) => {
    if (acc[0] === 0) return row;
    let num = (row[0] * acc[1]) + (row[1] * acc[0]);
    let den = row[1] * acc[1];
    acc = [num, den];
    acc = simplifyFraction(acc);
    return acc;
  }, [0, 0]);
  if (sum[0] === sum[1]) return [sum[1]];
  return sum;
}

function game(nDim) {
  let board = makeBoard(nDim);
  return sumBoard(board);
}

Test.describe("make row", () => {
  Test.assertSimilar(makeRow(0, 0), []);
  Test.assertSimilar(makeRow(8, 0), [[1, 2], [2, 3], [3, 4], [4, 5], [5, 6], [6, 7], [7, 8], [8, 9]]);
})

Test.describe("sum board", () => {
  Test.assertSimilar(sumBoard(makeBoard(8)), [32]);
})

Test.describe("simplify fraction", () => {
  Test.assertSimilar(simplifyFraction([1, 2]), [1, 2]);
  Test.assertSimilar(simplifyFraction([4, 6]), [2, 3]);
  Test.assertSimilar(simplifyFraction([990, 1000]), [99, 100]);
  Test.assertSimilar(simplifyFraction([10000, 20000]), [1, 2]);
})

Test.describe("make board", () => {
  Test.assertSimilar(makeBoard(0), []);
  // Test.assertSimilar(makeBoard(8), [
  //   [[1, 2], [2, 3], [3, 4], [4, 5], [5, 6], [6, 7], [7, 8], [8, 9]],
  //   [[1, 3], [2, 4], [3, 5], [4, 6], [5, 7], [6, 8], [7, 9], [8, 10]],
  //   [[1, 4], [2, 5], [3, 6], [4, 7], [5, 8], [6, 9], [7, 10], [8, 11]],
  //   [[1, 5], [2, 6], [3, 7], [4, 8], [5, 9], [6, 10], [7, 11], [8, 12]],
  //   [[1, 6], [2, 7], [3, 8], [4, 9], [5, 10], [6, 11], [7, 12], [8, 13]],
  //   [[1, 7], [2, 8], [3, 9], [4, 10], [5, 11], [6, 12], [7, 13], [8, 14]],
  //   [[1, 8], [2, 9], [3, 10], [4, 11], [5, 12], [6, 13], [7, 14], [8, 15]],
  //   [[1, 9], [2, 10], [3, 11], [4, 12], [5, 13], [6, 14], [7, 15], [8, 16]]]);
})

Test.describe("Basic tests", function () {
  Test.assertSimilar(game(0), [0])
  Test.assertSimilar(game(1), [1, 2])
  Test.assertSimilar(game(8), [32])
  // Test.assertSimilar(game(101), [10201, 2])
  // Test.assertSimilar(game(40), [800])
})
