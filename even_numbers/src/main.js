function evenNumbers(array, number) {
  let allEvenNumbers = array.filter(n => n % 2 == 0);
  return allEvenNumbers.slice(number * -1)
}