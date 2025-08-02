# 🦀 Swimo: Fullstack Rust App (Axum + Leptos)

Welcome to **Swimo**, a full-stack Rust web application.  
It uses **Axum** for a blazing-fast backend API and **Leptos (WASM)** for a reactive, component-driven frontend — all written entirely in Rust.

---

## 📁 Project Structure

```

swimo/
├── backend/           # Axum backend API
│   ├── Cargo.toml
│   └── src/main.rs
├── frontend/          # Leptos frontend (compiled to WebAssembly)
│   ├── Cargo.toml
│   ├── index.html
│   └── src/lib.rs
├── Cargo.toml         # Workspace manifest
└── README.md          # Project documentation

````

---

## 🚀 Features

- ✅ **Pure Rust** frontend & backend (no JS)
- ⚡ Fast & type-safe HTTP API with [`axum`](https://docs.rs/axum)
- 🎯 Reactive UI in WebAssembly with [`leptos`](https://leptos.dev)
- 🌐 Frontend fetches live data from the backend
- 🧩 Modular, workspace-based architecture

---

## 🧰 Prerequisites

Ensure you have the following installed:

- [Rust (Stable)](https://www.rust-lang.org/tools/install)
- [`trunk`](https://trunkrs.dev/) for building the frontend:
  ```bash
  cargo install trunk
````

* [WebAssembly target](https://rustwasm.github.io/):

  ```bash
  rustup target add wasm32-unknown-unknown
  ```

---

## 🧑‍💻 Getting Started

### 1. Clone and navigate

```bash
git clone https://github.com/yourusername/swimo.git
cd swimo
```

### 2. Run the backend

```bash
cargo run -p backend
```

Visit: [http://localhost:3000/api/hello](http://localhost:3000/api/hello)

You should see:

```json
{ "msg": "Hello from Rust API!" }
```

---

### 3. Run the frontend (in a new terminal tab)

```bash
cd frontend
trunk serve --open
```

This will:

* Build the Leptos UI to WASM
* Start a dev server at [http://127.0.0.1:8080](http://127.0.0.1:8080)

It fetches data from the backend and renders it dynamically.

---

## 🛠️ Tech Stack

| Layer    | Tool      | Description                          |
| -------- | --------- | ------------------------------------ |
| Backend  | `axum`    | Web framework built on `hyper`       |
| Backend  | `serde`   | JSON serialization & deserialization |
| Frontend | `leptos`  | Reactive WASM frontend framework     |
| Frontend | `reqwasm` | HTTP client in WASM                  |
| Tooling  | `trunk`   | Build & serve Rust WASM frontends    |
| Language | `Rust`    | 💖 Both frontend and backend         |


## 🧪 Testing & Debugging

* Backend errors appear in terminal (`cargo run -p backend`)
* Frontend (WASM) errors show in browser console
* Use `console_error_panic_hook` for readable stack traces in WASM:

  ```rust
  console_error_panic_hook::set_once();
  ```

---

## 🗺️ Roadmap Ideas

* [ ] Connect a database (e.g., `sqlx`, `sea-orm`)
* [ ] Add frontend routing with `leptos_router`
* [ ] Add form submission and POST endpoints
* [ ] Bundle frontend and backend into one Axum server
* [ ] Deploy to Netlify/Vercel + Fly.io/Render

---

## 💬 Feedback & Contributions

Feel free to open issues, suggest improvements, or fork and build on top!

---

## 📝 License

MIT License © 2025 Paul Webo(@Xi9d)
```
