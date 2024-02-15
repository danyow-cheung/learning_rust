use std::rc::Rc;

fn main(){
    // rust can infer all these types 
    let s :Rc<String> = Rc::new("ohomyholy".to_string());
    let t :Rc<String> = s.clone();
    let u :Rc<String> = s.clone();
    assert!(s.contains("oh"));
    assert_eq!(t.find("holy"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);

    //  s.push_str(" noodles"); // fail 
    // Rust的記憶體和線程安全保證取決於確保沒有任何值同時共亯和可變。
    //  Rust假設Rc指針的引用通常可能是共亯的，所以它不能是可變的
    
}