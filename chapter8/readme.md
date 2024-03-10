# Crates and Modules 

假設你正在編寫一個程式，從單個細胞的水准向上類比蕨類植物的生長。 你的程式，就像蕨類植物一樣，一開始會很簡單，所有的程式碼可能都在一個檔案中——只是一個想法的孢子。 隨著它的成長，它將開始有內部結構。 不同的作品會有不同的用途。 它將分支到多個檔案中。 它可能覆蓋整個目錄樹。 隨著時間的推移，它可能會成為整個軟件生態系統的重要組成部分。

This chapter covers the features of Rust that help keep your program organized: crates and modules.

## Crates 
rust程式是由crate製成的。 每個主機殼都是一個Rust項目：單個庫或可執行文件的所有原始程式碼，以及任何相關的測試、示例、工具、配寘和其他垃圾。


The easiest way to see what crates are and how they work together is to use cargo build with the --verbose flag to build an existing project that has some dependen‐ cies. We did this, using “A Concurrent Mandelbrot Program” on page 35 as our exam‐ ple. The results are shown here:
```
cd mandelbrot 
cargo clean 
cargo build --verbose 
```
為了可讀性，我們重新格式化了rustc命令列，並删除了許多與我們的討論無關的編譯器選項，將其替換為省略號`(...)`
您可能還記得，當我們完成時，Mandelbrot程式的main.rs包含三個外部主機殼聲明：
```
extern crate num;
extern crate image;
extern crate crossbeam;
```
These lines simply tell Rust that num, image, and crossbeam are external libraries, not part of the Mandelbrot program itself.
We also specified in our `Cargo.toml` file which version of each crate we wanted:
```
[dependencies]
num = "0.1.27"
image = "0.6.1"
crossbeam = "0.2.8"
```
一旦獲得了所有的原始程式碼，Cargo就會編譯所有的板條箱。 它為項目依賴關係圖中的每個主機殼運行一次Rust編譯器rustc。 在編譯庫時，Cargo使用--crate typelib選項。 這個命令告訴rustc不要尋找main（）函數，而是生成一個.rlib檔案，該檔案包含一個稍後rustc命令可以用作輸入的表單中的編譯程式碼。 在編譯程式時，Cargo使用--crate類型的bin，結果是tar‐get平臺的二進位可執行文件：例如，Windows上的mandelbrot.exe。

### build profiles

There are several configuration settings you can put in your *Cargo.toml* file that affect the rustc command lines that cargo generates.

| command line          | cargo.toml section used |
| --------------------- | ----------------------- |
| cargo build           | [profile.debug]         |
| cargo build --release | [profile.release]       |
| cargo test            | [profile.test]          |

預設值通常很好，但我們發現的一個例外是，當您想要使用探查器時——一種量測程式CPU時間的工具。 為了從探查器中獲得最佳數據，您需要同時啟用優化（通常僅在發佈版本中啟用）和調試符號（通常只在調試版本中啟用。 要同時啟用這兩種功能，請將其添加到您的Cargo.toml中：

```
[profile.release]
debug = true
```



調試設定將-g選項控制為rustc。 使用此配寘，當您鍵入貨物構建-發佈時，您將獲得一個帶有調試符號的二進位檔案。 優化設置不受影響。


## modules

模塊是Rust的命名空間。 它們是組成Rust程式或庫的函數、類型、常數等的容器。

crate是關於項目之間的程式碼共亯，而模塊是關於項目中的程式碼組織。 它們看起來是這樣的：

```rust
mod spores {
  use cells::Cell;
  
  pub struct Spore{...}
  
  pub fn produce_spore(factory:&mut Sporangium)->Spore{
    ..
  }
 fn recombine(parent:&mut Cell{
   ...
  })
}
```



A module is a collection of *items*, 

 <u>The **pub** keyword makes an item public,</u> so it can be accessed from outside the module. Anything that isn’t marked pub is private.

```rust
let s = spores::produce_spore(&mut factory);
spores::recombine(&mut cell);
```



模塊可以嵌套，通常看到的模塊只是子模塊的集合：

```rust
mod plant_structures{
	pub mod roots{
	..
	}
	pub mod stems{
	...
	}
	pub mod leaves{
	...
	}
}
```

通過這種管道，我們可以寫出一個完整的程式，包含大量的程式碼和整個模塊層次結構，所有這些都在一個原始檔案中。 事實上，這樣工作很痛苦，所以還有其他選擇



### Modules in Separate Files 

A module can also be written like this:

```rust
mod spores;
```

