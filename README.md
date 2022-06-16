# wasm-game

Initialize the project:

```sh
$ npm init rust-webpack 'your-package-name'
```

Install dependencies and wasm-pack:

```
$ npm install
$ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 
```

After the installation completes, you can now run a development server:

```sh
$ npm start
```

When you see `｢wdm｣: Compiled successfully.` , you can browse your app
at http://localhost:8080.

## How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

## How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```

## How to run unit tests

```sh
# Runs tests in Firefox
npm test -- --firefox

# Runs tests in Chrome
npm test -- --chrome

# Runs tests in Safari
npm test -- --safari
```

## References

* https://rustwasm.github.io/docs/wasm-pack/tutorials/hybrid-applications-with-webpack/using-your-library.html