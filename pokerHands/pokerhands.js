var Result = { "win": 1, "loss": 2, "tie": 3 }

function PokerHand(hand) {
  this.value = hand.split(' ').map(p=>({suit: suitName(p[1]), value: valueName(p[0])}));
  this.sortHand();
  this.handValue = this.categorizeHand();
}

const suitName = (suit) => ({'S': 'SPADE', 'H': 'HEART', 'C': 'CLUB', 'D':'DIAMOND'}[suit])
const valueName = (val) => {
  let int = parseInt(val);
  if (int) {
    return val
  }
  return {'T': '10', 'J': 'JACK', 'Q': 'QUEEN', 'K': 'KING', 'A': 'ACE'}[val]
}

const HandValues = [
  'Highcard',
  'Pair',
  'Two Pairs',
  'Three of a kind',
  'Straight',
  'Flush',
  'Full house',
  'Four of a kind',
  'Straight flush',
  'Royal flush'
]

const CardValues = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "JACK", "QUEEN", "KING", "ACE"]

let cardValue = a => CardValues.findIndex(x=>x==a)

PokerHand.prototype.compareCards = function (a,b) {return cardValue(a.value)-cardValue(b.value)};

PokerHand.prototype.sortHand = function(){
  this.value.sort(this.compareCards)
}

PokerHand.prototype.highCard = function(){
  let highCard = this.value.slice(-1)[0].value;
  return highCard;
}

PokerHand.prototype.isSequence = function(){
  let previousCard;
  let anyOutOfSequence = false
  for (let card of this.value) {
    if (previousCard && this.compareCards(card, previousCard) !== 1) {
      anyOutOfSequence = true
    }
    previousCard = card;
  }
  return !anyOutOfSequence
};

PokerHand.prototype.isRoyalFlush = function(){
  return this.highCard() === 'ACE' && this.isSequence()
};

PokerHand.prototype.isStraightFlush = function(){
  return this.isSequence() && this.isFlush();
};

PokerHand.prototype.sameValueCount = function(){
  let s = new Set([2,45,5]);
  return 4;
};

PokerHand.prototype.isFourOfAKind = function(){
  return this.sameValueCount() === 4;
};

PokerHand.prototype.isFlush = function(){
  let firstSuit = this.value.slice(-1)[0].suit;
  return this.value.every(card => card.suit == firstSuit)
};

PokerHand.prototype.categorizeHand = function(){
  
  if (this.isRoyalFlush()) {
    return ['Royal flush', this.highCard()]
  }
  if (this.isStraightFlush()) {
    return ['Straight flush', this.highCard()]
  }
  if (this.isFourOfAKind()) {
    return ['Four of a kind', this.highCard()]
  }
  if (this.isFlush()) {
    return ['Flush', this.highCard()]
  }
  return ['Highcard', this.highCard()]
}

PokerHand.prototype.rankHand = function(hand) {
  return HandValues.findIndex(handName => handName == hand.handValue[0]) + (cardValue(hand.highCard()) / 100)
}

PokerHand.prototype.compareWith = function(hand){
  let comparison = this.rankHand(this) - this.rankHand(hand);
  if (comparison == 0) return Result.tie;
  if (comparison > 0) return Result.win;
  if (comparison < 0) return Result.loss
}

import {Test, describe, it} from '../codewarsTestFramework'

describe("sequence", function () {
  it('is sequence', () => Test.assertEquals(new PokerHand("2H 3H 4H 5H 6H").isSequence(), true))
  it('is not sequence', () => Test.assertEquals(new PokerHand("2H 3H 4H 8H 6H").isSequence(), false))
})

describe("is royal flush", function () {
  it('yes', () => Test.assertEquals(new PokerHand("KS AS TS QS JS").isRoyalFlush(), true))
  it('no', () => Test.assertEquals(new PokerHand("2H 3H 4H 8H 6H").isRoyalFlush(), false))
})

describe("is straight flush", function () {
  it('yes', () => Test.assertEquals(new PokerHand("2H 3H 4H 5H 6H").isStraightFlush(), true))
  it('no', () => Test.assertEquals(new PokerHand("2H 3H 4H 8H 6H").isStraightFlush(), false))
})

describe("is four of a kind", function () {
  it('yes', () => Test.assertEquals(new PokerHand("AS AH 2H AD AC").isFourOfAKind(), true))
  it('no', () => Test.assertEquals(new PokerHand("2H 3H 4H 8H 6H").isFourOfAKind(), false))
})

describe("has hand value", function () {
  it('["Straight flush","6"] ', () => Test.assertEquals(new PokerHand("2H 3H 4H 5H 6H").handValue[0], "Straight flush" ))
})

describe("If a poker hand is compared to another poker hand then:", function () {
    it("Highest straight flush wins", function() { assert(Result.loss, "2H 3H 4H 5H 6H", "KS AS TS QS JS");});
    it("Straight flush wins of 4 of a kind", function() { assert(Result.win, "2H 3H 4H 5H 6H", "AS AD AC AH JD");});
    it("Highest 4 of a kind wins", function() { assert(Result.win, "AS AH 2H AD AC", "JS JD JC JH 3D");});
    it("4 Of a kind wins of full house", function() { assert(Result.loss, "2S AH 2H AS AC", "JS JD JC JH AD");});
//     it("Full house wins of flush", function() { assert(Result.win,  "2S AH 2H AS AC", "2H 3H 5H 6H 7H");});
//     it("Highest flush wins", function() { assert(Result.win, "AS 3S 4S 8S 2S", "2H 3H 5H 6H 7H");});
//     it("Flush wins of straight", function() { assert(Result.win, "2H 3H 5H 6H 7H", "2S 3H 4H 5S 6C");});
//     it("Equal straight is tie", function() { assert(Result.tie, "2S 3H 4H 5S 6C", "3D 4C 5H 6H 2S");});
//     it("Straight wins of three of a kind", function() { assert(Result.win, "2S 3H 4H 5S 6C", "AH AC 5H 6H AS");});
//     it("3 Of a kind wins of two pair", function() { assert(Result.loss, "2S 2H 4H 5S 4C", "AH AC 5H 6H AS");});
//     it("2 Pair wins of pair", function() { assert(Result.win, "2S 2H 4H 5S 4C", "AH AC 5H 6H 7S");});
//     it("Highest pair wins", function() { assert(Result.loss, "6S AD 7H 4S AS", "AH AC 5H 6H 7S");});
//     it("Pair wins of nothing", function() { assert(Result.loss, "2S AH 4H 5S KC", "AH AC 5H 6H 7S");});
//     it("Highest card loses", function() { assert(Result.loss, "2S 3H 6H 7S 9C", "7H 3C TH 6H 9S");});
//     it("Highest card wins", function() { assert(Result.win, "4S 5H 6H TS AC", "3S 5H 6H TS AC");});
//     it("Equal cards is tie", function() { assert(Result.tie, "2S AH 4H 5S 6C", "AD 4C 5H 6H 2C");});
});

function assert(expected, player, opponent){
  var p = new PokerHand(player);
  var o = new PokerHand(opponent);
  Test.assertEquals(p.compareWith(o), expected);
}
