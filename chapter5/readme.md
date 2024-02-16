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

```



