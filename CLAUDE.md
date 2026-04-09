# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This directory (`notebooks/lab/`) is a learning workspace containing educational code, experiments, and course materials organized by topic. It functions as a lab environment for studying various programming languages, AI/ML concepts, and operating systems. Each subdirectory is independent with its own build system and dependencies.

## Directory Structure

- **`ai/`** – AI and machine learning experiments
  - `agent/` – AI agent implementations (Python with `pyproject.toml`)
  - `coursera-ml/` – Coursera Machine Learning course materials (git submodule)
  - `coursera-dl/` – Coursera Deep Learning course materials (git submodule)
- **`lang/`** – Programming language learning exercises
  - `cpp/` – C++ assignments (CS106L course, git submodule)
  - `golang/` – Go language exercises (simple programs with `go.mod`)
  - `rust/` – Rust language exercises (Cargo projects)
- **`os/`** – Operating systems study materials  
  - `ostep-code/` – Code examples from "Operating Systems: Three Easy Pieces" (git submodule)
  - `unix/` – Unix v1/v6 source code study and tools (git submodule)
- **`misc/`** – Miscellaneous code snippets and assignments

## Build Systems and Dependencies

Each category uses different build tools:

### C/C++ (OSTEP and Unix code)
- **Build command**: `make` (in each directory with a Makefile)
- **Clean command**: `make clean`
- **Compilation**: Uses `gcc` with standard flags (`-Wall`, `-pthread` for threads)
- **Examples**: See `os/ostep-code/intro/Makefile` for typical compilation patterns

### Python (AI agents)
- **Package manager**: Uses `pyproject.toml` with `uv` or `pip`
- **Dependencies**: Defined in `pyproject.toml` under `[project]` section
- **Install**: `uv sync` or `pip install -e .`

### Rust
- **Package manager**: `cargo` with `Cargo.toml`
- **Build**: `cargo build`
- **Run**: `cargo run`

### Go
- **Module system**: `go.mod` files
- **Build**: `go build`
- **Run**: `go run`

## Git Submodules

Several directories are git submodules pointing to external repositories:
- `lang/cpp/cs106l-assignments`
- `os/ostep-code` 
- `os/unix`
- `ai/coursera-ml`
- `ai/coursera-dl`

To update submodules: `git submodule update --init --recursive`

## Development Notes

1. **No unified build**: Each directory is independent. Navigate to the specific directory before building.
2. **Educational focus**: Most code is for learning purposes, not production systems.
3. **Simple structure**: Many directories contain standalone `.c` files that compile directly with `gcc`.
4. **Minimal dependencies**: Most C code only requires standard libraries; Python code uses common AI/ML libraries.
5. **Cross-references**: The OSTEP code follows the book structure with chapter-based directories.

## Working with Specific Projects

### OSTEP Code (`os/ostep-code/`)
- Organized by book chapters (CPU virtualization, concurrency, persistence)
- Each subdirectory has its own `Makefile` and `README.md`
- Compile with `make`, run individual executables directly
- See `os/ostep-code/README.md` for chapter mapping

### Unix Study (`os/unix/`)
- Contains Unix v1/v6 source code and analysis tools
- Has a root `Makefile` and subdirectory `Makefile`s
- Tools include `apout` (PDP-11 emulator) and `disaout` (disassembler)

### AI Agents (`ai/agent/`)
- Python-based agent implementations
- Uses OpenAI and Tavily APIs (check `pyproject.toml` for dependencies)
- Follow standard Python packaging with `pyproject.toml`

### Language Exercises
- **C++**: Follows CS106L course structure with assignments
- **Go**: Simple command-line programs demonstrating language features
- **Rust**: Basic Rust programs using Cargo