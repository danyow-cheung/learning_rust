use std::collections::HashMap;
type Table = HashMap<String ,Vec<String>>;
// show的參數表的類型已從table更改為&table：
// 我們現在傳遞的不是按值傳遞錶（從而將所有權轉移到函數中），而是共亯引用。 
// 這是唯一的文字變化
fn show(table :&Table){
    for (artist,works) in table{
        println!("works by {}",artist);
        for work in works{
            println!(" {}",work);
        }
    }
}

// 現在，如果我們想編寫一個函數來按字母順序排列每個藝術家的作品
// ，那麼共亯引用是不够的，因為共亯引用不允許修改。
// 相反，排序函數需要對錶進行可變引用：
fn sort_works(table: &mut Table){
    for (_artist,works) in table{
        works.sort();
    }
}

fn main(){
    let mut table = Table::new();
    
    table.insert("Gesualdo".to_string(),vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),vec!["The Musicians".to_string(),"The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),vec!["Perseus with the head of  Medusa".to_string(),"a salt cellar".to_string()]);
    // 這裡&起到的是引用的作用
    // 如果沒有& 那麼變量的指針就會轉移，導致table是空value的情況
    // 添加 & 起到共享引用，讀內容而已
    // 注意： 如果這裡添加了&，那麼函數裡面也要添加 & 
    show(&table); 

    assert_eq!(table["Gesualdo"][0],"many madrigals");

    sort_works(&mut table);
    show(&table); 

    
}