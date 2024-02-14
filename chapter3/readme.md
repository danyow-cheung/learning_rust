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





### Slices

