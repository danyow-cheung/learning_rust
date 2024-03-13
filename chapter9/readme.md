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





## tuple-like structs



## Unit-Like Structs



## Struct Layout





## Definin Methods with impl



## Generic Structs



## Structs with Lifetime Parameters



## Deriving Common Traits for Struct Types



## Interior Mutability

