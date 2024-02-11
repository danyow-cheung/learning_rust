# A Tour of Rust
在本章中，我們將查看幾個簡短的程式，瞭解Rust的語法、類型和語義如何結合在一起，以支持安全、併發和高效的程式碼。 我們將完成下載和安裝Rust的過程，展示一些簡單的數學程式碼，試用基於協力廠商庫的web服務器，並使用多個線程加快繪製Mandelbrot集的過程。

## Downloading and Installing Rust
https://www.rust-lang.org/
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
dyMBP:~ danyow$ cargo --version
cargo 1.76.0 (c84b36747 2024-01-18)
dyMBP:~ danyow$ rustc --version
rustc 1.76.0 (07dca489a 2024-02-04)
dyMBP:~ danyow$ rustdoc --version
rustdoc 1.76.0 (07dca489a 2024-02-04)

```

- `cargo`是Rust的編譯管理器、包管理器和通用工具。 您可以使用Cargo啟動一個新項目，構建和運行您的程式，並管理您的程式碼所依賴的任何外部庫。
- `rustc`是Rust編譯器。 通常我們讓Cargo為我們調用編譯器，但有時直接運行它是有用的。
- `rustdoc`是Rust檔案工具。 如果你在程式的原始程式碼中以適當的形式編寫檔案，rustdoc可以從中構建格式良好的HTML。 像rustc一樣，我們通常讓Cargo為我們運行rustdoc。

為了方便起見，Cargo可以為我們創建一個新的Rust包，其中適當安排了一些標準中繼資料：
`$cargo new --bin hello`
已創建二進位（應用程序）`hello`項目
該命令創建一個名為hello的新包目錄，--bin標誌訓示Cargo將其準備為可執行文件，而不是庫。 查看包的頂級目錄：

