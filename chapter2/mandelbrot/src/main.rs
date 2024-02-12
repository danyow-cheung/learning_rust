// fn main() {
//     println!("Hello, world!");
// }

extern crate num;
use num::Complex;

use std::str::FromStr;

#[allow(dead_code)]
fn complex_square_add_loop(c:Complex<f64>){
    let mut z = Complex{ re:0.0, im:0.0};
    loop{
        z = z*z +c ;
    }
}

// 這個函數採用我們想要測試Mandelbrot集合中成員身份的複數c，
// 以及在放弃並聲明c可能是成員之前嘗試的反覆運算次數的限制。
fn escape_time(c: Complex<f64> ,limit: u32)-> Option<u32>{
    let mut z = Complex{ re: 0.0 ,im:0.0};
    for i in 0..limit{
        z = z*z +c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}


///將字串's'解析為座標對，如“400x600”或“1.0,0.5”。
fn parse_pair<T:FromStr>(s:&str,separator:char)-> Option<(T,T)>{
    match s.find(separator){
        None => None,
        Some(index)=>{
            match (T::from_str(&s[..index]),T::from_str(&s[index+1..])){
                (Ok(l),Ok(r)) => Some((l,r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair(){
    assert_eq!(parse_pair::<i32>("",        ","),None);
    assert_eq!(parse_pair::<i32>("10",        ","),None);
    assert_eq!(parse_pair::<i32>(",10",        ","),None);
    assert_eq!(parse_pair::<i32>("10,20",        ","),Some((10,20)));
    assert_eq!(parse_pair::<i32>("10,20xy",        ","),None);
    assert_eq!(parse_pair::<i32>("0.5x",        "x"),None);
    assert_eq!(parse_pair::<i32>("0.5x0.5",        "x"),Some((0.5,1.5)));
}

fn parse_complex(s:&str)-> Option<Complex <f64>>{
    match parse_pair(s,","){
        Some((re,im))=> Some(Complex{re,im}),
        None=>None
    }
}
#[test]
fn test_parse_complex(){
    assert_eq!(parse_complex("1.25,-0.0625"),
                   Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}
