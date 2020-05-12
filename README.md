<div align="center">
  <h1><code>space-invaders-wasm</code></h1>
</div>

## About
Ported version of [space-invaders using SDL2](https://github.com/knightpp/rs8080)

## Needed tools
`wasm-pack`, `npm` and Rust compiler.

You can install `wasm-pack` in two ways. `cargo install wasm-pack` or from [the site](https://rustwasm.github.io/wasm-pack/)
## Usage

```
git clone https://github.com/knightpp/space-invaders-wasm.git
cd space-invaders-wasm
wasm-pack build
cd ./www/
# init webpack dev server
npm run start
# navigate to http://localhost:8080/ in your web browser
```

