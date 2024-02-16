fn main()
{
    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(*m == 64);

    // 由於參考文獻在Rust中被廣泛使用。 運算符隱式取消引用其左操作數（如果需要）：
    struct Anime {name:&'static str , bechdel_pass:bool };

    let aria = Anime {name : "Aria",bechdel_pass:true};
    let anime_ref = &aria;
    assert_eq!(anime_ref.name,"Aria");

    // Equivalent to the above, but with the dereference written out:
    assert_eq!((*anime_ref).name, "Aria");

    // 這個 若方法調用需要，運算子還可以隱式借用對其左操作數的引用。 例如，Vec的排序方法採用對向量的可變引用，囙此此處顯示的兩個調用是等效的：
    let mut x = vec![1973,1968];
    v.sort();
    (&mut v).sort(); 
    
}
