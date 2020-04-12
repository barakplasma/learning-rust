function rowWeights(array){
  let whichTeam = 1;
  let split = (acc, cur) => {
    if (whichTeam === 1) {
      acc[0].push(cur)
    }
    if (whichTeam === 2) {
      acc[1].push(cur)
    }
    whichTeam = whichTeam === 1 ? 2 : 1;
    return acc;
  }
  let sum = (arr) => {
    return arr.reduce((acc, cur) => acc+cur, 0)
  }
  let teams = array.reduce(split, [[], []])
  return teams.map(sum)
}