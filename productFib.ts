const { expect } = require('chai');
const _ = require('lodash');

/**
 * https://www.codewars.com/kata/5541f58a944b85ce6d00006a/solutions/javascript
 * The Fibonacci numbers are the numbers in the following integer sequence (Fn):

0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, ...

such as

F(n) = F(n-1) + F(n-2) with F(0) = 0 and F(1) = 1.

Given a number, say prod (for product), we search two Fibonacci numbers F(n) and F(n+1) verifying

F(n) * F(n+1) = prod.

Your function productFib takes an integer (prod) and returns an array:

[F(n), F(n+1), true] or {F(n), F(n+1), 1} or (F(n), F(n+1), True)
depending on the language if F(n) * F(n+1) = prod.

If you don't find two consecutive F(m) verifying F(m) * F(m+1) = prodyou will return

[F(m), F(m+1), false] or {F(n), F(n+1), 0} or (F(n), F(n+1), False)
F(m) being the smallest one such as F(m) * F(m+1) > prod.

Examples
productFib(714) # should return [21, 34, true], 
                # since F(8) = 21, F(9) = 34 and 714 = 21 * 34

productFib(800) # should return [34, 55, false], 
                # since F(8) = 21, F(9) = 34, F(10) = 55 and 21 * 34 < 800 < 34 * 55
Notes: Not useful here but we can tell how to choose the number n up to which to go: we can use the "golden ratio" phi which is (1 + sqrt(5))/2 knowing that F(n) is asymptotic to: phi^n / sqrt(5). That gives a possible upper bound to n.

You can see examples in "Example test".

References
http://en.wikipedia.org/wiki/Fibonacci_number

http://oeis.org/A000045
 */
function fiboLoop(num){
    var a = 1, b = 0, temp;
  
    while (num >= 0){
      temp = a;
      a = a + b;
      b = temp;
      num--;
    }
  
    return b;
  }
const fib = _.memoize((n) => n <= 1 ? 1 : fib(n - 1) + fib(n - 2));
const pFib = _.memoize(n => fib(n) * fib(n + 1))

function productFib(prod) {
    const answer = [];
    let n = 0;

    while (pFib(n) < prod) {
        if (pFib(n) === prod) break;
        n++;
    }

    answer[0] = fib(n);
    answer[1] = fib(n + 1);
    answer[2] = answer[0] * answer[1] === prod;
    return answer;
}

const assertSimilar = (actual, expected) => expect(actual).to.deep.equal(expected);
assertSimilar(productFib(193864606), [10946, 17711, true])/*?.*/
assertSimilar(productFib(4895), [55, 89, true]) /*?.*/
assertSimilar(productFib(5895), [89, 144, false])/*?.*/
assertSimilar(productFib(74049690), [6765, 10946, true])/*?.*/
assertSimilar(productFib(84049690), [10946, 17711, false])/*?.*/
assertSimilar(productFib(447577), [610, 987, false])/*?.*/
assertSimilar(productFib(602070), [610, 987, true])/*?.*/

const fib2 = _.memoize(fiboLoop);
const pFib2 = _.memoize(n => fib2(n) * fib2(n + 1))

function productFib2(prod) {
    const answer = [];
    let n = 0;

    while (pFib2(n) < prod) {
        if (pFib2(n) === prod) break;
        n++;
    }

    answer[0] = fib2(n);
    answer[1] = fib2(n + 1);
    answer[2] = answer[0] * answer[1] === prod;
    return answer;
} // f

assertSimilar(productFib2(193864606), [10946, 17711, true])/*?.*/
assertSimilar(productFib2(4895), [55, 89, true]) /*?.*/
assertSimilar(productFib2(5895), [89, 144, false])/*?.*/
assertSimilar(productFib2(74049690), [6765, 10946, true])/*?.*/
assertSimilar(productFib2(84049690), [10946, 17711, false])/*?.*/
assertSimilar(productFib2(447577), [610, 987, false])/*?.*/
assertSimilar(productFib2(602070), [610, 987, true])/*?.*/