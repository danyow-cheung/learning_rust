# References

到目前為止，我們看到的所有指針類型——簡單的Box<T>堆指針，以及String和Vec值內部的指針——都是擁有指針的：當所有者被丟棄時，引用也會隨之而去。Rust還有稱為引用的非擁有指針類型，這對其引用的生存期沒有影響。

事實上，情况恰恰相反：引用永遠不能超過其引用。 您必須在程式碼中明確表示，任何引用都不可能超過它所指向的值。為了強調這一點，Rust指的是創建一個對某個值的引用作為對該值的借用：您借用的東西，最終必須歸還給它的所有者。

如果你在閱讀“你必須在程式碼中應用它”這句話時感到懷疑，那麼你就是一個優秀的夥伴。 參考資料本身並沒有什麼特別之處——在引擎蓋下，它們只是地址。 但對Rust來說，保證他們安全的規則是新穎的； 除了研究語言，你以前不會見過這樣的語言。 儘管這些規則是Rust最需要努力掌握的部分，但它們所防止的經典的、絕對日常的bug的廣度是驚人的，它們對多執行緒程式設計的影響是解放的。 這又是魯斯特的激進賭注。

舉個例子，讓我們假設我們要建立一個文藝復興時期殺人藝術家和他們以作品聞名的表格。 Rust的標準庫包括一個雜湊表類型，囙此我們可以這樣定義我們的類型：

> demo.rs



但如果你已經閱讀了上一章關於動作的部分，這個表演的定義應該會提出一些問題。 特別是，HashMap不是Copy——它不可能是，因為它擁有一個動態分配的錶。 囙此，當程式調用show（table）時，整個結構被移到函數中，變數錶未初始化。 如果調用程式碼現在嘗試使用錶，它將遇到問題：

```
show(table);
assert_eq!(table["Gesuladp"][0],"many madrigals");
```

遇到報錯

```
error[E0382]: use of moved value: `table`
      --> references_show_moves_table.rs:29:16
| 28 | | 29 | | |
show(table);
     ----- value moved here
assert_eq!(table["Gesualdo"][0], "many madrigals");
           ^^^^^ value used here after move
       = note: move occurs because `table` has type `HashMap<String, Vec<String>>`,
               which does not implement the `Copy` trait
```

事實上，如果我們研究show的定義，外部for迴圈獲得雜湊表的所有權並完全消耗它； 內部for迴圈對每個向量都做同樣的操作。 （我們早些時候在“liberté，égalité，fraternité”考試中看到了這種行為。）由於移動語義的原因，<u>我們試圖將其列印出來，從而完全破壞了整個結構類比。</u> 





正確的處理方法是使用引用。 引用允許您在不影響其所有權的情况下訪問值。 references come in two kinds:

- 共享引用允許您讀取但不能修改其引用。 但是，您可以一次擁有任意多個對特定值的共享引用。 <u>運算式&e生成對e值的共享引用； 如果e的類型是T，那麼&e的類型為&T，</u>發音為“ref T”。 共享引用是Copy。
- 如果您有一個對值的可變引用，您可以讀取和修改該值。 但是，您可能不會同時啟動對該值的任何其他引用。 運算式<u>&mut e生成對e值的可變引用； 您將其類型寫成&mut T，發音為“ref mute T”</u>。 可變引用不是Copy。



> 簡單記憶就是，& 指代的就是引用，添加mut 就可以對引用的內容進行更改



當我們以將值的<u>所有權轉移到函數的管道將值傳遞給函數時，我們就說我們是按值傳遞的。</u> 如果我們將函數作為對該值的引用，則表示我們通過引用傳遞了該值。 例如，我們修復了<u>show函數，將其更改為通過引用而不是通過值接受</u>錶。 許多語言都有這種區別，但它在Rust中尤為重要，因為它闡明了所有權是如何受到影響的。





## References as Values

前面的例子展示了引用的一個非常典型的用法：允許函數訪問或操作結構而不獲取所有權。 但`references`	比這更靈活，所以讓我們看看一些例子，以更詳細地瞭解正在發生的事情。

### Rust References Versus C++ References 

如果您熟悉C++中的引用，那麼它們確實與Rust引用有一些共同之處。 最重要的是，它們都只是機器級別的地址。 但在實踐中，Rust的參考文獻有一種非常不同的感覺。
在C++中，引用是通過轉換隱式創建的，也隱式取消引用：

> versus.rs



### Assigning References

指定給Rust引用會使其指向一個新值：

