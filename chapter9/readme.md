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

第二種結構類型被稱為類元組結構，因為它類似於元組：

```rust
struct Bounds(usize,usize)
```

構造這種類型的值就像構造元組一樣，只是必須包含結構名稱：

```rust
let image_bounds = Bounds(1024,768);
```

在元組結構中的數值可以叫做元素element，就像元組一樣

```rust
assert_eq!(image_bounds.0*image_bounds.1,786432);
```

類元組結構的單個元素可能是公共的，也可能不是：

```rust
pub struct Bounds(pub usize,pub usize);
```

運算式Bounds（1024768）看起來像一個函數調用，事實上它是：定義類型也隱式定義了一個函數：

```rust
fn Bounds(elem0:usize,elem1:usize)->Bounds{...}
```

在最基本的層次上，命名欄位和類元組結構非常相似。 選擇使用哪一個可以歸結為易讀性、歧義性和簡潔性的問題。 如果您將使用。 運算子來獲取值的組成部分，按名稱識別欄位可以為讀者提供更多資訊，而且可能更能防止拼寫錯誤。 如果您通常使用模式匹配來查找元素，那麼類似元組的結構可以很好地工作。



簇狀結構適用於新類型，即具有單個組件的結構，您可以定義該組件以進行更嚴格的類型檢查。 例如，如果您使用的是僅ASCII文字，則可以定義一個新類型，如下所示：

```rust
struct Ascii(Vec<u8>);
```

對ASCII字串使用這種類型比簡單地傳遞`Vec<u8>`緩衝區並在注釋中解釋它們要好得多。 newtype有助於Rust捕捉其他位元組緩衝區傳遞給期望ASCII文字的函數的錯誤。 我們將在第21章中給出一個使用newtype進行有效類型轉換的例子。

## Unit-Like Structs

第三種結構有點晦澀：它聲明了一個根本沒有元素的結構類型：

```rust
struct Onesuch;
```

這種類型的值不佔用記憶體，很像`type()`。 Rust實際上並不需要將類似單元的結構值存儲在記憶體中，也不需要生成程式碼來操作它們，因為它可以僅從值的類型中告訴它可能需要瞭解的關於值的一切。 但從邏輯上講，空結構是一種與其他類型一樣具有值的類型，或者更準確地說，是一種只有一個值的類型：
` let o=Onesuch;`

Unit-like structs can also be useful when working with traits, which we’ll describe in chapter11 





## Struct Layout

在記憶體中，命名欄位和類元組結構都是相同的：可能是混合類型的值的集合，在記憶體中以特定的管道排列。 例如，在本章的前面，我們定義了這個結構：



```rust
struct GrayscaleMap{
  pixels:Vec<u8>,
  size:(usize,usize)
}
```

A GrayscaleMap value is laid out in memory as diagrammed in Figure 9-1.

<img src= "struct_layout.png">



與C和C++不同，Rust沒有具體承諾如何在記憶體中對結構的欄位或元素進行排序； 這張圖只顯示了一種可能的排列方式。 然而，Rust確實承諾將欄位的值直接存儲在結構的區塊中。 JavaScript、Python和Java會將點數和大小值分別放在各自的堆分配塊中，並讓GrayscaleMap的欄位指向它們，而Rust則將點數和尺寸直接嵌入GrayscaleMap值中。 只有點數向量所擁有的堆分配緩衝區保留在其自己的塊中。
您可以要求Rust使用#[repr（C）]内容以與C和C++相容的管道佈局結構。 我們將在第21章中對此進行詳細介紹。



## Definin Methods with impl

在整本書中，我們一直在對各種值調用方法。 我們已經用`v.push（e）`將元素推送到向量上，用`v.len（）`獲取它們的長度，用`r.expect（“msg”）`檢查Result值中的錯誤，等等。
您可以在定義的任何結構類型上定義方法。 Rust方法不是像C++或Java那樣出現在結構定義中，而是出現在一個單獨的impl塊中。 例如

> impl.rs

一个Impl块只是FN定义的集合，每个定义都成为块顶部指定的结构类型上的方法。在这里，我们定义了一个公共结构队列，然后给出了两种公共方法，Push and Pop



方法也称为关联功能，因为它们与Spe -Cific类型相关。关联功能的相反功能是一个自由功能，该功能未定义为Impl块的项目。





在我们的示例中，推动和流行方法将队列字段称为self.olter and self.younger。与C ++和Java不同，其中“此”对象的成员在方法体中直接可见为不合格的标识符，Rust方法必须明确使用自我来指调用其所调用的值，类似于Python方法使用自我的方式，以及JavaScript方法使用此方法的方式。



由于Push and Pop需要修改队列，因此它们都采用和Mut self。但是，当您调用方法时，您不需要自己借用可变的参考。普通的方法调用语法隐含地处理了这一点。因此，有了这些定义，您可以使用这样的队列



> 还是那个文件

简单地编写q.push（...）借用了对Q的可变引用，就好像您已经写了（＆mut q）.push（...），因为那是推动方法的自我所需要的。

