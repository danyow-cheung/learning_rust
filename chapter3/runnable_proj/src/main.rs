// fn main() {
//     println!("Hello, world!");
// }

// =======================arrays.rs=======================
// fn main(){
//     let lazy_caterer:[u32;6] = [1,2,3,4,5,11];
//     let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

//     assert_eq!(lazy_caterer[3],4);
//     assert_eq!(taxonomy.len(),3);
//     // 排序
//     let mut chaos=[3,5,4,1,2]; 
//     chaos.sort();
//     assert_eq!(chaos, [1, 2, 3, 4, 6]); 
//     // assert_eq! 这句是断言，如果正确的话就走下面的代码
//     // 否则报错
//         // thread 'main' panicked at src/main.rs:15:5:
//         // assertion `left == right` failed
//         // left: [1, 2, 3, 4, 5]
//         // right: [1, 2, 3, 4, 6]
//         // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//     println!("The chaos array is  {:?}",chaos);
    
// }

//=======================vectors.rs=======================
fn main(){
    let mut v = vec![2,3,5,7];
    assert_eq!(v.iter().fold(1,|a,b| a*b),210);
    
    v.push(11);
    v.push(13);
    assert_eq!(v.iter().fold(1,|a,b| a*b),30030);
    println!("The v array is  {:?}",v);
    
    // vec！ 宏也可以使用Vec::new來創建
    let mut v2 = Vec::new();
    v2.push("step");
    v2.push("on");
    v2.push("no");
    v2.push("pets");
    assert_eq!(v2, vec!["step", "on", "no", "pets"]);
    println!("The v2 array is  {:?}",v2);

    // 另一種可能性是根據反覆運算器生成的值構建向量：
    let v3: Vec<i32> = (0..5).collect();
    assert_eq!(v3,[0,1,2,3,4]);
    println!("The v3 array is  {:?}",v3);


    // 向量的切片
    // A palindrome!
    let mut v = vec!["a man", "a plan", "a canal", "panama"]; 
    // println!("The v array is   {:?} - 1",v);
    
    v.reverse();
    // println!("The v array is  {:?} - 2",v);

    // Reasonable yet disappointing:
    assert_eq!(v, vec!["panama", "a canal", "a plan", "a man"]);

    // println!("The v array is  {:?} - 3 ",v);
    
     
    // 如果你事先知道向量需要的元素數量，而不是Vec::new，
    // 你可以調用Vec::with_capacity來創建一個向量，
    // 它的緩衝區足够大，可以從一開始就容納所有元素； 然後，您可以一次向向量中添加一個元素，而不會引起任何重新分配。 
    // vec！ 宏使用了這樣的技巧，因為它知道最終向量將有多少元素。 請注意，這只會建立向量的初始大小； 
    // 如果你超出了你的估計，`向量會像往常一樣簡單地擴大它的存儲空間。`
    let mut v  = Vec::with_capacity(2);
    assert_eq!(v.len(),0);
    assert_eq!(v.capacity(),2); 
    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);
    v.push(3);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 4);


    // 向量的插入和刪除
    let mut v = vec![10,20,30,40,50];
    // 在index=3插入35
    v.insert(3,35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);
    // Remove the element at index 1.
    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);
    
    // 您可以使用pop方法删除最後一個元素並返回它。更準確地說，從Vec＜T＞中彈出一個值會返回Option＜T＞：如果向量已經為空，則返回None；如果其最後一個元件為v，則返回Some（v）：

    assert_eq!(v.pop(), Some(50)); 
    assert_eq!(v.pop(), Some(40)); 
    // assertion `left == right` failed  left: Some(35) right: None
    // assert_eq!(v.pop(), None);

    // 還可以用循環來迭代向量
    let languages : Vec<String> = std::env::args().skip(1).collect();
    for l in languages{
        println!("{}: {}", l, 
            if l.len()%2==0{"functional"}
            else{ "imperative"
        });
    }
    // output is 
    // lisp: functional
    // scheme: functional
    // c: imperative
    // c++: imperative
    // python: functional
    // java: functional
    // go: functional

}
// // 您也可以通過重複給定值一定次數來構建向量，同樣使用模仿數組文字的語法：
// fn new_pixel_buffer(rows:usize,cols:usize)->Vec<u8>{
//     vec![0;rows*cols]
// }

