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



測試通常使用斷言！ 和assert_eq！ Rust標準庫中的宏。 明確肯定 如果expr為true，則（expr）成功。 否則，它會驚慌失措，從而導致測試失敗。 assert_eq！ （v1，v2）就像assert！ （v1＝＝v2），但如果斷言失敗，則錯誤消息顯示這兩個值。



您可以在普通程式碼中使用這些宏來檢查不變數，但請注意assert！ 和assert_eq！ 甚至包含在發佈版本中。 使用debug_assert！ 和debug_assert_eq！ 而是編寫僅在調試構建中檢查的斷言。
要測試錯誤案例，請將#[should_panic]内容添加到測試中：

```rust
//this test passes only if division by zero causes a panic 
//as we claimed in the previous chapter
#[test]
#[should_panic(expected="divide by zero")]
fn test_divide_by_zero_error(){
  1/0;//should panic
}
```

標記為#[test]的函數是有條件編譯的。 當您運行貨物測試時，cargo會在啟用測試和測試工具的情况下構建程式的副本。 普通貨物構建或貨物構建-發佈跳過測試程式碼。 這意味著您的單元測試可以與它們測試的程式碼一起使用，如果需要，可以訪問內部實現的詳細資訊，但不需要運行時成本。 然而，這可能會導致一些警告。 例如



囙此，當您的測試內容豐富到需要支持程式碼時，慣例是將它們放在一個測試模塊中，並僅使用#[cfg]内容聲明整個模塊進行測試：

```rust
#[cfg(test)]
mod tests{
  fn rougly_equal(a:f64,b:f64)-> bool{
    (a-b).abs()<1e-6
  }
  #[test]
  fn trig_works(){
    use std::f64::consts::PI;
    assert!(rougly_equal(PI.sin(),0.0));
  }
}
```



Rust的測試工具使用多個線程一次運行多個測試，默認情况下，Rust程式碼的一個很好的方面是執行緒安全的。 （要禁用此功能，請運行單個測試，貨物測試testname；或者將環境變數RUST_test_THREADS設定為1。）

### Integration Tests

你的`fern simulator`器繼續生長。 您已决定將所有主要功能放入一個可供多個可執行文件使用的庫中。 如果能像最終用戶那樣使用fern_sim.rlib作為外部主機殼，讓一些測試與庫連結，那就太好了。 此外，有些測試是從二進位檔案加載保存的類比開始的，在src目錄中有這些大的測試檔案會很尷尬。 集成測試有助於解决這兩個問題。





集成測試是`.rs file`，位於項目src目錄旁邊的測試目錄中。 當您運行貨物測試時，cargo將每個集成測試編譯為一個單獨的、獨立的板條箱，與您的庫和Rust測試工具連結。 以下是一個示例：

> tests/unfurl.rs

請注意，集成測試包括一個extern crate聲明，因為它使用fern_sim作為庫。 集成測試的要點是，他們從外部看到您的主機殼，就像用戶一樣。 他們測試板條箱的公共API。
貨物測試運行單元測試和集成測試。 要僅在特定檔案中運行集成測試，例如`tests/unfold.rs--use`命令`cargo test--test unroll`。



### Documentation

命令cargo doc為您的庫創建HTML檔案：

```
cargo doc --no-deps --open
```

--no-deps選項告訴Cargo只為fern_sim本身生成檔案，而不是為它所依賴的所有板條箱生成檔案。
--open選項告訴Cargo以後在瀏覽器中打開檔案。



檔案是根據庫的pub功能生成的，再加上您附加到它們上的任何檔案注釋。 我們已經在本章中看到了一些檔案評論。 它們看起來像是評論：

```rust
pub fn produce_spore(factory:&mut Sporangium)->Spore{
  ...
}
```

但是，當Rust看到以三個斜杠開頭的注釋時，它會將其視為#[doc]内容。 Rust對待前面的例子與此完全相同：

```rust
#[doc="Simulate the production of a spore by meiosis"]
pub fn produce_spore(factory:&mut Sporangium)->Spore{
  ...
}
```



編譯或測試庫時，將忽略這些内容。 當您生成檔案時，檔案對公共特性的注釋會包含在輸出中。
同樣，注釋以//開頭！ 被視為#！ [doc]内容，並附加到封裝特性，通常是模塊或主機殼。 例如，fern_sim/src/lib.rs檔案的開頭可能如下：

`//!simulate the growth of ferns from the level of `

