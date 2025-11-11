# ⚡ Railway Deployment - Quick Start

## 🚀 5-Minute Deployment

### **1. Login Railway**
```
https://railway.app → Login with GitHub
```

### **2. New Project**
```
New Project → Deploy from GitHub → Select: Muhammad-Fauzil-Adim-Portofolio-BE
```

### **3. Add Database**
```
+ New → Database → PostgreSQL
```

### **4. Set Environment**
```
Backend Service → Variables → Add:
DATABASE_URL = ${{Postgres.DATABASE_URL}}
```

### **5. Setup Database**
```
PostgreSQL → Data → Query → Run:
```
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
```

### **6. Get URL**
```
Backend Service → Settings → Domains → Generate Domain
Copy: https://your-app.up.railway.app
```

### **7. Update Frontend**
```typescript
// fe/src/services/ProjectService.ts
private static baseUrl = 'https://your-app.up.railway.app';
```

### **8. Test**
```bash
curl https://your-app.up.railway.app/api/projects
```

## ✅ Done!

**Full Guide:** See `RAILWAY_DEPLOYMENT_GUIDE.md`

---

## 🔧 Common Commands

### **View Logs:**
```
Railway Dashboard → Backend Service → Deployments → Latest → Logs
```

### **Redeploy:**
```bash
git push  # Auto-deploys
```

### **Database Query:**
```
PostgreSQL Service → Data → Query
```

---

## 🐛 Quick Fixes

**Build Failed?**
```
Check: Cargo.toml exists
Check: All dependencies listed
```

**Database Error?**
```
Verify: DATABASE_URL is set
Verify: Table created
```

**CORS Error?**
```
Check: Vercel domain in CORS config
Redeploy after changes
```

---

## 💡 Tips

- ✅ Free tier: $5/month credit
- ✅ Auto-deploy on git push
- ✅ Zero-downtime deployments
- ⚠️ Ephemeral storage (files deleted on redeploy)
- 💡 Use cloud storage for uploads (S3, Cloudinary)

---

**Need Help?** Check `RAILWAY_DEPLOYMENT_GUIDE.md` for detailed steps!
