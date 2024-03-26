# Enums and Patterns 
本章的第一个主题很有影响力，就像山丘一样古老，很乐意帮助你在短时间内（以一定的价格）完成很多事情，在许多文化中都有很多名字。但这不是魔鬼。它是一种用户定义的数据类型，长期以来被ML和Haskell黑客称为求和类型、区分并集或代数数据类型。在Rust中，它们被称为枚举，或者简称为枚举。与魔鬼不同的是，它们非常安全，而且它们所要求的价格也不算太低。


C++ and C# have enums; you can use them to define your own type whose values are a set of named constants.

This kind of enum works in Rust, too. But Rust takes enums much further. A Rust enum can also contain data, even data of varying types. 


和c不同的是，rust里面的枚举返回的是安全类型的

只要一个值可能是这样或那样，枚举就很有用。使用它们的“代价”是必须使用模式匹配安全地访问数据，这是我们本章后半部分的主题。
如果您在Python中使用过解包或在JavaScript中使用过析构函数，那么模式可能也很熟悉，但Rust对模式的理解更深入。Rust模式有点像所有数据的正则表达式。它们用于测试一个值是否具有特定的所需形状。它们可以同时将结构或元组中的多个字段提取到局部变量中。和正则表达式一样，它们也很简洁，通常都是在一行代码中完成的。

## Enums 
Simple, C-style enums are straightforward:
```c
enum Ordering{
    Less,
    Equal,
    Greater
}
```
这声明了一个类型Ordering，它有三个可能的值，称为变体或构造函数：`Ordering:：Less、Orderer:：Equal`和`Ordering::Greater`。这个特定的枚举是标准库的一部分，因此Rust代码可以单独导入它：
```rust
use std::cmp::Ordering;

fn compare(n:i32,m:i32)->Ordering{
    if n<m{
        Ordering::Less
    }elise if n>m{
        Ordering::Greater
    }else{
        Ordering::Equal
    }
}
```
或者用这样的constructors
```rust
use std::cmp::Ordering;
use std::cmp::Ordering*; //* to import all children 

fn compare(n:i32,m:i32){
    if n<m{
        Less
    }else if n>m{
        Greater
    }else{
        Equal
    }
}
```
导入构造函数后，我们可以编写Less而不是Ordering:：Less，等等，但因为这不太明确，所以通常认为不导入它们是更好的风格，除非它使代码更可读
