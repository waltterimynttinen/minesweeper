# Minesweeper Game
This is a classic implementation of the Minesweeper game built using Rust and WebAssembly!

## Prerequisites
Before running the Minesweeper game, ensure you have the following prerequisites installed:

- Rust: The game is built using Rust programming language. You can download and install Rust from [here](https://www.rust-lang.org/learn/get-started).
- wasm-pack: This is a tool for building and working with Rust and WebAssembly projects. You can install it by following the instructions [here](https://rustwasm.github.io/wasm-pack/installer/).
- npm: npm is the package manager for JavaScript. It is used to manage project dependencies and build scripts. You can install npm by downloading Node.js from [here](https://nodejs.org/), which includes npm.

## Getting Started
1. Clone the repository: `git clone https://github.com/your-username/minesweeper.git`
2. Navigate into the repository: `cd minesweeper`
3. Build the project with wasm-pack: `wasm-pack build --target web`
4. Install serve: `npm install serve`
5. Run the game: `serve`
6. Enjoy!

## Tutorial
After running the game, head on over to http://localhost:3000/ (should be this by default).  
Use left click on a cell to reveal it or right click to flag it. To remove a flag, right click a flagged cell again.
If you accidentally click on a bomb, the game is over. Refresh the page to try again.  
To stop playing, close serve.  
