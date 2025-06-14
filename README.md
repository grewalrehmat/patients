# ⚙️ MedVault-RS — AI-Enhanced Medical Record System (Rust Backend)

A high-performance, memory-safe, and AI-integrated EHR system powered by Rust, built for hospitals, labs, and research centers to securely store, access, and analyze patient data in real-time.

## 🚀 Project Overview

**MedVault-RS** is a full-stack healthcare application that utilizes the power of **Rust** for backend processing, guaranteeing security and speed. It provides:
- Role-based access to patient records.
- AI-assisted diagnostics from lab reports.
- Intern/trainee token-based access, revoked upon exit.
- Mortality rate tracking and departmental analytics.

## Installations
- Clone the repository
  ```bash
  git clone https://github.com/example/patients.git
  ```
- Navigate to the repo
  ```bash
  cd patients/
  ```
-Run 
```bash
cargo run
```
for the program to run

## 🔍 Core Features

- 🧑‍⚕️ Doctor/Trainee/Admin login system.
- 📁 Secure patient records & lab data storage.
- 📑 OCR + NLP-based report parsing.
- 🧠 AI microservice for diagnosis suggestions.
- 📊 Mortality rate computation & dashboard.
- 🔐 Dynamic access control for interns.

---

## 🧰 Tech Stack

### 💻 Frontend

- **React JS** (Web Dashboard)
- **React Native** (Mobile App)
- **Axios** (API communication)

### 🔧 Backend (Rust)

- **Actix-Web** → High-performance async web framework.
- **Diesel ORM** → SQL-safe and compile-time checked database queries.
- **jsonwebtoken / OAuth2** → For access token generation & auth.
- **serde / serde_json** → JSON serialization.

### 🧠 AI Microservice (Python)

- **Flask or FastAPI**
- **Tesseract OCR** → Extract text from reports.
- **spaCy / HuggingFace Transformers** → Analyze and predict conditions.
- **Returns JSON** with potential diagnoses and insights.

### 🗃️ Data Storage

- **PostgreSQL** → Main patient, report & user database.
- **Redis** → Session management, caching frequent queries.

---

## 🛡️ Access Control

- **Doctors**: Full access to all patients.
- **Trainees**: Access linked to supervisor only.
- **Admin**: CRUD permissions on users + logs.

Interns receive tokens that auto-expire on exit from institution (handled by backend).