```rust
let x= 10;
let y = 20;
let mut r = &x;
if b{ r=&y;}
assert!(*r==10 || *r ==20);
```

<img src ='assign_ref.png'>

這與C++非常不同，在C++中，賦值給引用會將值存儲在其引用中。 <u>沒有辦法將C++引用指向初始化時使用的位置之外的其他位置。</u>





### References to References

Rust permits references to references:

```rust
struct Point {x:i32,y:i32}
let point = Point{x:1000,y:729};
let r : &Ppomt = &point;
let rr: &&Point = &r;
let rrr: &&&Point = &rr;

assert_eq!(rrr.y,729);
```



（為了清晰起見，我們已經寫出了參考類型，但您可以省略它們；這裡沒有Rust無法自行推斷的內容。）。 運算符會根據查找目標所需的引用數進行查找：



<img src =' ref2ref.png'>





### Comparing References

喜歡。 運算子，Rust的比較運算子“看穿”任意數量的引用，只要兩個操作數具有相同的類型：

```
letx=10; lety=10;
letrx=&x; letry=&y;
let rrx = &rx; let rry = &ry;
    assert!(rrx <= rry);
    assert!(rrx == rry);
```

這裡的最終斷言是成功的，即使rrx和rry指向不同的值（即rx和ry），因為==運算子跟隨所有引用並對它們的最終目標x和y執行比較。這幾乎總是您想要的行為，尤其是在編寫泛型函數時。 如果你真的想知道兩個引用是否指向同一個記憶體，你可以使用std::ptr::eq，它將它們作為地址進行比較：

```
assert!(rx==ry);
assert!(!std::ptr::eq(rx,ry));
```



### References Are Never Null

Rust引用從不為空。 沒有類似於C的NULL或C++的nullptr； 引用沒有默認的初始值（無論其類型如何，在初始化之前都不能使用任何變數）； Rust不會將整數轉換為引用（不安全程式碼之外），囙此您無法將零轉換為引用。



Rust沒有空指針





### Borrowing References to Arbitrary Expressions

> 借用對任意運算式的引用



儘管C和C++只允許您將&運算子應用於某些類型的運算式，但Rust允許您借用對任何類型運算式值的引用：

```rust
fn factorial(n:usize)->usize{
(1..n+1).fold(1,|a,b|a*b)
}
let r = &factorial(6);
assert_eq!(r+&1009,1729);
```

在這種情況下，Rust只需創建一個匿名變數來保存運算式的值，並對此進行引用。 此匿名變數的生存期取決於您對引用所做的操作：



- 如果您立即將引用分配給let語句中的某個變數（或使其成為正在立即分配的某個結構或數組的一部分），那麼Rust會使匿名變數在let初始化變數的時間內有效。 在前面的例子中，Rust會對r的引用執行此操作。
- 如果您立即將引用分配給let語句中的某個變數（或使其成為正在立即分配的某個結構或數組的一部分），那麼Rust會使匿名變數在let初始化變數的時間內有效。 在前面的例子中，Rust會對r的引用執行此操作。





### References to Slices and Trait Objects

到目前為止，我們顯示的引用都是簡單的地址。 然而，Rust還包括兩種胖指針，兩個字的值攜帶某個值的地址，以及使用該值所需的一些進一步資訊。



除了攜帶這些額外的數據外，切片和特徵對象引用的行為與我們在本章中展示的其他類型的引用一樣：它們不擁有自己的引用； 不允許他們的壽命超過他們的參照物； 它們可以是可變的或共亯的； 等等



## Reference Safety 

到目前為止，我們已經介紹了它們，引用看起來很像C或C++中的普通指針。 但這些都是不安全的； Rust是如何控制其引用的？ 也許，**瞭解規則的最佳管道是嘗試打破它們。 我們將從盡可能簡單的例子開始，然後添加有趣的複雜性並解釋它們是如何工作的。**



### Borrowing a Local Variable 

這裡是個例子，你不能借用引用到本地變量，然後再把引用帶出循環

```rust
{let r ;
	{
	let x = 1;
	r = &x ;
	}
	assert_eq!(*r,1);
}
```

The Rust compiler rejects this program, with a detailed error message:

```
  --> references_dangling.rs:8:5
|
7 | r = &x;
| - borrow occurs here 8|}
       |     ^ `x` dropped here while still borrowed
    9  |     assert_eq!(*r, 1);  // bad: reads memory `x` used to occupy
    10 | }
       | - borrowed value needs to live until here
```

Rust的抱怨是，x只存在到內部塊的末尾，而引用在外部塊的末尾仍然存在，這使它成為一個懸空指針，這是禁止的。



