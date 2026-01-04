# ClearTodo

A minimalist desktop todo application that floats on the top-right corner of your screen.

![ClearTodo](https://via.placeholder.com/380x500/1a1a1a/ffffff?text=ClearTodo)

## Features

- Add, edit, and delete tasks
- Double-click a task to enter edit mode
- Mark tasks as completed
- Auto-save data locally
- Transparent floating window with blur effect
- Always on top

## Tech Stack

- **Frontend**: Vue 3 + TypeScript + Vite
- **Backend**: Tauri 2 (Rust)
- **UI**: Native CSS (no framework dependencies)

## Requirements

- Node.js 18+
- Rust 1.70+
- pnpm

## Getting Started

```bash
# Install dependencies
pnpm install

# Development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

## Project Structure

```
ClearTodo/
├── src/                 # Vue frontend code
│   ├── App.vue         # Main component
│   └── main.ts         # Entry point
├── src-tauri/          # Rust backend code
│   ├── src/lib.rs      # Tauri app configuration
│   └── tauri.conf.json # Tauri configuration
└── package.json        # Node dependencies
```

## Window Configuration

Default settings:
- Width: 380px
- Height: 500px
- Position: Top-right corner
- Transparent background with blur effect
- Always on top, skip taskbar

## License

MIT
