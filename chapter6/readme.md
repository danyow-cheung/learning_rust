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



在初始化變數之前使用變數是錯誤的。 （這與移動值後使用值的錯誤密切相關。Rust真的希望你只在值存在的時候使用它們！）







您可能偶爾會看到似乎重新聲明現有變數的程式碼，如下所示：

````rust
for lin in  file.lines(){
	let line  = lin?;
}
````



let聲明創建了一個不同類型的新的第二個變數。 line_result的類型為result＜String，io：：Error＞。 第二個變數，行，是一個字串。賦予第二個和第一個變數相同的名稱是合法的。 在這本書中，我們將堅持在這種情況下使用_result尾碼，以便所有變數都有不同的名稱。





塊也可以包含項聲明。 項只是可以全域出現在程式或模塊中的任何聲明，例如fn、結構或use。
後面的章節將詳細介紹各項內容。 現時，fn就是一個充分的例子。 任何塊都可以包含fn：

```rust
use std::io;
use std::cmp::Ordering;

fn show_files()-> io::Result<()>{
  let mut v = vec![];
  
  fn cmp_by_timestamp_then_name(a: &FileInfo,b:&FileInfo)-> Ordering{
    a.timestamp.cmp(&b.timestamp)// first compare timestamps
    .reverse() // newest file first 
    .then(a.path.cmp(&b.path))// compare paths tot break ties
  }
  v.sort_by(cmp_by_timestamp_then_name);
}
```



當在塊內部聲明fn時，它的作用域是整個塊——也就是說，它可以在整個封閉塊中使用。 但是嵌套的fn無法訪問恰好在作用域中的局部變數或引數。 例如，函數cmp_by_timestamp_then_name不能直接使用v。 （Rust也有閉包，它可以查看封閉的範圍。請參閱第14章。）
一個塊甚至可以包含一個完整的模塊。 這可能看起來有點過分——我們真的需要能够把語言的每一部分都嵌套在每一部分中嗎-- 但是程式師（尤其是使用宏的程式師）有一種方法可以找到語言所提供的每一點正交性的用途。



## if and match

````rust
if condtion1{
  bllock1 
}else if condtion2{
  block2
}else {
  block_n
}
````

每一個表達式都必須是布爾類型，與形式一樣，Rust不會隱式地將數位或指針轉換為布林值。



與C不同，條件周圍不需要括弧。 事實上，如果存在不必要的括弧，rustc將發出警告。 但是，**大括弧是必需的。**
else-if塊以及最終的else塊都是可選的。 一個沒有else塊的if運算式的行為就像它有一個空的else塊一樣。
匹配運算式有點像C開關語句，但更靈活。 一個簡單的例子：

```rust
match code {
  0=> println!("OK"),
  1> println!("Wires "),
 2=> println!("User Aslepp"),
  _> println!("Unrecongnized error{}",code)
}
```

这个代码是switch關鍵字會做的，取決於code的數值，四個分句代碼實際上都會運行。

其中_ 匹配一切在未知的情況下。囙此它充當預設值：case。



編譯器可以使用跳轉錶來優化這種匹配，就像C++中的switch語句一樣。 當比賽的每個手臂都產生一個常數值時，也會應用類似的優化。 在這種情況下，編譯器構建這些值的數組，並將匹配項編譯為數組訪問。 除了邊界檢查之外，編譯後的程式碼中根本沒有分支。



火柴的多功能性源於每只手臂=>左側可以使用的各種支撐圖案。 上面，每個模式都只是一個常數整數。 我們還展示了區分兩種Option值的匹配運算式：



```rust
match param.get("name") {
 Some(name)=> println!("helo,{}",name),
  None=> println!("Wires ")
}
```

匹配運算式的一般形式是：

```rust
match value {
  pattern => expr,
}
```



<u>如果運算式是塊，則可以去掉arm後面的逗號。</u>
Rust從第一個模式開始，依次對照每個模式檢查給定的值。 當模式匹配時，將計算相應的expr，並且匹配運算式是完整的； 沒有進一步的圖案被檢查。 至少有一個模式必須匹配。 Rust禁止不包含所有可能值的匹配運算式：

```rust
let score = match card.rand{
  Jack => 10,
  Queen =>10,
  Ace=>11
};// error none xhaustive patterns
```

all blocks of an if expression must produce values of the s<u>ame type</u> 

````rust
let s_p = if with_wings {Pet::BUzzard}else{Pet::Hyena} // ok

let f_n = if user.is_hobbit(){"ele"} else {9}; //error 

let best_s = if is_hocke() {"Prea"};//error
````



Similarly ,all arms of a match expression must have the same type 

```rust
let s_p = match favor.element{
  Fire => Pet::RedPanda,
  Air=> Pet::Bird,
  Water=> Pet::Orca,
  _=>None // error incompatible types
}
```

### if let 

there is one more `if ` form, the `if let `form

```rust
if let pattern = expr{
block1
}else{
block2
}
```

給定的expr要麼與模式匹配，在這種情況下block1運行，要麼不匹配，block2運行。 有時，這是從Option或Result中獲取數據的好方法：

```rust
if let Some(cookie) = request.session_cookie{
  return restore_session(cookie);
}

if let Err(err) = present_cheesy_anti_robot_task(){
  log_robot_attempt(err);
  politely_accuse_user_of_being_a_robot();
  
}else{
  session.mark_as_human();
}

```

從不嚴格要求使用if let，因為match可以做if let可以做的所有事情。if let運算式是只有一種模式的匹配的簡寫：

```rust
match expre{
  pattern => {block1}
  _=> {block2}
}
```



## Loops

四種循環的表達方式

```rust
while condition{
block
}

while let pattern = expr{
  block
}

loop{
  block
}

for pattern in collection{
  block
}
```



在rust中，循環是表達式，不會產生特定數值



**while let**迴圈類似於if let。 在每次迴圈反覆運算開始時，expr的值要麼與給定的模式匹配（在這種情況下塊運行），要麼不匹配（在那種情况下迴圈退出）。



for in c and rust 

```c
for (int i =0;i<20;i++){
	printf("%d\n",i);
}
```

```rust
for i in 0..20{
  println!("{}",i)
}
```

