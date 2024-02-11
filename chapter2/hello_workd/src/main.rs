//
// use聲明將Write和FromStr這兩個特性引入作用域。 
// 但現在我們只簡單地說，特性是類型可以實現的方法的集合。 
//     儘管我們從不在程式的其他地方使用名稱Write或FromStr，
//     但特性必須在範圍內才能使用其方法。 在當前情况下：

use std::io::Write;
use std::str::FromStr;
//
// 任何實現Write特性的類型都有一個Write_fmt方法，用於將格式化文字寫入流。 
//      std::io::Stderr類型實現了Write，我們將使用writeln！ 宏以列印錯誤消息； 該宏擴展為使用writefmt方法的程式碼。
// 任何實現FromStr特性的類型都有一個from_str方法，該方法試圖從字串中解析該類型的值。
//      u64類型實現FromStr，我們將調用u64：：from_str來解析命令列參數。

fn main() {
    // println!("Hello, world!");
    // 我們聲明一個可變的局部變數數，並將其初始化為一個空向量。 
    // Vec是Rust的可增長向量類型，類似於C++的std::vector、Python的list 
    // 即使向量被設計為動態增長和收縮，我們仍然必須為Rust標記變數mut，以便將數位推到它的末尾。

    let mut numbers = Vec::new();
    for args in std::env::args().skip(1){ //在這裡，我們使用for迴圈來處理命令列參數，設定變數
        // arg依次傳遞給每個參數，並計算迴圈體。
        numbers.push(u64::from_str(&args).expect("error pushing argument"));

    }
    if numbers.len()==0{
        writeln!(std::io::stderr(),"Usage gcd NUMBER...").unwrap();
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..]{
        d = gcd(d,*m);
    }
    println!("The greatest common divisor of {:?} is {}",numbers, d);
}
fn gcd(mut n:u64,mut m:u64)-> u64{
    assert !(n!=0 && m!=0);
    while m!=0{
        if m<n{
            let t =m;
            m=n;
            n=t;
        }
        m = m%n;
    }
    n
}
#[test]
fn test_gcd(){
    assert_eq!(gcd(14,15),1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3*7*11*13*19),3*11);
}