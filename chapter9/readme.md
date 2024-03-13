# Structs

Rust結構，有時稱為結構，類似於C和C++中的結構類型、Python中的類和JavaScript中的對象。 結構將各種類型的多個值組合成一個值，囙此可以將它們作為一個單元進行處理。 給定一個結構，您可以讀取和修改它的各個組件。 結構可以具有與其關聯的方法，這些方法對其組件進行操作。





Rust有三種`結構類型named-field`，`命名欄位類元組 tuple-like` 和`類單元unit-like`，它們在引用組件的管道上有所不同：命名欄位結構為每個組件提供名稱，而類元組結構則根據它們的出現順序來識別它們。 單元式結構根本沒有組件； 這些並不常見，但比你想像的更有用。



## Named -Field Structs 

The definition of a named-field struct type looks like this:

```rust
//a rectangle of eight-bit grayscale pixels
struct GrayscaleMap{
  pixels: Vec<u8>,
  size:  (usize,usize)
}
```

This declares a type GrayscaleMap with two fields named pixels and size, of the given types. 

Rust中的約定是，所有類型（包括結構）的名稱都要大寫每個單詞的第一個字母，比如GrayscaleMap，一種稱為CamelCase的約定。 欄位和方法是小寫的，單詞用底線分隔。
這被稱為snake_case

You can construct a value of this type with a struct expression, like this:

> named-field.rs



結構運算式以類型名稱（GrayscaleMap）開頭，並列出名稱和
每個欄位的值，全部用大括弧括起來。 還有填充的簡寫
來自局部變數或具有相同名稱的參數的欄位：



與所有其他項一樣**，structs默認情况下是私有的，僅在模塊中可見，**其中
他們被宣佈了。 您可以通過在結構的定義前加首碼，使其在模塊外可見

`GrayscaleMap` 它的每個欄位也是如此，默認情况下這些欄位也是私有的：

```rust
//a rectangle of eight-bit grayscale pixels
pub struct GrayscaleMap{
    pub pixels: Vec<u8>,
    pub size:  (usize,usize)
}

```



Even if a struct is declared pub, its fields can be private:

```rust
//a rectangle of eight-bit grayscale pixels
pub struct GrayscaleMap{
    pixels: Vec<u8>,
    size:  (usize,usize)
}

```

其他模塊可以使用此結構及其可能具有的任何公共方法，但不能
按名稱訪問專用欄位，或使用結構運算式創建新欄位
GrayscaleMap值。 也就是說，創建結構值需要結構的所有欄位
可見。 這就是為什麼不能編寫結構運算式來創建新的String或
Vec。 這些標準類型是結構，但它們的所有欄位都是私有的。 為了創建一個，
您必須使用像Vec::new（）這樣的公共方法。





創建命名欄位結構值時，可以使用相同的另一個結構
鍵入為您忽略的欄位提供值。 在結構運算式中，如果命名欄位
後面跟著`.. EXPR`則任何未提及的欄位從EXPR取其值，
其必須是相同結構類型的另一個值。 假設我們有一個結構repre‐
在遊戲中扮演怪物：

>  named-field.rs



對於程式師來說，最好的童話故事是《魔法學徒》：一個新手魔術師
附魔掃帚為他工作，但不知道如何在
工作完成了。 用斧頭把掃帚砍成兩半只會產生兩把掃帚
一半的大小，但以與原作相同的盲目奉獻繼續執行任務：



## tuple-like structs



## Unit-Like Structs



## Struct Layout





## Definin Methods with impl



## Generic Structs



## Structs with Lifetime Parameters



## Deriving Common Traits for Struct Types



## Interior Mutability

