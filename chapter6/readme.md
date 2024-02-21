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



為了與Rust的移動語義保持一致，值上的for迴圈會消耗該值：

```rust
let string: Vec<String> = error_messages();
for s in strings{			// each string is moved into s here
  println!("{}",s);
}// and dropped here 
println!("{}errors",strings.len());
```

這可能很不方便。 簡單的補救方法是迴圈引用集合。 那麼，迴圈變數將是對集合中每個項的引用：

```rust
for rs in &strings{
	println!("String {:?} is a address {:p}.",*rs,rs);
}
```



這裡&strings的類型是&Vec＜String＞，rs的類型是&String。在mut引用上反覆運算為每個元素提供一個mut引用：

```rust
for rs in &mut strings{  // the type of rs is &mut string
rs.push("\n"); 	// add a newline to each string
}
```



break運算式退出封閉迴圈。 （在Rust中，break只在迴圈中起作用。在匹配運算式中不需要break，這與switch語句在這方面不同。）

`continue`關鍵字跳到下一個循環

```rust
for line in input_lines{
  let trimmed = trim_comments_and_whitespace(line);
  if trimmed.is_empty(){
    continue;
  }
  ...
}
```



在for迴圈中，continue前進到集合中的下一個值。 如果沒有更多的值，則迴圈退出。 類似地，在while迴圈中，continue重新檢查迴圈條件。 如果現在為false，則迴圈退出。
迴圈可以標記為具有生存期。 在以下示例中，'search:是外部for迴圈的標籤。 囙此，break‘search退出了那個迴圈，而不是內部迴圈。

```rust
'search
for room in apartment{
	for spot in room.hiding_spots(){
    if spot.contains(keys){
      println!("Your keys are {} in the {}",spot,room);
      break 'search;
    }
  }
}
```

標籤也可以與continue一起使用。





## return expressions

返回運算式退出當前函數，並向調用方返回一個值。 

不帶值的return是return（）的簡寫：

```rust
fn f(){ // return type omitted :defaults to ()
return ;  // return value omitted : defaults to()
}
```

就像中斷的表達一樣，返回可以放弃正在進行的工作。



```rust
// 下面的代碼意思一樣
let output = File::create(filename)?;

let output = match File::create(filename){
  Ok(f)=>f,
  Err(err)=> return Err(err)
}
```

我們放弃所有這些並退出封閉函數，返回從File::create（）得到的任何錯誤。



## Why Rust has loop

Rust編譯器的幾個部分分析程式中的控制流。

- Rust檢查通過函數的每條路徑是否返回預期返回類型的值。 要正確地執行此操作，它需要知道是否有可能到達函數的末尾。

- Rust檢查本地變數是否從未在未初始化的情况下使用。 這需要檢查通過函數的每條路徑，以確保在沒有經過初始化變數的程式碼的情况下，無法到達使用變數的位置。

- Rust警告無法訪問的程式碼。 如果沒有通過函數的路徑到達程式碼，則程式碼是不可訪問的。

這些被稱為**流量敏感分析**。 它們並不是什麼新鮮事； 多年來，Java一直在進行類似於Rust的“定義分配”分析。



當執行這種規則時，語言必須在簡單性和聰明性之間取得平衡，簡單性使程式師更容易弄清楚編譯器有時在說什麼，聰明性有助於消除錯誤警告和編譯器拒絕完全安全的程式的情况。 Rust追求簡單。 它的流敏感分析根本不檢查迴圈條件，而是簡單地假設程式中的任何條件都可以是真或假。
這導致Rust拒絕某些安全程式：

```rust
fn wait_for_process(process:&mut Process)->i32{
  while true{
    if process.wait(){
      return process.exit_code();
    }
  }
}// error:not all control paths return a value
```

這裡的錯誤是假的。 實際上不可能到達函數的末尾
而不返回值。
迴圈運算式是作為這個問題的“說出你的意思”解決方案提供的。



Rust的類型系統也受到控制流的影響。 前面我們說過if運算式的所有分支都必須具有相同的類型。 但是，在以break或return運算式、無限迴圈或調用panic結束的塊上強制執行此規則是愚蠢的！ （）或std::process:exit（）。 所有這些表達的共同點是，它們永遠不會以通常的管道結束，從而產生價值。 中斷或返回突然退出當前塊； 無限迴圈永遠不會結束； 等等



所以在Rust中，這些運算式沒有正常類型。 **不正常完成的運算式被指定了特殊類型！**， 而且它們不受關於類型必須匹配的規則的約束。 你可以看到！ 在std::process::exit（）的函數簽名中：

```rust
fn exit(code: i32)-> !
```

`!`關鍵字意味著exit永不返回，這是一個發散函數。



您可以使用相同的語法編寫不同的函數，在某些情况下這是非常自然的：

```rust
fn serve_forever(socket:ServerSocket,handler:ServerHandler)->!{
	socket.listen();
  loop{
    let s = socket.accept();
    handler.handle(s);
  }
}
```



