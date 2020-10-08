var Result = { "win": "win", "loss": "loss", "tie": "tie" }

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

PokerHand.prototype.isStraight = function(){
  return this.isSequence();
};

PokerHand.prototype.maxSameValueCount = function(){
  return this.sameValueCount()[0];
};

PokerHand.prototype.sameValueCount = function(){
  let count = CardValues.map((value) => ({value, count: 0}));
  for (let val of this.value) {
    let i = count.findIndex(c => c.value == val.value);
    count[i].count++;
  }
  count.sort((a, b) => b.count-a.count)
  return count;
};

PokerHand.prototype.isFourOfAKind = function(){
  return {true: this.maxSameValueCount().count === 4, ...this.maxSameValueCount()};
};

PokerHand.prototype.isThreeOfAKind = function(){
  return {true: this.maxSameValueCount().count === 3, ...this.maxSameValueCount()};
};

PokerHand.prototype.isFullHouse = function(){
  return this.isThreeOfAKind().true && this.isPair().true;
};

PokerHand.prototype.isTwoPair = function(){
  return {true: this.sameValueCount()[0].count === 2 && this.sameValueCount()[1].count === 2, ...this.maxSameValueCount()};
};

PokerHand.prototype.isPair = function(){
  return {true: this.maxSameValueCount().count >= 2, ...this.maxSameValueCount()};
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
  if (this.isFourOfAKind().true) {
    return ['Four of a kind', this.isFourOfAKind().value]
  }
  if (this.isFullHouse()) {
    return ['Full house', this.highCard()]
  }
  if (this.isFlush()) {
    return ['Flush', this.highCard()]
  }
  if (this.isStraight()) {
    return ['Straight', this.highCard()]
  }
  if (this.isThreeOfAKind().true) {
    return ['Three of a kind', this.isThreeOfAKind().value]
  }
  if (this.isTwoPair().true) {
    return ['Two Pairs', this.isPair().value]
  }
  if (this.isPair().true) {
    return ['Pair', this.isPair().value]
  }
  return ['Highcard', this.highCard()]
}

PokerHand.prototype.rankHand = function() {
  return HandValues.findIndex(handName => handName == this.handValue[0]) + (cardValue(this.highCard()) / 100)
}

PokerHand.prototype.compareWith = function(hand){
  let comparison = this.rankHand() - hand.rankHand();
  if (comparison == 0) return Result.tie;
  if (comparison > 0) return Result.win;
  if (comparison < 0) return Result.loss
}

module.exports = {
  Result,
  PokerHand,
}