早些時候，我們包括孢子模塊的主體，包裹在捲曲的支架中。 在這裡，我們告訴Rust編譯器孢子模塊位於一個單獨的檔案中，稱為spores.rs：

```rust
//spores.rs

/// a cell made by an adult fern,,,
pub strcut Spore{...}

///simulate the production of a spore by meiosis
pub fn product_spore(factory:&mut Sporangium)->Spore{
  ...
}

///mix genes to prepare for meiosis (part of interphase)
fn recombine(parent:&mut Cell){
  ...
}
```



spores.rs僅包含組成模塊的項目。 它不需要任何形式的模範來聲明它是一個模塊。

程式碼的位置是這個孢子模塊和我們在上一節中顯示的版本之間的唯一區別。 關於什麼是公共的和什麼是私人的規則在任何一種情况下都是完全相同的。 Rust從不單獨編譯模塊，即使它們在單獨的檔案中：當你構建Rust主機殼時，你正在重新編譯它的所有模塊。



模塊可以有自己的目錄。 當Rust看到mod孢子時；， 它檢查`spores.rs`和`spors/mod.rs`； 如果兩個檔案都不存在，或者兩者都存在，那就是錯誤。 對於
在這個例子中，我們使用了spores.rs，因為孢子模塊沒有任何子模塊。 但是考慮一下我們前面寫的plant_structures模塊。 如果我們决定將該模塊及其三個子模塊折開為各自的檔案，則生成的項目將如下所示：

```
fern_sim/
    ├── Cargo.toml
    └── src/
        ├── main.rs
        ├── spores.rs
        └── plant_structures/
            ├── mod.rs
            ├── leaves.rs
            ├── roots.rs
            └── stems.rs

```



在`main.rs`可以定义模块

```rust
pub mod plant_structures;
```

This causes Rust to load *plant_structures/mod.rs*, which declares the three submodules:

```rust
//in plant_structures/mod.rs
pub mod roots;
pub mod stems;
pub mod leaves;
```



The content of those three modules is stored in separate files named *leaves.rs*, *roots.rs*, and *stems.rs*, located alongside *mod.rs* in the plant_structures directory.



### Paths and Imports

`::`運算子用於訪問模塊的功能。 項<u>目中任何位置的程式碼都可以通過寫出其絕對路徑來引用任何標準庫</u>功能：

```rust
if s1>s2{
	::std::mem::swap(&mut s1,&mut s2);
}
```

這個函數名`::std::mem::swap`是一個絕對路徑，因為它以一個雙冒號開頭。 路徑`::std`是名額准庫的頂級模塊。`::std::mem`是標準庫中的一個子模塊，並且`::std::mem::swap`是該模塊中的一個公共函數。



You could write all your code this way, spelling out ::std::f64::consts::PI and ::std::collections::HashMap::new every time you want a circle or a dictio‐ nary, but it would be tedious to type and hard to read. The alternative is to *import* features into the modules where they’re used:

```rust
use std::mem;
if s1>s2{
  mem::swap(&mut s1,&mut s2);
}
```



use聲明使名稱mem成為貫穿封閉塊或模塊的`::std::mem`的本地別名。 使用聲明中的路徑自動為絕對路徑，囙此不需要前。



我們可以使用`std::mem::swap`編寫； 以導入交換函數本身而不是mem模塊。 然而，我們上面所做的通常被認為是最好的風格：導入類型、特徵和模塊（如std::mem），然後使用相對路徑訪問其中的函數、常數和其他成員。



Several names can be imported at once:

```rust
use std::collections::{HashMap,HashSet}; // import path 
use std::io::prelude::*;		//import everything
```



This is just shorthand for writing out all the individual imports:

```rust
use std::collections::HashMap;
use std::collections::HashSet;

//all public items in std::io::prelude
use std::io::prelude::Read;
use std::io::prelude::Write;
use std::io::prelude::BufRead; use std::io::prelude::Seek;
```

模塊不會自動從其父模塊繼承名稱。 例如，假設我們`proteins/mod.rs`中有這樣的內容：

```rust
//proteins/mod.rs
pub enum AminoAcid{...}
pub mod synthesis;
```



Then the code in *synthesis.rs* does not automatically see the type AminoAcid:

```rust
//proteins/synthesis.rs
pub fn synthesize(seq:&[AminoAcid])  //error can't find the type  
```



Instead, each module starts with a blank slate and must import the names it uses:

```
//proteins/synthesis.rs
use super::AminoAcid;
pub fn sysnthesize(seq:&[AminoAcid]) //ok 

```

