# 🚀 Portfolio Backend API

Backend API untuk Portfolio Website yang dibangun dengan **Rust**, **Actix-Web**, dan **PostgreSQL**.

## 📋 Features

- ✅ **Project Management API** - CRUD operations untuk projects
- ✅ **Multiple Image Upload** - Support upload multiple images per project
- ✅ **Image Order Preservation** - Maintain upload order dengan index prefix
- ✅ **Category Filtering** - Filter projects by category
- ✅ **PostgreSQL Database** - Reliable data storage
- ✅ **CORS Enabled** - Ready untuk frontend integration
- ✅ **Static File Serving** - Serve uploaded images

## 🛠️ Tech Stack

- **Rust** - Programming language
- **Actix-Web** - Web framework
- **PostgreSQL** - Database
- **Tokio** - Async runtime
- **Serde** - Serialization/deserialization
- **UUID** - Unique identifiers
- **Chrono** - Date/time handling

## 📦 Installation

### Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Cargo

### Setup

1. **Clone repository:**
```bash
git clone https://github.com/FauzilAdim/Muhammad-Fauzil-Adim-Portofolio-BE.git
cd Muhammad-Fauzil-Adim-Portofolio-BE
```

2. **Install dependencies:**
```bash
cargo build
```

3. **Setup database:**
```bash
# Create database
createdb portfolio_db

# Run SQL setup
psql -d portfolio_db -f setup_projects_table_v2.sql
```

4. **Configure environment:**
```bash
# Copy .env.example to .env
cp .env.example .env

# Edit .env with your database credentials
```

5. **Run server:**
```bash
cargo run
```

Server akan running di `http://localhost:8080`

## 🔧 Environment Variables

Create `.env` file:

```env
DATABASE_URL=postgresql://username:password@localhost/portfolio_db
RUST_LOG=info
```

## 📡 API Endpoints

### Projects

#### Get All Projects
```http
GET /api/projects
```

#### Get Projects by Category
```http
GET /api/projects?category=web_development
```

Categories:
- `web_development`
- `mobile_development`
- `design_&_ui/ux`

#### Get Project by ID
```http
GET /api/projects/{id}
```

#### Create Project with Upload
```http
POST /api/projects/create-with-upload
Content-Type: multipart/form-data

Fields:
- name: string
- description: string
- category: string
- files: file[] (multiple images)
```

#### Update Project
```http
PUT /api/projects/{id}
Content-Type: application/json

{
  "name": "Project Name",
  "description": "Description",
  "images": ["/uploads/image1.jpg"],
  "category": "web_development"
}
```

#### Delete Project
```http
DELETE /api/projects/{id}
```

## 📁 Project Structure

```
be/
├── src/
│   ├── main.rs              # Entry point
│   ├── models/
│   │   └── project.rs       # Project model
│   ├── dtos/
│   │   └── project_dto.rs   # Data transfer objects
│   ├── handlers/
│   │   └── project_handler.rs  # API handlers
│   ├── repositories/
│   │   └── project_postgres.rs # Database operations
│   └── services/
│       └── project_service.rs  # Business logic
├── uploads/                 # Uploaded images (gitignored)
├── Cargo.toml              # Dependencies
├── .env                    # Environment variables (gitignored)
└── setup_projects_table_v2.sql  # Database schema
```

## 🗄️ Database Schema

```sql
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    images TEXT[] NOT NULL,
    category VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## 🧪 Testing

Test API dengan HTML test file:
```bash
# Open in browser
open test_project_api.html
```

Atau gunakan curl:
```bash
# Get all projects
curl http://localhost:8080/api/projects

# Create project with upload
curl -X POST http://localhost:8080/api/projects/create-with-upload \
  -F "name=My Project" \
  -F "description=Project description" \
  -F "category=web_development" \
  -F "files=@image1.jpg" \
  -F "files=@image2.jpg"
```

## 🚀 Deployment

### Production Build
```bash
cargo build --release
```

### Run Production
```bash
./target/release/employee
```

## 📝 Notes

- **Image Order:** Images are saved with index prefix (`uuid_000_timestamp.jpg`) to preserve upload order
- **First Image:** First uploaded image is used as cover/thumbnail
- **File Storage:** Images stored in `/uploads` directory
- **CORS:** Enabled for frontend at `http://localhost:3001` and `http://localhost:3002`

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is open source and available under the MIT License.

## 👤 Author

**Muhammad Fauzil Adim**

- GitHub: [@FauzilAdim](https://github.com/FauzilAdim)

## 🔗 Related

- Frontend Repository: [Portfolio Frontend](https://github.com/FauzilAdim/Muhammad-Fauzil-Adim-Portofolio-FE)

---

Made with ❤️ using Rust & Actix-Web
