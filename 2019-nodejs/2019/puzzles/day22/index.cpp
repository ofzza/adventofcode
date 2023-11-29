#include <algorithm>
#include <cstdio>
#include <cstdint>
#include <vector>
#include <cassert>

typedef __int128 int128_t;

constexpr int128_t n = 119315717514047ll;
constexpr int128_t reps = 101741582076661ll;
constexpr int128_t x = 2020;

static inline int128_t mod(int128_t a, int128_t b) {
	return (a >= 0) ? (a % b) : (b + a % b);
}
static int128_t gcd_extended(int128_t a, int128_t b, int128_t& x, int128_t& y) {
	if (a == 0) {
		x = 0;
		y = 1;
		return b;
	}

	int128_t x1, y1;
	int128_t gcd = gcd_extended(b % a, a, x1, y1);
	x = y1 - (b / a) * x1;
	y = x1;
	return gcd;
}
static int128_t modular_inverse(int128_t b, int128_t n) {
	int128_t x, y;
	int128_t g = gcd_extended(b, n, x, y);
	return (g != 1) ? -1 : mod(x, n);
}
static int128_t modular_divide(int128_t a, int128_t b, int128_t n) {
	a = mod(a, n);
	int128_t inv = modular_inverse(b, n);
	return (inv == -1) ? -1 : (a * inv) % n;
}
static int128_t modular_power(int128_t base, int128_t exponent, int128_t n) {
	assert(exponent >= 0);
	if (exponent == 0) {
		return (base == 0) ? 0 : 1;
	}

	int128_t bit = 1;
	int128_t power = mod(base, n);
	int128_t out = 1;
	while (bit <= exponent) {
		if (exponent & bit) {
			out = mod(out * power, n);
		}
		power = mod(power * power, n);
		bit <<= 1;
	}

	return out;
}

int main(int argc, char** argv)
{

	// Open file
	FILE* f = fopen("./input.txt", "r");
	if (!f) { return 1; }

	// Read from file and compose shuffle transformation
	int128_t a = 1;
	int128_t b = 0;
	char line[256];
	while (fgets(line, sizeof(line), f) != nullptr) {
		int64_t arg;
		if (sscanf(line, "cut %lld", &arg) == 1) {
			if (arg < 0) { arg += n; }
			a = mod(a, n);
			b = mod(b - arg, n);
		}
		else if (sscanf(line, "deal with increment %lld", &arg) == 1) {
			a = mod(arg * a, n);
			b = mod(arg * b, n);
		} else /* if (line === "deal into new stack") */ {
			a = mod(-1 * a, n);
			b = mod(-1 * b + -1, n);
		}
	}
	fclose(f);

	// Compose repeated shuffles transformation
	int128_t A = modular_power(a, reps, n);
	int128_t B = mod(b * modular_divide(modular_power(a, reps, n) - 1, a - 1, n), n);

	// Calculate position before shuffle: x = (x' - B) / A | n
	int128_t result = mod(modular_divide(mod(x - B, n), A, n), n);
	printf("Result: %lld\n", int64_t(result));
	printf("- line: A=%lld, B=%lld\n", int64_t(a), int64_t(b));
	printf("- reps: A=%lld, B=%lld\n", int64_t(A), int64_t(B));
}
