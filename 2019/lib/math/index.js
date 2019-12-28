// MATH

// Calculate prime factorization of a number
module.exports.primeFactor = function primeFactor (n) {
  // Initialize factors and a prime sieve generator
  const factors = [],
        sieve = module.exports.primeSieve(Math.round(n / 2) + 1);
  let remainder = n;
  // Test sieved primes and collect factors
  while (true) {
    // Find next factor
    while (true) {
      const prime = sieve.next();
      if (!prime.done) {
        // Test a factor
        while (remainder % prime.value === 0) {
          // Register a factor
          factors.push(prime.value);
          remainder = remainder / prime.value;
          // Test if done factoring
          if (remainder === 1) {
            return factors;
          }
        }
      } else {
        // Done testing, haven't found all factors - error
        throw new Error(`Didn't find all factors while searching entire [0, 0.5 * n] space - math is broken!`);
      }
    }
  }
};

// Generate (sieve) primes up to a limit
module.exports.primeSieve = function * primeSieve (limit) {
  // Initialize sieve
  const sieve = Array(limit);
  // Do sieve
  for (let i = 2; i < limit; i++) {
    // Check if already marked as multiple of prime
    if (sieve[i]) { continue; }
    // Check if prime
    yield i;
    // Remove all upcoming multiples
    for (let j = 2; (i * j) < limit; j++) {
      sieve[i * j] = true;
    }
  }
};

module.exports.checkOverflow = function (a, { suppressOverflowWarning = false } = {}) {
  if (!suppressOverflowWarning && (a > 9e14)) {
    console.error(`Warning:  ${ a } > 9e14 OVERFLOW!`);
  }
  return (a > 9e14);
}

// Normalize value to a module
module.exports.modNormalize = function (a, m) {
  if (!module.exports.checkOverflow(a, { suppressOverflowWarning: true })) {
    return (a >= 0 ? (a % m) : (a % m) + m);
  } else {
    throw new Error('Overflown number can\'t be normalized!');
  }
};

// Modular multiplication
module.exports.modMultiply = function (a, b, m) {
  // Try multiplying without hitting overflow
  for (let i = 0; Math.pow(2, i) < 9e14; i++) {
    try {
      let split   = Math.pow(2, i),
          result  = module.exports.modNormalize((a / split), m) * module.exports.modNormalize((b / split), m);
      result = module.exports.modNormalize(module.exports.modNormalize(split, m) * module.exports.modNormalize(result, m), m);
      result = module.exports.modNormalize(module.exports.modNormalize(split, m) * module.exports.modNormalize(result, m), m);
      return result;
    } catch (err) { }
  }
  throw new Error('Modular multiplication failing to prevent overflow!');
};

// Modular exponentiation
module.exports.modPower = function (b, e, m) {
  // Get binary representation of exponent
  const ebin = e.toString(2).split('').map(b => parseInt(b)).reverse();
  // Calculate power for each binary segment separately
  let result = 1,
      next = module.exports.modNormalize(b, m);
  for (let i = 0; i < ebin.length; i++) {
    if (next === 1) { break; }
    if (ebin[i] === 1) {
      result = module.exports.modMultiply(result, next, m);
    }
    next = module.exports.modMultiply(next, next, m);
  }
  // Return result
  return result;
};

// Modular division
module.exports.modDivide = function (a, b, m) {
  a = module.exports.modNormalize(a, m);
  let inv = modInverse(b, m); 
  if (inv === undefined) {
    throw new Error('Division not defined!');
  } else {
    return module.exports.modMultiply(inv, a, m);
  }
};

// Find modular inverse, using (extended Euclidean Algorithm)
function modInverse (b, m) { 
    gcd = gcdExtended(b, m); 
    if (gcd.value !== 1) { return undefined; }
    return module.exports.modNormalize(gcd.x, m); 
} 

// Extended Euclidean Algorithm
function gcdExtended (b, m) {
  if (b === 0)  { 
    return { value: m, x: 0, y: 1 }; 
  } 
  const gcd = gcdExtended(module.exports.modNormalize(m, b), b),
        x   = gcd.y - Math.trunc(m / b) * gcd.x,
        y   = gcd.x;
  return {
    value:  gcd.value,
    x:      x,
    y:      y
  }; 
}
