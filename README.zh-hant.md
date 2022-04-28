<div align="center">
<img alt="Savlo" src="assets/logo.svg" />
<p>

[English](https://github.com/salvo-rs/salvo/blob/main/README.md) [简体中文](https://github.com/salvo-rs/salvo/blob/main/README.zh-hans.md) [繁體中文](https://github.com/salvo-rs/salvo/blob/main/README.zh-hant.md)<br><br>
[![build status](https://github.com/salvo-rs/salvo/workflows/ci-linux/badge.svg?branch=main&event=push)](https://github.com/salvo-rs/salvo/actions)
[![build status](https://github.com/salvo-rs/salvo/workflows/ci-macos/badge.svg?branch=main&event=push)](https://github.com/salvo-rs/salvo/actions)
[![build status](https://github.com/salvo-rs/salvo/workflows/ci-windows/badge.svg?branch=main&event=push)](https://github.com/salvo-rs/salvo/actions)
<br>
[![crates.io](https://img.shields.io/crates/v/salvo)](https://crates.io/crates/salvo)
[![Documentation](https://docs.rs/salvo/badge.svg)](https://docs.rs/salvo)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Rust Version](https://img.shields.io/badge/rust-1.56%2B-blue)](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html)
<br>
[![codecov](https://codecov.io/gh/salvo-rs/salvo/branch/main/graph/badge.svg)](https://codecov.io/gh/salvo-rs/salvo)
[![Download](https://img.shields.io/crates/d/salvo.svg)](https://crates.io/crates/salvo)
[![Website](https://img.shields.io/website?down_color=lightgrey&down_message=offline&up_color=blue&up_message=online&url=https%3A%2F%2Fsalvo.rs)](https://salvo.rs)
![License](https://img.shields.io/crates/l/salvo.svg)
</p>
</div>

Salvo 是一個極其簡單易用卻又功能強大的 Rust Web 後端框架. 僅僅需要基本的 Rust 基礎即可寫成功能強大的後端服務器, 我們的目標是: 編碼最簡單, 功能不缺失, 性能有保障.

## 🎯 功能特色
  - 基於hyper, tokio 的異步 Web 後端框架;
  - 支持 Websocket;
  - 統一的中間件和句柄接口, 中間件係統支持在句柄之前或者之後運行;
  - 簡單易用的路由係統, 支持路由嵌套, 在任何嵌套層都可以添加中間件;
  - 集成 multipart 錶單處理, 處理上傳文件變得非常簡單;
  - 支持 Acme, 自動從 [let's encrypt](https://letsencrypt.org/) 獲取 TLS 證書;
  - 支持從多個本地目錄映射成一個虛擬目錄提供服務.

## ⚡️ 快速開始
你可以查看[實例代碼](https://github.com/salvo-rs/salvo/tree/main/examples),  或者訪問[官網](https://salvo.rs/book/quick-start/hello_world/).


創建一個全新的項目:

```bash
cargo new hello_salvo --bin
```

添加依賴項到 `Cargo.toml`

```toml
[dependencies]
salvo = { version = "0.21", features = ["full"] }
tokio = { version = "1", features = ["full"] }
```

在 `main.rs` 中創建一個簡單的函數句柄, 命名為`hello_world`, 這個函數隻是簡單地打印文本 ```"Hello World"```.

```rust
use salvo::prelude::*;

#[fn_handler]
async fn hello_world(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Text::Plain("Hello World"));
}
```

### 中間件
Salvo 中的中間件其實就是 Handler, 冇有其他任何特別之處. **所以書寫中間件並不需要像其他某些框架需要掌握泛型關聯類型等知識. 隻要你會寫函數就會寫中間件, 就是這麼簡單!!!**

### 可鏈式書寫的樹狀路由係統

正常情況下我們是這樣寫路由的：

```rust
Router::with_path("articles").get(list_articles).post(create_article);
Router::with_path("articles/<id>")
    .get(show_article)
    .patch(edit_article)
    .delete(delete_article);
```

往往查看文章和文章列錶是不需要用戶登錄的, 但是創建, 編輯, 刪除文章等需要用戶登錄認證權限才可以. Salvo 中支持嵌套的路由係統可以很好地滿足這種需求. 我們可以把不需要用戶登錄的路由寫到一起：

```rust
Router::with_path("articles")
    .get(list_articles)
    .push(Router::with_path("<id>").get(show_article));
```

然後把需要用戶登錄的路由寫到一起， 並且使用相應的中間件驗證用戶是否登錄：
```rust
Router::with_path("articles")
    .hoop(auth_check)
    .post(list_articles)
    .push(Router::with_path("<id>").patch(edit_article).delete(delete_article));
```

雖然這兩個路由都有這同樣的 ```path("articles")```, 然而它們依然可以被同時添加到同一個父路由, 所以最後的路由長成了這個樣子:

```rust
Router::new()
    .push(
        Router::with_path("articles")
            .get(list_articles)
            .push(Router::with_path("<id>").get(show_article)),
    )
    .push(
        Router::with_path("articles")
            .hoop(auth_check)
            .post(list_articles)
            .push(Router::with_path("<id>").patch(edit_article).delete(delete_article)),
    );
```

```<id>```匹配了路徑中的一個片段, 正常情況下文章的 ```id``` 隻是一個數字, 這是我們可以使用正則錶達式限製 ```id``` 的匹配規則, ```r"<id:/\d+/>"```. 

還可以通過 ```<*>``` 或者 ```<**>``` 匹配所有剩餘的路徑片段. 為了代碼易讀性性強些, 也可以添加適合的名字, 讓路徑語義更清晰, 比如: ```<**file_path>```.

### 文件上傳
可以通過 Request 中的 get_file 異步獲取上傳的文件:

```rust
#[fn_handler]
async fn upload(req: &mut Request, res: &mut Response) {
    let file = req.get_file("file").await;
    if let Some(file) = file {
        let dest = format!("temp/{}", file.filename().unwrap_or_else(|| "file".into()));
        if let Err(e) = std::fs::copy(&file.path, Path::new(&dest)) {
            res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
        } else {
            res.render("Ok");
        }
    } else {
        res.set_status_code(StatusCode::BAD_REQUEST);
    }
}
```

### 更多示例
您可以從 [examples](./examples/) 文件夾下查看更多示例代碼, 您可以通過以下命令運行這些示例：

```
cargo run --bin --example-basic_auth
```

您可以使用任何你想運行的示例名稱替代這裏的 ```basic_auth```.

這裏有一個真實的項目使用了 Salvo：[https://github.com/driftluo/myblog](https://github.com/driftluo/myblog).


## 🚀 性能
Benchmark 測試結果可以從這裏查看:

[https://web-frameworks-benchmark.netlify.app/result?l=rust](https://web-frameworks-benchmark.netlify.app/result?l=rust)

[https://www.techempower.com/benchmarks/#section=test&runid=785f3715-0f93-443c-8de0-10dca9424049](https://www.techempower.com/benchmarks/#section=test&runid=785f3715-0f93-443c-8de0-10dca9424049)
[![techempower](assets/tp.jpg)](https://www.techempower.com/benchmarks/#section=test&runid=785f3715-0f93-443c-8de0-10dca9424049)

## 🩸 貢獻

非常歡迎大家為項目貢獻力量，可以通過以下方法為項目作出貢獻:

  - 在 issue 中提交功能需求和 bug report;
  - 在 issues 或者 require feedback 下留下自己的意見;
  - 通過 pull requests 提交代碼;
  - 在博客或者技術平臺發錶 Salvo 相關的技術文章。

All pull requests are code reviewed and tested by the CI. Note that unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Salvo by you shall be dual licensed under the MIT License, without any additional terms or conditions.
## ☕ 支持

`Salvo`是一個開源項目, 如果想支持本項目, 可以 ☕ [**在這裏買一杯咖啡**](https://www.buymeacoffee.com/chrislearn). 
<p style="text-align: center;">
<img src="assets/alipay.png" alt="Alipay" width="320"/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="assets/weixin.png" alt="Weixin" width="320"/>
</p>


## ⚠️ 開源協議

Salvo 項目採用以下開源協議:
* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
