# cassandra-rs
A Rust implementation of Apache Cassandra, written for educational purposes. This project explores how NoSQL databases work under the hood by implementing core Cassandra features from scratch.

## 📖 About

`cassandra-rs` is a learning-oriented implementation of a Cassandra driver in Rust. Rather than being a production-ready library, it serves as a deep dive into:

- How the **Cassandra Native Protocol** (v4/v5) works
- How binary protocols are serialized and deserialized in Rust
- How connection pooling and async I/O interact with a distributed database
- How CQL (Cassandra Query Language) queries are prepared, bound, and executed
