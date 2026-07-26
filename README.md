# Rust Notes Manager CLI

A command-line note management application built with **Rust** using **Domain-Driven Design (DDD)** principles.

This project demonstrates how to build a layered backend application with secure authentication, SQLite persistence, repository abstractions, and an interactive command-line interface. While the application runs entirely in the terminal, the primary goal is to showcase backend architecture and clean code organization.

---

## Features

### User Management

* Register a new user
* Secure user login
* Password hashing using Argon2
* Session management
* Change password
* Logout

### Note Management

* Create notes
* View all notes
* View a single note
* Update notes
* Delete notes

### Persistence

* SQLite database
* Automatic database initialization
* Repository pattern separating domain logic from infrastructure

### Command Line Interface

* Interactive menus
* Secure password input
* Colorized terminal output
* Simple and intuitive navigation

---

# Architecture

The project follows a layered Domain-Driven Design architecture.

```text
CLI Layer
    │
    ▼
Application Services
    │
    ▼
Domain Layer
    │
    ▼
Repository Traits
    │
    ▼
Infrastructure
    │
    ▼
SQLite Database
```

Each layer has a single responsibility and depends only on the layer beneath it through abstractions.

---

# Project Structure

```text
src/
│
├── app.rs
│
├── auth/
│   ├── hasher.rs
│   ├── service.rs
│   └── session.rs
│
├── cli/
│   ├── commands.rs
│   ├── input.rs
│   ├── menu.rs
│   ├── output.rs
│   ├── router.rs
│   └── mod.rs
│
├── domains/
│   ├── user/
│   │   ├── builder.rs
│   │   ├── commands.rs
│   │   ├── dto.rs
│   │   ├── entity.rs
│   │   ├── repo.rs
│   │   ├── service.rs
│   │   └── mod.rs
│   │
│   └── note/
│       ├── builder.rs
│       ├── commands.rs
│       ├── dto.rs
│       ├── entity.rs
│       ├── repo.rs
│       ├── service.rs
│       └── mod.rs
│
├── infrastructure/
│   ├── database/
│   └── storage/
│
└── shared/
    ├── validators.rs
    ├── traits.rs
    ├── errors.rs
    ├── utils.rs
    └── mod.rs
```

---

# Design Principles

This project emphasizes separation of concerns.

## Domain Layer

Contains the business rules and application models.

* Entities
* Builders
* DTOs
* Repository interfaces
* Services

---

## Infrastructure Layer

Responsible for persistence.

It contains SQLite implementations of the repository traits defined by the domain. This keeps the domain independent of the database implementation.

---

## Authentication Layer

Handles authentication concerns independently of the domain.

Responsibilities include:

* Password hashing
* Password verification
* Session management

---

## CLI Layer

Acts as the presentation layer.

Responsibilities include:

* Displaying menus
* Reading user input
* Displaying output
* Routing commands

The CLI contains no business logic.

---

## Shared Layer

Contains reusable components used throughout the application.

Examples include:

* Validators
* Utility functions
* Shared traits
* Common errors

---

# Technologies

* Rust
* SQLite
* Rusqlite
* UUID
* Chrono
* Argon2
* Dialoguer
* Colored

---

# Running the Project

Clone the repository:

```bash
git clone <repository-url>
```

Enter the project:

```bash
cd rust-notes-cli
```

Build the project:

```bash
cargo build
```

Run the application:

```bash
cargo run
```

---

# Example Workflow

```text
Register
      ↓
Login
      ↓
Create Note
      ↓
View Notes
      ↓
Update Note
      ↓
Delete Note
      ↓
Change Password
      ↓
Logout
```

---

# Key Rust Concepts Demonstrated

* Ownership and Borrowing
* Lifetimes
* Traits
* Generic Programming
* Builder Pattern
* Repository Pattern
* Domain-Driven Design
* Error Handling with Result
* Dependency Injection
* Module Organization

---

# Future Improvements

Possible future enhancements include:

* Note search
* Tags
* Categories
* Markdown support
* Note encryption
* Pagination
* Export to Markdown or PDF
* Configuration files
* Comprehensive unit tests
* Integration tests
* Logging
* Undo functionality

---

# License

This project is provided for educational and portfolio purposes.

---

# Author

**Amin**

GitHub: https://github.com/Aminszn
