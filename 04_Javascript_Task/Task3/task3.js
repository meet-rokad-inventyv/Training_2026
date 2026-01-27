const check = (arr) => {
  let sum1 = arr.reduce((acc, sum) => acc + sum, 0);
  console.log(sum1);
  return new Promise((resolve, reject) => {
    if (sum1 > 35) {
      resolve("Resolved");
    } else {
      reject("rejected");
    }
  });
};

function fun2(first_ele, arr1) {
  let arr2 = Symbol();
  const obj2 = {};
  obj2[arr2] = [6, 8];
  obj2[arr2] = [first_ele, ...arr1, ...obj2[arr2]];
  console.log(obj2[arr2]);

  check(obj2[arr2])
    .then((res) => {
      console.log(res);
    })
    .catch((err) => {
      console.log(err);
    });
}

function fun1() {
  let arr1 = Symbol();
  const obj1 = {};
  obj1[arr1] = [1, 2, 3, 4, 5];
  let first_ele = obj1[arr1].shift();
  fun2(first_ele, obj1[arr1]);
}

fun1();