如果方法不需要修改其自我，则可以将其定义为进行共享参考。用于审查

like this 

```rust
impl Queue{
    pub fn is_empty(&self)->bool{
        self.older.is_empty() &&self.younger,is_empty()
    }
}
```

Again, the method call expression knows which sort of reference to borrow:

```rust
assert!(q.is_empty());
q.push('☉');
assert!(!q.is_empty())
```



Or, if a method wants to take ownership of self, it can take self by value

```rust
impl Queue {
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}
```



但是请注意，由于拆分凭价值使自己的自我从Q中移出，因此将Q从Q中移出。由于Split的自我现在拥有队列，因此它能够将独立矢量从中移出，然后将它们退回到Caller



You can also define methods that don’t take self as an argument at all. These become
functions associated with the struct type itself, not with any specific value of the type.
Following  the  tradition  established  by  C++  and  Java,  Rust  calls  these  static  methods.
They’re often used to provide constructor functions, like t

```rust
impl Queue{pub fn new()->Queue{
    Queue{older:Vec::new(),younger:Vec::new()}
}
}
// 实例化
fn main(){
    let mut q = Queue::new();
    q.push("*");
}
```

## Generic Structs

我们对队列的早期定义不令人满意：它写成存储字符，但其结构或方法完全没有特定于字符。如果我们要定义另一个符合字符串值的结构，则可以识别代码，除非Char将被字符串替换。那会浪费时间





幸运的是，rust结构可能是通用的，这意味着它们的定义是一个模板，您可以将其插入所喜欢的任何类型。例如，这是队列的定义，可以保留任何键入的值

```rust
pub struct Queue<T>{
    older:Vec<T>,
    younger:Vec<T>
}
```

您可以在队列<t>中读取<t>为“对于任何元素t类型...”。因此，此定义示为“对于任何类型T，队列<t>是Vec <T>类型的两个字段。”例如，在队列<string>中，t是字符串，因此较旧的和更年轻的类型vec <string>。在队列<char>中，T是char，我们得到了一个与我们开始使用的特定特定定义相同的结构。实际上，VEC本身是一个通用结构，在此中定义



在通用结构定义中，<Angle Brackets>中使用的类型名称称为类型参数。通用结构的IMPL块看起来像

> generic_struct.rs

您可以将行读取impl <t>队列<t>读为“对于任何类型T，以下是队列<t>上可用的方法。”然后，您可以将类型参数t用作方法definitio中的类型



我们已经在上述代码中使用了Rust的速记作为自我参数；到处写出队列<t>成为一种口腔和分心。作为另一个速记，无论是否通用，每个Impl块都将特殊类型的参数self（请注意骆驼名）定义为我们正在添加的方法。在前面的202 |第9章：结构代码，自我是队列<t>，因此我们可以缩写队列:: new的定义有点



For static method calls, you can supply the type parameter explicitly using the turbofish `::<> notation`

> generic_struct.rs

In  fact,  this  is  exactly  what  we’ve  been  doing  with  Vec,  another  generic  struct  type,
throughout the book.
It’s not just structs that can be generic. Enums can take type parameters as well, with a
very similar syntax. We’ll show that in detail in “Enums” on page 212





## Structs with Lifetime Parameters

如果结构类型包含参考，则必须命名这些参考文献的寿命。例如，这是一种可能会引用某些最伟大和最少要素的结构

```rust
struct Exterma<'elt>{
    greatest: & 'elt i32,
    least: &'elt i32
}
```

此前，我们邀请您认为诸如结构队列<t>之类的声明是指在任何特定类型T的情况下，您可以制作一个符合该类型的队列<t>。具有生命周期参数的simi structs |203很大程度上，您可以将struct Extrema <'Elt>视为意味着，鉴于任何特定的终生'Elt，您可以制作一个具有该寿命的参考的极端<'Elt>



这是扫描切片并返回极值的功能，其字段是指其元素

```rust
struct Exterma<'elt>{
    greatest: & 'elt i32,
    least: &'elt i32
}

fn find_extrema<'s>(slice : &'s[i32])-> Exterma<'s>{
    let mut greatest = &slice[0];
    let mut least = &slice[0];
    
    for i in 1..slice.len(){
        if slice[i]<*least{least = &slice[i];}
        if slice[i]>*greatest{greatest = &slice[i];}
        
    }
    Exterma{greatest,least}
}
```

在这里，由于find_extrema借用了具有寿命的slice元素，因此我们返回的极端结构也将其用作其参考文献的寿命。Rust总是会进一步呼叫的终身参数，因此find_extrema的呼叫不必提及

```rust
struct Exterma<'elt>{
    greatest: & 'elt i32,
    least: &'elt i32
}

fn find_extrema<'s>(slice : &'s[i32])-> Exterma<'s>{
    let mut greatest = &slice[0];
    let mut least = &slice[0];
    
    for i in 1..slice.len(){
        if slice[i]<*least{least = &slice[i];}
        if slice[i]>*greatest{greatest = &slice[i];}
        
    }
    Exterma{greatest,least}
}

fn main(){
    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48)
}
```



