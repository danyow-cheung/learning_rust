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

要导入当前模块中声明的枚举的构造函数，请使用自导入：

```rust
enum Pet{
    Orca,
    Giraffe,
}
use self::Pet::*;
```
在内存中，C样式枚举的值存储为整数。有时，告诉Rust要使用哪些整数是有用的：
```rust
enum HttpStatus{
    Ok = 200,
    NotModified =304,
    NotFound = 404,
    ...
}
```
否则，Rust将为您分配数字，从0开始。默认情况下，Rust使用可以容纳C样式枚举的最小内置整数类型来存储它们。大多数适合单个字节。
```rust
use std::mem::size_of;
assert_eq!(sizeof::<Ordering>(),1);
assert_eq!(sizeof::<HttpStatus>(),2); // 404 doesn't fit in a u8
```
您可以通过向枚举添加#[repr]属性来覆盖Rust对内存中表示的选择。有关详细信息，请参阅第21章。
允许将C样式枚举强制转换为整数：
```rust
assert_eq!(HttpStatus::Ok as i32 ,200);
```
但是，从整数到枚举的另一个方向的强制转换不是。与C和C++不同，Rust保证枚举值只是枚举声明中拼写的值之一。从整数类型到枚举类型的未经检查的强制转换可能会破坏这种保证，因此这是不允许的。您可以编写自己的已检查转换：
```rust
fn http_status_from_u32(n:u32)->Option<HttpStatus>{
    match n {
        200=>Some(HttpStatus::Ok),
        304=>Some(HttpStatus::NotModified),
        404=>Some(HttpStatus::NotFound),
        ...
        _=>None
    }
}
```
或者使用enum_primitive板条箱。它包含一个为您自动生成这种转换代码的宏。
与structs一样，编译器将为您实现==运算符等功能，但您必须提出要求。
```rust
#[derive(Copy,Clone,Debug,PartiaEq)]
enum TimeUnit{
    Seconds,Minutes,Hours,Days,Months,Years
}
```
枚举可以有方法，就像结构一样：
```rust
impl TimeUnit{
    //return the plural noun for this time unit 
    fn plural(self)->&'static str{
        match self{
            TimeUnit::Seconds=>"seconds",
            TimeUnit::Minutes=>"minutes",
            TimeUnit::Hours=>"hours",
            TimeUnit::Days => "days", 
            TimeUnit::Months => "months", 
            TimeUnit::Years => "years"
        }
    }
    fn singular(self)->&'static str{
        self.plural().trim_right_matches("s")
    }
}
```
So much for C-style enums. The more interesting sort of Rust enum is the kind that contains data.

## Enums with Data 