雖然對於人類讀者來說，這個程式被破壞是顯而易見的，但值得一看Rust本身是如何得出這個結論的。 即使是這個簡單的例子也顯示了Rust用來檢查更複雜程式碼的邏輯工具。





### Receiving References as Parameters 

當我們傳遞對函數的引用時，Rust如何確保該函數安全使用它？ 假設我們有一個函數f，它接受一個引用並將其存儲在一個全域變數中。 我們需要對此進行一些修改，但以下是第一條：

```rust
static mut STASH: &i32;
fn f(p:&i32){STASH=p;}

```



Rust的全域變數等價物被稱為靜態變數：它是一個在程式啟動時創建的值，並一直持續到程式終止。 （與任何其他聲明一樣，Rust的模塊系統控制靜態可見的位置，囙此它們在其生命週期中只是“全域”的，而不是可見性。）我們在第8章中介紹了靜態，但現在我們只列出一些剛剛顯示的程式碼不遵循的規則：

- 每個靜態都必須初始化。
- 可變靜態本質上不是執行緒安全的（畢竟，任何線程都可以在任何時候訪問靜態），即使在單執行緒程式中，它們也可能成為其他類型的可重入問題的犧牲品。 由於這些原因，您只能在不安全的塊中訪問可變靜態。 在這個例子中，我們不關心那些特定的問題，所以我們只會拋出一個不安全的塊，繼續前進。



經過這些修訂，我們現在有以下內容：

```
static mut STASH : &i32 = &128;
fn f(p: &i32) { // still not good enough
unsafe { STASH = p;
} }


```



我們差不多完成了。 要瞭解剩下的問題，我們需要寫出Rust讓我們省略的一些內容。 這裡所寫的f的簽名實際上是以下內容的簡寫：

```rust
fn f<'a>(p : &'a i32){...}
```



這裡，lifetime‘a（發音為“tick a”）是f的lifetime參數。您可以將＜'a＞讀取為“對於任何lifetime’a”，囙此當我們編寫fn f＜'a>（p:&'a i32）時，我們定義了一個函數，該函數引用了具有任何給定lifetime‘a的i32。





在這一點上，很明顯，我們的函數不能只接受任何引用作為論據。 但它應該能够接受具有“靜態生存期”的引用：將這樣的引用存儲在STASH中不能創建懸空指針。 事實上，以下程式碼編譯得很好：

```rust
static mut STASH:&i32 = &10;
fn f(p:&'static i32){
  unsafe{
    STASH = p ; 
  }
}
```



這一次，f的簽名闡明了p必須是一個具有lifetime“static”的引用，囙此將其存儲在STASH中不再有任何問題。 我們只能將f應用於對其他靜態的引用，但這是唯一可以肯定的，不會讓STASH以任何管道懸空。 所以我們可以寫：

``` rust
static WORTH_POINTING_AT: i32 = 1000; f(&WORTH_POINTING_AT);
```

因为`WORTH_POINTING_AT`是静态的，the type of &WORTH_POINTING_AT is &'static i32, which is safe to pass to f.



不過，退一步看，當我們修改正確性時，注意到f的簽名發生了什麼：原來的f（p:&i32）最終變成了f（p:&’static i32）。 換句話說，我們無法編寫一個在全域變數中隱藏引用的函數，而不在函數的簽名中反映這一意圖。 在Rust中，函數的簽名總是暴露身體的行為。





相反，如果我們確實看到一個函數的簽名像g（p:&i32）（或者寫出了生存時間g<’a>（p:&'a i32）），我們可以看出它不會將參數p隱藏在比調用更持久的任何地方。 沒有必要研究g的定義； 簽名本身告訴我們g對它的論點能做什麼和不能做什麼。 當您試圖建立對函數的調用的安全性時，這個事實最終會非常有用。





### Passing References as Arguments

Now that we’ve shown how a function’s signature relates to its body, let’s examine how it relates to the function’s callers. Suppose you have the following code:

> ref.rs



如果我們試圖將&x傳遞給前面的函數f，該函數將其參數存儲在靜態中，該怎麼辦？

``` 
fn f(p: &'static i32) { ... } letx=10;
f(&x);

```

這無法編譯：引用&x的壽命不能超過x，但通過將其傳遞給f，我們將其約束為至少與“static”一樣長。 沒有辦法讓這裡的每個人都滿意，所以Rust拒絕了程式碼。



### Returning References

函數通常會引用某個資料結構，然後返回對該結構某個部分的引用。 例如，這裡有一個函數，它返回對切片中最小元素的引用：

