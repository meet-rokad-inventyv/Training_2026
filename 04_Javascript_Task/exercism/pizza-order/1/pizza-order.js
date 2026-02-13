/// <reference path="./global.d.ts" />
//
// @ts-check

/**
 * Determine the price of the pizza given the pizza and optional extras
 *
 * @param {Pizza} pizza name of the pizza to be made
 * @param {Extra[]} extras list of extras
 *
 * @returns {number} the price of the pizza
 */
export function pizzaPrice(pizza, ...extras) {
  let totalCost = 0;
    
    if (pizza === 'Margherita')
      totalCost += 7;
    else if (pizza === 'Caprese')
      totalCost += 9;
    else if (pizza === 'Formaggio')
      totalCost += 10;

    function rec(i) {
      if (i === extras.length) 
        return;
      if (extras[i] === 'ExtraToppings') 
        totalCost += 2;
      else
        totalCost += 1;
      rec(i + 1);
    }
    rec(0);
    return totalCost;
}

/**
 * Calculate the price of the total order, given individual orders
 *
 * (HINT: For this exercise, you can take a look at the supplied "global.d.ts" file
 * for a more info about the type definitions used)
 *
 * @param {PizzaOrder[]} pizzaOrders a list of pizza orders
 * @returns {number} the price of the total order
 */
export function orderPrice(pizzaOrders) {
  let total = 0;
  for(const order of pizzaOrders){
    total += pizzaPrice(order.pizza, ...order.extras);
  }
  return total;
}