The keyword super has a special meaning in imports: it’s an alias for the parent module. Similarly, self is an alias for the current module.

```rust
//in proteins/mod.rs 
//import from a submodule
use self::synthesize::synthesize;

//import names from an enum
//so we can write `Lys` for lysine ,rather than AminoAcid::Lys
use self::AminoAcid::*;
```

默認情况下，導入中的路徑被視為絕對路徑，self和super允許您覆蓋該路徑並從相對路徑導入。



（當然，這裡的`AminoAcid`例子與我們早些時候提出的只輸入類型、特徵和模塊的風格規則不同。如果我們的程式包括長胺基酸序列，根據奧威爾的第六條規則，這是合理的：“打破這些規則中的任何一條都比說任何完全野蠻的話都要早。”）
子模塊可以訪問其父模塊中的私有項，但必須按名稱導入每個子模塊。 使用super::*； 只進口標有pub的商品。
模塊與檔案不是一回事，但模塊與Unix檔案系統的檔案和目錄之間有一個自然的相似之處。 use關鍵字創建別名，就像ln命令創建連結一樣。 路徑和檔名一樣，有絕對形式和相對形式。 self和super就像。 和 特殊目錄。 外部主機殼將另一個主機殼的根模塊移植到您的項目中。 這很像安裝檔案系統。







### The standard Prelude 

標準前奏曲

我們剛才說過，就導入的名稱而言，每個模塊都以“白板”開頭。 但這張石板並不是完全空白的。
首先，標準庫std會自動與每個項目連結。 就好像你的lib.rs或main.rs包含了一個不可見的聲明：

```rust
extern crate std;
```

此外，一些特別方便的名稱，如Vec和Result，包含在標準前奏中並自動導入。 Rust的行為就像每個模塊，包括根模塊，都是從以下導入開始的：

```rust
use std::prelude::v1::*;
```



標準序曲包含幾十種常用的特徵和類型。 它不包含std。囙此，如果您的模塊引用std，則必須顯式導入它，如下所示：

```rust
use std;
```

Usually, it makes more sense to import the particular feature of std that you’re using.



### Items ,the Building Blocks of Rust

模塊由項目組成。 有幾種項目，清單實際上是該語言的主要特徵清單：

- function

- types

  用戶定義的類型是使用struct、enum和trait關鍵字引入的。 我們將及時為他們每人奉獻一章； 一個簡單的結構如下所示：

  ```rust
  pub struct Fern{
    pub roots : RootSet,
    pub stems : StemSet
  }
  ```

  結構的欄位，甚至是私有欄位，都可以在聲明該結構的整個模塊中訪問。 在模塊之外，只能訪問公共欄位。
  事實證明，通過模塊而不是像Java或C++那樣通過類來強制執行存取控制，對軟體設計有著驚人的幫助。 它减少了鍋爐板上的“getter”和“setter”方法，並且在很大程度上消除了對C++朋友聲明之類的任何東西的需要。 一個模塊可以定義幾個緊密配合的類型，例如frond::LeafMap和frond：：LeafMapIter，根據需要訪問彼此的私有欄位，同時仍然對程式的其他部分隱藏這些實現細節。

  

- type aliases 

  類型別名

  As we’ve seen, the type keyword can be used like typedef in C++, to declare a new name for an existing type:

  ````rust
  type Table = HashMap<String,Vec<String>>;
  ````

  The type Table that we’re declaring here is shorthand for this particular kind of HashMap.

  ```rust
  fn show(table:&Table){
  	...
  }
  ```

  方法使用impl塊附加到類型：

  ```rust
  impl Cell{
    pub fn distance_from_origin(&self)->f64{
      f64::hypot(self.x,self.y)
  }
  }
  ```

  語法在第9章中解釋。 impl塊不能標記為pub。 相反，單獨的方法被標記為pub，以使它們在當前模塊之外可見。
  私有方法，如私有結構欄位，在聲明它們的整個模塊中都是可見的。

- Constants 变量

  const關鍵字引入了一個常數。 語法和let一樣，只是它可能被標記為pub，並且類型是必需的。 此外，UPPERCASE_NAME是常數的常規用法：

  ```rust
  pub const ROOM_TEMPERATURE:f64 = 20.0;//degrees celsius
  ```

  static關鍵字引入了一個靜態項，這幾乎是同一件事：

  ```rust
  pub static ROOM_TEMPERATURE:f64 = 68.0;//drgrees fahrenheit
  ```

  常數有點像C++定義：值在使用的每個地方都會編譯到程式碼中。 靜態是在程式開始運行之前設定的變數，並一直持續到程式退出。 在程式碼中為幻數和字串使用常數。 對大量數據使用statics，或者在需要借用常數值引用的任何時候使用statics。

  沒有mut常數。 Statics可以標記為mut，但如第5章所述，Rust無法強制執行其關於互斥訪問mut Statics的規則。 囙此，它們本質上是非執行緒安全的，安全程式碼根本無法使用它們：

  ````rust
  static mut PACKETS_SERVED:usize=0;
  println!("{} served", PACKETS_SERVED); // error: use of mutable static
  ````

  

