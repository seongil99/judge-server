# Judge Server for Maple Judge Platform

A high-performance, secure code execution and evaluation system built in Rust for the Maple Judge competitive programming platform. This microservice handles the critical task of safely executing and judging submitted code against test cases while providing detailed feedback and performance metrics.

## Overview

The Judge Server is a core component of the Maple Judge ecosystem that provides:

**🔒 Secure Code Execution**
- Sandboxed execution environment using Linux seccomp for maximum security isolation
- Prevents malicious code from accessing system resources or escaping the container
- Memory and CPU time limits enforcement to prevent resource exhaustion

**⚡ High-Performance Judging**
- Asynchronous message processing via RabbitMQ for scalable submission handling
- Concurrent execution support for multiple submissions
- Efficient test case evaluation with detailed verdict reporting

**📊 Comprehensive Evaluation**
- Multiple verdict types: Accepted (AC), Wrong Answer (WA), Time Limit Exceeded (TLE), Memory Limit Exceeded (MLE), Runtime Error (RE), Compilation Error (CE)
- Precise execution time and memory usage tracking
- Support for multiple programming languages and compilation processes

**🎯 Real-time Communication**
- Message-driven architecture using RabbitMQ for decoupled, scalable processing
- JSON-based submission and result protocols
- Automatic result publishing back to the platform

## Architecture

The system consists of several key components:

- **Consumer Module** (`consumer.rs`): Handles incoming submission requests from RabbitMQ
- **Executor Module** (`executor.rs`): Manages secure code compilation and execution
- **Judge Module** (`judge.rs`): Evaluates program output against expected results
- **Filter Module** (`filter.rs`): Implements seccomp-based security filtering
- **Publisher Module** (`publisher.rs`): Sends judgment results back to the platform

## Project Structure

```
judge-server/
├── 📁 src/
│   ├── 🦀 main.rs           # Application entry point
│   ├── 📥 consumer.rs       # RabbitMQ message consumer
│   ├── ⚙️  executor.rs       # Code compilation & execution
│   ├── ⚖️  judge.rs          # Test case evaluation logic
│   ├── 🛡️  filter.rs        # Seccomp security filtering
│   └── 📤 publisher.rs      # Result publishing to queue
├── 📁 test_cases/
│   ├── 📁 input/            # Test case input files
│   ├── 📁 output/           # Expected output files
│   └── 📁 result/           # Execution results
├── 📁 result/               # Runtime metrics (time/memory)
├── 🐳 Dockerfile           # Production container image
├── 🔧 Dockerfile.develop   # Development container image
├── 🐙 docker-compose.yml   # Multi-service orchestration
├── 📦 Cargo.toml           # Rust dependencies & metadata
├── 🐍 sender.py            # Test submission sender
├── 🐍 receiver.py          # Test result receiver
└── 📖 README.md            # Project documentation

Flow Diagram:
┌─────────────┐    ┌──────────────┐    ┌─────────────┐
│   RabbitMQ  │───▶│   Consumer   │───▶│  Executor   │
│   (Queue)   │    │ (Receives)   │    │ (Compiles)  │
└─────────────┘    └──────────────┘    └─────────────┘
                                              │
                                              ▼
┌─────────────┐    ┌──────────────┐    ┌─────────────┐
│  Publisher  │◀───│    Judge     │◀───│   Sandbox   │
│ (Results)   │    │ (Evaluates)  │    │ (Executes)  │
└─────────────┘    └──────────────┘    └─────────────┘
```

## Technologies

- **Rust**: High-performance systems programming language for memory safety and concurrency
- **RabbitMQ (lapin)**: Asynchronous message broker for reliable submission queuing
- **seccompiler**: Linux security framework for system call filtering and sandboxing
- **Docker**: Containerization for consistent deployment and isolation
- **Serde**: Efficient serialization/deserialization for JSON message handling
- **Tracing**: Structured logging and observability

## Setup

### Prerequisites

- Rust (latest stable version)
- Docker and Docker Compose
- RabbitMQ server

### Installation

1. Clone the repository
2. Build the project:
   ```
   cargo build --release
   ```
3. Or use Docker:
   ```
   docker-compose up -d
   ```

## Usage

The judge server connects to a RabbitMQ instance and listens for submission requests. Each request should include the necessary information about the submission (code, language, problem ID, etc.).

For development purposes, you can use the provided `sender.py` and `receiver.py` scripts to test the functionality.

