function firstNonConsecutive (arr) {
  let previous = arr[0];
  let firstNonConsecutiveN = arr.find((x, i) => {
    let notFirst = i > 0;
    let notEqual = (x - 1 - previous) > 0;
    if (notFirst && notEqual) {
      return true;
    }
    previous = x;
    return false;
  });
  return firstNonConsecutiveN !== undefined ? firstNonConsecutiveN : null ;
}