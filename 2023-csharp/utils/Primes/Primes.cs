namespace ofzza.aoc.utils.primes;

using System;

/// <summary>
/// Utility class containing typical prime manipulation functions
/// </summary>
public class Primes {

  /// <summary>
  /// Calculates factors of a number
  /// </summary>
  /// <param name="n">Number to calculate factors of</param>
  /// <returns>Array of factors</returns>
  public static long[] GetFactors (long n) {
    var factors = new List<long>();
    var a = n;
    for (var i = 2; i<Math.Sqrt(n); i++) {
      if (a % i == 0) {
        factors.Add(i);
        a = n / i;
        i--;
      }
      if (a == 1) return factors.ToArray();
    }
    if (a != 1) { factors.Add(a); }
    return factors.ToArray();
  }

  /// <summary>
  /// Calculates least common multiple of a set of numbers
  /// </summary>
  /// <param name="ns">Numbers to calculate the least common multiple of</param>
  /// <returns>Least common multiple of a set of provided numbers</returns>
  public static long GetLeastCommonMultiple (long[] ns) {
    // Get factors of each member of the set of numbers
    var factors = ns.Select(n => Primes.GetFactors(n)).ToArray();
    // Count factors
    var counts = new Dictionary<long, long>();
    var count = 0;
    foreach (var f in factors) {
      for (var i=0; i<f.Length; i++) {
        // If last of same value, log factor
        if (i == f.Length - 1 || f[i + 1] != f[i]) {
          if (!counts.ContainsKey(f[i])) {
            counts.Add(f[i], count + 1);
          }
          else if (counts[f[i]] < count + 1) {
            counts[f[i]] = count + 1;
          }
          count = 0;
        }
        // Count one of multiple of same value
        else {
          count++;
        }
      }
    }
    // Return least common multiple
    long multiple = 1;
    foreach (long n in counts.Keys) multiple *= (long)Math.Pow(n, counts[n]);
    return multiple;
  }  
}
