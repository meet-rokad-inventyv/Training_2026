// @ts-check

/**
 * Calculates the sum of the two input arrays.
 *
 * @param {number[]} array1
 * @param {number[]} array2
 * @returns {number} sum of the two arrays
 */
export function twoSum(array1, array2) 
{
  let sum1 = 0;
  let sum2 = 0;
  
   for(let index = 0;index<array1.length;index++){
     sum1 = sum1*10 + array1[index]
   }

  for(let index = 0;index<array2.length;index++){
     sum2 = sum2*10 + array2[index]
   }

  return sum1+sum2;
}

/**
 * Checks whether a number is a palindrome.
 *
 * @param {number} value
 * @returns {boolean} whether the number is a palindrome or not
 */
export function luckyNumber(value) 
{
  let original = value;
  let reversed = 0 ;
  while(value>0)
  {
    let digit = value % 10;
    reversed = reversed*10 + digit;
    value=Math.floor(value / 10);
  }
  return original === reversed;
}

/**
 * Determines the error message that should be shown to the user
 * for the given input value.
 *
 * @param {string|null|undefined} input
 * @returns {string} error message
 */
export function errorMessage(input) 
{
  return (input == '' || input === undefined || input === null)  ? 'Required field' : !Number(input) ? 'Must be a number besides 0' : '';
}
