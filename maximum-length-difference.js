const { expect } = require('chai');

/**
"You are provided two arrays that contain strings. Find the
greatest difference between the longest string in one array
and the shortest string of the other array."
*/

function mxdiflg(a1, a2) {
    const length = x => x.length;
    const lengths = x => [...x].map(length);

    if (a1.length <= 0 || a2.length <= 0) return -1;

    const a1s = lengths(a1);
    const a2s = lengths(a2);
    
    const checkPossibility = (first, second) => Math.abs(Math.min.apply(null, first) - Math.max.apply(null, second))

    const possibilityA = checkPossibility(a1s, a2s)
    const possibilityB = checkPossibility(a2s, a1s)

    return Math.max(possibilityA, possibilityB)
}


const args = [
    {
        '0': ['dd', 'qqqqppxx', 'yyyuuufffbbbb', 'ieeeejjj', 'gkkkbbww'],
        '1': ['ppssssnn'], '2': 6
    },
    {
        '0': ['ddxfffzz'],
        '1':
            ['qqqlllleddww',
                'zzzkkqqrdddl',
                'mmmmm',
                'jjjxrrrrzzzzqqqzzuuaaa',
                'cccwwwvvvv',
                'vvveccdddasss'], '2': 14
    },
    {
        '0':
            ['hoqq',
                'bbllkw',
                'oox',
                'ejjuyyy',
                'plmiis',
                'xxxzgpsssa',
                'xxwwkktt',
                'znnnnfqknaz',
                'qqquuhii',
                'dvvvwz'],
        '1': ['cccooommaaqqoxii', 'gggqaffhhh', 'tttoowwwmmww'], '2': 13
    }
    ,
    {
        '0':
            ['kkkkqqttttqyyyaaaadddwee',
                'uiii',
                'sss',
                'mmqjjjuubbb',
                'stttzzzjjj',
                'eexiiimm',
                'ddooouuuu',
                'ssssllaaaaakgghh',
                'uuumbbbcchnnbbbdddduuuucc',
                'cckkd'],
        '1':
            ['gguuuubbbbmmkkki',
                'vvwwv',
                'sssoorgggllleexxzzz',
                'jjj',
                'kkrrzzzz',
                'eqqqqoooppp',
                'jjjccrrrrgzzxxxaaa',
                'bbffftdd'], '2': 22
    }
    ,
    {
        '0': ['vvvxxxxxrrwwgeeebbbbw'],
        '1': ['ssseccciiiimmmtttx', 'mbbb', 'wwwjjkkfffnmm', 'bboooouuullm'], '2': 17
    },
]

args.forEach((t) => {
    expect(mxdiflg(t[0], t[1])).to.equal(t[2])
})