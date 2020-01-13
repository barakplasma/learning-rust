const {assert} = require('chai');

export function nbMonths(startPriceOld: number, startPriceNew: number, savingperMonth: number, percentLossByMonth: number){
  let currentBalance = 0, months = 0, currentDepreciationRate = percentLossByMonth;
  let currentPrices = [startPriceOld, startPriceNew];
  function difference (prices: number[], balance: number) {
    let assets = (balance + prices[0]);
    let liabilities = prices[1];
    let diff = Math.round( assets - liabilities );
    return diff;
  }
  const depreciateAsset = (p: number) => p - (p * (currentDepreciationRate/100))

  while (difference(currentPrices, currentBalance) < 0) {
    currentBalance += savingperMonth;
    currentPrices = currentPrices.map(depreciateAsset)
    if (months % 2 === 0) {
      currentDepreciationRate += .5;
    }
    months++;
  }

  return [months, difference(currentPrices, currentBalance)]
}

assert.deepEqual(nbMonths(2000, 8000, 1000, 1.5), [6, 766])
assert.deepEqual(nbMonths(12000, 8000, 1000, 1.5) ,[0, 4000])
assert.deepEqual(nbMonths(8000, 8000, 1000, 1.5), [0, 0])