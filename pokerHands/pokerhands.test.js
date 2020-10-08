const {Result, PokerHand} = require('./pokerhands');

describe("sameValueCount", function () {
  it('most common is a 2', () => expect(new PokerHand("2H 2H 2H 2H 6H").sameValueCount().value).toBe( "2"))
  it('most same value is 4', () => expect(new PokerHand("2H 2H 2H 2H 6H").sameValueCount().count).toBe( 4))
  it('1', () => expect(new PokerHand("2H 3H 4H 8H 6H").sameValueCount().count).toBe( 1))
})

describe("sequence", function () {
  it('is sequence', () => expect(new PokerHand("2H 3H 4H 5H 6H").isSequence()).toBe( true))
  it('is not sequence', () => expect(new PokerHand("2H 3H 4H 8H 6H").isSequence()).toBe( false))
})

describe("is royal flush", function () {
  it('yes', () => expect(new PokerHand("KS AS TS QS JS").isRoyalFlush()).toBe( true))
  it('no', () => expect(new PokerHand("2H 3H 4H 8H 6H").isRoyalFlush()).toBe( false))
})

describe("is straight flush", function () {
  it('yes', () => expect(new PokerHand("2H 3H 4H 5H 6H").isStraightFlush()).toBe( true))
  it('no', () => expect(new PokerHand("2H 3H 4H 8H 6H").isStraightFlush()).toBe( false))
})

describe("is four of a kind", function () {
  it('yes', () => expect(new PokerHand("AS AH 2H AD AC").isFourOfAKind().true).toBe( true))
  // it('no', () => expect(new PokerHand("2H 3H 4H 8H 6H").isFourOfAKind().true).toBe( false))
})

describe("has hand value", function () {
  it('["Straight flush","6"] ', () => expect(new PokerHand("2H 3H 4H 5H 6H").handValue[0]).toBe( "Straight flush" ))
})

describe("If a poker hand is compared to another poker hand then:", function () {
    it("Highest straight flush wins", function() { assert(Result.loss, "2H 3H 4H 5H 6H", "KS AS TS QS JS");});
    it("Straight flush wins of 4 of a kind", function() { assert(Result.win, "2H 3H 4H 5H 6H", "AS AD AC AH JD");});
    it("Highest 4 of a kind wins", function() { assert(Result.win, "AS AH 2H AD AC", "JS JD JC JH 3D");});
    it("4 Of a kind wins of full house", function() { assert(Result.loss, "2S AH 2H AS AC", "JS JD JC JH AD");});
    it("Full house wins of flush", function() { assert(Result.win,  "2S AH 2H AS AC", "2H 3H 5H 6H 7H");});
    it("Highest flush wins", function() { assert(Result.win, "AS 3S 4S 8S 2S", "2H 3H 5H 6H 7H");});
    it("Flush wins of straight", function() { assert(Result.win, "2H 3H 5H 6H 7H", "2S 3H 4H 5S 6C");});
    it("Equal straight is tie", function() { assert(Result.tie, "2S 3H 4H 5S 6C", "3D 4C 5H 6H 2S");});
    it("Straight wins of three of a kind", function() { assert(Result.win, "2S 3H 4H 5S 6C", "AH AC 5H 6H AS");});
    it("3 Of a kind wins of two pair", function() { assert(Result.loss, "2S 2H 4H 5S 4C", "AH AC 5H 6H AS");});
    it("2 Pair wins of pair", function() { assert(Result.win, "2S 2H 4H 5S 4C", "AH AC 5H 6H 7S");});
    it("Highest pair wins", function() { assert(Result.loss, "6S AD 7H 4S AS", "AH AC 5H 6H 7S");});
    it("Pair wins of nothing", function() { assert(Result.loss, "2S AH 4H 5S KC", "AH AC 5H 6H 7S");});
    it("Highest card loses", function() { assert(Result.loss, "2S 3H 6H 7S 9C", "7H 3C TH 6H 9S");});
    it("Highest card wins", function() { assert(Result.win, "4S 5H 6H TS AC", "3S 5H 6H TS AC");});
    it("Equal cards is tie", function() { assert(Result.tie, "2S AH 4H 5S 6C", "AD 4C 5H 6H 2C");});
});

function assert(expected, player, opponent){
  var p = new PokerHand(player);
  var o = new PokerHand(opponent);
  expect(p.compareWith(o)).toBe( expected);
}