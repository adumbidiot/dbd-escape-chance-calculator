function factorial(n) {
    if(n == 0 || n == 1) return 1;
    return n * factorial(n - 1);
}

function binomialCoefficient(n, k) {
    return factorial(n) / (factorial(k) * factorial(n - k));
}

/// Binomial theorem impl
function calcHookEscapeChanceBinomialTheorem(escapeChance, numTries) {
    let x = 0;
    
    let ret = 0.0;
    for(let i = 0; i <= x; i++)
        ret += binomialCoefficient(numTries, i) * Math.pow(escapeChance, i) * Math.pow(1.0 - escapeChance, numTries - i);
    return 1.0 - ret;
}

function calcHookEscapeChance(escapeChance, numTries) {
    let ret = 0.0;
    for(let i = 0; i < numTries; i++) {
        ret += Math.pow((1.0 - escapeChance), i) * escapeChance;
    }
    
    return ret;
}

let numUpTheAnteI = 0
let numUpTheAnteII = 0;
let numUpTheAnteIII = 0;
let numSaltyLips = 4;
let hasSlipperyMeat = false;

let numTries = 3;
let chance = 0.04;
chance += (0.03 * numSaltyLips);
chance += (0.03 * numUpTheAnteI);
chance += (0.06 * numUpTheAnteII);
chance += (0.09 * numUpTheAnteIII);

if(hasSlipperyMeat) {
    numTries += 3;
    chance += 0.04;
}

console.log(calcHookEscapeChance(chance, numTries)); 
console.log(calcHookEscapeChanceBinomialTheorem(chance, numTries)); 