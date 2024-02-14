# Basic Types

Rust's types serve serveral goals:
- safety
- Efficiency
- Concision

Rust是一種靜態類型的語言：在不實際運行程式的情况下，編譯器會檢查每個可能的執行路徑是否只以與其類型一致的管道使用值。 這使得Rust能够及早發現許多程式設計錯誤，這對Rust的安全保障至關重要。
下面的兩段代碼，內容完全是一模一樣的
```rust
fn build_vector() -> Vec<i16>{
    let mut v : Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}
fn build_vector()->Vec<i16>{
    let mut v  = Vec::new();
    v.push(10);
    v.push(20);
    v
}
```

## Machine types 
Rust的類型系統的基礎是一組固定寬度的數位類型，選擇這些類型是為了匹配幾乎所有現代處理器直接在硬體中實現的類型，以及布林類型和字元類型。
Rust的數位類型的名稱遵循規則模式，以比特為組織拼寫其寬度及其使用的表示形式：

| Size(bits)   | Unsigned integer | Signed integer | Floating-point |
| ------------ | ---------------- | -------------- | -------------- |
| 8            | u8               | i8             |                |
| 16           | u16              | i16            |                |
| 32           | u32              | i32            | f32            |
| 64           | u64              | i64            | f64            |
| machine word | usize            | isize          |                |



### Integer Types

| Type  | Range                          |      |
| ----- | ------------------------------ | ---- |
| u8    | 0 to                           |      |
| u16   |                                |      |
| u32   |                                |      |
| u64   |                                |      |
| usize | 0 to either 2^32 -1 or 2^64 -1 |      |



### Floating-point types

| Type | Range |
| ---- | ----- |
| f32  | 0 to  |
| f64  |       |



### The bool type 

Rust的布林類型bool通常有兩個值，true和false。 像==和<這樣的比較運算符會產生布林結果：2<5的值為true。

```rust
assert_eq!(false as i32,0);
assert_eq!(true as i32,1);

```



### Chararcters

Rust對孤立的單個字元使用char類型，但對字串和文字流使用UTF-8編碼。 囙此，字串將其文字表示為UTF-8位元組序列，而不是字元數組。
字元文字是用單引號括起來的字元，如“8”或“！”。 您可以使用任何您喜歡的Unicode字元：' 錆'  是表示日語漢字sabi（鐵銹）的字元文字。
與位元組文字一樣，少數字元需要反斜線轉義：



| Character       | Rust character literal |
| --------------- | ---------------------- |
| Single quote    | '\                     |
| Backslash,\     | -\\                    |
| Newline         | '\n'                   |
| Carriage return | '\r'                   |
| Tab             | '\t'                   |





## Tuples

元組的每個元素都可以有不同的類型，而數組的元素必須都是相同的類型。 此外，元組只允許常數作為索引，如t.4。 你不能寫t.i或t[i]來得到第i個元素。





## Pointer types

Rust有幾種表示記憶體位址的類型。
這是Rust和大多數帶有垃圾收集的語言之間的一個很大區別。 在Java中，如果類Tree包含欄位Tree left；， 然後左邊是對另一個單獨創建的Tree對象的引用。 在Java中，對象在物理上從不包含其他對象。



### References

&String類型的值（發音為“ref-String”）是對String值的引用，&i32是對i32的引用，依此類推。





- 在Rust中，引用被視為基本指針類型。
- 引用可以指向任何值，無論它是在堆棧中還是堆中。
- &運算子用於生成對值的引用。
- 運算式*r是指引用r所指向的值。
- 與C指針一樣，Rust中的引用在超出範圍時不會自動釋放資源。



和c語言中的空指針不同，rust指針裡面不會有空，而且一半rust的引用一般是不會變的





### Boxes

在堆中分配值的最簡單方法是使用Box::new：

```rust
let t = (12,'eggs')
let b = Box::new(t);
```

t的類型是（i32，&str），所以b的類型是Box<（i32、&str）>。 Box::new（）分配足够的記憶體以包含堆上的元組。



