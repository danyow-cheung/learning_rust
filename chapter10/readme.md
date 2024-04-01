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

### Enums with Data 
有些程序总是需要显示精确到毫秒的完整日期和时间，但对于大多数应用程序来说，使用粗略的近似值更方便用户，比如“两个月前”。我们可以编写一个枚举来帮助实现这一点：
```rust
// a timestamp that has been deliberately rounded off ,so our program 
// says '6 months ago' we can write an enum to help with that 
#[derive(Copy,Clone,Debug,PartialEq)]
enum RoughTime{
    InThePast(TimeUnit,u32),
    JustNow,
    InTheFuture(TimeUnit,u32)
}
```
此枚举中的两个变体InThePast和InTheFuture接受了参数。
这些被称为元组变体。与元组结构一样，这些构造函数也是创建新的RoughTime值的函数。
```rust
let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years,40*20+7);
let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours,3);
```
枚举也可以有结构变体，它包含命名字段，就像普通的结构一样：
```rust
enum Shape{
    Sphere{center:Point3d,radius:f32},
    Cuboid{corner1:Point3d,corner2:Point3d}
}
let unit_sphere = Shape::Sphere{center:ORIGIN,radius:1.0};
```
总之，Rust有三种枚举变体，与我们在前一章中展示的三种结构相呼应。没有数据的变体对应于类似单元的结构。元组变体的外观和功能与元组结构一样。结构变体具有大括号和命名字段。单个枚举可以具有所有三种类型的变体。
```rust
enum RelationshipStatus{
    Single,
    InARelationship,
    ItsComplicated(Option<String>),
    ItsExtremelyComplicated {
        car: DifferentialEquation,
        cdr: EarlyModernistPoem
    }
}
```
公共枚举的所有构造函数和字段都是自动公共的。

### Enums in Memory
在内存中，带有数据的枚举存储为一个小整数标记，再加上足够的内存来容纳最大变量的所有字段。标记字段供Rust内部使用。它告诉是哪个构造函数创建了值，因此它有哪些字段。


### Rich Data Structures Using Enums 
枚举对于快速实现树状数据结构也很有用。例如，假设Rust程序需要处理任意JSON数据。在内存中，任何JSON文档都可以表示为这种Rust类型的值：
> chapter10\json.rs

这不是一个假设的例子。在serde_json中可以找到一个非常相似的枚举，这是一个Rust structs的序列化库，是crates.io上下载量最大的板条箱之一。

表示Object的HashMap周围的Box只用于使所有Json值更加紧凑。在内存中，Json类型的值占用四个机器字。String和Vec值是三个单词，Rust添加了一个标记字节。Null和布尔值

值中没有足够的数据来占用所有空间，但所有Json值的大小必须相同。多余的空间没有用。图10-2显示了Json值在内存中的实际外观示例。
HashMap更大。如果我们必须在每个Json值中为其留出空间，它们将非常大，大约有八个字。但是Box＜HashMap＞是一个字：它只是一个指向堆分配数据的指针。我们可以通过拳击更多的场地使Json更加紧凑。

### Generic Enums 
枚举可以是通用的。标准库中的两个示例是该语言中使用最多的数据类型：
```rust
enum Option<T>{
    None,
    Some(T)
}

enum Result<T,E>{
    Ok(T),
    Err(E)
}
```
到目前为止，这些类型已经足够熟悉了，泛型枚举的语法与泛型结构的语法相同。一个不明显的细节是，当类型T是Box或其他智能指针类型时，Rust可以消除Option＜T＞的标记字段。
Option＜Box＜i32＞作为单个机器字存储在内存中，0表示None，非零表示Some装箱值。通用数据结构只需几行代码即可构建：
```rust
// a ordered collection of `T`s 
enum BinaryTree<T>{
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

// a part of a binary tree 
struct TreeNode<T>{
    element: T,
    left:BinaryTree<T>,
    right:BinaryTree<T>
}
```

这几行代码定义了一个BinaryTree类型，该类型可以存储任意数量的T类型的值。





A sketch of a value of type` BinaryTree<&str> `is shown in Figure 10-3. As with `Option<Box<T>>`, Rust eliminates the tag field, so a `BinaryTree` value is just one machine word.



<img src= 'binary_tree.jpg'>

Building any particular node in this tree is straightforward:

> binary_tree.rs

现在是我们在引言中提到的“价格”。枚举的标记字段占用的内存很少，最坏的情况下最多可达8个字节，但这通常可以忽略不计。

