# 📸 Project API Guide

API untuk mengelola portfolio projects dengan upload image.

---

## 🎯 Features

- ✅ Upload image ke server
- ✅ Create project dengan image URL
- ✅ Get all projects
- ✅ Filter projects by category
- ✅ Get project by ID
- ✅ Update project
- ✅ Delete project
- ✅ Serve static images

---

## 📋 Setup Database

Jalankan SQL script untuk membuat tabel projects:

```bash
psql -U postgres -d postgres -f be/setup_projects_table.sql
```

Atau buka DBeaver dan jalankan:

```sql
CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    image VARCHAR(500) NOT NULL,
    category VARCHAR(100) NOT NULL,
    created_at VARCHAR(50),
    updated_at VARCHAR(50)
);

CREATE INDEX IF NOT EXISTS idx_projects_category ON projects(category);
```

---

## 🚀 API Endpoints

### Base URL
```
http://localhost:8080/api
```

### 1. Upload Image

**Endpoint:** `POST /api/projects/upload`

**Content-Type:** `multipart/form-data`

**Request:**
```bash
curl -X POST http://localhost:8080/api/projects/upload \
  -F "file=@/path/to/image.jpg"
```

**Response:**
```json
{
  "status": "success",
  "message": "Image uploaded successfully",
  "data": {
    "filename": "abc123.jpg",
    "url": "/uploads/abc123.jpg"
  }
}
```

---

### 2. Create Project

**Endpoint:** `POST /api/projects`

**Content-Type:** `application/json`

**Request Body:**
```json
{
  "name": "Design Poster Event",
  "description": "Design poster untuk event kampus menggunakan Photoshop dengan tema modern",
  "image": "/uploads/poster1.jpg",
  "category": "design_&_ui/ux"
}
```

**Response:**
```json
{
  "status": "success",
  "message": "Project added successfully",
  "data": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "name": "Design Poster Event",
    "description": "Design poster untuk event kampus...",
    "image": "/uploads/poster1.jpg",
    "category": "design_&_ui/ux",
    "created_at": "10/05/2025",
    "updated_at": "10/05/2025"
  }
}
```

---

### 3. Get All Projects

**Endpoint:** `GET /api/projects`

**Response:**
```json
{
  "status": "success",
  "message": "5 projects found",
  "data": [
    {
      "id": "...",
      "name": "Design Poster Event",
      "description": "...",
      "image": "/uploads/poster1.jpg",
      "category": "design_&_ui/ux",
      "created_at": "10/05/2025",
      "updated_at": "10/05/2025"
    },
    ...
  ]
}
```

---

### 4. Filter Projects by Category

**Endpoint:** `GET /api/projects?category={category}`

**Categories:**
- `web` - Web Development
- `mobile` - Mobile Development
- `design_&_ui/ux` - Design & UI/UX

**Example:**
```bash
curl http://localhost:8080/api/projects?category=web
```

**Response:**
```json
{
  "status": "success",
  "message": "3 projects found",
  "data": [...]
}
```

---

### 5. Get Project by ID

**Endpoint:** `GET /api/projects/{id}`

**Example:**
```bash
curl http://localhost:8080/api/projects/123e4567-e89b-12d3-a456-426614174000
```

**Response:**
```json
{
  "status": "success",
  "message": "Project found",
  "data": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "name": "Design Poster Event",
    ...
  }
}
```

---

### 6. Update Project

**Endpoint:** `PUT /api/projects/{id}`

**Content-Type:** `application/json`

**Request Body (all fields optional):**
```json
{
  "name": "Updated Project Name",
  "description": "Updated description",
  "image": "/uploads/new-image.jpg",
  "category": "web"
}
```

**Response:**
```json
{
  "status": "success",
  "message": "Project updated successfully",
  "data": {
    "id": "...",
    "name": "Updated Project Name",
    ...
  }
}
```

---

### 7. Delete Project

**Endpoint:** `DELETE /api/projects/{id}`

**Example:**
```bash
curl -X DELETE http://localhost:8080/api/projects/123e4567-e89b-12d3-a456-426614174000
```

**Response:**
```json
{
  "status": "success",
  "message": "Project deleted successfully",
  "data": null
}
```

---

### 8. Access Uploaded Images

**Endpoint:** `GET /uploads/{filename}`

**Example:**
```
http://localhost:8080/uploads/poster1.jpg
```

Images are served as static files and can be accessed directly in browser or used in `<img>` tags.

---

## 🧪 Testing

### Using HTML Tester (Recommended)

1. Jalankan backend:
   ```bash
   cd be
   cargo run
   ```

2. Buka file `be/test_project_api.html` di browser

3. Test semua endpoints dengan UI yang user-friendly

### Using curl

