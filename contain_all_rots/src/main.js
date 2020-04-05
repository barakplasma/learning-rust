function containAllRots(str, arr) {
  if (str === "") return true;
  let rotations = [];
  for (let i = 0; i < str.length; i++) {
    rotations.push(str.slice(i, str.length)+str.slice(0, i));
  }
  return rotations.every(rot => arr.includes(rot));
}