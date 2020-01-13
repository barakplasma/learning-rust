// https://www.codewars.com/kata/roman-numerals-decoder/train/javascript

/*

Create a function that takes a Roman numeral as its argument and returns its value as a numeric decimal integer. You don't need to validate the form of the Roman numeral.

Modern Roman numerals are written by expressing each decimal digit of the number to be encoded separately, starting with the leftmost digit and skipping any 0s. So 1990 is rendered "MCMXC" (1000 = M, 900 = CM, 90 = XC) and 2008 is rendered "MMVIII" (2000 = MM, 8 = VIII). The Roman numeral for 1666, "MDCLXVI", uses each letter in descending order.

Example:

solution('XXI'); // should return 21
Help:

Symbol    Value
I          1
V          5
X          10
L          50
C          100
D          500
M          1,000

*/

export function solution(roman) {
    const conversionMap = new Map([
     ['I',    1],
     ['V',    5],
     ['X',   10],
     ['L',   50],
     ['C',  100],
     ['D',  500],
     ['M', 1000],
     [undefined, 0],
    ]);
    const answer = roman.split('').reduce((acc, cur, i, arr) => {
        const currentValue = conversionMap.get(cur) || 0;
        const nextValue = conversionMap.get(arr[i+1]) || 0;
        if (currentValue < nextValue) {
            return acc - currentValue;
        }
        return acc + currentValue;
    }, 0);
    return answer;
 }

import {expect} from 'chai';

[
    ['I', 1],
    ['XXI', 21],
    ['MMVIII', 2008],
    ['MDCLXVI', 1666],
    ['IV', 4],
    ['MMXVII', 2017]
].forEach(
    p => expect(solution(p[0])).to.equal(p[1])
);