# Employee Management Backend - Rust + PostgreSQL

Backend API untuk Employee Management System menggunakan Actix-Web dan PostgreSQL.

## 🚀 Setup Database PostgreSQL

### 1. Pastikan PostgreSQL sudah terinstall dan berjalan

Cek apakah PostgreSQL sudah berjalan:
```bash
# Windows
pg_ctl status

# Atau cek service
services.msc
# Cari "postgresql" dan pastikan statusnya "Running"
```

### 2. Buat Database (jika belum ada)

Buka DBeaver atau psql dan jalankan:
```sql
-- Jika ingin buat database baru (opsional)
CREATE DATABASE employee_db;

-- Atau gunakan database 'postgres' yang sudah ada
```

### 3. Setup Tabel Employees

Jalankan script SQL yang ada di file `setup_database.sql`:

**Cara 1: Menggunakan DBeaver**
1. Buka DBeaver
2. Connect ke database PostgreSQL kamu (localhost:5432)
3. Klik kanan pada database "postgres" → SQL Editor → New SQL Script
4. Copy paste isi file `setup_database.sql`
5. Klik Execute (atau tekan Ctrl+Enter)

**Cara 2: Menggunakan psql**
```bash
# Masuk ke psql
psql -U postgres -d postgres

# Jalankan script
\i setup_database.sql

# Atau langsung dari command line
psql -U postgres -d postgres -f setup_database.sql
```

### 4. Konfigurasi Environment Variables

Edit file `.env` sesuai dengan konfigurasi PostgreSQL kamu:

```env
PG_HOST=localhost
PG_USER=postgres
PG_PASS=         # Isi dengan password PostgreSQL kamu (kosongkan jika tidak ada password)
PG_DB=postgres   # Atau ganti dengan nama database yang kamu buat
PG_PORT=5432
```

**PENTING:** Jika PostgreSQL kamu menggunakan password, isi `PG_PASS` dengan password yang benar!

## 🔧 Menjalankan Backend

### 1. Install Dependencies
```bash
cargo build
```

### 2. Jalankan Server
```bash
cargo run
```

Server akan berjalan di: `http://127.0.0.1:8080`

### 3. Test API

**Get All Employees:**
```bash
curl http://localhost:8080/api/employees
```

**Add Employee:**
```bash
curl -X POST http://localhost:8080/api/employees \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test User",
    "position": "Developer",
    "email": "test@example.com"
  }'
```

## 📋 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/employees` | Get all employees |
| GET | `/api/employees/{id}` | Get employee by ID |
| POST | `/api/employees` | Create new employee |
| PUT | `/api/employees/{id}` | Update employee |
| DELETE | `/api/employees/{id}` | Delete employee |

## 🐛 Troubleshooting

### Error: "PG_PASS not set" atau "Connection refused"

**Solusi:**
1. Pastikan PostgreSQL service berjalan
2. Cek username dan password di file `.env`
3. Pastikan port 5432 tidak diblokir firewall

### Error: "relation employees does not exist"

**Solusi:**
Jalankan script `setup_database.sql` untuk membuat tabel employees.

### Error: "password authentication failed"

**Solusi:**
1. Cek password PostgreSQL kamu
2. Update `PG_PASS` di file `.env`
3. Atau reset password PostgreSQL:
```bash
# Windows (run as Administrator)
psql -U postgres
ALTER USER postgres PASSWORD 'password_baru';
```

### Error: "CORS policy"

**Solusi:**
Backend sudah dikonfigurasi untuk menerima request dari:
- http://localhost:3000
- http://localhost:3001
- http://localhost:5173

Pastikan frontend berjalan di salah satu port tersebut.

## 📦 Dependencies

- `actix-web` - Web framework
- `actix-cors` - CORS middleware
- `tokio-postgres` - PostgreSQL driver
- `deadpool-postgres` - Connection pooling
- `serde` - Serialization/deserialization
- `uuid` - UUID generation
- `dotenv` - Environment variables

## 🎯 Struktur Project

```
be/
├── src/
│   ├── config.rs           # Database configuration
│   ├── main.rs             # Application entry point
│   ├── models/             # Data models
│   ├── dtos/               # Data Transfer Objects
│   ├── repositories/       # Database operations
│   ├── services/           # Business logic
│   └── handlers/           # HTTP handlers
├── .env                    # Environment variables
├── setup_database.sql      # Database setup script
└── Cargo.toml             # Rust dependencies
```

## ✅ Checklist Setup

- [ ] PostgreSQL terinstall dan berjalan
- [ ] Database dibuat (atau gunakan database 'postgres')
- [ ] Tabel 'employees' sudah dibuat (jalankan setup_database.sql)
- [ ] File .env sudah dikonfigurasi dengan benar
- [ ] Backend berhasil dijalankan dengan `cargo run`
- [ ] Test API dengan curl atau Postman
- [ ] Frontend sudah terhubung ke backend

## 🔗 Next Steps

Setelah backend berjalan, jalankan frontend:
```bash
cd ../fe
npm install
npm run dev
```

Frontend akan berjalan di: `http://localhost:3001`
