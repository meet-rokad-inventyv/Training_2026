function check(array2) {
  let sum = array2.reduce((acc, x) => acc + x, 0);
  console.log(sum);
  return new Promise((resolve, reject) => {
    if (sum > 35) {
      resolve("resolved successfully");
    } else {
      reject("rejected");
    }
  });
}

function fun1() {
  let array1 = [1, 2, 3, 4, 5];
  first_element = array1.shift(); 
  fun2(first_element, array1);
}

function fun2(first_element, array1) {
  let array2 = [6, 7, 8, 9];
  array2 = [first_element, ...array1, ...array2];
  console.log(array2);
  check(array2)
    .then((res) => console.log(res))
    .catch((err) => {
      console.log(err);
    });
}
fun1();