當然，如果函數能够正常返回，Rust就會認為這是一個錯誤。

## Function and Method calls

The syntax for calling functions and methods is the same in Rust as in many other languages:

```rust
let x = gcd(1302,462); 	// function call
let room = player.location(); // method call
```

In the second example here, player is a variable of the made-up type Player, which has a made-up .location() method.



rust通常在引用和被引用的值之間做明顯的區別。

**If you pass a &i32 to a function that expects an i32, that’s a type error.**

注意到`. operator` 的限制沒有那麼嚴格。



在方法callplayer.location（）中，player可能是player、&player類型<u>的引用或</u>Box＜player＞或Rc＜player＞類型的<u>智慧指針</u>。 .location（）方法可能通過值或引用獲取玩家。 相同的.location（）語法在所有情况下都有效，因為Rust的。 `. operator`會根據需要自動取消引用播放機或借用對它的引用。



第三種語法用於調用靜態方法，如Vec::new（）。

```rust
let mut numberes = Vec::new();
```



靜態方法和非靜態方法之間的區別與面向對像語言中的相同：<u>非靜態方法對值調用（</u>如my_vec.len（）），靜態方<u>法對類型調用</u>（如vec::new（））。

當然，方法調用可以被連結：

```rust
Iron::new(router).http("localhost:3000").unwrap();
```



Rust語法的一個怪癖是，在函數調用或方法調用中，泛型類型的常用語法Vec<T>不起作用：

```rust
return Vec<i32>::with_capacity(1000); // error 
let ramp = (0..n).collect(Vec<i32>)();// error 
```



問題是，在運算式中，<是小於運算子。 Rust編譯器建議在這種情況下編寫`：：＜T＞`而不是`＜T＞`，這就解决了問題：

```rust
return Vec::<i32>::with_capacity(1000); // error 
let ramp = (0..n).collect::Vec<i32>();// error 
```

符號`：：＜…＞` 在Rust社區被親切地稱為渦輪魚。 或者，通常可以删除類型參數並讓Rust推斷它們：

```rust
return Vec::with_capacity(10);
let rmap :Vec<i32> = (0..n).collect()
```



## Field and Elements

使用熟悉的語法訪問結構的欄位。 元組是相同的，只是它們的欄位有數位而不是名稱：

```rust
game.black_pawns // struct field
coords.1 		// tuple element
```



如果點左側的值是引用或智慧指針類型，則會自動取消引用，就像方法調用一樣。

方括號訪問數組、切片或向量的元素：

`places[i] // array element`

方括號左側的值將<u>自動取消引用。</u>



像這三個運算式被稱為`lvalues`，因為它們可以出現在賦值的左側：

```rust
game.black_pawns = 0x00ff0000_00000000_u64; coords.1 = 0;
pieces[2] = Some(Piece::new(Black, Knight, coords));
```

當然，只有當game、coords和pieces被聲明為mut變數時，才允許這樣做。





當然，只有當game、coords和pieces被聲明為mut變數時，才允許這樣做。
從數組或向量中選取切片非常簡單：

```
let second_half=&game_moves[midpoint..end];
```



這裡game_moves可以是數組、切片或向量； 不管怎樣，結果都是一個長度為端點-中點的借用切片。 game_moves被認為是在second_half的生命週期中借用的。
這個 `..operator`允許省略任一操作數； 根據存在的操作數，它最多可生成四種不同類型的對象：

```rust
.. //Rangerfull
a .. // RangerForm (start:a)
..b // Rangeto(end:b)
a ..b // Range(start:A ,end:B)
```



Rust範圍是半開放的：它們包括起始值（如果有的話），但不包括結束值。 範圍0。。 4包括數位0、1、2和3。
只有包含起始值的範圍才是可反覆運算的，因為迴圈必須有起始位置。 但是在數組切片中，所有四種形式都是有用的。 如果省略了範圍的開始或結束，則默認為要切片的數據的開始或終止。
囙此，經典的分治排序算灋quicksort的實現在一定程度上可能看起來是這樣的：

```rust
fn quicksort<T:Ord>(slice :&mut [T]){
  if slice.len()<=1{
    return ; 	// nothing to sort
  }
  // Parition the slice into two parts,front and back
  let pivot_index = partitions(slice);
  
  //Recursively sort the front half of 'slice'
  quicksort(&mut slice[..pivot_index]);
  // and the back half
  quicksort(&mut slice[pivot_index+1..]);
}
```



## Reference Operators

第5章介紹了運算子&和&mut的地址。



一元*運算子用於訪問引用所指向的值。 正如我們所看到的，當您使用時，Rust會自動跟隨引用。 運算子來訪問欄位或方法，囙此只有當我們想讀取或寫入引用所指向的整個值時，*運算子才是必要的。
例如，有時反覆運算器會生成引用，但程式需要底層值：

