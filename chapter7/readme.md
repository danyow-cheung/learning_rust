# Error Handling 

This chapter covers the two different kinds of error-handling in Rust: panic and Results.

使用`result`處理普通錯誤。 這些通常是由程式之外的事情引起的，如錯誤輸入、網絡中斷或許可權問題。 這種情況的發生不取決於我們； 即使是一個沒有bug的程式也會不時遇到它們。 本章的大部分內容都是針對這種錯誤的。 不過，我們將首先報導恐慌，因為這是兩者中比較簡單的一個。

## Panic

 A program panics when it encounters something so messed up that there must be a bug in the program itself. Something like:

- 越界數組訪問
- 數值除與0
- 對恰好為None的Option調用.unwrap（）
- Assertion failure



（還有宏恐慌！（），用於您自己的程式碼發現錯誤的情况，囙此需要直接引發恐慌。 `panic()`接受可選的println！ （）樣式的參數，用於生成錯誤消息



這些條件的共同點是，它們都是程式師的錯——不要說得太細。 一個很好的經驗法則是：“不要驚慌”。
但我們都會犯錯。 當這些不該發生的錯誤發生時，會發生什麼？ 值得注意的是，Rust為您提供了一個選擇。 Rust可以在發生死機時展開堆棧，也可以中止行程。 展開是默認設置。

### Unwinding 

當海盜從一次突襲中瓜分戰利品時，船長會得到一半的戰利品。 普通船員的收入與另一半的收入相等。 （海盜討厭分數，所以如果任何一個分數都沒有出來，結果會四捨五入，剩下的分數會留給船上的鸚鵡。）

```rust
fn pirate_share(total:u64,crew_size:usize)->u64{
  let half = total/2;
  half/crew_size as u64
}
```

這可能會持續好幾個世紀，直到有一天船長是突襲中唯一的倖存者。 如果我們將crew-size為零傳遞給這個函數，它將被零除。 在C++中，這將是未定義的行為。 在Rust中，它會引發恐慌，通常情况如下：

- 終端會打印報錯信息

- 堆棧已展開。 這很像C++異常處理。

  任何短暫的數值，本地變量或者當前函數使用的參數將按照與創建順序相反的順序删除。

  一旦清理了當前的函數調用，我們就轉到它的調用者，以同樣的管道删除它的變數和參數。 然後該函數的調用程式，依此類推

- 最後，線程才會退出。

  如果恐慌線程是主線程，那麼整個行程將退出（退出程式碼為非零）。

也許恐慌是這個有序過程的一個誤導性名稱。 恐慌不是崩潰。 這不是未定義的行為。 它更像是Java中的RuntimeException或C++中的std::logic_error。 行為定義明確； 這不應該發生。



Panic同時是安全的，因為它不會危害rust的安全規則。即使您設法在標準庫方法的中間恐慌，它也不會在記憶體中留下懸空指針或半初始化的值。 其思想是，Rust在發生任何錯誤之前捕獲無效的數組訪問，或者不管是什麼。 繼續操作是不安全的，所以Rust展開堆棧。 但流程的其餘部分可以繼續運行。



理想情况下，我們都將擁有從不恐慌的無錯誤代碼。 但沒有人是完美的。 您可以使用線程和catch_unwind（）來處理panic，使您的程式更加健壯。 一個重要的警告是，這些工具只會引起恐慌。 並不是每一次恐慌都是這樣進行的。



### Aborting

堆棧展開是默認的恐慌行為，但有兩種情况下Rust不會嘗試展開堆棧。



如果一個.drop（）方法觸發了第二次恐慌，而Rust在第一次恐慌之後仍在嘗試清理，則認為這是致命的。 rust會停止展開並中止整個過程。



Also, Rust’s panic behavior is customizable. If you compile with -C panic=abort, the *first* panic in your program immediately aborts the process. 



這就結束了我們在rust中對恐慌的討論。 沒有什麼好說的，因為普通的Rust程式碼沒有義務處理恐慌。 即使您確實使用了線程或catch_unwind（），所有的緊急處理程式碼也可能集中在幾個地方。 期望程式中的每個函數都能預測和處理自己程式碼中的錯誤是不合理的。 其他因素造成的錯誤是另一回事。





## Result

