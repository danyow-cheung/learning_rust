fn main(){
    let mut v = vec![2,3,5,7];
    assert_eq!(v.iter().fold(1,|a,b| a*b),210);
    
    v.push(11);
    v.push(13);
    assert_eq!(v.iter().fold(1,|a,b| a*b),30030);
    println!("The chaos array is  {:?}",v);

    
}

fn new_pixel_buffer(rows:usize,cols:usize)->Vec<u8>{
    vec![0;rows*cols]
}