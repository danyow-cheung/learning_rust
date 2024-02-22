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

  

  