Rust沒有例外。 相反，可能失敗的函數具有這樣的返回類型：

`fn get_weather(location:LatLing)-> Result<WeatherReport,io::Error>`



Result（結果）類型表示可能的故障。 當我們調用get_weather（）函數時，它將返回一個成功結果Ok（weather），其中weather是一個新的WeatherReport值，或者返回一個錯誤結果Err（error_value），其中error_value是解釋錯誤的io::error。



Rust要求我們在調用此函數時編寫某種錯誤處理。 如果不對Result執行某些操作，我們就無法訪問WeatherReport，如果未使用Result值，則會收到編譯器警告。



### Catching Errors

最好捕錯的方式是使用match表達式

```rust
match get_weather(hometown){
	Ok(report)=> {
    display_weather(hometown,&report);
  }
  Err(err)=>{
    println!("error querying the weather{}",err);
    schedule_weather_retry(;)
  }
}
```



This is Rust’s equivalent of try/catch in other languages. 



match有點冗長，所以Result＜T，E＞提供了各種方法，這些方法在特定的常見情况下很有用。 這些方法中的每一種在實現過程中都有一個匹配運算式。 （有關Result方法的完整清單，請參閱線上檔案。此處列出的方法是我們使用最多的方法。）

- `result.is_ok()` ,`result.is_err()`返回布爾值

- `result.ok()`

  返回成功值（如果有）作為Option＜T＞。 如果結果是成功的結果，則返回Some（success_value）； 否則，它將返回None，從而忽略錯誤值。

- `result.err()`

  返回錯誤值（如果有）作為Option＜E＞

  

- `result.unwrap_or(fallback)`

  返回成功值，如果有。

  if result is a success result,otherwise it returns fallback discarfing the error value 

  ```rust
  // a fairly safe prediction for suthern california
  const THE_USUAL:WeatherReport = WeatherReport::Sunny(72);
  // get a real weather report 
  let repot = get_weather(los_angeles).unwrap_or(THE_USUAL);
  display_weather(los_angeles,&report);
  ```

  這是.ok（）的一個很好的替代方案，因為返回類型是T，而不是Option＜T＞。 當然，只有當存在適當的回退值時，它才有效。

  

- `result.unwrap_or_else(fallback_fn)`

  是相同的，但不是直接傳遞回退值，而是傳遞函數或閉包。

  這適用於如果不打算使用回退值，則計算回退值將是浪費的情况。只有當我們有錯誤結果時，才會調用fallback_fn。

  ```rust
  let report = get_weather(hometown).unwrap_or_else(|_err| vague_prediction(hometown));
  ```

  

- `result_unwrap`

  如果結果為成功結果，也返回成功值。 然而，若結果是錯誤結果，則此方法會死機。 這種方法有其用途； 我們稍後會詳細討論。

  

- `result_expect(message)`

  它與.unwrap（）相同，但允許您提供一個消息，以便在出現恐慌時列印。

  

  

最後，借用Result中值的引用的兩種方法：

- `result.as_ref()`

  `Result<T,E>` 轉換為`<Result &T,&E>`

  借用對現有結果中成功或錯誤值的引用

- `result.as_mut()`

  is the same, but borrows a mutable reference. The return type is `Result<&mut T, &mut E>.`

  



最後兩個方法有用的一個原因是，除了.is_ok（）和.is_err（）之外，這裡列出的所有其他方法都會消耗它們運算的結果。也就是說，**它們按值接受self參數。 有時，在不破壞結果的情况下訪問結果內部的數據是非常方便的**，這就是.as_ref（）和.as_mut（）為我們所做的。





### Result Type Aliases

結果類型別名

`fn remove_file(path:&Path)->Result<()>`



類型別名是對類型名稱的一種簡寫。 模塊通常定義Result類型別名，以避免重複模塊中幾乎每個函數都一致使用的錯誤類型。 例如，標準庫的std::io模塊包括以下程式碼行：

`pub type Result<T> = result::Result<T,Error>;`

這定義了一個公共類型std::io::Result＜T＞。 它是Result＜T，E＞的別名，但將std::io::Error硬編碼為錯誤類型。 在實際應用中，這意味著如果您編寫，請使用std::io； 那麼Rust將把io::Result＜String＞理解為Result＜String，io::Error＞的簡寫。



