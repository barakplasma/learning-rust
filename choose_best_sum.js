const assert = require('assert');
// https://www.codewars.com/kata/55e7280b40e1c4a06d0000aa/train/rust
/**
 * 
 * @param {number} maximum_towns 
 * @param {number} maximum_distance 
 * @param {Array<number>} list_distances_between_towns list of distances between these towns
 */
function chooseBestSum(maximum_distance, maximum_towns, list_distances_between_towns) {
  let current_maximum_distance = 0;
  if (list_distances_between_towns.length > maximum_towns) {
    return current_maximum_distance == 0 ? null : current_maximum_distance;
  }
  current_maximum_distance = chooseBestSum(maximum_distance - current_maximum_distance, maximum_towns - 1, list_distances_between_towns.slice(1))
}

assert.equal(chooseBestSum(163, 3, [50, 55, 56, 57, 58]), 163)

assert.equal(chooseBestSum(163, 3,  [50]), null)

assert.equal(chooseBestSum(230, 3, [91, 74, 73, 85, 73, 81, 87]), 228)