`//!individual cells on up`

檔案注釋的內容被視為Markdown，這是一種簡單HTML格式的簡寫符號。 星號用於*italics*和**bold類型**，空行被視為段落分隔符號，等等。但是，您也可以使用HTML； 檔案注釋中的任何HTML標記都會逐字逐句地複製到檔案中。



You can use `backticks` to set off bits of code in the middle of running text. In the output, these snippets will be formatted in a fixed-width font. Larger code samples can be added by indenting four spaces.

```rust
    /// A block of code in a doc comment:
    ///
    ///     if everything().works() {
    ///					println!("ok");
    ///		}
```

You can also use Markdown fenced code blocks. This has exactly the same effect.

```
    /// Another snippet, the same code, but written differently:
    ///
    /// ```
    /// if everything().works() {
    ///     println!("ok");
    /// }
    /// 
```

Whichever format you use, an interesting thing happens when you include a block of code in a doc comment. Rust automatically turns it into a test.



### Doc-Tests

當您在Rust庫主機殼中運行測試時，Rust會檢查檔案中出現的所有程式碼是否都能正常運行。 它通過獲取檔案注釋中出現的每個程式碼塊，將其編譯為一個單獨的可執行主機殼，將其與庫連結並運行來實現這一點。



這裡是一個檔案測試的獨立示例。 通過運行`cargo new range`創建一個新項目，並將此程式碼放在ranges/src/lib.rs中：



使用`cargo doc`就可以看到文件

再`cargo test`就可以拿到測試結果

```
dyMBP:ranges danyow$ cargo test
   Compiling ranges v0.1.0 (/Users/danyow/Desktop/self-learning/code/chapter8/ranges)
    Finished test [unoptimized + debuginfo] target(s) in 0.70s
     Running unittests src/lib.rs (target/debug/deps/ranges-ec0128393d3d8a72)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/ranges-c38fa540cf0104cd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ranges