當線上檔案中出現類似Result＜（）＞的內容時，您可以按一下識別字Result以查看正在使用的類型別名並瞭解錯誤類型。 在實踐中，這通常是從上下文中顯而易見的。



### Printing Errors

標準庫定義了幾個具有無聊名稱的錯誤類型：std::io::error、std::fmt::error、std：：str::Utf8Error等等。所有這些都實現了一個com介面，即std::error::error特性，這意味著它們共亯以下功能：

- 打印都使用`println!()`,使用｛｝格式說明符列印錯誤通常只顯示一條簡短的錯誤消息。 或者，您可以使用｛：？｝格式說明符進行列印，以獲得錯誤的“調試”視圖。

- `err.description()` 返回一個引用的字符型錯誤信息

- `err.cause()` 返回一個`Option<&Error>`引發錯誤的潜在錯誤（如果有的話）。

  例如，網絡錯誤可能會導致銀行交易失敗，進而導致你的船被收回。 如果err.description（）是“船被收回”，那麼err.crease（）可能會返回一個關於失敗交易的錯誤； its的.description（）可能是“未能將300美元轉移到United Yacht Supply”，而its的.ccause（）則可能是一個io：錯誤，其中包含了導致所有大驚小怪的特定網絡中斷的詳細資訊。 第三個錯誤是根本原因，所以它的.cocause（）方法將返回None。



列印錯誤值不會同時列印出其原因。 如果您想確保列印所有可用資訊，請使用此功能：

```rust
use std::error::Error;
use std::io::{Write,stederr};

// dump an error message to 'stderr'
fn print_error(mut err:&Error){
  let _ = writln!(stderr(),"errpr{}",err);
  while let Some(cause) ==err.cause(){
    let _ = writln!(stderr(),"caused by {}",cause);
    err = cause;
  }
}
```

標準庫的錯誤類型不包括堆棧跟踪，但錯誤鏈主機殼使您可以輕鬆定義自己的自定義錯誤類型，該類型支持在創建堆棧跟踪時獲取堆棧跟踪。 它使用回溯主機殼來捕獲堆棧。



### Propagating Errors

在大多數情况下，當我們嘗試一些可能失敗的東西時，我們不想立即發現並處理錯誤。 在每個可能出錯的地方都使用10行匹配語句，這簡直是太多的程式碼了。



相反，如果發生錯誤，我們通常希望讓調用方處理它。我們希望錯誤向上傳播到調用堆棧。



在rust中有`a?`操作符可以實現。可以添加這個操作符到任何的表達式中。比如說

```rust
let weather = get_weather(hometown)?;
```



`?`的行為 取决於此函數是返回成功結果還是返回錯誤結果

- 如果成功，它打開Result以獲取內部的成功值。 這裡的天氣類型不是Result<WeatherReport，io::Error>，而是WeatherReport。
- 如果錯誤。它立即從封閉函數返回，將錯誤結果向上傳遞到調用鏈。 為了確保這一點有效？ 只能在具有Result返回類型的函數中使用。

沒有什麼神奇的`? operator` 您可以使用匹配運算式來表達相同的內容，儘管它要複雜得多：

```rust
// 和上面的代碼一樣的功能，但是更複雜
let weather = match get_weather(hometown){
  Ok(sucess_value)=> success_value,
  Err(err)=> return Err(err)
};
```



還可以用`try!()`宏來捕獲錯誤

```rust
let weather = try!(get_weather(hometown));
```

The macro expands to a match expression, like the one above.



人們很容易忘記錯誤在程式中的普遍性，尤其是在與作業系統介面的程式碼中。 這個 `?`運算子有時幾乎出現在函數的每一行：



```rust
use std::fs;
use std::io;
use std::path::Path;

fn move_all(src:&Path,dst:&Path)-> io::Result<()>{
  for entry_result in src.read_dir()?{
    let entry = entry_result?;
    let dst_file = dst.join(entry.file_name());
    fs::rename(entry.path(),dst_file)?;
  }
  Ok(())
}
```



### Working with Multiple Error Types

通常，不止一件事會出錯。 假設我們只是從文字檔中讀取數位

```rust
use std::io::{self,BufRead};

fn read_numbers(file: &mut BufRead)-> Result<Vec <i64>,io::Error>{
  let mut numbers = vec![];
  for line_result in file.lines(){
    let line = line_result?;
    numbers.push(line.parse()?);
  }
  Ok(numbers)
}
```





