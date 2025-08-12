# 🌦 Weathermap (Rust + GTK4)

A simple weather application built in Rust with GTK4, designed to run on Linux, including **postmarketOS**.

## 📦 Setup Instructions (postmarketOS)

### 1. Clone the Repository

```bash
git clone https://github.com/Kindaglia/rust-lab.git
cd rust-lab
cd weathermap-cargo-rust-gtk4
```

### 2. Update System Packages

```bash
sudo apk update && sudo apk upgrade
```

### 3. Install Rust & Cargo

```bash
sudo apk add rust cargo
```

### 4. Install GTK4 and Development Dependencies

```bash
sudo apk add gtk4.0-dev 
```

### 5. Run the Application

```bash
cargo run --release
```

