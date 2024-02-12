// 我们从两个外部板条箱指令开始，这两个指令使我们在Cargo.toml
// 文件中引用的铁板条箱和mime板条箱可用于我们的程序。

extern crate iron;
#[macro_use] extern crate mime; // extern crate mime项之前的
// #[mmacro_use]属性提醒Rust，我们计划使用此crate导出的宏。

// Rust程式師通常會收集他們所有的外部主機殼，並將聲明放在檔案的頂部，但這並不是絕對必要的：Rust允許以任何順序進行解密，只要它們出現在適當的嵌套級別即可 内容是該規則的例外：它們必須在使用之前出現。）
extern crate router;
use router::Router;


extern crate urlencoded;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

use iron::prelude::*;
// 接下来，我们将使用声明引入这些板条箱的一些公共特性。
// 声明使用iron:：prelude:：*使iron:：prelude模块的所有公共名称在我们自己的代码中直接可见。

use iron::status;

fn main() {
    let mut router = Router::new();
    router.get("/",get_form,"root");
    router.get("/gcd",post_gcd,"gcd");

    println!("Serving on http://localhost:3000....");
    // Iron::new : create a server ,set it 監聽3000端口號
    // Iron::new(get_form) 指示服务器应该使用该功能来处理所有请求
    Iron::new(get_form).http("localhost:3000").unwrap(); 
}
// _request: 给参数一个以_开头的名称会告诉Rust，我们希望该变量未被使用，所以它不应该警告我们。

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok); // Http 狀態碼
    response.set_mut(mime!(Text/Html;Charset=Utf8)); // media type of the content
    // Rust d的 raw string 語法
    // 字母r、零个或多个散列标记（即#字符）、双引号，然后是字符串的内容，以另一个双引号结尾，后跟相同数量的散列标记。

    response.set_mut(r#"
    <title>GCD Calculatot</title>
    <form action = "/gcd" method="post">
        <input type="text" name = "n" />
        <input type="text" name = "n" />
        <button type="submit" >Compute GCD</button>
    </form>
    "#);
    Ok(response)
}

fn post_gcd(request: &mut Request )-> IronResult<Response>{
    let mut response = Response::new();
    // 它調用request.get_ref:：＜UrlEncodedBody＞（）將請求的正文解析為一個錶，將査詢參數名稱映射到值數組； 我
    let form_data = match request.get_ref::<UrlEncodedBody>(){
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing from data: {:?}\n",e));
            return Ok(response);
        }
        Ok(map) => map
    };
    // 在該錶中，它找到名為“n”的參數的值，HTML表單將輸入的數位放入網頁中。 該值將不是單個字串，而是字串的向量，因為査詢參數名稱可以重複。

    let unparsed_numbers = match form_data.get("n"){
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(
                format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums
    };
    let mut numbers = Vec::new();

    //它遍歷字串的向量，將每個字串解析為無符號的64比特數位，如果任何字串解析失敗，則返回相應的失敗頁。

    for unparsed in unparsed_numbers{
        match u64::from_str(&unparsed){
            Err(_)=>{
                response.set_mut(status::BadRequest);
                response.set_mut(format!("Value for 'n' parameters not a number:{:?}\n",unparsed));
                return Ok(response);
            }
            Ok(n)=>{numbers.push(n);}
        }
    }
    // 計算數字的最大公約數
    let mut d = numbers[0];
    for m in &numbers[1..]{
        d = gcd(d,*m);
    }
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html;Charset=Utf8));
    response.set_mut(format!("The greatest common divisor of the numbers{:?} is <b>{}<b> \n",numbers,d));
    Ok(response)

}

fn gcd(mut n:u64,mut m:u64)-> u64{
    assert !(n!=0 && m!=0);
    while m!=0{
        if m<n{
            let t =m;
            m=n;
            n=t;
        }
        m = m%n;
    }
    n
}
