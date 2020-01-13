

function IntStorage(byteLength: number, ints: number[]) {
    const binaryStore = ints.map(int => int.toString(2).padStart(byteLength, '0'));
    const length = binaryStore.reduce((acc, n) => {
        const lengthOfCurrentNumber = n.toString().length;
        return acc + (lengthOfCurrentNumber)
    }, 0);
    const read = (from = 0, count = 6) => {
        if (from < 0) throw new Error('invalid from parameter');
        if (count < 0 || count > 32) throw new Error('invalid count parameter');
        if (count === 0) return 0;
        if (from + count > length) throw new Error('out of bounds error');
        return Number.parseInt(binaryStore.join('').slice(from, from + count), 2);
    }
    return {
        length,
        binaryStore,
        read,
    }
}

var instance = new IntStorage(6, [43, 36, 17]);
instance.read()  /*?*/