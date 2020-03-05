// from https://www.codewars.com/kata/55466989aeecab5aac00003e/train/typescript

export class G964 {
  public static sqInRect(l: number, w: number): number[] {
    if (l === w ) return null;
    let squares: number[] = [];
    let remaining: number[] = [l, w];
    let remaining_dimension = type => () => Math[type].apply(null, remaining)
    let smallest_dimension = remaining_dimension("min"); 
    let largest_dimension = remaining_dimension("max"); 
    while (smallest_dimension() >= 1) {
      let remaining_largest_dimension = largest_dimension();
      let remaining_smallest_dimension = smallest_dimension();
      squares.push(remaining_smallest_dimension);
      remaining = [remaining_largest_dimension-remaining_smallest_dimension, remaining_smallest_dimension];
    }
    return squares;
  }
}

import { assert } from "chai";

function testing(l, w, expected) {
  assert.deepEqual(G964.sqInRect(l, w), expected)
}

testing(5, 5, null);
testing(20, 14, [14, 6, 6, 2, 2, 2])
testing(5, 3, [3, 2, 1, 1]);