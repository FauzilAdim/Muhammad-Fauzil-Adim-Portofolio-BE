-- Setup Database untuk Employee Management System
-- Jalankan script ini di PostgreSQL kamu

-- Buat tabel employees jika belum ada
CREATE TABLE IF NOT EXISTS employees (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    position VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Buat index untuk performa query
CREATE INDEX IF NOT EXISTS idx_employees_email ON employees(email);
CREATE INDEX IF NOT EXISTS idx_employees_position ON employees(position);

-- Insert sample data (opsional)
INSERT INTO employees (name, position, email) VALUES
    ('Muhammad Fauzil Adim', 'Fullstack Developer', 'fauzil@example.com'),
    ('John Doe', 'Frontend Developer', 'john@example.com'),
    ('Jane Smith', 'Backend Developer', 'jane@example.com')
ON CONFLICT (email) DO NOTHING;

-- Tampilkan data yang sudah ada
SELECT * FROM employees;
