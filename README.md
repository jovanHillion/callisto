# Callisto 🪐

A voxel engine built in Rust as a learning project, focused on understanding GPU rendering fundamentals and progressive optimization techniques.

## Objective

Callisto aims to render voxel worlds correctly and efficiently through a software-first approach. The project is built incrementally  starting from basic triangle rendering, moving toward a full voxel engine with real optimizations.

This is both a **learning project** and a **technical exploration** of:
- How a GPU rendering pipeline works (wgpu / WebGPU)
- How voxel data can be structured and traversed efficiently
- What optimization strategies (chunking, greedy meshing, occlusion culling...) look like in practice

## Tech Stack

| Tool | Role |
|------|------|
| [Rust](https://www.rust-lang.org/) | Systems language |
| [wgpu](https://wgpu.rs/) | Cross-platform GPU rendering (Vulkan / Metal / DX12) |
| [winit](https://github.com/rust-windowing/winit) | Window creation and event handling |
| [WGSL](https://www.w3.org/TR/WGSL/) | Shader language |

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- A GPU supporting Vulkan, Metal, or DirectX 12

### Build & Run

```bash
git clone https://github.com/your-username/callisto.git
cd callisto
cargo run
```

### Controls

| Key | Action |
|-----|--------|
| `Escape` | Quit |
| `Arrows` | Move the camera and zoom in, zoom out |
| Mouse click | Randomize background color |

## Project Status

🚧 Early development  currently rendering basic geometry with wgpu.

## Usefull links

- [Learn WGPU](https://sotrh.github.io/learn-wgpu/)
- [WebGPU Fundamentals](https://webgpufundamentals.org/)
- [VoxBox](https://voxbox.store/)
- [Camera](https://learnopengl.com/Getting-started/Camera)