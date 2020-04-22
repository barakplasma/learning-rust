import expect from 'expect';

/**
 * @typedef {(passphrase: string[], shift_by: number) => string[]} transformer
*/

  /**
 * shift each letter by a given number but the transformed letter must be a letter (circular shift)
 * keep such as non alphabetic and non digit characters,
 * @type {transformer}
 */
let shifter = function (s, n) {
  /**
   * 
   * @param {string} c 
   */
  let shift = c => {
    if (/[a-zA-Z]/.test(c)) {
      let index = c.charCodeAt(0);
      let firstCharacter = /[a-z]/.test(c) ? 'a'.charCodeAt(0) : 'A'.charCodeAt(0);
      let alpha_index = index - firstCharacter;
      let shifted_index = (alpha_index + n) % 26;
      return String.fromCharCode(shifted_index + firstCharacter);
    }
    return c;
  }
  return s.map(shift)
}

/**
 * reverse the whole result.
 * @type {transformer}
 */
let reverser = (s) => {
  return s.reverse()
}

/**
 * downcase each letter in odd position, upcase each letter in even position (the first character is in position 0),
 * @type {transformer}
 */
let caser = (s) => {
  return s.map((c, i) => i % 2 == 0 ? c.toLocaleUpperCase() : c.toLocaleLowerCase())
}

/**
 * replace each digit by its complement to 9,
 * @type {transformer}
 */
let complementer = s => {
  return s.map((c) => {
    if (/[0-9]/.test(c)) {
      return (9-parseInt(c)).toString();
    }
    return c
  });
}

/**
 * 
 * @param {string} s original passphrase
 * @param {number} n shift by this much
 * @returns {string} transformed passphrase
 */
function playPass(s, n) {
  /**
   * @type {transformer[]}
   */
  const transformations = [
    caser,
    shifter,
    reverser,
    complementer
  ];

  return transformations.reduce((acc, cur) => {
    return cur(acc.split(''), n).join('')
  }, s);
}

expect(playPass("I LOVE YOU!!!", 1)).toEqual("!!!vPz fWpM J")

expect(playPass("MY GRANMA CAME FROM NY ON THE 23RD OF APRIL 2015", 2)).toEqual("4897 NkTrC Hq fT67 GjV Pq aP OqTh gOcE CoPcTi aO")