enums的真正缺点（如果可以这样调用的话）是Rust代码不能把谨慎抛到九霄云外，也不能试图访问字段，无论它们是否真的存在于值中：let r=shape.radius；//错误：类型“Shape”上没有字段“radius”访问枚举中数据的唯一方法是安全的方法：使用模式。



## Patterns

Recall the definition of our RoughTime type from earlier in this chapter:

```rust
enum RoughTime{
    InThePast(TimeUnit,u32),
    JustNow,
    InTheFuture(TimeUnit,u32)
}
```

假设您有一个RoughTime值，并且希望将其显示在网页上。你
需要访问值中的TimeUnit和u32字段。Rust不允许您访问
直接写入rough_time.0和rough_time.1，因为毕竟，值可能是RoughTime:：JustNow，它没有字段。

使用`match`表达式

> patterns.rs



枚举、结构或元组上的模式匹配就像Rust进行简单的从左到右扫描一样，检查模式的每个组件，看看值是否匹配。如果不匹配，Rust将转到下一个模式。



当一个模式包含简单的标识符（如单位和计数）时，这些标识符将成为遵循该模式的代码中的局部变量。值中存在的任何内容都会被复制或移动到新的变量中。Rust以单位存储TimeUnit:：Months，以计数存储1，运行第8行，并返回字符串“1 Months from now”。

该输出有一个小的语法问题，可以通过在匹配中添加另一个手臂来解决：



### Literals ,Variables and Wildcards in Patterns 

不幸的是，即使使用了新代码，`RoughTime:：InTheFuture（TimeUnit:：Hours，1）`仍然存在问题：结果“一小时后”并不完全正确。这就是英语。这也可以通过在比赛中增加另一只手臂来解决。
如本例所示，模式匹配与枚举协同工作，甚至可以测试它们所包含的数据，使匹配成为C的switch语句的强大而灵活的替代品。

到目前为止，我们只看到了与枚举值匹配的模式。还有更多。rust模式是它们自己的小语言，总结在表10-1中。我们将在本章剩余的大部分时间里介绍此表中显示的功能。



```rust
match meadow.count_rabbits(){
    0=>{} // nothing to say 
    1=>println!("A rabbit is nosing around the clover"),
    n=>println!("There are {}rabbits hopping about in the meadow".n)
}
```





如果草地上没有兔子，则模式0匹配。1匹配（如果只有一个）。如果有两只或两只以上的兔子，我们到达第三种模式，n。这个模式只是一个变量名。它可以匹配任何值，并移动或复制匹配的值
转换为新的局部变量。因此，在这种情况下，meadow.count_rabbits（）的值存储在一个新的局部变量n中，然后我们打印它

Other literals can be used as patterns too, including Booleans, characters, and even strings:

```rust
let calendar = match settings.get_string("calendar"){
    "gregorian"=>Calendar::Gregorian,
    "chinese"=>Calendar::Chinese,
    "ethiopian"=>Calendar::Ethiopian.
    other=>return parse_error("calendar",other)
}
```



在这个例子中，other充当一个catch-all模式，就像前面例子中的n一样。这些模式与switch语句中的默认情况起着相同的作用，匹配与任何其他模式都不匹配的值。

如果您需要一个catch-all模式，但不关心匹配的值，则可以使用单个下划线_作为模式，即通配符模式：

```rust
let caption=match photo.tagged_pet(){
    Pet::Tyrannosaur=>"RRRRRRRRRAAAAAAAAAAHHHHHHHHHHH",
    Pet::Samoyed=> "*dog thoughts",
    _=>"i am cute,love me "// generic caption works for any pet 
};
```



通配符模式匹配任何值，但不将其存储在任何位置。由于Rust需要每个匹配表达式来处理所有可能的值，因此通常需要在末尾使用通配符。即使你非常确定剩余的病例不会发生，你也必须至少添加一个恐慌的后备措施`panic`：

```rust
// there are many shapes ,but we only support 'selecting' 
// either some text,or everything in a rectangle area
// you cam't select an sllipse or trapezoid 
match document.selection(){
    Shape::TextSpan(start,end)=>paint_text_selection(start,end),
    Shape::Rectangle(rect)=> paint_rect_selection(rect),
    _=>panic!("unexpected selection type ")
}
```

值得注意的是，现有的变量不能在模式中使用。假设我们正在实现一个具有六边形空间的棋盘游戏，玩家只需点击即可移动一块。要确认单击有效，我们可以尝试以下操作：

```rust
fn check_move(current_hex:Hex,click:Point)=>game::Result<Hex>{
    match point_to_hex(click){
        None=> Err("That's not a game space"),
        Some(current_hex)=> 
    }
}
```

