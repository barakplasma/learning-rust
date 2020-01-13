const { expect } = require('chai');

const RightToLeftDigitIterator = n => ({
    [Symbol.iterator]() {
        const nAsString = n.toString();
        let step = nAsString.length;
        return {
            next() {
                step--;
                if (step >= 0) return { value: Number(nAsString[step]), done: false }
                return { value: undefined, done: true }
            }
        }
    }
})

function digitize(n) {
    const iterable = RightToLeftDigitIterator(n);
    return [...iterable]
}

expect(digitize(35231)).to.deep.equal([1, 3, 2, 5, 3]);
