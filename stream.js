const assert = require('assert');

const greet = () => {
  let enc = 'utf8';
  let sourceBuffer = Buffer.from('hello world!');
  let destinationBuffer = Buffer.from('aaaaaaaaaaaa');
  destinationBuffer.write(sourceBuffer.toString())
  return destinationBuffer.toString();
}
assert.equal(greet(), "hello world!")
