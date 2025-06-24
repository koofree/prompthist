# PromptHist

AI Prompt History & Insights Tool - A desktop application built with Tauri 2.0, Next.js 14.2.3, and TypeScript.

## 🚀 Quick Start

### Prerequisites

- **Node.js** >= 18.0.0
- **pnpm** >= 8.0.0 (recommended package manager)
- **Rust** (for Tauri development)

### Installation

```bash
# Install dependencies with pnpm
pnpm install

# Start development server
pnpm run tauri:dev

# Build for production
pnpm run tauri:build
```

## 📝 Available Scripts

### Development

- `pnpm run dev` - Start Next.js development server
- `pnpm run tauri:dev` - Start Tauri development mode
- `pnpm run build` - Build Next.js for production
- `pnpm run tauri:build` - Build Tauri application

### Code Quality

- `pnpm run lint` - Run ESLint
- `pnpm run lint:fix` - Fix ESLint issues automatically
- `pnpm run format` - Format code with Prettier
- `pnpm run format:check` - Check code formatting

## 🛠️ Tech Stack

- **Framework**: Tauri 2.0 + Next.js 14.2.3
- **Language**: TypeScript
- **Styling**: Tailwind CSS + shadcn/ui
- **State Management**: Zustand
- **Database**: SQLite with encryption
- **Package Manager**: pnpm (fast, efficient, disk space optimized)

## 🔧 Development Setup

This project uses **pnpm** for faster installs and better disk space efficiency:

- **Faster installs**: Up to 2x faster than npm
- **Disk space efficient**: Uses hard links and deduplication
- **Strict dependency resolution**: Prevents phantom dependencies
- **Better monorepo support**: Built-in workspace features

### Why pnpm?

- ⚡ **Performance**: Significantly faster package installation
- 📦 **Space efficient**: Saves disk space through content-addressable storage
- 🔒 **Security**: Better dependency isolation and strict peer dependencies
- 🚀 **Modern**: Built for modern JavaScript development workflows

## 📁 Project Structure

```
prompthist/
├── src/                 # Next.js frontend source
├── src-tauri/          # Tauri backend (Rust)
├── public/             # Static assets
├── .vscode/            # VS Code settings
└── pnpm-workspace.yaml # pnpm workspace configuration
```

For detailed technical information, see [TECH_STACK.md](./TECH_STACK.md).

## License

MIT
