use num::complex::Complex;

fn main() {
    let c = Complex { re: 0.2, im: 0.1 };

    let time = escape_time(c, 10);
    let value = match time {
        None => None,
        Some(i) => Some(i),
    };
    println!("Escape time is: {}", value.unwrap());
}

// fn complex_square_add_loop(c: Complex<f64>) {
//     let mut z = Complex { re: 0.24, im: 0.3 };
//     let limit = 5;
//     for i in 0..limit {
//         z = z * z + c;
//         println!("z - {}", z);
//     }
// }

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.3, im: 1.1 };
    for _i in 0..limit{
        if z.norm_sqr() > 4.0 {
            return Some(_i);
        }
        z = z * z + c;
    }

    None
}
