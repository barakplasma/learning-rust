const expect = require('chai').expect;
// https://www.codewars.com/kata/52761ee4cffbc69732000738/solutions/javascript 
class ForcesParser {
  /**
   * 
   * @param forcesFormat {String}
   * @param worth {{[forceType: string]: number}}
   * @param roster {String}
   */
  constructor(forcesFormat, worth, roster) {
    this.forcesFormat = forcesFormat;
    this.worth = worth;
    this.roster = roster;
    this.findForcesIndexes();
    this.countForcesFromRoster();
    this.findWorth()
  }
  findForcesIndexes() {
    this.forcesIndex = this.forcesFormat.split(', ');
  }

  forcesToIndexes() {
    this.forces = this.roster.split(' ')
  }

  countForcesFromRoster() {
    this.forcesToIndexes();
    this.labelForces();
  }

  findWorth() {
    this.labeledForcesWorth = {};
    for (let forceType in this.labeledForces) {
      this.labeledForcesWorth[forceType] = this.labeledForces[forceType] * this.worth[forceType]
    }
  }

  sumOfWorth() {
    let sum = 0;
    for (let forceType in this.labeledForcesWorth) {
      sum += this.labeledForcesWorth[forceType];
    }
    return sum;
  }

  labelForces() {
    this.labeledForces = this.forces.reduce((labeledForces, currentForce, currentForceIndex)=> {
      labeledForces[this.forcesIndex[currentForceIndex]] = currentForce;
      return labeledForces;
    }, {})
  }
}

/**
 * 
 * @param good {String}
 * @param evil {String}
 */
function goodVsEvil(good, evil){
  const whoWon = {
    good: "Battle Result: Good triumphs over Evil",
    evil: "Battle Result: Evil eradicates all trace of Good",
    tie: "Battle Result: No victor on this battle field"
  }
  const goodWorth = {
    Hobbits: 1,
    Men: 2,
    Elves: 3,
    Dwarves: 3,
    Eagles: 4,
    Wizards: 10
  }
  const goodFormat = "Hobbits, Men, Elves, Dwarves, Eagles, Wizards";
  const evilWorth = {
    Orcs: 1,
    Men: 2,
    Wargs: 2,
    Goblins: 2,
    Uruk_Hai: 3,
    Trolls: 5,
    Wizards: 10
  }
  const evilFormat = "Orcs, Men, Wargs, Goblins, Uruk_Hai, Trolls, Wizards"
  const goodForces = new ForcesParser(goodFormat, goodWorth, good);
  const goodForcesSum = goodForces.sumOfWorth();
  const evilForces = new ForcesParser(evilFormat, evilWorth, evil);
  const evilForcesSum = evilForces.sumOfWorth();
  return goodForcesSum > evilForcesSum ? whoWon.good : goodForcesSum === evilForcesSum ? whoWon.tie : whoWon.evil;
}


expect( goodVsEvil('1 1 1 1 1 1', '1 1 1 1 1 1 1')).to.equal( 'Battle Result: Evil eradicates all trace of Good', 'Evil should win' );
expect( goodVsEvil('0 0 0 0 0 10', '0 1 1 1 1 0 0')).to.equal( 'Battle Result: Good triumphs over Evil', 'Good should win' );
expect( goodVsEvil('1 0 0 0 0 0', '1 0 0 0 0 0 0')).to.equal( 'Battle Result: No victor on this battle field', 'Should be a tie' );