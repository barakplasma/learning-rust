// https://www.codewars.com/kata/where-my-anagrams-at/javascript

import { assert } from 'chai';

function anagrams(word1, words) {
  const sortString = str => [...str].sort((a, b) => a.localeCompare(b)).join('');
  let ss1 = sortString(word1);
  return words.filter(
    word2 => {
      let ss2 = sortString(word2);
      return ss1 === ss2;
    }
  );
}

assert.deepEqual(anagrams('laser', ['lazing', 'lazy', 'lacer']), []);
assert.deepEqual(anagrams('abba', ['aabb', 'abcd', 'bbaa', 'dada']), ['aabb', 'bbaa']);
assert.deepEqual(anagrams('racer', ['crazer', 'carer', 'racar', 'caers', 'racer']), ['carer', 'racer']);
assert.deepEqual(anagrams('abba', [
  'aabb',
  'abab',
  'abbaa',
  'abbab',
  'abbba',
  'abcd',
  'baaab',
  'baab',
  'baba',
  'babaa',
  'bbaa']), ['aabb', 'abab', 'baab', 'baba', 'bbaa']);