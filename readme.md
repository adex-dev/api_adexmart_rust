# AdexMart Backend

Backend API untuk aplikasi POS (Point of Sale) UMKM bernama **AdexMart**.  
Project ini dibangun menggunakan bahasa pemrograman Rust dengan performa tinggi, asynchronous runtime modern, dan arsitektur backend yang scalable.

Frontend aplikasi dapat diunduh di:

[AdexMart POS Frontend](https://github.com/adex-dev/adexmartpos)

---

# Teknologi yang Digunakan

- Rust
- Axum
- PostgreSQL
- BCrypt
- Tokio
- Serde
- SQLx
- ChaCha20Poly1305

---

# Fitur

- Authentication Login
- Hash Password menggunakan BCrypt
- REST API
- Management Produk
- Management Stok
- Transaction POS
- Management User
- Dashboard POS
- Sistem Backend Async
- JSON Response API
- ChaCha20Poly1305 digunakan sebagai encryption data token yang dikirim pada json token access pada saat login

---

# Struktur Teknologi

| Teknologi | Keterangan |
|---|---|
| Rust | Bahasa utama backend |
| Axum | Web framework asynchronous |
| PostgreSQL | Database utama |
| BCrypt | Hashing password |
| Tokio | Async runtime |
| Serde | Serialization & Deserialization JSON |

---

# Instalasi

## Clone Repository

```bash
git clone https://github.com/adex-dev/api_adexmart_rust.git api3
cd api3
```

## Jalankan Project

### development

```bash
cargo run
```

### production

```bash
cargo build --release
```

## Endpoint

| Method | Endpoint                | Keterangan    |
| ------ | ----------------------- | ------------- |
| POST   | `/api/v1/auth/login`    | Login user    |
| POST   | `/api/v1/auth/register` | Register user |
| GET    | `/api/v1/products`      | List produk   |
| POST   | `/api/v1/products`      | Tambah produk |
| PUT    | `/api/v1/products/:id`  | Update produk |
| DELETE | `/api/v1/products/:id`  | Hapus produk  |


## Tujuan Project

AdexMart dibuat untuk membantu UMKM dalam:

- Manajemen Penjualan
- Manajemen Produk
- Monitoring Stok
- Transaksi Kasir
- Dashboard Penjualan
- Sistem POS Modern

## Production Recommendation

Disarankan menggunakan:

- Nginx Reverse Proxy
- Docker
- PostgreSQL Dedicated Server
- HTTPS SSL
- Systemd Service Linux

## License

MIT License

Author

Developed by [Adex-dev](https://github.com/adex-dev)
