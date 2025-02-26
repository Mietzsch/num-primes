use num_bigint::BigUint;
use num_primes::Generator;

#[test]
fn calling_gets_biguint_with_random() {
    let p = Generator::new_prime(512);

    gets_biguint(&p); // generates the error
}

fn gets_biguint(_x: &BigUint) {}
