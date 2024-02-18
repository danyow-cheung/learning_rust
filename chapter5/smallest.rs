fn smallest(v:&[i32])-> &i32{
    let mut s = &v[0];
    for r in &v[1..]{
        if *r < *s {s=r;}
    }
    s
}

fn main(){
    // 下面这种写法不行，因为lifetime不能寄出到外面
    // let s ;
    // {
    //     let paranola =  [9,4,1,0,1,4,9];
    //     s = smallest(&paranola);
    // }
    // assert_eq!(*s,0);
    
    // 下面就可以,函數簽名的生命週期使Rust能够評估傳遞給函數的引用和函數返回的引用之間的關係，並確保它們被安全使用。
    
    let paranola =  [9,4,1,0,1,4,9];
    let s = smallest(&paranola);
    assert_eq!(*s,0);

}
