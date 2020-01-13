export function solution(str: string, ending: string): boolean {
    const escapedEnding = ending.replace(/[\W]/g, (str) => {return `\\${str}`});
    return new RegExp(`${escapedEnding}$`).test(str);
}

// import the type of assertion library you wish to use (Chai recommended)
import {assert} from "chai";

assert.equal(solution('abcde', 'cde'), true);
assert.equal(solution('abcde', 'abc'), false);
assert.equal(solution('abc', ''), true);

const args = [
    { '0': 'samurai', '1': 'ai', '2': true },
    { '0': 'sumo', '1': 'omo', '2': false },
    { '0': 'ninja', '1': 'ja', '2': true },
    { '0': 'sensei', '1': 'i', '2': true },
    { '0': 'fails', '1': 'ails', '2': true },
    { '0': 'abc', '1': '', '2': true },
    { '0': ':-)', '1': ':-(', '2': false },
    { '0': '!@#$%^&*() :-)', '1': ':-)', '2': true }
].forEach(t => {
    assert.equal(solution(t[0], t[1]), t[2]);
})