> smallest.rs

我們以通常的管道從該函數的簽名中省略了生存期。 當函數將單個引用作為參數並返回單個引用時，Rust假設兩者必須具有相同的生存期。 明確地寫出來會給我們：

```
fn smallest<'a>(v: &'a [i32]) -> &'a i32 { ... }
```





### Structs Containing References

Rust如何處理存儲在資料結構中的引用？ 這是我們之前看到的相同錯誤程式，只是我們將引用放在了一個結構中：

```rust
struct S { r: &i32
}
let s; {
let x=10;
s=S{r:&x}; }
assert_eq!(*s.r, 10); // bad: reads from dropped `x`
```





Whenever a reference type appears inside another type’s definition, you must write out its lifetime. You can write this:

```rust
struct S { r: &'static i32
}
```



> struct_contain_ref.rs



現在S類型有一個生存期，就像參考類型一樣。您為類型S創建的每個值都會有一個新的生存期“a”，它會受到您如何使用該值的約束。 你存儲在r中的任何引用的生存期最好包含“a”，“a”必須比你存儲S的任何地方的生存期都長。





回到前面的程式碼，運算式S｛r:&x｝創建了一個具有一些生存期“a”的新S值。當您將&x存儲在r欄位中時，您將“a”約束為完全位於x的生存期內。





我們不能在這裡省略S的生存期參數：Rust需要知道t的生存期與S中引用的生存期之間的關係，以便對t應用與對S和普通引用相同的檢查。
我們可以給s靜態壽命。 這項工作：

```rust
struct T{
s:S<'static>
}
```



The other approach would be to give T its own lifetime parameter, and pass that to S:

```rust
struct T<'a>{
s:S<'a>
}
```





不僅僅是像s這樣的引用和類型具有生存期。 Rust中的每個類型都有一個生存期，包括i32和String。大多數類型都是“靜態”的，這意味著這些類型的值可以生存多久； 例如，Vec<i32>是自包含的，在任何特定變數超出範圍之前都不必删除它。 但是像Vec<&'a i32>這樣的類型有一個必須用“a”括起來的生存期：它必須在其引用仍然存在時被丟棄。







### Distinct Lifetime Parameters

假设有两个结构

```
struct S<'a>{
	x: &'a  i32;
	y: &'a  i32
}
```



兩個引用使用相同的生存期“a”。如果您的程式碼想要執行以下操作，這可能會成為一個問題：

```
let x = 10;
let r;
{
	let y = 20;
	{
			let s = S{x:&x,y:&y};
			r = s.x ;
	}
}
```



此程式碼不創建任何懸空指針。 對y的引用停留在s中，在y之前超出了範圍。 對x的引用最終以r結束，r的壽命並不比x長。



這些約束是不可能滿足的：沒有一個壽命比y的範圍短，但比r的範圍長。 鏽跡斑斑。
問題的出現是因為S中的兩個引用都有相同的生存期。更改S的定義以使每個引用都有不同的生存期可以解决所有問題：

```
struct S<'a>{
	x: &'a  i32;
	y: &'b  i32
}
```





### Omitting Lifetime Parameters

到目前為止，我們在這本書中已經展示了許多返回引用或將其作為參數的函數，但我們通常不需要說明哪個生命週期是哪個。 生命就在那裡； Rust只是讓我們在它們應該是什麼的時候忽略它們。



在最簡單的情况下，如果函數不返回任何引用（或其他需要生存期參數的類型），則永遠不需要為參數寫出生存期。 Rust只是為每個需要的點指定一個不同的生存期。 例如

```rust
struct S<'a,'b>{
	x: &'a  i32,
	y: &'b  i32
}

fn sum_r_xy(r:&i32,s:S)->i32{
  r+s.x+s.y;
}

```

这个函数`sum_r_xy`的生命周期比下面的短

`fn sum_r_xy<'a,'b,'c>(r:&'a i32,s:S<'b,'c>)->i32`



如果你的參數中有多個生命週期，那麼就沒有自然的理由更喜歡一個而不是另一個作為返回值，Rust會讓你詳細說明發生了什麼。
但作為最後一種簡寫，如果你的函數是某種類型的方法，並通過引用獲取其自身參數，那麼這就打破了平局：Rust假設self的生命週期是你返回值中給出一切的時間。 （self參數指的是調用方法的值，在C++、Java或JavaScript中相當於Rust的值，或者在Python中相當於self的值。我們將在第198頁的“用impl定義方法”中介紹方法。）



**Rust assumes that whatever you’re borrowing, you’re borrowing from self.**





