# 🦀 Rust RAG AI

A simple **Retrieval-Augmented Generation (RAG)** project built in **Rust**.  
It connects to an LLM (OpenAI or Hugging Face) and can answer questions either:
- Using **documents** (RAG mode), or  
- Using **direct LLM chat** (no retrieval).  

---

## 🚀 Features
- Written in **Rust**
- Uses **async (Tokio) runtime**
- Supports **OpenAI API** (customizable for Hugging Face too)
- Simple **CLI interface** for:
  - Indexing documents
  - Chatting with the LLM
  - Running in pure LLM mode (no files)

---

## 🛠️ Setup

### 1. Install Rust
If you don’t have Rust yet:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

 *** ###2. Clone this repository ***

git clone https://github.com/monikamunusamy/rust-rag-ai.git
cd rust-rag-ai




