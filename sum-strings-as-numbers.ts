const { expect } = require('chai');

const BigNumber = require('bignumber.js');

function sumStrings(a, b) {
    if (!b) return a;
    if (!a) return b;
    const bigA = new BigNumber(a);
    const bigB = new BigNumber(b);
    return bigA.plus(bigB).toString(10);
}

expect(sumStrings('123', '456')).to.equal('579');
expect(sumStrings('123', undefined)).to.equal(0);
expect(sumStrings('712569312664357328695151392', '8100824045303269669937')).to.equal('712577413488402631964821329');