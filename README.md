# 🧮 calc-web

A calculator served in the browser, built with **Rust** and **Axum**. It is the web evolution of my [CLI calculator](https://github.com/izanestor0x/rust-cli-calculator): the same calculation logic, now behind an HTTP server and a small web page.

> 🚧 **Work in progress.** The calculation module is done; the web server and the page are being built step by step.

## 🎯 Goal

Learn how a Rust web backend works by taking something I already understand (a calculator) and putting it on the web:

- Keep the **logic** (`calculator.rs`) completely separate from the **interface** (console or web).
- Serve it with an async HTTP server.
- Build a simple page with two inputs, an operation selector and a button.

## 🧱 Project structure

```
calc-web/
├── Cargo.toml
├── src/
│   ├── main.rs          # HTTP server (Axum + Tokio)
│   └── calculator.rs    # Pure calculation logic, no I/O
└── static/
    └── index.html       # The web page
```

`calculator.rs` knows nothing about the keyboard, the screen or HTTP. It receives numbers and returns a result, so it can be reused from any interface.

## 🛠️ Tech stack

| Crate    | What it is used for                                  |
| -------- | ---------------------------------------------------- |
| `axum`   | Web framework: routes and handlers                   |
| `tokio`  | Async runtime that runs the server                   |
| `serde`  | Turns URL parameters into Rust structs               |

## 🗺️ Roadmap

- [x] Project setup with Axum, Tokio and Serde
- [x] `calculator` module with `add`, `subtract`, `multiply`, `divide` and `calculate`
- [ ] Basic server with a "Hello, world!" route
- [ ] Test route that calls `calculate`
- [ ] `/calculate?option=1&a=5&b=3` route reading URL parameters
- [ ] Handle division by zero without crashing the server
- [ ] Web page with a form and `fetch`
- [ ] Serve the page from `/`
- [ ] Error handling with `Result`
- [ ] Unit tests for the calculator module

## 🔌 Planned API

```
GET /calculate?option=1&a=5&b=3   →   8
```

| `option` | Operation      |
| -------- | -------------- |
| `1`      | Addition       |
| `2`      | Subtraction    |
| `3`      | Multiplication |
| `4`      | Division       |

## 🚀 Run it

You need [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
git clone https://github.com/izanestor0x/calc-web.git
cd calc-web
cargo run
```

Once the server is ready, open `http://localhost:3000` in your browser.

## 📚 What I'm practicing

`modules` · `pub` · `async / await` · `Tokio` · `Axum routes and handlers` · `serde` · `separating logic from I/O`

## 📄 License

This is a learning project. Feel free to use it as a reference.
