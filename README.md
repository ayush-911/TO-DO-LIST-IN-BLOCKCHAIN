# 📝 To-Do List on Blockchain (Soroban Smart Contract)
<img width="1920" height="1080" alt="Screenshot (6)" src="https://github.com/user-attachments/assets/73b6bbbf-a0e7-464e-8841-5395178ec584" />


## 📌 Project Description

This project is a decentralized To-Do List built using **Soroban smart contracts** on the **Stellar blockchain**. It allows users to create, manage, and track their tasks securely on-chain without relying on centralized systems.

The purpose of this project is to demonstrate how blockchain technology can be applied to everyday productivity tools while ensuring transparency, immutability, and user ownership of data.

---

## ⚙️ What It Does

* Allows users to add tasks to their personal to-do list
* Stores tasks securely on the Stellar blockchain
* Enables users to retrieve all their tasks anytime
* Allows users to mark tasks as completed

Each user's tasks are stored independently and require authentication for modification, ensuring security and privacy.

---

## ✨ Features

* 🔐 User-authenticated task management
* 📦 Decentralized on-chain storage
* ✅ Mark tasks as completed
* 📄 Retrieve all tasks anytime
* 👤 Per-user task isolation
* ⚡ Built using Soroban (Rust-based smart contracts)

---

## 🔗 Deployed Smart Contract Link

**Contract ID:**
CBV44ENXTHJQTQMNA2LQOS6O3ADSD3FDUKHV636HMMZJ3PGTBUQIMYX6

**Explorer Link:**
https://lab.stellar.org/r/testnet/contract/CBV44ENXTHJQTQMNA2LQOS6O3ADSD3FDUKHV636HMMZJ3PGTBUQIMYX6

---

## 🚀 Tech Stack

* Rust (Soroban SDK)
* Stellar Blockchain (Testnet)

---

## 🛠️ Smart Contract Functions

### ➤ `add_task`

Adds a new task for a user.

**Parameters:**

* `user` → Address of the user
* `content` → Task description

---

### ➤ `get_tasks`

Fetches all tasks for a user.

**Parameters:**

* `user` → Address of the user

---

### ➤ `complete_task`

Marks a task as completed.

**Parameters:**

* `user` → Address of the user
* `task_id` → ID of the task

---

## 📖 How to Use

### 1. Add Task

```bash
stellar contract invoke \
  --id CBV44ENXTHJQTQMNA2LQOS6O3ADSD3FDUKHV636HMMZJ3PGTBUQIMYX6 \
  --source-account alice \
  --network testnet \
  -- add_task \
  --user alice \
  --content "task1"
```

---

### 2. Get Tasks

```bash
stellar contract invoke \
  --id CBV44ENXTHJQTQMNA2LQOS6O3ADSD3FDUKHV636HMMZJ3PGTBUQIMYX6 \
  --source-account alice \
  --network testnet \
  -- get_tasks \
  --user alice
```

---

### 3. Complete Task

```bash
stellar contract invoke \
  --id CBV44ENXTHJQTQMNA2LQOS6O3ADSD3FDUKHV636HMMZJ3PGTBUQIMYX6 \
  --source-account alice \
  --network testnet \
  -- complete_task \
  --user alice \
  --task_id 1
```

---

## 🔮 Future Improvements

* 🗑️ Task deletion feature
* ⏰ Task deadlines and timestamps
* 🔥 Priority levels for tasks
* 🌐 Frontend UI (React + Stellar SDK)
* 📱 Mobile-friendly interface

---

## 🔮 Future Improvements
- Task deletion  
- Deadlines  
- Frontend UI  

---

## 👨‍💻 Author

Ayush Kr Trivedi

---

## 📜 License

This project is open-source and available for educational and development purposes.
=======

