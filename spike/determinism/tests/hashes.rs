//! Platform comparison of libm and std transcendentals.

#![expect(
    clippy::disallowed_methods,
    reason = "this spike measures the banned std methods on purpose"
)]

use determinism_spike::{Binary, Unary, hash_all, hash_each, inputs};

const LIBM_EXPECTED: u64 = 0xbe13_1fd1_68e2_f9fd;

fn libm_unary() -> Vec<(&'static str, Unary)> {
    vec![
        ("sin", libm::sin),
        ("cos", libm::cos),
        ("tan", libm::tan),
        ("exp", libm::exp),
        ("log", libm::log),
    ]
}

fn libm_binary() -> Vec<(&'static str, Binary)> {
    vec![
        ("atan2", libm::atan2),
        ("pow", libm::pow),
        ("hypot", libm::hypot),
    ]
}

fn std_unary() -> Vec<(&'static str, Unary)> {
    vec![
        ("sin", f64::sin),
        ("cos", f64::cos),
        ("tan", f64::tan),
        ("exp", f64::exp),
        ("log", f64::ln),
    ]
}

fn std_binary() -> Vec<(&'static str, Binary)> {
    vec![
        ("atan2", f64::atan2),
        ("pow", f64::powf),
        ("hypot", f64::hypot),
    ]
}

fn combined(unary: &[(&str, Unary)], binary: &[(&str, Binary)]) -> u64 {
    let u: Vec<Unary> = unary.iter().map(|(_, f)| *f).collect();
    let b: Vec<Binary> = binary.iter().map(|(_, f)| *f).collect();
    hash_all(&u, &b)
}

#[test]
fn input_count_is_at_least_100k() {
    let n = inputs().len();
    println!("INPUTS {n} os={}", std::env::consts::OS);
    assert!(n >= 100_000);
}

#[test]
fn libm_hash_is_constant() {
    let hash = combined(&libm_unary(), &libm_binary());
    println!("LIBM_HASH {hash:#018x} os={}", std::env::consts::OS);
    for (name, h) in hash_each(&libm_unary(), &libm_binary()) {
        println!("LIBM_FN {name} {h:#018x}");
    }
    assert_eq!(hash, LIBM_EXPECTED, "libm hash {hash:#018x}");
}

#[test]
fn std_hash_is_printed() {
    let hash = combined(&std_unary(), &std_binary());
    println!("STD_HASH {hash:#018x} os={}", std::env::consts::OS);
    for (name, h) in hash_each(&std_unary(), &std_binary()) {
        println!("STD_FN {name} {h:#018x}");
    }
}

#[test]
fn std_versus_libm_mismatch_counts() {
    let xs = inputs();
    let same = |a: f64, b: f64| a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan());
    for ((name, s), (_, l)) in std_unary().iter().zip(libm_unary().iter()) {
        let n = xs.iter().filter(|&&x| !same(s(x), l(x))).count();
        println!(
            "MISMATCH {name} {n}/{} os={}",
            xs.len(),
            std::env::consts::OS
        );
    }
    for ((name, s), (_, l)) in std_binary().iter().zip(libm_binary().iter()) {
        let n = xs
            .windows(2)
            .filter(|p| !same(s(p[0], p[1]), l(p[0], p[1])))
            .count();
        println!(
            "MISMATCH {name} {n}/{} os={}",
            xs.len() - 1,
            std::env::consts::OS
        );
    }
}