### raw pointers 

~~Rust也有原始指針類型*mut T和*const T。原始指針實際上就像C++中的指針一樣。 使用原始指針是不安全的，因為Rust不會努力跟踪它指向的內容。例如，原始指針可能為null，也可能指向已釋放或現在包含不同類型值的記憶體。 C++的所有經典指針錯誤都是為您提供的~~

有c語言中類似的原生指針，但是最好不要用





## Arrays,Vectors, and Slices

Rust有三種類型用於表示記憶體中的值序列：

- `[T;N]`表示一個由N個值組成的數組，每個值的類型為T。數組的大小是編譯時確定的常數，是類型的一部分； <u>不能追加新元素，也不能收縮數組</u>。
- `Vec<T>` 稱為Ts的向量，<u>是一個動態分配的、可增長的T類型值序列</u>。向量的元素位於堆中，囙此您可以隨意調整向量的大小：將新元素推到它們上，將其他向量附加到它們上、删除元素等等。
- `&[T]`,`&mut [T]` 稱為Ts的共享切片和Ts的可變切片，是對一系列元素的引用，這些元素是其他值（如數組或向量）的一部分。 您可以將切片視為指向其第一個元素的指針，以及從該點開始可以訪問的元素數量的計數。 
  - 可變切片`&mut [T]`允許您讀取和修改元素，但不能共享；
  -  共享切片`&[T]`允許您在多個讀取器之間共享存取權限，但不允許您修改元素。



給定這三種類型中任意一種的值v，運算式v.len（）給出了v中元素的數量，而v[i]指的是v的第i個元素。
i必須是一個`usize`value 不能使用任何其他整數類型作為索引。





### Arrays

有幾種方法可以寫入數組值。 最簡單的方法是在方括號內寫入一系列值：

> arrays.rs
>
> 怎么run single 的 .rs file
>
>  ` rustc <filename>.rs` 然后再`./<filename>`



### Vectors

向量`Vec<T>`是在堆上分配的T類型元素的可調整大小的數組。

有幾種方法可以創建向量。 最簡單的是使用`vec！ marco`，其中為我們提供了一個看起來非常像數組文字的向量語法：

> vectors.rs





### Building Vectors Element By Element

Building a vector one element at a time isn’t as bad as it might sound. Whenever a vector outgrows its buffer’s capacity, it chooses a new buffer twice as large as the old one. Suppose the vector starts with a buffer that can hold only one element: as it grows to its final capacity, it’ll have buffers of size 1, 2, 4, 8, and so on until it reaches its final size of 2*n*, for some *n*. If you think about how powers of two work, you’ll see that the total size of all the previous, smaller buffers put together is 2*n*–1, very close to the final buffer size. Since the number of actual elements is at least half the buffer size, the vector has always performed less than two copies per element!

What this means is that using Vec::with_capacity instead of Vec::new is a way to gain a constant factor improvement in speed, rather than an algorithmic improve‐ ment. For small vectors, avoiding a few calls to the heap allocator can make an observable difference in performance.



通过逐个元素地构建向量并使用`Vec::with_capacity`而不是`Vec::new`，可以在速度上获得一个恒定因素的改进，而不是一个算法上的改进。对于小型向量来说，避免了对堆分配器的多次调用可以在性能上产生可观的差异。这是因为`Vec`在增长时会选择一个比旧缓冲区大两倍的新缓冲区，这种增长方式使得向量的总大小接近2^n的最终大小，其中n是增长过程中缓冲区的数量。由于实际元素的数量至少是缓冲区大小的一半，因此向量每个元素的复制操作都不会超过两次。



### Slices

一個寫[T]而不指定長度的切片是數組或向量的一個區域。 由於切片可以是任何長度，<u>囙此切片不能直接存儲在變數中或作為函數參數傳遞。 切片總是通過引用傳遞的。</u>



對切片的引用是一個fat pointer：一個兩個字的值，包括指向切片的第一個元素的指針和切片中元素的數量。



