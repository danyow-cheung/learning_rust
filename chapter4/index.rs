fn main(){
    // build a vector of the strings 
    let mut x = Vec::new();
    for i in 101..106{
    x.push(i.to_string());
    }

    // 1. pop a value off the end of the vector
    let fifth = x.pop().unwrap();
    assert_eq!(fifth,"105");

    // 2. Move a value out of the middle of the vector,and move the last
    let second = x.swap_remove(1);
    assert_eq!(second,"102");

    // 3. swap in another value for the one we're taking out 
    let third = std::mem::replace(&mut x[2],"substitute".to_string());
    assert_eq!(third,"103");

    // let see what's left of our vector 
    assert_eq!(x,vec!["101","104","substitute"]);

}