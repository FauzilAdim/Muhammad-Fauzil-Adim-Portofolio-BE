# 🔌 Test Koneksi Database

## Cara Cek Password PostgreSQL Kamu

Dari screenshot DBeaver kamu, terlihat ada password yang tersimpan. Untuk mendapatkan password yang benar:

### Opsi 1: Cek di DBeaver
1. Buka DBeaver
2. Klik kanan pada koneksi "postgres" → **Edit Connection**
3. Di tab **Main**, lihat field **Password**
4. Klik icon "Show password" (biasanya icon mata) untuk melihat password
5. Copy password tersebut

### Opsi 2: Coba Password Default
Password default PostgreSQL biasanya:
- `postgres`
- `admin`
- `root`
- Atau password yang kamu set saat install PostgreSQL

### Opsi 3: Reset Password PostgreSQL
Jika lupa password, reset dengan cara:

**Windows:**
1. Buka Command Prompt as Administrator
2. Jalankan:
```bash
psql -U postgres
# Jika diminta password, tekan Enter (atau coba password default)

# Setelah masuk, reset password:
ALTER USER postgres PASSWORD 'password_baru';
```

---

## Update File .env

Setelah tahu password yang benar, edit file `be/.env`:

```env
PG_HOST=localhost
PG_USER=postgres
PG_PASS=password_kamu_disini    # ⚠️ GANTI INI!
PG_DB=postgres
PG_PORT=5432
```

---

## Test Koneksi Backend

Setelah update `.env`, test koneksi dengan menjalankan backend:

```bash
cd be
cargo run
```

### ✅ Jika Berhasil:
Kamu akan lihat:
```
🚀 Server starting on http://127.0.0.1:8080
📊 Database: PostgreSQL
🌐 CORS enabled for frontend
```

### ❌ Jika Gagal:

**Error: "password authentication failed for user postgres"**
- Password di `.env` salah
- Cek password di DBeaver dan update `.env`

**Error: "connection refused"**
- PostgreSQL service tidak berjalan
- Buka Services (services.msc) dan start service PostgreSQL

**Error: "database postgres does not exist"**
- Database belum dibuat
- Buat database di DBeaver atau gunakan database lain

---

## Quick Test dengan psql

Test koneksi langsung dengan psql:

```bash
# Test dengan password dari .env
psql -h localhost -p 5432 -U postgres -d postgres

# Jika berhasil connect, coba query:
SELECT version();
\dt
SELECT * FROM employees;
```

Jika psql berhasil connect dengan password tertentu, gunakan password yang sama di file `.env`!

---

## Troubleshooting

| Masalah | Solusi |
|---------|--------|
| Lupa password | Reset password dengan `ALTER USER postgres PASSWORD 'new_password';` |
| Service tidak jalan | Buka services.msc, start service PostgreSQL |
| Port 5432 diblokir | Cek firewall atau ganti port di `.env` dan `postgresql.conf` |
| Database tidak ada | Buat database baru atau gunakan database 'postgres' |

---

## Next Step

Setelah koneksi berhasil:
1. ✅ Jalankan script `setup_database.sql` untuk membuat tabel
2. ✅ Jalankan backend: `cargo run`
3. ✅ Test API: `curl http://localhost:8080/api/employees`
4. ✅ Jalankan frontend: `npm run dev`