## Sharing Versus Mutation

> 共享與突變

到目前為止，我們已經討論了Rust如何確保沒有引用指向超出範圍的變數。 但還有其他方法可以引入懸空指針。 這裡有一個簡單的案例：

```rust
let v = vec![4,8,19,27,34,10];
let r = &v;
let aside = v; // move vector to aside
r[0];					// bad: uses 'v',which is now unitialized
```

對的賦值將向量移到一邊，使v未初始化，將r變成一個懸空指針



儘管v在r的整個生命週期中都處於作用域中，但這裡的問題是，**v的值被移到了其他地方，導致v未初始化，而r仍然引用它。**當然，Rust會發現錯誤：



在其整個生命週期中，共亯引用將其引用設為只讀：您不能為引用賦值或將其值移動到其他位置。 在這段程式碼中，r的生存期包含移動向量的嘗試，囙此Rust拒絕該程式。 如果按此處所示更改程式，則沒有問題：

```rust
let v = vec![4,8,19,27,34,10];
{
	let r = &v;
	r[0];
	}
let aside = v;
```

在這個版本中，r早些時候超出了範圍，引用的生存期在v之前結束移到一邊，一切都很好。

這是一種不同的`wreak havoc`。 假設我們有一個方便的函數來用切片的元素擴展向量：

```rust
fn extend(vec: &mut Vec<f64>,slice:&[f64]){
  for elt in slice{
    vec.push(*elt);
  }
}
```



這是標準庫對向量的extend_from_slice方法的一個不太靈活（優化程度也低得多）的版本。 我們可以使用它從其他向量或數組的切片中構建向量：

```rust
let mut wave = Vec::new();
let head = vec![0.0,1.0];
let tail = [0.0,-1.0];

extend(&mut wave,&head);
extend(&mut wave,&tail);

assert_eq!(wave,vec![0.0,1.0,0.0,-1,0]);
```



所以我們在這裡建立了一個正弦波的週期。 如果我們想添加另一個起伏，我們可以將向量附加到它本身嗎？

```rust
extent(&mut wave,&wave);
assert_eq!(wave,vec![0.0, 1.0, 0.0, -1.0,
0.0, 1.0, 0.0, -1.0]);
```



這在隨意檢查時可能看起來不錯。 但請記住，當我們向向量添加元素時，如果其緩衝區已滿，則必須分配一個具有更多空間的新緩衝區。 假設wave從四個元素的空間開始，所以當extend試圖添加第五個元素時，必須分配一個更大的緩衝區。





extend函數的vec參數借用了wave（由調用者擁有），後者為自己分配了一個新的緩衝區，其中有八個元素的空間。 但slice繼續指向舊的四元素緩衝區，該緩衝區已被丟棄。

這種問題並不是Rust獨有的：在許多語言中，在指向集合的同時修改集合是一個微妙的領域。 在C++中，std::vector規範提醒您，“（向量緩衝區的）重新分配會使引用序列中元素的所有引用、指針和反覆運算器無效。





這種bug特別困難的是，它不會一直發生。 在測試中，向量可能總是有足够的空間，緩衝區可能永遠不會被重新分配，問題可能永遠不會曝光。



換句話說，我們可以借用對向量的可變引用，也可以借用對其元素的共亯引用，但這兩個引用的壽命可能不重疊。 在我們的例子中，兩個引用的生存期都包含要擴展的調用，囙此Rust拒絕該程式碼。



這些錯誤都源於違反Rust的突變和共亯規則：

- shared access是只讀權限。
- Mutable access是獨佔訪問。

<img src ='versus.png'>



Rust報告稱，擴展示例違反了第二條規則：由於我們已經將可變引用引入到wave，囙此該可變引用必須是到達向量或其元素的唯一途徑。 對切片的共亯引用本身就是訪問元素的另一種管道，違反了第二條規則。





## Taking Arms Against a Sea of Objects

面對對象編程，這種體系結構有許多優點，但這些優點並沒有在圖中顯示出來：最初的進展很快，很容易破解，幾年後，你將很容易證明完全重寫是合理的。 （提示AC/DC的“通往地獄的高速公路”。）





Rust的一個迷人之處在於，所有權模式在通往地獄的高速公路上設定了減速帶。 在Rust中創建一個迴圈需要一些努力——兩個值，每個值都包含指向另一個的引用。 您必須使用智慧指針類型，如Rc和內部可變性——這是我們還沒有涉及的主題。 Rust更喜歡指針、所有權和資料流程在一個方向上通過系統，

