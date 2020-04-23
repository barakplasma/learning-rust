const expect = require('expect');

/**
 * Returns an alphabetically sorted list of friends by last name in (last, first) format (all uppercased)
 * @param {string} listOfFriends
 */
function meeting(listOfFriends) {
     return listOfFriends
      .split(';')
      .map(fullName => fullName.split(':').map(part => part.toLocaleUpperCase()))
      .map(nameParts => ({first: nameParts[0], last: nameParts[1]}))
      .sort((friendA, friendB) => {
        let compareLast = friendA.last.localeCompare(friendB.last);
        if (compareLast == 0) {
          let compareFirst = friendA.first.localeCompare(friendB.first);
          return compareFirst;
        }
        return compareLast
      })
      .reduce((acc, cur) => {
        return `${acc}(${cur.last}, ${cur.first})`
      },"")
}

function testing(s, exp) {
  console.log("Testing:\n", s)
  let ans = meeting(s)
  console.log("Actual:\n", ans)
  console.log("Expect:\n", exp)
  expect(ans).toBe(exp)
}

testing("Alexis:Wahl;John:Bell;Victoria:Schwarz;Abba:Dorny;Grace:Meta;Ann:Arno;Madison:STAN;Alex:Cornwell;Lewis:Kern;Megan:Stan;Alex:Korn",
  "(ARNO, ANN)(BELL, JOHN)(CORNWELL, ALEX)(DORNY, ABBA)(KERN, LEWIS)(KORN, ALEX)(META, GRACE)(SCHWARZ, VICTORIA)(STAN, MADISON)(STAN, MEGAN)(WAHL, ALEXIS)");
testing("John:Gates;Michael:Wahl;Megan:Bell;Paul:Dorries;James:Dorny;Lewis:Steve;Alex:Meta;Elizabeth:Russel;Anna:Korn;Ann:Kern;Amber:Cornwell",
  "(BELL, MEGAN)(CORNWELL, AMBER)(DORNY, JAMES)(DORRIES, PAUL)(GATES, JOHN)(KERN, ANN)(KORN, ANNA)(META, ALEX)(RUSSEL, ELIZABETH)(STEVE, LEWIS)(WAHL, MICHAEL)");
testing("Alex:Arno;Alissa:Cornwell;Sarah:Bell;Andrew:Dorries;Ann:Kern;Haley:Arno;Paul:Dorny;Madison:Kern",
  "(ARNO, ALEX)(ARNO, HALEY)(BELL, SARAH)(CORNWELL, ALISSA)(DORNY, PAUL)(DORRIES, ANDREW)(KERN, ANN)(KERN, MADISON)");