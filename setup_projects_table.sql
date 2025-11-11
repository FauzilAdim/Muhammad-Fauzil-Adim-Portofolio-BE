-- Setup Table Projects untuk Portfolio
-- Jalankan script ini di PostgreSQL kamu

-- Buat tabel projects
CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    image VARCHAR(500) NOT NULL,
    category VARCHAR(100) NOT NULL,
    created_at VARCHAR(50),
    updated_at VARCHAR(50)
);

-- Buat index untuk performa query
CREATE INDEX IF NOT EXISTS idx_projects_category ON projects(category);
CREATE INDEX IF NOT EXISTS idx_projects_created_at ON projects(created_at);

-- Insert sample data (opsional)
INSERT INTO projects (name, description, image, category, created_at, updated_at) VALUES
    ('Design Poster Event', 'Design poster untuk event kampus menggunakan Photoshop dengan tema modern dan colorful', '/uploads/poster1.jpg', 'design_&_ui/ux', '10/05/2025', '10/05/2025'),
    ('E-commerce Website', 'Website e-commerce dengan fitur lengkap menggunakan React dan Node.js', '/uploads/ecommerce.jpg', 'web', '09/05/2025', '09/05/2025'),
    ('Mobile Banking App', 'Aplikasi mobile banking dengan UI/UX modern menggunakan React Native', '/uploads/banking.jpg', 'mobile', '08/05/2025', '08/05/2025')
ON CONFLICT (id) DO NOTHING;

-- Tampilkan data yang sudah ada
SELECT * FROM projects ORDER BY created_at DESC;
