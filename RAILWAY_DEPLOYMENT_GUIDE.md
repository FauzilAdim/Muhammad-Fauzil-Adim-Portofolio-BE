# 🚂 Railway Deployment Guide

Panduan lengkap untuk deploy Portfolio Backend API ke Railway.app

---

## 📋 Prerequisites

- ✅ GitHub account
- ✅ Railway account (sign up di https://railway.app)
- ✅ Backend code sudah di-push ke GitHub
- ✅ Domain Vercel frontend (untuk CORS)

---

## 🚀 Step-by-Step Deployment

### **Step 1: Sign Up / Login ke Railway**

1. Buka https://railway.app
2. Click **"Login"** atau **"Start a New Project"**
3. Login dengan **GitHub account**
4. Authorize Railway untuk access GitHub repos

---

### **Step 2: Create New Project**

1. Di Railway dashboard, click **"New Project"**
2. Pilih **"Deploy from GitHub repo"**
3. Select repository: **`Muhammad-Fauzil-Adim-Portofolio-BE`**
4. Railway akan otomatis detect Rust project

---

### **Step 3: Add PostgreSQL Database**

1. Di project dashboard, click **"+ New"**
2. Pilih **"Database"**
3. Select **"Add PostgreSQL"**
4. Railway akan create PostgreSQL instance
5. Wait sampai database ready (status: Active)

---

### **Step 4: Setup Environment Variables**

1. Click pada **Backend service** (bukan database)
2. Go to **"Variables"** tab
3. Click **"+ New Variable"**
4. Add variable:

```
DATABASE_URL = ${{Postgres.DATABASE_URL}}
```

**Cara:**
- Ketik `DATABASE_URL` di field Name
- Ketik `${{Postgres.DATABASE_URL}}` di field Value
- Click **"Add"**

**Optional variables:**
```
RUST_LOG = info
PORT = 8080
```

5. Click **"Deploy"** untuk apply changes

---

### **Step 5: Setup Database Schema**

Setelah deployment pertama selesai:

1. Go to **PostgreSQL service**
2. Click **"Data"** tab
3. Click **"Query"** atau **"Connect"**
4. Copy isi file `setup_projects_table_v2.sql`
5. Paste dan **Execute** query:

```sql
CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    images TEXT[] NOT NULL,
    category VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_projects_category ON projects(category);
CREATE INDEX idx_projects_created_at ON projects(created_at DESC);
```

6. Verify table created:
```sql
SELECT * FROM projects;
```

---

### **Step 6: Get Deployment URL**

1. Go back to **Backend service**
2. Click **"Settings"** tab
3. Scroll to **"Domains"** section
4. Click **"Generate Domain"**
5. Railway akan generate URL seperti:
   ```
   https://your-app-name.up.railway.app
   ```
6. **Copy URL ini** - kamu akan butuh untuk frontend!

---

### **Step 7: Test API**

Test API dengan curl atau browser:

```bash
# Test health check
curl https://your-app-name.up.railway.app/api/projects

# Should return:
{
  "status": "success",
  "message": "Projects retrieved successfully",
  "data": []
}
```

---

### **Step 8: Update Frontend**

Update frontend untuk connect ke Railway backend:

**File: `fe/src/services/ProjectService.ts`**

```typescript
export class ProjectService {
  // Change this to your Railway URL
  private static baseUrl = 'https://your-app-name.up.railway.app';
  
  // ... rest of code
}
```

**Commit dan push ke Vercel:**
```bash
cd fe
git add .
git commit -m "feat: Connect to Railway backend"
git push
```

Vercel akan auto-deploy dengan backend URL baru!

---

## 🔧 Configuration Details

### **Environment Variables Explained:**

| Variable | Value | Description |
|----------|-------|-------------|
| `DATABASE_URL` | `${{Postgres.DATABASE_URL}}` | PostgreSQL connection string |
| `RUST_LOG` | `info` | Logging level |
| `PORT` | Auto-set by Railway | Server port |

### **Railway Auto-Detection:**

Railway otomatis detect:
- ✅ Rust project (via `Cargo.toml`)
- ✅ Build command: `cargo build --release`
- ✅ Start command: `./target/release/employee`
- ✅ Port binding: `0.0.0.0:$PORT`

---

## 📊 Monitoring & Logs

### **View Logs:**
1. Go to Backend service
2. Click **"Deployments"** tab
3. Click latest deployment
4. View **"Build Logs"** dan **"Deploy Logs"**

### **View Metrics:**
1. Go to Backend service
2. Click **"Metrics"** tab
3. See CPU, Memory, Network usage

---

## 🐛 Troubleshooting

### **Problem: Build Failed**

**Solution:**
1. Check build logs
2. Verify `Cargo.toml` is correct
3. Ensure all dependencies are in `Cargo.toml`

### **Problem: Database Connection Error**

**Solution:**
1. Verify `DATABASE_URL` variable is set
2. Check PostgreSQL service is running
3. Verify database schema is created

### **Problem: CORS Error**

**Solution:**
1. Verify Vercel domain is allowed in CORS
2. Check `src/main.rs` CORS configuration
3. Redeploy after changes

### **Problem: 502 Bad Gateway**

**Solution:**
1. Check if app is listening on `0.0.0.0:$PORT`
2. Verify `PORT` environment variable
3. Check deploy logs for errors

---

## 💰 Railway Free Tier

**Free Plan Includes:**
- ✅ $5 free credit per month
- ✅ 500 hours execution time
- ✅ 1GB RAM
- ✅ 1GB disk
- ✅ Shared CPU

**Estimated Usage:**
- Small portfolio: ~$3-5/month
- Free tier is enough for hobby projects!

**Monitor Usage:**
1. Go to **"Usage"** tab
2. Check credit consumption
3. Set up billing alerts

---

## 🔄 Auto-Deploy

Railway auto-deploys on every push to `main` branch:

```bash
# Make changes
git add .
git commit -m "feat: Add new feature"
git push

# Railway will automatically:
# 1. Pull latest code
# 2. Build Rust app
# 3. Deploy new version
# 4. Zero-downtime deployment
```

---

## 📁 File Storage (Important!)

**⚠️ Railway uses ephemeral storage!**

Uploaded files will be **deleted** on redeploy.

**Solutions:**

### **Option 1: Use Cloud Storage (Recommended)**
- AWS S3
- Cloudinary
- Supabase Storage
- Vercel Blob

### **Option 2: Railway Volumes**
1. Add persistent volume
2. Mount to `/uploads`
3. Costs extra

**For now:** Test dengan ephemeral storage, nanti migrate ke cloud storage.

---

## ✅ Deployment Checklist

- [ ] Railway account created
- [ ] GitHub repo connected
- [ ] PostgreSQL database added
- [ ] Environment variables set
- [ ] Database schema created
- [ ] Deployment successful
- [ ] API tested (GET /api/projects)
- [ ] Frontend updated with Railway URL
- [ ] CORS working
- [ ] Test upload project

---

## 🎉 Success!

Jika semua steps berhasil:

✅ Backend running di Railway  
✅ PostgreSQL database active  
✅ API accessible via HTTPS  
✅ Frontend connected  
✅ CORS configured  
✅ Auto-deploy enabled  

**Your Portfolio is LIVE!** 🚀

---

## 📞 Support

**Railway Docs:** https://docs.railway.app  
**Railway Discord:** https://discord.gg/railway  
**GitHub Issues:** Create issue di repository  

---

## 🔗 Quick Links

- **Railway Dashboard:** https://railway.app/dashboard
- **Backend Repo:** https://github.com/FauzilAdim/Muhammad-Fauzil-Adim-Portofolio-BE
- **Railway Docs:** https://docs.railway.app/deploy/deployments

---

Made with ❤️ for easy deployment!
