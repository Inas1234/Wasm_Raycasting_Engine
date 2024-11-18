# Raycasting Engine in Rust (WASM)

Welcome to my Raycasting Engine written in Rust and compiled to WebAssembly! This project brings classic raycasting graphics—akin to early first-person games like Wolfenstein 3D—directly to your browser. The engine is lightweight, performant, and easily accessible via GitHub Pages.

## Live Demo

Check out the live demo of the raycasting engine hosted on GitHub Pages: [Raycasting Engine Demo](https://inas1234.github.io/Wasm_Raycasting_Engine/)

## Features

- **Real-time Rendering**: Uses raycasting to render a pseudo-3D environment directly in your browser.
- **WebAssembly**: Written in Rust and compiled to WebAssembly for excellent performance.
- **Interactive**: Navigate through the environment using arrow keys.

## Getting Started

To get started with running the project locally or contributing, follow the steps below.

### Prerequisites

- **Rust**: Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.
- **wasm-pack**: Install `wasm-pack` for compiling Rust to WebAssembly:
  ```sh
  cargo install wasm-pack
  ```

### Building the Project

1. Clone the repository:
   ```sh
   git clone https://github.com/Inas1234/Wasm_Raycasting_Engine.git
   cd wasm-raycasting-engine
   ```
2. Build the project using `wasm-pack` and `make`:
   ```sh
   make
   ```
3. Serve the project locally (using Python's HTTP server for static files):
   ```sh
   make serve
   ```

### Running the Project

Once served, open your browser and navigate to the local server (usually `http://localhost:8000`). You should see the raycasting engine in action!

## Directory Structure

- **src/**: Contains the Rust source code for the raycasting engine.
- **static/**: All the web content, including WebAssembly files and textures, are here.

## Controls

- **Arrow Keys**: Move around the environment.
- **Space**: Interact (for future features).

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve the engine or add new features.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Acknowledgments

- **Rust/Wasm Community**: For great tooling and documentation.
- **Inspiration**: Classic 2.5D games like Wolfenstein 3D that inspired this project.

## Contact

For questions or suggestions, please contact me via [GitHub Issues](https://github.com/Inas1234/Wasm_Raycasting_Engine/issues).

---

Happy coding, and enjoy exploring the 3D world from your browser!

