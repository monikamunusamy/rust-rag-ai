# 🦀 Rust RAG AI

## 🤖 Minimal Retrieval-Augmented Generation (RAG) in Rust

This project demonstrates how to build a **Retrieval-Augmented Generation (RAG)** system completely in **Rust**, powered by **OpenAI** or **Hugging Face** models.  

The idea is simple:  
📚 Store documents → 🔍 Retrieve the most relevant chunks → 💬 Get AI-powered answers.  

---

## ✨ Why this project?
- 🦀 100% **Rust-based** with async/await  
- 📂 Works with your own **documents**  
- ⚡ Fast & lightweight RAG pipeline  
- 🔑 Easy to extend into **chatbots**, **APIs**, or **research tools**

---

## 🛠️ Setup

### 1️⃣ Install Rust
If you don’t have Rust yet:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### **2️⃣ Clone this repository**
```bash
git clone https://github.com/monikamunusamy/rust-rag-ai.git
cd rust-rag-ai
```

### **3️⃣ Set your API key**
Create a .env file in the project root:
```bash
OPENAI_API_KEY=sk-xxxxxx
```
### **▶️ Usage**
📥 Index your data
```bash
cargo run -- index
```

### **💬 Ask questions**
```bash
cargo run -- chat "What is Rust borrowing?"
```


### **📂 Project Structure**

rust-rag-ai/
├── Cargo.toml      # Rust dependencies
├── src/
│   └── main.rs     # Main entry point
├── data/           # Example documents
├── index/          # Vector index files
└── .env            # API keys (ignored in git)

### **📝 Example**
```bash

$ cargo run -- chat "Explain Rust borrowing"
Answer: In Rust, borrowing lets you reference data without taking ownership...
```

### **👩‍💻 About the Author**

This project is built by Monika Munusamy 🌸, exploring the intersection of AI and Rust systems programming.

🔗 Connect with me on:

GitHub

LinkedIn

🚀 Let’s build smarter AI systems — open, efficient, and scalable!

