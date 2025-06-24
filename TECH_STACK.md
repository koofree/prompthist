# PromptHist Tech Stack

## Framework: **Tauri 2.0 + Next.js 14.2.3 + TypeScript**

## Core Technologies

- **Framework**: Tauri 2.0
- **Backend Language**: Rust
- **Frontend**: Next.js 14.2.3 + TypeScript (App Router)
- **Styling**: Tailwind CSS + shadcn/ui
- **State Management**: Zustand
- **Database**: SQLite with encryption
- **HTTP Client**: Reqwest (for Ollama)

## Package Management

- **Package Manager**: PNPM 10.12.2
- **Workspace Configuration**: Monorepo support with `pnpm-workspace.yaml`
- **Performance**: Fast, disk space efficient installations
- **Security**: Strict dependency resolution and isolation

## Configuration Files

### Frontend Dependencies & Scripts

**File**: `package.json`

- Next.js 14.2.3 (stable version)
- React with TypeScript (managed by Next.js)
- Tauri 2.0 plugins
- Tailwind CSS ecosystem
- Development and build scripts

### Rust Dependencies

**File**: `src-tauri/Cargo.toml`

- Tauri 2.0 with required features
- Database, encryption, and system monitoring crates
- HTTP client for Ollama integration

### Next.js Configuration

**File**: `next.config.js`

- Static export configuration for Tauri
- Asset optimization settings
- Build output directory configuration

### Tauri Application Configuration

**File**: `src-tauri/tauri.conf.json`

- Application metadata and window settings
- Build commands and frontend integration
- Security policies and permissions
- Bundle configuration for all platforms

### Styling Configuration

**Files**:

- `tailwind.config.js` - Tailwind CSS configuration
- `postcss.config.js` - PostCSS plugins configuration

### TypeScript Configuration

**File**: `tsconfig.json`

- Compiler options for Next.js
- Path mapping and module resolution
- Build target and library settings

### Code Quality Configuration

**File**: `eslint.config.js`

- Next.js ESLint rules
- TypeScript linting configuration
- Code formatting standards

## System Integration

### Tauri Commands (Rust Backend)

Rust functions exposed to frontend via Tauri's command system for:

- System monitoring operations
- Encrypted database operations
- Local LLM integration
- File system operations

### Frontend Services (Next.js)

TypeScript services for:

- Tauri API integration
- State management with Zustand
- React Query for data fetching
- Component libraries integration

## Database & Security

### Local Storage

- **SQLite**: Encrypted local database
- **Key Management**: OS keychain integration
- **Memory Protection**: Secure memory handling

### System Monitoring

- **Input Monitoring**: Cross-platform keyboard/mouse tracking
- **Window Detection**: Active application monitoring
- **Permissions**: Tauri 2.0 capability-based security

## Local LLM Integration

### Ollama Integration

- HTTP client for local Ollama API
- Prompt processing and response handling
- Model management and configuration

## Build Targets

Tauri 2.0 creates native installers for:

- **Windows**: `.msi` installer and `.exe` portable
- **macOS**: `.dmg` disk image and `.app` bundle
- **Linux**: `.deb`, `.rpm`, and `.AppImage` packages

## Security Model

### Tauri 2.0 Security

**File**: `src-tauri/capabilities/main.json`

- Capability-based permission system
- Granular security controls
- File system access permissions
- System API restrictions

## Key Benefits

### **🚀 Next.js 14.2.3 Advantages:**

- **Stability**: LTS version with proven reliability
- **App Router**: Modern routing with layouts and nested routes
- **Performance**: Built-in optimizations and code splitting
- **Developer Experience**: Hot reload, error overlay, TypeScript support

### **🎨 Modern UI Stack:**

- **Tailwind CSS**: Utility-first CSS framework
- **shadcn/ui**: Accessible component library
- **Responsive Design**: Mobile-first approach
- **Dark Mode**: Built-in theme switching

### **⚡ Performance & Security:**

- **Static Export**: Optimized builds for Tauri
- **Native Performance**: Rust backend with web frontend
- **Cross-Platform**: Single codebase for all desktop platforms
- **Offline-First**: No external dependencies required

This Next.js 14.2.3 + Tauri 2.0 combination provides a stable, secure, and performant foundation for PromptHist with excellent maintainability and developer experience.
