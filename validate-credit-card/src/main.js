function validate(n){
  let number = String(n);

  let sum = parseInt(number.slice(-1));

  for (let i in number.slice(0,-1)) {
      let value = parseInt(number.charAt(i));

      if (i % 2 === 0) {
          value *= 2;
      }

      if (value > 9) {
          value -= 9;
      }

      sum += value;
  }

  return sum % 10 === 0;
}

Test.assertEquals(validate(123), false);
Test.assertEquals(validate(1), false);
Test.assertEquals(validate(2121), true);
Test.assertEquals(validate(1230), true);
Test.assertEquals(validate(8675309), false);
Test.assertEquals(validate(4111111111111111), true);
Test.assertEquals(validate(26), true);
Test.assertEquals(validate(2626262626262626), true);
Test.assertEquals(validate(91), true);
Test.assertEquals(validate(92), false);
Test.assertEquals(validate(912030), true);
Test.assertEquals(validate(922030), false);