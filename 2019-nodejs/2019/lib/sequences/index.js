// SEQUENCE GENERATORS

// Generates numbers in range with ascending digits
module.exports.ascendingDigits = function * ascendingDigits ({ min, max, validationFn } = {}) {
  let n = min;
  while (n <= max) {
    // Split number into digits
    const digits = [];
    while (n > 0) { digits.push(n % 10); n = Math.trunc(n / 10); }
    // Get next number with only ascending digits
    for (let i = (digits.length - 1); i >= 0; i--) {
      if ((i < (digits.length - 1)) && (digits[i + 1] > digits[i])) {
        for (let j = i; j >= 0; j--) { digits[j] = digits[i + 1]; }
      }
      n += digits[i] * Math.pow(10, i);
    }
    // Validate and yield number
    if ((n <= max) && (!validationFn || validationFn(n, digits))) {
      yield n;
    }
    // Next candidate number
    n++;
  }
}

// Generates all permutations of an array
module.exports.permutations = function * permutations (values = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]) {
  // Generate trivial case
  if (values.length < 1) { return yield []; }
  // Generate step-down settings, and add next step
  for (let settingTemplate of permutations(values.slice(1))) {
    for (let i=0; i<=settingTemplate.length; i++) {
      const setting = [...settingTemplate];
      setting.splice(i, 0, values[0]);
      yield setting;
    }
  }
}