由于返回类型使用与参数相同的寿命是如此普遍，因此Rust可以让我们省略一个明显的候选人的生命。我们也可以写出find_extrema这样的签名，没有任何含义的变化：

```rust


fn find_extrema（slice：＆[i32]） - > Extrema {...}，
```

我们可能意味着极端<'static>，但这很不寻常，但是。Rust为CON CAS提供了速记





## Deriving Common Traits for Struct Types

结构类型很容写

```rust
struct Point{
    x:f64,
    y:f64
}
```

但是，如果您开始使用此点类型，您会很快注意到这有点痛苦。如书面，点不可复制或可包裹。您无法用`println！（“ {：}”，Point）`;并且它不支持==和！=运算符。



这些功能中的每一个都在Rust中名称 - 复制，克隆，调试和Partialeq。它们被称为特征。在第11章中，我们将展示如何手工实现自己的结构。但是，在这些标准特征和其他一些特征的情况下，除非您想要某种自定义行为，否则您无需手动实施它们。生锈可以以机械精度自动为您实施它们。只需将＃[derive]属性添加到Struc

```rust
#[derive(Copy,Clone,Debug,PartiaEq)]
struct Point{
    x:f64,
    y:f64
}
```

只要其每个字段都实现特征，就可以自动实现这些特征。我们可以要求Rust得出PartiaLeq的点



RUST还可以得出部分端口，这将增加对比较操作的支持<，>，<=和> =。我们在这里还没有这样做，因为比较两个点以查看一个“比另一个小”是否实际上是一件很奇怪的事情。没有一个关于要点的常规秩序。因此，我们选择不支持这些操作员的点值。这样的情况是Rust使我们写下＃[derive]属性而不是自动衍生其可能的每个特征的原因之一。另一个原因是，提出特征是自动的公共功能，因此，可复制性，可克隆性等都是您结构公共API的一部分，因此应故意选择。我们将详细描述Rust的标准特征，并确定哪个特征是＃[得出]，在第13章中



## Interior Mutability

可变性就像其他任何事物：过剩，会导致问题，但您通常只想一点。例如，假设您的蜘蛛机器人控制系统具有一个中央结构，其中包含设置和I/O句柄。它是在机器人靴子时设置的，而且价值永远不会改变

> interior_mut.rs

他为网络构造，捕食，毒液流控制等构建，每个人都有一个RC <spiderrobot>智能指针。回想一下RC代表参考计数，并且RC框中的价值总是共享的，因此总是不可变



现在，假设您想使用标准文件类型向Spiderrobot结构添加一点记录。有一个问题：文件必须是mut。所有撰写的方法都需要使用mut参考



这种情况经常出现。我们需要的是在原本不变的值（蜘蛛体结构）内的一些可变数据（文件）。这称为内部可变性。Rust提供了几种口味。在本节中，我们将讨论两种最直接的类型：Cell <T>和Refcell <T>，均在STD :: Cell模块中





单元格是一个包含单个T型的私人值的结构。关于单元格的唯一特殊之处在于即使您无法访问单元格本身，也可以获取并设置字段

- **Cell::new(value)** 创建Cell然后输入值
- **cell.get()** 返回复制的cell中的数值
- **cell.set(value) ** 将给定值存储在单元格中，删除先前存储的值

这种方法将自我作为非穆特参考

```rust
fn set(&self,value:T) // not '&mut self'
```
A Cell would be handy if you were adding a simple counter to your SpiderRobot.
You could write:



这很简单，但并不能解决我们的日志记录问题。Cell不让你
对共享值调用mut方法。.get（）方法返回中的值的副本
因此，只有当T实现了Copy特性时，它才能工作。对于测井，我们需要一个
文件可复制，而文件不可复制


在这种情况下，正确的工具是RefCell。与Cell＜T＞一样，RefCell＜T＞是一种泛型类型
包含一个T类型的值。与Cell不同，RefCell支持借用引用取决于其T值：

- RefCell::new(value) -> creates a new RefCell, moving value into it.
- ref_cell.borrow()   ->returns a Ref<T>, which is essentially just a shared reference to the value stored in ref_cell.
- ref_cell.borrow_mut() ->returns a RefMut<T>, essentially a mutable reference to the value in ref_cell.

这很像正常引用的工作方式。唯一的区别是，正常情况下，
当您借用对变量的引用时，Rust会在编译时进行检查，以确保
你在安全地使用参考资料。如果检查失败，则会出现编译器错误。RefCell
使用运行时检查强制执行相同的规则。所以，如果你违反了规则，你会得到一个
惊恐

无论一个结构是否有命名字段或类似元组，它都是其他值的集合：如果我有一个SpiderSenses结构，那么我就有一个指向共享SpiderRobot结构的Rc指针，我有眼睛，我有加速度计，等等。所以结构的本质是单词“and”：我有X和Y。但如果有另一种类型围绕单词“or”构建呢？也就是说，当你有这样一个类型的值时，你会有X还是Y？事实证明，这些类型非常有用，在Rust中无处不在，它们是下一章的主题。