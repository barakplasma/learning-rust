// https://www.codewars.com/kata/5a25ac6ac5e284cfbe000111/train/rust

function childColor(colours) {
  if (colours[0] === colours[1] || colours.length === 1) {
    return colours[0];
  }
  colours.sort();
  if (colours[0] === 'B' && colours[1] === 'G'){
    return 'R'
  }
  if (colours[0] === 'G' && colours[1] === 'R'){
    return 'B'
  }
  if (colours[0] === 'B' && colours[1] === 'R'){
    return 'G'
  }
}

function overLappingWindows(arr){
  let windows = [];
  for (let window = 0; window < arr.length - 1; window++) {
    windows.push(arr.slice(window, window+2))
  }
  return windows;
}

function triangle(row) {
  let colorChunks = row.split('');

  while (colorChunks.length > 1) {
    colorChunks = overLappingWindows(colorChunks)
    let newChunks = [];
    for (chunk of colorChunks) {
      newChunks.push(childColor(chunk));
    }
    colorChunks = newChunks;
  }
  return colorChunks[0]
}