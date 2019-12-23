// MATH

// Calculate prime factorization of a number
module.exports.primefact = function primefact (n) {
  // Initialize factors and a prime sieve generator
  const factors = [],
        sieve = module.exports.primesieve(Math.round(n / 2) + 1);
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
module.exports.primesieve = function * primesieve (limit) {
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
