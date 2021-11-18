// https://www.codewars.com/kata/52724507b149fa120600031d/train/javascript
const digitMap = {
  0: 'zero',
  1: 'one',
  2: 'two',
  3: 'three',
  4: 'four',
  5: 'five',
  6: 'six',
  7: 'seven',
  8: 'eight',
  9: 'nine',
  10: 'ten',
  11: 'eleven',
  12: 'twelve',
  13: 'thirteen',
  14: 'fourteen',
  15: 'fifteen',
  16: 'sixteen',
  17: 'seventeen',
  18: 'eighteen',
  19: 'nineteen',
  20: 'twenty',
  30: 'thirty',
  40: 'fourty',
  50: 'fifty',
  60: 'sixty',
  70: 'seventy',
  80: 'eighty',
  90: 'ninety'
}

class NumericPlace {
  constructor(position = 1, multiplier, n) {
    this.position = position;
    this.multiplier = multiplier;
    this.n = n;
    this.getDigit();
    this.words = typeof this.toWords() === 'string'? this.toWords().trim() : undefined;
  }
  
  getDigit() {
    this.digit = +this.n.toString().split('').reverse().slice(this.position, this.position+1) * this.multiplier;
  }
  
  toWords() {
    return this.digit ? digitMap[this.digit] : undefined;
  }
}

class OnesPlace extends NumericPlace {
  constructor(n) {
    super(0, 1, n);
  }
}

class TensPlace extends NumericPlace {
  constructor(n) {
    super(1, 10, n);
  }
}

class HundredsPlace extends NumericPlace {
  constructor(n) {
    super(2, 1, n);
  }
  
  toWords(n) {
    return this.digit && this.digit !== 0 ? digitMap[this.digit] + ' hundred' : undefined;
  }
}

class ThousandsPlace extends NumericPlace {
  constructor(n) {
    super(3, 1, n);
  }
  
  toWords(n) {
    return this.digit && this.digit !== 0 ? digitMap[this.digit] + ' thousand' : undefined;
  }
}

class TensAndOnes {
  constructor(n) {
    let tens = new TensPlace(n)
    let ones = new OnesPlace(n)
    this.words = tens.words && ones.words ? `${tens.words}-${ones.words}` : 
      tens.words && !ones.words ? tens.words : ones.words;
  }
}

function number2words(n) {
  if (n <= 20) {
    return digitMap[n]
  }
  return [new ThousandsPlace(n), new HundredsPlace(n), new TensAndOnes(n)]
    .map(x => x && x.words)
    .filter(x => x && x.words !== 'undefined')
    .join(' ')
}

/*

describe("Tests", () => {
  it("test", () => {
Test.assertEquals(number2words(0),"zero");
Test.assertEquals(number2words(1),"one");
Test.assertEquals(number2words(8),"eight");
Test.assertEquals(number2words(10),"ten");
Test.assertEquals(number2words(19),"nineteen");
Test.assertEquals(number2words(20),"twenty");
Test.assertEquals(number2words(22),"twenty-two");
Test.assertEquals(number2words(54),"fifty-four");
Test.assertEquals(number2words(80),"eighty");
Test.assertEquals(number2words(98),"ninety-eight");
Test.assertEquals(number2words(100),"one hundred");
Test.assertEquals(number2words(301),"three hundred one");
Test.assertEquals(number2words(793),"seven hundred ninety-three");
Test.assertEquals(number2words(800),"eight hundred");
Test.assertEquals(number2words(650),"six hundred fifty");
Test.assertEquals(number2words(1000),"one thousand");
Test.assertEquals(number2words(1003),"one thousand three");

  });
});


*/