Rust gives us a compiler error:

```
    numbers.push(line.parse()?);     // parsing integers can fail
                 ^^^^^^^^^^^^^ the trait `std::convert::From<std::num::ParseIntError>`
                               is not implemented for `std::io::Error`
```



現在，只需注意Rust抱怨它無法將std::num::ParseIntError值轉換為std::io::Error類型。



line_result的類型為result＜String，std::io::Error＞。 parse（）行的類型為Result＜i64，std::num::parse IntError＞。 read_numbers（）函數的返回類型僅適用於io::Errors。 Rust試圖通過將ParseIntError轉換為io::Error來處理它，但沒有這樣的轉換，所以我們得到了一個類型錯誤



一個更簡單的方法是使用Rust中內寘的內容。 所有標準庫錯誤類型都可以轉換為`Box<std::error::error>`類型，表示“任何錯誤”。囙此，處理多個錯誤類型的一個簡單方法是定義以下類型別名：

```rust
type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T,GenError>;
```



然後，將read_numbers（）的返回類型更改為GenResult＜Vec＜i64＞。 通過此更改，函數將編譯。 這個 操作員根據需要自動將任何一種類型的錯誤轉換為GenError。
順便說一句，`? operator`使用您自己可以使用的標準方法進行自動轉換。 要將任何錯誤轉換為GenError類型，請調用GenError::from（）：

```rust
let io_error = io::Error::new(
	io::ErrorKind::Other,"timed out");
return Err(GenError::from(io_error));
```





如果您正在調用一個返回GenResult的函數，並且您想處理一種特定類型的錯誤，但讓所有其他錯誤傳播出去，請使用通用方法`error.downcast_ref::<ErrorType>()`。 它借用了對錯誤的引用，如果它可能是您要查找的特定類型的錯誤：

```rust
loop{
	match compile_project(){
    Ok(())=>return Ok(());
    Err(err)=>{
      if let Some(mse)= err.downcast_ref::<MissingSemicolonError>(){
        insert_semicoln_in_source_code(mse.file(),mse.line())?;
        continue; //try again 
      }
      return Err(err);
    }
  }
}
```



### Dealing with Errors That "Can't Happen"

有時我們只知道錯誤是不會發生的。 例如，假設我們正在編寫程式碼來解析設定檔，並且在某一點上我們發現檔案中的下一個內容是一串數位：

```rust
if next_char.is_digit(10){
  let start = current_index;
  current_index = skip_digits(&line,current_index);
  let digits = &line[start..current_index];
}
```



我們想把這個數位串轉換成一個實際的數位。 有一種標準方法可以做到這一點：

```rust
let mut = digits.parse<u64>();

```



Now the problem: the `str.parse::<u64>()` method doesn’t return a u64. It returns a

Result. It can fail, **because some strings aren’t numeric.**

如果我們正在編寫的程式碼已經返回GenResult，我們可以添加一個`?` 否則，我們將面臨不得不為不可能發生的錯誤編寫錯誤處理程式碼的令人惱火的前景。 最好的選擇是使用.unwrap（），這是我們前面提到的Result方法。

```rust
let mut = digits.parse<u64>().unwrap();
```



這就像？ 除了如果我們對這個錯誤的看法是錯誤的，如果它能發生，那麼在這種情況下，我們會感到恐慌。
事實上，我們對這個特殊案例的看法是錯誤的。 如果輸入包含一個足够長的數位串，那麼這個數位將太大，無法放入u64。



當一個錯誤表明情况非常嚴重或奇怪，以至於恐慌正是你想要處理的時候，這些方法也很有用。
```rust 
fn print_file_age（filename:&Path，last_modified:SystemTime）｛
let age=last_modified.elapsed().expect（"時間時鐘飄逸"）;
}

```

這裡，只有當系統時間早於創建檔案時，.`elapsd（）`方法才會失敗。 如果檔案是最近創建的，並且在程式運行時系統時鐘被向後調整，則可能會發生這種情況。 根據此程式碼的使用管道，在這種情況下恐慌是一個合理的判斷調用，而不是處理錯誤或將其傳播給調用者。





