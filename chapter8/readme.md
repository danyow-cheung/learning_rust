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

```
```

