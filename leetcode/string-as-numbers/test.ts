// two strings that represent two numbers too long to be represented as primitive types
// Add them together and return the result

// No need to run the code - i.e no unit tests
// Clarity, correctness of code
// Space and run time complexity not promary focus of interview

// Alternatives - string stream, divide and conquer (max number length, split and then add)

// Method 1: Brute Force
let number1: string = "123456789123456789123456789123456789";
let number2: string = "123456789123456789123456789123456789";
const n1Length = number1.length;
const n2Length = number2.length;
// Assumption n1 is bigger than n2
let additionResult: number[] = new Array(n1Length).fill(0);

// Edge case: one string longer than the other

// parseInt

// Assumption strings are same length
for (let string1Iter = 0; string1Iter < n1Length; string1Iter++) {
  // let carryOverload: number = 0;
  const currentN1Value = parseInt(number1[string1Iter], 10);
  const currentN2Value = parseInt(number2[string1Iter], 10);
  // Only works for value < 10
  const resultantSum =
    currentN1Value + currentN2Value + additionResult[string1Iter];

  let carry = 0;

  // carryOverload = resultantSum % 10;
  if (resultantSum < 10) {
    additionResult[string1Iter] = resultantSum;
  } else if (resultantSum >= 10) {
    // For values >= 10
    // convert back into a string and then split again?
    let stringResult: string;
    stringResult = resultantSum.toString();
    additionResult[string1Iter] = parseInt(stringResult[1]);
    additionResult[string1Iter] = parseInt(stringResult[0]);
  } else {
    throw new Error("Edge case");
  }
}

// Recursive function

// Reverse array
