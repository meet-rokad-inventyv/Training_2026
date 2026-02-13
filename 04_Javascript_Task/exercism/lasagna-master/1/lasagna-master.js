/// <reference path="./global.d.ts" />
// @ts-check

/**
 * Implement the functions needed to solve the exercise here.
 * Do not forget to export them so they are available for the
 * tests. Here an example of the syntax as reminder:
 *
 * export function yourFunction(...) {
 *   ...
 * }
 */
export function cookingStatus(time)
{
  time = time ?? undefined;
  return time === 0 ? 'Lasagna is done.' : time===undefined ? 'You forgot to set the timer.' : 'Not done, please wait.'
}

export function preparationTime(layers,averageTime = 2)
{
  return layers.length*averageTime;
}

export function quantities(layers)
{
  let obj = {
    noodles: 0,
    sauce: 0
  };
     for(let layer of layers)
     { 
      
      if(layer == 'noodles'){
        obj[layer]=obj[layer]+50;
      }
      else if(layer == 'sauce')
      {
        obj[layer]=obj[layer]+0.2;
      }
       
    }
    return obj;
}

export function addSecretIngredient(friendsList,myList)
{
  myList.push(friendsList[friendsList.length - 1]);
}

export function scaleRecipe(recipe, portions){
  let factor = portions/2;
  let scaledRecipe = {};

  for(let ingredient in recipe){
    scaledRecipe[ingredient] = recipe[ingredient] * factor;
  }

  return scaledRecipe;
}