```rust
print(&v[0..2]);// print the first two elements of v
print(&a[2..]);// print elements of a starting with a[2]
print(&sv[1..3]);
// print v[1] and v[2]
```



## String Types

字串文字的指針類型為const char*。 標準庫還提供了一個類std::string，用於在運行時動態創建字串。



### String Literals

字串文字用雙引號括起來。 它們使用與字元文字相同的反斜線轉義序列：

```
let speech = "\"Ouch!\" said the well.\n"
println!(speech)
println!("In the room the women come and go,
        Singing of Mount Abora");


```



不能簡單地在原始字串前面加一個反斜線就將雙引號字元包括在內——記住，我們說過無法識別轉義序列。 然而，這也是有治癒方法的。 原始字串的開始和結束可以用磅符號標記：
```rust
println!(r###"
        This raw string started with 'r###"'.
        Therefore it does not end until we reach a quote mark ('"')
        followed immediately by three pound signs ('###'):
"###);
```

您可以根據需要添加盡可能少或盡可能多的磅符號，以明確原始字串的結束位置。



### Byte Strings

帶b首碼的字串文字是位元組字串。這樣的字串是u8值（即位元組）的切片，而不是Unicode文字：

``` 
let method = b"GET";
assert_eq!(method, &[b'G', b'E', b'T']);

```



### Strings in Memory

Rust字串是Unicode字元的序列，但它們不會作為字元數組存儲在記憶體中。 相反，它們是使用UTF-8（一種可變寬度編碼）存儲的。 字串中的每個ASCII字元都存儲在一個位元組中。 其他字元佔用多個位元組。



字串有一個可調整大小的緩衝區，用於容納UTF-8文字。 緩衝區是在堆上分配的，囙此它可以根據需要或請求調整緩衝區的大小。 在本例中，麵條是一個擁有8位元組緩衝區的字串，其中7個正在使用中。 您可以將String視為Vec<u8>，它保證保存格式良好的UTF-8； 事實上，String就是這樣實現的。





A String or &str’s .len() method returns its length. The length is measured in bytes, not characters:



It is impossible to modify a &str:

```
let mut s = "hello";
s[0] = 'c'; // error: the type `str` cannot be mutably indexed s.push('\n'); // error: no method named `push` found for type `&str`

```





### String 

`&str `is very much like` &[T]`: a fat pointer to some data. String is analogous to` Vec<T>`: 





### Using Strings

Strings support the == and != operators. 

```
assert!("ONE".to_lowercase()=="one");

```

Strings also support the comparison operators <, <=, >, and >=,

```
assert!("peanut".contains("nut")); 
assert_eq!("ಠ_ಠ".replace("ಠ", "■"), "■_■"); assert_eq!(" clean\n".trim(), "clean");
for word in "veni, vidi, vici".split(", ") { 		assert!(word.starts_with("v"));
}


```





### Other String-like Types

Rust保證字串是有效的UTF-8。 有時，程式確實需要能够處理無效的Unicode字串。 這種情況通常發生在Rust程式必須與其他不執行任何此類規則的系統進行互操作時。 例如，在大多數作業系統中，使用檔名創建檔案很容易
這不是有效的Unicode。 當Rust程式遇到這種檔名時，應該怎麼辦？
Rust的解決方案是為這些情况提供一些類似字串的類型：
•對於Unicode文字，請堅持使用字串和&str。
•使用檔名時，請改用std::path::PathBuf和&path。
•當使用根本不是字元數據的二進位數據時，請使用Vec<u8>和&[u8]。
•使用作業系統提供的本機形式的環境變數名和命令列參數時，請使用OsString和&OsStr。
•當與使用null終止字串的C庫進行互操作時，請使用std::ffi::CString和&CStr。





## Beyond the basics

類型是Rust的覈心部分。 我們將在整本書中繼續討論類型並介紹新的類型。 特別是，Rust的用戶定義類型賦予了語言很大的風格，因為這就是定義方法的地方。 有三種用戶定義類型，



