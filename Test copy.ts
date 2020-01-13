import {expect} from 'chai';

export const Test = {
    assertSimilar: (actual: any, expected: any, msg = undefined) => expect(actual, msg).to.deep.equal(expected),
    assertEquals: (actual: any, expected: any, msg = undefined) => expect(actual, msg).to.deep.equal(expected)
}

Test.assertEquals(1,3);