```bash
# 1. Upload image
curl -X POST http://localhost:8080/api/projects/upload \
  -F "file=@image.jpg"

# 2. Create project
curl -X POST http://localhost:8080/api/projects \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Project",
    "description": "Test description",
    "image": "/uploads/image.jpg",
    "category": "web"
  }'

# 3. Get all projects
curl http://localhost:8080/api/projects

# 4. Filter by category
curl http://localhost:8080/api/projects?category=web

# 5. Get by ID
curl http://localhost:8080/api/projects/{id}

# 6. Update
curl -X PUT http://localhost:8080/api/projects/{id} \
  -H "Content-Type: application/json" \
  -d '{"name": "Updated Name"}'

# 7. Delete
curl -X DELETE http://localhost:8080/api/projects/{id}
```

---

## 📊 Data Structure

### Project Model

```typescript
{
  id: string;              // UUID
  name: string;            // Project name
  description: string;     // Project description
  image: string;           // Image URL (e.g., "/uploads/image.jpg")
  category: string;        // "web" | "mobile" | "design_&_ui/ux"
  created_at: string;      // Format: "DD/MM/YYYY"
  updated_at: string;      // Format: "DD/MM/YYYY"
}
```

### Categories

1. **web** - Web Development projects
2. **mobile** - Mobile Development projects
3. **design_&_ui/ux** - Design & UI/UX projects

---

## 🔧 Integration with Frontend

### Example: Fetch Projects in SolidJS

```typescript
// services/ProjectService.ts
export interface Project {
  id: string;
  name: string;
  description: string;
  image: string;
  category: string;
  created_at?: string;
  updated_at?: string;
}

export class ProjectService {
  private static baseUrl = 'http://localhost:8080/api';
  
  static async getAllProjects(): Promise<Project[]> {
    const response = await fetch(`${this.baseUrl}/projects`);
    const result = await response.json();
    return result.data || [];
  }
  
  static async getProjectsByCategory(category: string): Promise<Project[]> {
    const response = await fetch(`${this.baseUrl}/projects?category=${category}`);
    const result = await response.json();
    return result.data || [];
  }
  
  static async createProject(data: Omit<Project, 'id' | 'created_at' | 'updated_at'>): Promise<Project> {
    const response = await fetch(`${this.baseUrl}/projects`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data)
    });
    const result = await response.json();
    return result.data;
  }
  
  static async uploadImage(file: File): Promise<string> {
    const formData = new FormData();
    formData.append('file', file);
    
    const response = await fetch(`${this.baseUrl}/projects/upload`, {
      method: 'POST',
      body: formData
    });
    const result = await response.json();
    return result.data.url;
  }
}
```

### Example: Display Projects

```typescript
// components/Projects.tsx
import { createSignal, createEffect, For } from 'solid-js';
import { ProjectService, type Project } from '../services/ProjectService';

const Projects = () => {
  const [projects, setProjects] = createSignal<Project[]>([]);
  const [category, setCategory] = createSignal<string>('all');
  
  createEffect(async () => {
    const cat = category();
    const data = cat === 'all' 
      ? await ProjectService.getAllProjects()
      : await ProjectService.getProjectsByCategory(cat);
    setProjects(data);
  });
  
  return (
    <div>
      <select onChange={(e) => setCategory(e.target.value)}>
        <option value="all">All Projects</option>
        <option value="web">Web Development</option>
        <option value="mobile">Mobile Development</option>
        <option value="design_&_ui/ux">Design & UI/UX</option>
      </select>
      
      <For each={projects()}>
        {(project) => (
          <div>
            <img src={`http://localhost:8080${project.image}`} alt={project.name} />
            <h3>{project.name}</h3>
            <p>{project.description}</p>
            <span>{project.category}</span>
          </div>
        )}
      </For>
    </div>
  );
};
```

---

## 🐛 Troubleshooting

### Error: "No file uploaded"
- Pastikan menggunakan `multipart/form-data`
- Pastikan field name adalah `file`

### Error: "Error creating file"
- Folder `uploads` tidak ada atau tidak punya permission
- Backend akan otomatis create folder, tapi pastikan ada write permission

### Error: "Project not found"
- ID project salah atau tidak ada di database
- Cek dengan GET all projects untuk lihat ID yang valid

### Images tidak muncul
- Pastikan backend berjalan
- Cek URL image: `http://localhost:8080/uploads/filename.jpg`
- Pastikan file ada di folder `be/uploads/`

---

## ✅ Checklist

- [ ] Database table `projects` sudah dibuat
- [ ] Backend berjalan di port 8080
- [ ] Folder `uploads` sudah dibuat (otomatis saat upload pertama)
- [ ] Test upload image berhasil
- [ ] Test create project berhasil
- [ ] Test get all projects berhasil
- [ ] Test filter by category berhasil
- [ ] Image bisa diakses via browser

---

## 📝 Notes

- Image disimpan di folder `be/uploads/`
- Filename di-sanitize untuk keamanan
- Jika upload file dengan nama sama, akan overwrite
- Format date: DD/MM/YYYY (e.g., "10/05/2025")
- Category harus exact match (case-sensitive)

---

**Selamat! API Project sudah siap digunakan!** 🎉

Test dengan `be/test_project_api.html` untuk memastikan semua berfungsi dengan baik.
