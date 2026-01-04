package numbers

// Converts a whole number to ration number
func ToRationalNumber[T WholeNumber](n T) RationalNumber {
	return RationalNumber{ Numerator: int64(n), Denominator: 1 }
}

// Rational number type
type RationalNumber struct {
	Numerator int64
	Denominator int64
}

// Reduces the internal representation of the rational number
func (r RationalNumber) Reduce() RationalNumber {
	// If numerator is zero
	if r.Numerator == 0 {
		r.Denominator = 1
	}
	// If both numerator and denominator are equal
	if r.Numerator == r.Denominator {
		r.Numerator = 1
		r.Denominator = 1
	}
	// If both numerator and denominator negative
	if r.Numerator < 0 && r.Denominator < 0 {
		r.Numerator = -1 * r.Numerator
		r.Denominator = -1 * r.Denominator
	}
	// Try to reduce
	if r.Numerator != 1 && r.Numerator != 0 && r.Numerator != -1 && r.Denominator % r.Numerator == 0 {
		r.Denominator = int64(float64(r.Denominator) / float64(r.Numerator))
		r.Numerator = 1
	}
	// TODO: Reduce by common devisors ...

	// Return self
	return r
}

// Calculates an inverted rational number
func (r RationalNumber) Invert() RationalNumber {
	return RationalNumber{ Numerator: r.Denominator, Denominator: r.Numerator }
}

// Calculates a rational number by adding the provided rational number argument
func (r RationalNumber) Add(a RationalNumber) RationalNumber {
	return RationalNumber{ Numerator: r.Numerator * a.Denominator + a.Numerator * r.Denominator, Denominator: r.Denominator * a.Denominator }.Reduce()
}

// Calculates a rational number by subtracting the provided rational number argument
func (r RationalNumber) Subtract(a RationalNumber) RationalNumber {
	return RationalNumber{ Numerator: r.Numerator * a.Denominator - a.Numerator * r.Denominator, Denominator: r.Denominator * a.Denominator }.Reduce()
}

// Calculates a rational number by multiplying the provided rational number argument
func (r RationalNumber) Multiply(a RationalNumber) RationalNumber {
	return RationalNumber{ Numerator: r.Numerator * a.Numerator, Denominator: r.Denominator * a.Denominator }.Reduce()
}

// Calculates a rational number by dividing the provided rational number argument
func (r RationalNumber) Divide(a RationalNumber) RationalNumber {
	return RationalNumber{ Numerator: r.Numerator * a.Denominator, Denominator: r.Denominator * a.Numerator }.Reduce()
}

// Converts a rational number to a float
func (r RationalNumber) ToFloat() float64 {
	return float64(r.Numerator) / float64(r.Denominator)
}
