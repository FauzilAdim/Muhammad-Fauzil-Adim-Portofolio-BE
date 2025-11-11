-- Setup Table Projects V2 - Multiple Images Support
-- Drop table lama dan buat yang baru

-- Drop table lama (HATI-HATI: Ini akan hapus semua data!)
DROP TABLE IF EXISTS projects;

-- Buat table baru dengan JSONB untuk multiple images
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    images JSONB NOT NULL,  -- Array of image URLs in JSON format
    category VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Buat index untuk performa
CREATE INDEX idx_projects_category ON projects(category);
CREATE INDEX idx_projects_created_at ON projects(created_at);
CREATE INDEX idx_projects_images ON projects USING GIN (images);  -- GIN index untuk JSONB

-- Insert sample data dengan multiple images
INSERT INTO projects (name, description, images, category) VALUES
    (
        'Brand Identity Design',
        'Complete brand identity design including logo, color palette, typography, and brand guidelines',
        '["uploads/brand-1.jpg", "uploads/brand-2.jpg", "uploads/brand-3.jpg"]'::jsonb,
        'design_&_ui/ux'
    ),
    (
        'E-commerce Platform',
        'Modern e-commerce platform with React and Node.js',
        '["uploads/ecommerce-1.jpg"]'::jsonb,
        'web_development'
    ),
    (
        'Mobile Banking App',
        'Secure mobile banking application with biometric authentication',
        '["uploads/banking-1.jpg", "uploads/banking-2.jpg"]'::jsonb,
        'mobile_development'
    );

-- Query untuk test
SELECT 
    id,
    name,
    category,
    images,
    jsonb_array_length(images) as image_count,
    images->0 as cover_image
FROM projects;

-- Query untuk get cover image (first image)
SELECT 
    id,
    name,
    images->0 as cover_image
FROM projects;

-- Query untuk get all images
SELECT 
    id,
    name,
    jsonb_array_elements_text(images) as image_url
FROM projects;