running 1 test
test src/lib.rs - overlap (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.65s
```

如果您將`--verbose`標誌傳遞給Cargo，您將看到它正在使用`rustdoc--test`來運行這兩個測試。 Rustdoc將每個程式碼樣本存儲在一個單獨的檔案中，添加幾行模範程式碼，以生成兩個程式。 以下是第一個：





如果程式碼甚至不需要編譯，請使用ignore而不是no_run。 如果程式碼塊根本不是Rust程式碼，請使用語言名稱，如c++或sh，或者使用text表示純文字。 rustdoc不知道數百種程式設計語言的名稱； 相反，它將不識別的任何注釋視為表明程式碼塊不是Rust。 這將禁用程式碼高亮顯示和檔案測試。



## Specifying Dependencies

We’ve seen one way of telling Cargo where to get source code for crates your project depends on: by version number.

```
    image = "0.6.1"
```

有幾種方法可以指定依賴關係，對於使用哪個版本，你可能想說一些相當微妙的事情，所以花幾頁時間在這方面是值得的。



首先，您可能想要使用根本沒有在crates.io上發佈的依賴項。 一種方法是指定Git存儲庫URL和修訂版：

```
 image = { git = "https://github.com/Piston/image.git", rev = "528f19c" }
```

這個特定的板條箱是開源的，託管在GitHub上，但你也可以很容易地指向託管在公司網絡上的私人Git存儲庫。如圖所示，你可以指定要使用的特定rev、標記或分支。 （這些都是告訴Git要檢查原始程式碼的哪個版本的方法。）



另一種選擇是指定一個包含主機殼原始程式碼的目錄：

```
    image = { path = "vendor/image" }
```

當您的團隊有一個單獨的版本控制存儲庫，其中包含幾個板條箱或整個依賴關係圖的原始程式碼時，這很方便。 每個主機殼都可以使用相對路徑指定其依賴項。





對依賴項進行這種級別的控制是非常强大的。 如果你决定你使用的任何開源板條箱都不完全符合你的喜好，你可以簡單地分叉：只需點擊GitHub上的分叉按鈕，然後更改你的Cargo.toml檔案中的一行。 你的下一個貨物構建將無縫地使用你的叉箱，而不是官方版本。



### Versions

當您在Cargo.toml檔案中寫入類似image=“0.6.1”的內容時，Cargo對此的解釋相當鬆散。 它使用了最新版本的映射，該映射被認為與版本0.6.1相容。



相容性規則改編自語義版本控制。
•以0.0開頭的版本號是如此原始，以至於Cargo從不認為它是
與任何其他版本相容。
•以0.x開頭的版本號（其中x為非零）被認為與0.x系列中的其他點發行版本相容。 我們指定了影像版本0.6.1，但如果可用，Cargo將使用0.6.3。 （這不是語義版本控制標準所說的關於0.x版本號的內容，但事實證明，該規則太有用了，不容忽視。）
•一旦項目達到1.0，只有新的主要版本會破壞相容性。 囙此，如果您要求2.0.1版本，Cargo可能會使用2.17.99，但不會使用3.0。



默認情况下，版本號是靈活的，因為否則使用哪個版本的問題會很快變得過於緊張。 假設一個庫libA使用了num=“0.1.31”，而另一個庫libB使用了num=“0.1.29”。 如果版本號需要完全匹配，則沒有任何項目能够同時使用這兩個庫。 允許Cargo使用任何相容版本是一個更實用的默認設置。
儘管如此，不同的項目在依賴性和遷移方面有不同的需求。 您可以使用運算子指定確切的版本或版本範圍：





您偶爾會看到的另一個版本規範是萬用字元*。 這條消息告訴Cargo，任何版本都可以。除非其他Cargo.toml檔案包含更具體的限制，否則Cargo將使用最新的可用版本。 doc.crates.io上的貨物檔案更詳細地涵蓋了版本規範。



請注意，相容性規則意味著不能純粹出於行銷原因選擇版本號。 它們實際上是有意義的。 它們是板條箱維護者和用戶之間的契约。 如果您維護的主機殼版本為1.7，並且您决定删除某個功能或進行任何其他不完全向後相容的更改，則必須將版本號提高到2.0。 如果你把它稱為1.8，你會
聲稱新版本與1.7相容，您的用戶可能會發現自己的構建已損壞。



### Cargo.lock

Cargo.toml中的版本號非常靈活，但我們不希望Cargo每次構建時都將我們陞級到最新的庫版本。 想像一下，在緊張的調試過程中，突然貨物構建將您陞級到新版本的庫。 這可能會造成難以置信的破壞。 調試過程中發生的任何更改都是不好的。 事實上，當涉及到庫時，從來沒有進行意外更改的好時機。



囙此，貨物有一個內寘的機制來防止這種情況發生。 第一次構建項目時，Cargo會輸出一個Cargo.lock檔案，記錄它使用的每個板條箱的確切版本。 以後的版本將查閱此檔案並繼續使用相同的版本。 只有當您告訴Cargo時，Cargo才會陞級到新版本，可以手動在Cargo.toml檔案中新增版本號，也可以運行Cargo更新：

```
cargo update
```

貨物更新僅陞級到與您在`cargo.toml`中指定的內容相容的最新版本。 如果您指定了image=“0.6.1”，並且希望陞級到0.10.0版本，則必須在Cargo.toml中進行更改。 下次構建時，Cargo將更新到圖像庫的新版本，並將新版本號存儲在Cargo.lock中。
前面的例子顯示Cargo正在更新crates.io上託管的兩個crate。存儲在Git中的依賴項也會發生類似的情况。 假設我們的Cargo.toml檔案包含以下內容：

```
image = { git = "https://github.com/Piston/image.git", branch = "master" }

```

如果cargo build看到我們有一個cargo.lock檔案，它將不會從Git存儲庫中選取新的更改。 相反，它讀取的是Cargo.lock，並使用與上次相同的修訂版。 但貨物更新將從master中選取，以便我們的下一個版本使用最新修訂版。
Cargo.lock是自動為您生成的，您通常不會手動編輯它。 儘管如此，如果您的項目是可執行的，您應該將Cargo.lock提交給版本控制。 這樣，構建您的項目的每個人都將始終獲得相同的版本。 您的Cargo.lock檔案的歷史記錄將記錄您的依賴關係更新。



如果你的項目是一個普通的Rust庫，那麼不用麻煩提交Cargo.lock。 庫的下游用戶將擁有Cargo.lock檔案，其中包含其整個依賴關係圖的版本資訊； 它們將忽略庫的Cargo.lock檔案。 在極少數情况下，您的項目是共亯庫（即輸出是.dll、.dlib或.so檔案），沒有這樣的下游貨物用戶，囙此您應該提交cargo.lock。
toml靈活的版本說明符使您可以輕鬆地在項目中使用Rust庫，並最大限度地提高庫之間的相容性。 Cargo.lock的記帳支持機器之間一致、可複製的構建。 它們結合在一起，大大有助於你避免依賴地獄。







## Publishing Crates to crates.io

You’ve decided to publish your fern-simulating library as open source software. Con‐ gratulations! This part is easy.

First, make sure Cargo can pack the crate for you.

```
cargo package
```

cargo package命令創建一個檔案（在本例中為target/package/firn_sim-0.1.0.crace），其中包含庫的所有原始檔案，包括cargo.toml。 這是您將上傳到crates.io與全世界共亯的檔案。 （您可以使用cargo package-list來查看包含哪些檔案。）然後，cargo通過從.crace檔案構建庫來仔細檢查其工作，就像您的最終用戶一樣。
Cargo警告說，Cargo.toml的[包]部分缺少一些對下游用戶很重要的資訊，例如您分發程式碼所依據的許可證。 警告中的URL是一個很好的資源，囙此我們不會在這裡詳細解釋所有欄位。 簡而言之，您可以通過在Cargo.toml中添加幾行來修復警告：

```

    [package]
    name = "fern_sim"
    version = "0.1.0"
    authors = ["You <you@example.com>"]
    license = "MIT"
    homepage = "https://fernsim.example.com/"
    repository = "https://gitlair.com/sporeador/fern_sim"
    documentation = "http://fernsim.example.com/docs"
    description = """
    Fern simulation, from the cellular level up.
    
```

Another problem that sometimes arises at this stage is that your *Cargo.toml* file might be specifying the location of other crates by path, as shown in “Specifying Dependen‐ cies” on page 185:

```
    image = { path = "vendor/image" }
```



For you and your team, this might work fine. But naturally, when other people down‐ load the fern_sim library, they will not have the same files and directories on their computer that you have. Cargo therefore *ignores* the path key in automatically down‐ loaded libraries, and this can cause build errors. The fix, however, is straightforward: if your library is going to be published on crates.io, its dependencies should be on crates.io too. Specify a version number instead of a path:

`image = "0.6.1"`
 If you prefer, you can specify both a path, which takes precedence for your own local

builds, and a version for all other users:
` image = { path = "vendor/image", version = "0.6.1" }`

Of course, in that case it’s your responsibility to make sure that the two stay in sync.



Lastly, before publishing a crate, you’ll need to log in to crates.io and get an API key. This step is straightforward: once you have an account on crates.io, your “Account Settings” page will show a cargo login command, like this one:

`cargo login api_key`

Cargo將金鑰保存在設定檔中，API金鑰應保密，如
密碼。 囙此，只能在您控制的電腦上運行此命令。



That done, the final step is to run cargo publish:

`cargo publish`

有了這個，你的圖書館就可以在crates.io上加入成千上萬的其他圖書館。



## Workspaces

隨著你的項目不斷發展，你最終會寫很多板條箱。 它們並排存在於一個源存儲庫中：

```
 fernsoft/
    ├── .git/...
    ├── fern_sim/
    │   ├── Cargo.toml
    │   ├── Cargo.lock
		│ ├── src/...
    │   └── target/...
    ├── fern_img/
    │   ├── Cargo.toml
    │   ├── Cargo.lock
		│ ├── src/...
    │   └── target/...
    └── fern_video/
        ├── Cargo.toml
        ├── Cargo.lock
        ├── src/...
        └── target/...
```

按照Cargo的工作方式，每個主機殼都有自己的構建目錄target，其中包含該主機殼所有依賴項的單獨構建。 這些構建目錄是完全獨立的。 即使兩個crate有一個共同的依賴項，它們也不能共亯任何已編譯的程式碼。 這是浪費。



您可以使用Cargo工作區來節省編譯時間和磁碟空間，Cargo工作空間是一個由板條箱組成的集合，共亯一個通用的構建目錄和Cargo.lock檔案。
您所需要做的就是在存儲庫的根目錄中創建一個Cargo.toml檔案，並在其中放入以下行：

```
   [workspace]
    members = ["fern_sim", "fern_img", "fern_video"]
```

其中fern_sim等是包含板條箱的子目錄的名稱。 删除這些子目錄中剩餘的Cargo.lock檔案和目標目錄。

完成此操作後，任何主機殼中的貨物構建都將自動在根目錄下創建並使用共亯構建目錄（在本例中為fernsoft/target）。 命令貨物構建——所有構建當前工作空間中的所有板條箱。 貨物測試和貨物單據也接受--all選項。