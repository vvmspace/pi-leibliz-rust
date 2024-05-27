fn main() {
    // $1 command param or u32::max_value()
    let max = std::env::args().nth(1).map_or(u32::max_value(), |n| n.parse().unwrap());


    let mut pi = 0.0;
    let mut sign = 1.0;

    for k in 0..=max {
        let term = sign / (2 * k + 1) as f64;
        pi += term;
        sign = -sign; // Alternate the sign
        let pi_approximation = 4.0 * pi;
        if k % 1_000_000_000 == 0 {
            println!("Iteration {}: π ≈ {:.15}, {:.7}% done",
                k,
                pi_approximation,
                k as f64 / n as f64 * 100.0
            );
        }
    }

    println!("Final approximation of π after {} iterations: {}", n, 4.0 * pi);
}
