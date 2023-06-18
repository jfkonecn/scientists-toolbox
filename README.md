# Scientists-Toolbox

Website which does engineering and scientific calculations.

Visit the website [here](https://jfkonecn.github.io/scientists-toolbox/)

## Build

1. Set wasm target

   ```sh
   rustup target add wasm32-unknown-unknown
   ```

2. Install [node](https://nodejs.org/en/)
3. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/)
4. Install [trunk](https://trunkrs.dev/)
5. Install [cargo-make](https://sagiegurari.github.io/cargo-make/)
6. Install Node Packages

   ```sh
   npm install
   ```

7. Run build

   ```sh
   cargo make dev
   ```

## Build Watch

1. Follow Build Steps
2. Run watch

   ```sh
   cargo make watch
   ```

3. Go [here](http://localhost:8080) to view the page
   - note you may have to hard refresh the browser if the page is blank
