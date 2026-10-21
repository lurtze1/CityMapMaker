//! Determinism spike: hashes the bit patterns of transcendental results so two platforms
//! can be compared with one number.

/// FNV-1a 64-bit hasher over bytes.
#[derive(Debug, Clone, Copy)]
pub struct Fnv1a(u64);

impl Default for Fnv1a {
    fn default() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
}

impl Fnv1a {
    /// Folds every byte of `bits` (little-endian) into the hash.
    pub fn write_u64(&mut self, bits: u64) {
        for byte in bits.to_le_bytes() {
            self.0 ^= u64::from(byte);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    /// Folds the bit pattern of `value` into the hash.
    pub fn write_f64(&mut self, value: f64) {
        self.write_u64(value.to_bits());
    }

    /// The current hash value.
    pub fn finish(self) -> u64 {
        self.0
    }
}

/// Knuth MMIX linear congruential generator.
#[derive(Debug, Clone, Copy)]
pub struct Lcg(u64);

impl Lcg {
    /// Starts the generator at `seed`.
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// Next raw 64-bit state.
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0
    }

    /// Uniform value in [0, 1) built from the top 53 bits, using only exact operations.
    pub fn next_unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// Number of inputs per region.
pub const PER_REGION: usize = 20_000;

/// The pi/2 constant, written as bits so no platform rounding is involved.
pub const HALF_PI: f64 = std::f64::consts::FRAC_PI_2;

/// Builds the fixed input list: seven regions of `PER_REGION` values each (140,000 total),
/// plus a set of hand-picked special values. Every value is produced with exact integer
/// arithmetic, `from_bits`, or + - * / only.
pub fn inputs() -> Vec<f64> {
    let mut lcg = Lcg::new(0x5eed_c17e_0000_0001);
    let mut out = Vec::with_capacity(7 * PER_REGION + 64);
    for _ in 0..PER_REGION {
        out.push(lcg.next_unit() * 20.0 - 10.0);
    }
    for _ in 0..PER_REGION {
        out.push((lcg.next_unit() * 2.0 - 1.0) * 1e-6);
    }
    for _ in 0..PER_REGION {
        let k = (lcg.next_u64() % 2001) as f64 - 1000.0;
        let offset = (lcg.next_unit() * 2.0 - 1.0) * 1e-9;
        out.push(k * HALF_PI + offset);
    }
    for _ in 0..PER_REGION {
        out.push((lcg.next_unit() * 2.0 - 1.0) * 1e6);
    }
    for _ in 0..PER_REGION {
        let mantissa = lcg.next_u64() & 0x000f_ffff_ffff_ffff;
        let sign = lcg.next_u64() & 0x8000_0000_0000_0000;
        out.push(f64::from_bits(sign | mantissa.max(1)));
    }
    for _ in 0..PER_REGION {
        out.push(lcg.next_unit() * 700.0 - 700.0 + lcg.next_unit() * 1400.0);
    }
    for _ in 0..PER_REGION {
        let exponent = 1 + lcg.next_u64() % 2046;
        let mantissa = lcg.next_u64() & 0x000f_ffff_ffff_ffff;
        out.push(f64::from_bits((exponent << 52) | mantissa));
    }
    out.extend_from_slice(&[
        0.0,
        -0.0,
        1.0,
        -1.0,
        HALF_PI,
        std::f64::consts::PI,
        std::f64::consts::TAU,
        1e-300,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        f64::MAX,
        f64::INFINITY,
        f64::NEG_INFINITY,
        709.782_712_893_384,
        -745.133_219_101_941_1,
        1e6,
        -1e6,
        0.5,
        2.0,
        10.0,
    ]);
    out
}

/// A unary transcendental under test.
pub type Unary = fn(f64) -> f64;
/// A binary transcendental under test.
pub type Binary = fn(f64, f64) -> f64;

/// Hashes `unary` functions over every input and `binary` functions over consecutive input
/// pairs (x, y) where y is the next input in the list. NaN results are canonicalised so only
/// NaN-ness counts, not payload.
pub fn hash_all(unary: &[Unary], binary: &[Binary]) -> u64 {
    let xs = inputs();
    let mut hasher = Fnv1a::default();
    for f in unary {
        for &x in &xs {
            hasher.write_f64(canonical(f(x)));
        }
    }
    for f in binary {
        for pair in xs.windows(2) {
            hasher.write_f64(canonical(f(pair[0], pair[1])));
        }
    }
    hasher.finish()
}

/// Per-function hashes, for locating a difference.
pub fn hash_each(unary: &[(&str, Unary)], binary: &[(&str, Binary)]) -> Vec<(String, u64)> {
    let mut out = Vec::new();
    for (name, f) in unary {
        out.push((name.to_string(), hash_all(&[*f], &[])));
    }
    for (name, f) in binary {
        out.push((name.to_string(), hash_all(&[], &[*f])));
    }
    out
}

fn canonical(value: f64) -> f64 {
    if value.is_nan() { f64::NAN } else { value }
}