- 模块

  a module can contain submodules, which can be public or private, like any other named item.

- 导入Imports

  use and extern crate declarations are items too. Even though they’re just aliases, they can be public:

  ```rust
  // in plant_structures/mod.rs
  pub use self::leaves::Leaf;
  pub use self::roots::Root;
  ```

  這意味著葉子和根是植物結構模型的公共項目。 它們仍然是`plant_structures:：Leaf::Leaf`和`plant_structures:：roots:：Root`的簡單別名。
  標準的序曲就是這樣一系列的`pub imports`作品。

  

- extern blocks 

  These declare a collection of functions written in some other language (typically C or C++), so that your Rust code can call them. We’ll cover extern blocks in Chapter 21.

  

## Turning A Program into a Library

當你的`fern simulator`開始起飛時，你决定你需要的不僅僅是一個程式。 假設您有一個命令列程式來運行類比並將結果保存在檔案中。 現在，您需要編寫其他程式來對保存的結果進行科學分析，實时顯示正在生長的植物的3D渲染圖，渲染逼真的圖片等等。所有這些程式都需要共亯基本的蕨類植物類比程式碼。 你需要建一個`library`



第一步是將現有項目分為兩部分：一個庫主機殼，其中包含所有共用代碼；另一個可執行文件，其中包含現有命令列程式所需的程式碼。
為了展示如何做到這一點，讓我們使用一個簡單的示例程式：

> folder fern_smi

把這個程式變成一個庫很容易。 以下是步驟：
1.將檔案src/main.rs重命名為src/lib.rs。
2.將pub關鍵字添加到src/lib.rs中的項目中，這些項目將成為我們庫的公共功能。
3.將主函數移到某個暫存檔案中。 我們一會兒再談。

主函数

```rust
fn main(){
    let mut fern = Fern{
        size:1.0,
        growth_rate:0.001
    };
    run_simulation(&mut fern,1000);
    println!("final fern size: {}", fern.size);
}
```

請注意，我們不需要更改Cargo.toml中的任何內容。 這是因為我們的小型Cargo.toml檔案將Cargo保留為默認行為。 默認情况下，貨物構建會查看源目錄中的檔案，並確定要構建的內容。 當它看到src/lib.rs檔案時，它就知道要構建一個庫。
src/lib.rs中的程式碼構成了庫的根模塊。 使用我們庫的其他板條箱只能訪問此根模塊的公共項。



## The src/bin Directory

讓原始的命令列fern_sim程式重新工作也是直接的：Cargo對與庫位於同一代碼庫中的小程式有一些內寘支持。
事實上，Cargo本身就是這樣寫的。 大部分程式碼都在Rust庫中。 我們在整本書中一直使用的貨物命令列程式是一個薄包裝程式，它調用庫來完成所有繁重的工作。 庫和命令列程式都位於同一個源存儲庫中。
我們也可以將程式和庫放在同一個代碼庫中。 將此程式碼放入名為src/bin/efern.rs的檔案中：



因為我們已經將這個檔放入src/bin中，所以Cargo將在下次運行貨物構建時編譯fern_sim庫和這個程式。 我們可以使用cargo run--bin efern來運行efern程式。 下麵是它的樣子，使用--verbose來顯示Cargo正在運行的命令：

```
cargo build --verbose 
cargo run --bin efern --verbose
```



當然，既然fern_sim是一個庫，我們還有另一個選擇。 我們可以把這個程式放在它自己的獨立項目中，放在一個完全獨立的目錄中，它自己的Cargo.toml將fern_sim列為依賴項：

```
[dependencies]
fern_sim=｛path=“../fern_sim”｝
```


也許這就是你將來為其他蕨類植物類比程式所做的。 src/bin目錄正好適合efern這樣的簡單程式。



## Atttibutes 属性

Rust程式中的任何項目都可以使用内容進行裝潢。 内容是Rust的包羅萬象的語法，用於向編譯器編寫各種指令和建議。 例如，假設您收到以下警告：

