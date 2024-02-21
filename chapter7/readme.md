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

