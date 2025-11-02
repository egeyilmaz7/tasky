# 📝 Tasky

Tasky is a small, lightweight command-line to-do manager written in Rust.

---

## 🚀 Features

* Add new tasks
* Remove tasks by number
* List all current tasks
* Persistent storage in `~/.config/tasky/tasks.json`

---

## 📦 Installation

### Install via Cargo

If you have Rust installed, just run:

```bash
cargo install --git https://github.com/egeyilmaz7/tasky
```

### Build from Source

```bash
git clone https://github.com/egeyilmaz7/tasky.git
cd tasky
cargo build --release
```

Move the binary into your `$PATH`:

```bash
sudo mv target/release/tasky /usr/local/bin/
```

---

## 💻 Usage

### Add a task

```bash
tasky add Buy milk
```

### Remove a task

```bash
tasky remove 1
```

### List tasks

```bash
tasky list
```

Example output:

```
All Tasks:

1. Buy milk
2. Finish Rust project
```

---

## 🧑‍💻 Contributing

Contributions are welcome! Fork, open issues, or submit pull requests.
Feel free to improve features, error handling, or UX. Just keep it respectful and fun.

---

## 📜 License

This project is licensed under the MIT License — see the `LICENSE` file for details.
Use it, modify it, learn from it, and contribute back!
