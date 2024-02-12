// fn main() {
//     println!("Hello, world!");
// }

extern crate num;
use num::Complex;

#[allow(dead_code)]
fn complex_square_add_loop(c:Complex<f64>){
    let mut z = Complex{ re:0.0, im:0.0};
    loop{
        z = z*z +c ;
    }
}

// 這個函數採用我們想要測試Mandelbrot集合中成員身份的複數c，
// 以及在放弃並聲明c可能是成員之前嘗試的反覆運算次數的限制。
fn escape_time(c: Complex<f64> ,limit: u32)-> Optino<u32>{
    let mut z = Complex{ re: 0.0 ,im:0.0};
    for i in 0..limit{
        z = z*z +c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}


