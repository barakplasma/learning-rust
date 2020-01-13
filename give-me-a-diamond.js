// https://www.codewars.com/kata/give-me-a-diamond/javascript

const { assert } = require('chai');

function diamond(n) {
    let diam = [1];
    if (n % 2 === 0 || n < 1) return null;
    const midpoint = Math.ceil(n / 2)
    for (let row = 1; row < n; row++) {
        let countStars = row < midpoint ? diam[row-1] + 2 : diam[row-1] - 2;
        diam.push(countStars);
    }
    const countStars = count => "*".repeat(count);
    const countSpaces= (str, i) => " ".repeat(Math.abs(n - midpoint - i)) + str;
    return diam.map(countStars).map(countSpaces).join('\n') + "\n";
}

assert.equal(diamond(1), "*\n")
assert.equal(diamond(3), " *\n***\n *\n")
assert.equal(diamond(5), "  *\n ***\n*****\n ***\n  *\n")
assert.equal(diamond(2), null)
assert.equal(diamond(-3), null)
assert.equal(diamond(0), null)