### Ignoring Errors

偶爾我們只想完全忽略一個錯誤。 例如，在我們的print_error（）函數中，我們必須處理列印錯誤觸發另一個錯誤的不太可能的情况。 例如，如果stderr通過筦道傳輸到另一個行程，並且該行程被終止，則可能會發生這種情況。 由於我們對這種錯誤無能為力，我們只想忽略它； 但是Rust編譯器警告未使用的Result值：

```rust
writeln!(stderr(),"error {}",err);  // warning:unused resutlt
let _ = writeln!(stderr(),"error {}",err); // ok ignore result
```



### Handling Errors in main()

在大多數生成Result的地方，讓錯誤冒泡到調用者是正確的行為。 這就是為什麼？ 是Rust中的單個角色。 正如我們所看到的，在一些程式中，它被連續用於許多行程式碼。

但是，如果您傳播錯誤足够長的時間，它最終會到達main（），這就是這種方法必須停止的地方。 main（）不能使用？ 因為它的返回類型不是Result。

```rust
fn main(){
  calculate_tides()?; // can't pass the buck any further
  //The simplest way to handle errors in main() is to use .expect().
  calculate_tides().expect("errors");
}
```



如果calculate_tides（）返回錯誤結果，則.expect（）方法會死機。 主線程中的恐慌會列印一條錯誤消息，然後使用非零退出程式碼退出，這大致是所需的行為。 我們一直在用這個做小程式。 這是一個開始。





### Declaring a Custom Error Type

假設您正在編寫一個新的JSON解析器，並且希望它有自己的錯誤類型。 （我們還沒有介紹用戶定義的類型；這將在幾章中介紹。但錯誤類型很方便，所以我們將在這裡進行一些預覽。）
大約您要編寫的最低程式碼是：

> error_type.rs

This struct will be called json::error::JsonError, and when you want to raise an error of this type, you can write:

```rust
return Err(JsonError){
  message:"expected ']' at end of array ".to_string(),
  line:current_lin,
  column:current_column
};)
```

這會很好用的。 但是，如果您希望您的錯誤類型像標準錯誤類型一樣工作，正如庫的用戶所期望的那樣，那麼您還有更多的工作要做：

```rust
use std;
use std::fmt;
// Error shoud be printable
impl fmt::Display for JsonError{
  fn fmt (&self,f:&mut fmt::Formatter)->Result<(),fmt::Error>{
    write!(f,"{}({}:{})",self.message,self.line,self.column)
  }
}

// Error should implement the std::error::Error trait
impl std::error::Error for JsonError{
  fn description(&self)-> &str{
    &self.message
  }
}
```



## Why Results?

現在我們已經足够瞭解Rust通過選擇Results而不是exceptions得到了什麼。 以下是設計的要點：

- Rust要求程式師在每一個可能發生錯誤的地方做出某種决定，並將其記錄在程式碼中。 這很好，因為否則，很容易因為疏忽而導致錯誤處理。
- 最常見的决定是允許錯誤傳播，這是用一個字元“？”編寫的。 囙此，錯誤筦道不會像在C和Go中那樣擾亂程式碼。 然而，它仍然是可見的：您可以查看一塊程式碼，並一目了然地看到所有傳播錯誤的地方。
- 由於錯誤的可能性是每個函數返回類型的一部分，所以很清楚哪些函數會失敗，哪些不會。 如果您將一個函數更改為易出錯函數，則會更改其返回類型，囙此編譯器會讓您更新該函數的下游用戶。
- Rust會檢查是否使用了Result值，這樣就不會意外地讓錯誤悄無聲息地通過（這是C中的一個常見錯誤）。
- 由於Result和其他資料類型一樣是一種資料類型，囙此很容易將成功和錯誤結果存儲在同一集合中。 這使得建立部分成功的模型變得容易。 例如，如果你正在編寫一個從文字檔加載數百萬條記錄的程式，並且你需要一種方法來處理可能的結果，即大多數會成功，但有些會失敗，那麼你可以使用Results向量在記憶體中表示這種情況。

代價是，與其他語言相比，您會發現自己在Rust中思考和工程錯誤處理的次數更多。 與許多其他領域一樣，Rust對錯誤處理的態度比你習慣的要嚴格一點。對於系統程式設計來說，這是值得的。