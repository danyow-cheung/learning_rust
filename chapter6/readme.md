# Expressions

在本章中，我們將介紹Rust的運算式，即構成Rust函數主體的構建塊。 一些概念，如閉包和反覆運算器，已經足够深入，我們稍後將專門用一整章來介紹它們。現時，我們的目標是在幾頁中涵蓋盡可能多的語法。



## An Expression Language

Rust在視覺上類似於C族語言，但這有點像詭計。 在C語言中，運算式和程式碼之間有著明顯的區別，它們看起來像這樣：

`5*(fahr -32)/9`

以及狀態更像這個樣子

```c++
for (;begin!=end ;++begin){
if (*begin==target)
break
}
```

Expressions have values. Statements don’t.

<u>Rust就是所謂的表達語言。 這意味著它遵循了一個古老的傳統，可以追溯到Lisp</u>，在那裡表達完成了所有的工作。
在C中，if和switch是語句。 它們不會產生值，並且不能在運算式的中間使用。 在Rust中，if和match可以生成值。

```rust
pixels[r*bounds.0 +c]= match escapes(Complex{
  re:point.0,im:point.1},255){
    None=>0,
    Some(count)=>255 - count as u8
};
```



if 關鍵字還可以用來初始化數值

```rust
let status = if cpu.temperature <= MAX_TEMP{
	HttpStatus:Ok
}else{
	HttpStatus::ServerError // server melted
}
```



匹配match運算式可以作為參數傳遞給函數或宏：

```rust
println!("Inside the vat,you see{}",match vat.contents{
	Some(brain)=>bran.desc(),
	None=>"nothing of interest"
});
```



C中的大多數控制流工具都是語句。 在Rust中，它們都是運算式。



## Blocks and Semicolons

> 塊和分號

塊也是運算式。 塊產生一個值，可以在任何需要值的地方使用：

```rust
let display_name = match post.author(){
  Some(author)=> author.name(),
  None=>{
    let network_info = post.get_network_metadata()?;
    let ip = network_info.client_address();
    ip.to_string()
  }
};
```

Some（author）=>之後的程式碼是簡單運算式author.name（）。 None=>之後的程式碼是一個塊運算式。 這對Rust來說沒有什麼區別。 塊的值是其最後一個運算式ip.to_string（）的值。



**請注意，該運算式後面沒有分號。**

`match`表達式

 Rust程式碼的大多數行都以分號或大括弧結尾，就像C或Java一樣。 如果一個塊看起來像C程式碼，在所有熟悉的地方都有分號，那麼它將像C塊一樣運行，其值為（）。 正如我們在第2章中提到的，當你離開分號時
在塊的最後一行，您將使該塊產生一個值——最終運算式的值。





在某些語言中，特別是JavaScript，您可以省略分號，而語言只需為您填充它們——這只是一個小小的方便。 這是不同的。 在Rust中，分號實際上是有意義的。

> 看下面的代碼

```rust
let msg = {
  let dandelion_control = puffball.open();
  
  dandelion_control.release_all_seeds(launch_codes);
  dandelion_control.get_status()
}
```



塊包含聲明並在末尾生成值的能力是一個巧妙的功能，很快就會讓人感覺很自然。 一個缺點是，當您意外遺漏分號時，會導致一條奇怪的錯誤消息。



Rust假設您故意省略了這個分號； 它不考慮這只是一個拼寫錯誤的可能性。 結果是出現混亂的錯誤消息。 當您看到所需的類型“（）”時，請先查找缺少的分號。





塊中也允許使用空語句。 一個空語句由一個游離的分號組成，全部由分號本身組成：

```rust
loop{
	work();
	play();
}
```

Rust遵循了C的傳統，允許這樣做。 空洞的話語只會傳達出一種淡淡的憂鬱感。 我們提到它們只是為了完整。



## Declarations

除了運算式和分號之外，一個塊還可以包含任意數量的解密。 最常見的是let聲明，它聲明局部變數：

```rust
let name : type = expr;
```

類型和初始值設定項是可選的。 分號是必需的。

`let`關鍵字還能定義一個變量，卻不初始化。這個變量可以通過稍後的賦值從而進行初始化。

```rust
let name ; 
if user.has_nickname(){
name = user.nickname();
}else{
name = generate_unique_name();
user.register(&name);
}
```



這裡有兩種不同的管道可以初始化局部變數名稱，但無論哪種管道，它都將被初始化一次，所以名稱不需要聲明為mut。

