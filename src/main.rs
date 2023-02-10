pub fn inverse_mod(a: u128, n: u128) -> Result<u128, &'static str> {
    /*
    Using extended euclidean algorithm for computing the inverse.
    */
    let (mut t, mut t_prime): (i128, i128) = (0, 1);
    let (mut r, mut r_prime): (i128, i128) = (n.try_into().unwrap(), a.try_into().unwrap());
    let mut q: i128;
    while r_prime != 0 {
        q = r / r_prime;
        (t, t_prime) = (t_prime, t - q * t_prime);
        (r, r_prime) = (r_prime, r - q * r_prime);
    }
    if r > 1 {
        return Err("No inverse");
    }
    match t {
        t if t < 0 => {
            let t: u128 = t.try_into().unwrap();
            Ok(t + n)
        }
        _ => Ok(t.try_into().unwrap()),
    }
}

pub fn pow_mod(m: u128, pow: u128, n: u128) -> u128 {
    /*
    Raise to pow mod n; using repeated squaring algorithm.
    */
    let mut result = 1;
    let mut tmp = m % n;
    let mut pow = pow;
    while pow != 0 {
        if pow & 1 == 1 {
            result = (result * tmp) % n;
        }
        tmp = (tmp * tmp) % n;
        pow >>= 1;
    }
    result
}

fn main() {

    let (p, q, d) = (47, 59, 157);
    let n = p * q;
    let phi_pq = (p - 1) * (q - 1);

    // 'ITS' encoded as in RSA (1977) paper, p.10
    let m = 920;

    let e = inverse_mod(d, phi_pq).unwrap();
    let encrypted = pow_mod(m, e, n);
    let decrypted = pow_mod(encrypted, d, n);

    // D(E(M)) == M
    assert_eq!(encrypted, 948);
    assert_eq!(decrypted, m);
}
