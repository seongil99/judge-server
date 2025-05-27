# Judge Server for Maple Judge Platform

A secure and efficient code execution and judging system written in Rust, designed to evaluate programming submissions for competitive programming contests and online coding platforms.

## Overview

This judge server:
- Receives submission requests via RabbitMQ
- Executes submitted code in a secure sandbox environment using seccomp
- Evaluates code against predefined test cases
- Returns detailed verdict information (Accepted, Wrong Answer, Time Limit Exceeded, etc.)
- Tracks and reports execution metrics (time and memory usage)

## Technologies

- **Rust**: Core programming language
- **RabbitMQ**: Message broker for communication
- **seccompiler**: Provides secure sandboxing for code execution
- **Docker**: Containerization for deployment

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