```rust
let padovan : Vec<u64> = compute_padovan_sequence(n);
for ele in &padovan{
	draw_triangle(turtle,*elem);
}// the type of elem is &u64,所以*elem is u64
```



## Arithmetic, Bitwise, Comparison, and Logical Operators

>  算術運算子、逐比特運算子、比較運算子和邏輯運算子



Rust的二進位運算子與許多其他語言中的二進位運算子類似。 為了節省時間，我們假設熟悉其中一種語言，並將重點放在Rust偏離傳統的幾點上。

Rust有常用的算術運算子+、-、*、/和%。



標準庫為未檢查的算術提供了a.wrappeng_add（b）等方法。



即使在發佈版本中，整數除以零也會引發恐慌。 整數有一個方法a.checked_div（b），它返回一個Option（如果b為零，則為None）並且從不恐慌。
一元-否定一個數位`Unary -negates a number`。 除無符號整數外，所有數位類型都支持它。 不存在一元+運算子`unary + operator`。



```rust
println!("{}", -100); // -100
println!("{}", -100u32); // error: can't apply unary `-` to type `u32` 
println!("{}", +100); // error: expected expression, found `+`
```



rust繼承在c中的`%`,運算符和位運算符號`&,|,^,<<,>>`。有不同的是，rust中使用`!`取代了`~`



這意味著！ n不能用在整數n上表示“n為零”。為此，寫n==0。



比特移位總是在有符號整數類型上進行符號擴展，在無符號整數類型中進行零擴展。 由於Rust具有無符號整數，囙此它不需要Java的>>運算子。
與C不同，逐位運算的優先順序高於比較，囙此如果您編寫x&BIT！= 0，這意味著（x&BIT）！= 0，正如您可能想要的那樣。 這比C的解釋x&（BIT！=0）有用得多，後者測試錯誤的比特！
Rust的比較運算子是==，！=，<，<=，>， 和 要比較的兩個值必須具有相同的類型。
Rust還有兩個短路邏輯運算子&&和||。 兩個操作數的類型都必須完全為bool。





## Assignment

=運算符可用於為多個變數及其欄位或元素賦值。 但是賦值在Rust中不像在其他語言中那樣常見，<u>因為默認情况下變數是不可變的。</u>
如第4章所述，賦值移動不可壓縮類型的值，而不是隱式複製它們。
支持複合賦值：

```rust
total += item.price;
```

這相當於`total=total+item.price`；。 支持其他操作員
也是：–=、*=等等。 

與C不同，Rust不支持鏈式賦值：你不能寫a=b=3來將值3同時賦給a和b。賦值在Rust中非常罕見，你不會錯過這個簡寫。
<u>Rust沒有C的遞增和遞減運算子++和--。</u>



## Type Casts

將值從一種類型轉換為另一種類型通常需要在Rust中進行顯式強制轉換。 Casts**使用as關鍵字**：

```rust
let x = 17; // x = i32
let index = x as usize; // x= usize
```

允許使用casts的情況：

- 數位可以從任何內寘的數位類型轉換為任何其他類型。

  然而，在撰寫本文時，將大的浮點值強制轉換為太小而無法表示的整數類型可能會導致未定義的行為。 即使在安全的Rust中，這也可能導致崩潰。 這是編譯器中的一個錯誤

- bool、char類型或類C枚舉類型的值可以強制轉換為任何整數類型。

我們說過轉換通常需要cast。 一些涉及參考類型的轉換非常簡單，即使沒有強制轉換，語言也會執行這些轉換。 一個簡單的例子是將mut引用轉換為非mut引用。



不過，可能會發生幾個更重要的自動轉換：

- `&String`類型自動轉換為`&str`
- `&Vec<i32>`自動轉換`&i[32]`
- `&Box<chessboarf>` 自動轉換`&checkboard`



這些被稱為deref強制，因為它們適用於實現deref內寘特性的類型。 <u>Deref強制的目的是使智慧指針類型（如Box）的行為盡可能類似於底層值</u>。 
用戶定義的類型也可以實現Deref特性。 



## Closures

Rust具有閉包、羽量級函數類值。 閉包通常由一個參數列表組成，在分隔號之間給出，後面跟著一個運算式：

```rust
let is_even = |x|x%2==0;
```

Rust推斷參數類型和返回類型。 你也可以像對待函數一樣，明確地寫出它們。 如果您確實指定了返回類型，那麼為了語法的健全性，**閉包的主體必須是塊**：

```rust
//example
let is_even = |x:u64|-> bool{x%2==0};
```



調用閉包使用與調用函數相同的語法：

```
assert_eq!(is_even(14),true)
```



## Precedence And Associativity

優先順序和關聯性



（與大多數程式設計語言一樣，當一個運算式包含多個相鄰運算子時，Rust具有運算子優先順序來確定運算順序。例如，在limit＜2*broom.size+1中，.運算子具有最高優先順序，囙此欄位訪問首先發生。）