```
  libgit2.rs: warning: type `git_revspec` should have a camel case name
        such as `GitRevspec`, #[warn(non_camel_case_types)] on by default

```



但你選擇這個名稱是有原因的，你希望Rust對此閉嘴。你可以通過在類型上添加#[allow]内容來禁用警告：

```rust
#[allow(non_camel_case_types)]
pub struct git_revspec{
...
}
```

條件編譯是使用#[cfg]内容編寫的另一個功能：

```rust
// Only include this module in the project if we're building for Android.
#[cfg(target_os = "android")] 
mod mobile;
```

#[cfg]的完整語法在Rust Reference中指定； 此處列出了最常用的選項：

| #[cfg(...)] option          | Enabled when                                                 |
| --------------------------- | ------------------------------------------------------------ |
| test                        | tests are enabled (compilling with `cargo test` or `rustc --test`) |
| debug_assertions            | Debug assertions are enabled (typically in nonoptimized builds) |
| unix                        | compiling for unix including macOS                           |
| windows                     |                                                              |
| target_pointer_width = "64" |                                                              |
| target_arch = "x86_64"      |                                                              |
| feature = "robots"          | The user-defined feature named"robots"is enabled (compiling withcargo build --feature robots or rustc --cfg feature='"robots"'). Features are declared in the[features]section of *Cargo.toml*. |

偶爾，我們需要對函數的內聯擴展進行微觀管理，這是一種我們通常樂於留給編譯器的優化。 我們可以使用#[inline]内容：

```rust
//adjust levels of ions etc in two adjacent cells
//due to osmosis between them 
#[inline]
fn do_osmosis(c1:&mut Cell,c2:&mut Cell){
  ...
}
```

有一種情况，如果沒有#[inline]，內聯就不會發生。 當在一個主機殼中定義的函數或方法被調用到另一個主機殼時，Rust不會內聯它，除非它是泛型的（它有類型參數）或明確標記為#[inline]。
否則，編譯器將#[inline]視為一個建議。 Rust還支持更堅持的#[inline（always）]，請求在每個調用網站內聯擴展函數，以及#[inlin（never）]，要求永遠不要內聯函數。
一些内容，如#[cfg]和#[allow]，可以附加到整個模塊並應用於其中的所有內容。其他内容，如#[test]和#[inline]，必須附加到單個項。 正如您對catch-all特性所期望的那樣，每個内容都是自定義的，並且有自己的一組受支持的參數。 Rust Reference詳細記錄了一整套受支持的内容。
要將内容附加到整個主機殼，請將其添加到main.rs或lib.rs檔案的頂部，在任何項目之前，並寫入#！ 相反

```rust
//libgit2_sys/lib.rs
#![allow(non_camel_case_types)]
pub struct git_revspec{
  ...
}
pub struct git_error{
  ...
}
```

這個 `#!`告訴Rust將一個内容附加到封閉項，而不是下一個：在這種情況下，`#！ [allow]`内容附加到整個`libgit2_sys crate `，而不僅僅是`struct git_revspec`。



`#！ `也可以在函數、結構等內部使用，但它通常只用於檔案的開頭，將内容附加到整個模塊或主機殼crate。 某些内容總是使用`#!` 語法，因為它們只能應用於整個主機殼crate。



例如，`#！ [feature]`内容用於打開Rust語言和庫的不穩定功能，這些功能是實驗性的，囙此可能存在錯誤，或者可能在未來更改或删除。 例如，在我們寫這篇文章的時候，Rust對128比特整數類型i128和u128有實驗性的支持； 但由於這些類型是實驗性的，您只能通過（1）安裝Rust的Nightly版本和（2）明確聲明您的板條箱使用它們來使用它們：

```rust
#![feature(i128_type)]
fn main(){
  // Do my math homework, Rust!
  println!("{}", 9204093811595833589_u128 * 19973810893143440503_u128);
}
```

隨著時間的推移，Rust團隊有時會穩定一個實驗特性，使其成為語言的標準部分。 這個 [feature]内容就會變得多餘，Rust會生成一條警告，建議您删除它。



## Tests and Documentation

測試是用#[test]内容標記的普通函數。

```rust
#[test]
fn math_works(){
  let x:i32 = 1;
  assert!(x.is_positive());
  assert_eq!(x+1,2);
}

```

cargo test runs all the tests in your project.

```
cargo test 
```

無論你的板條箱是可執行文件還是庫，這都是一樣的。 您可以通過將參數傳遞給Cargo來運行特定的測試：Cargo test math運行所有以其名義包含數學的測試。





## Specifying Dependencies

## Publishing Crates to crates.io

