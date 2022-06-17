# wasm-game


Install tools (if needeed):

```sh
$ ./install.sh 
```

Install dependencies :

```sh
$ npm install
```

After the installation completes, you can now run a development server:

```sh
$ npm start
```

When you see `｢wdm｣: Compiled successfully.` , you can browse your app
at http://localhost:8080.


```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start

# Builds the project and places it into the `dist` folder.
npm run build

# Runs tests in Firefox
npm test -- --firefox
```

## References

* https://rustwasm.github.io/docs/wasm-pack/tutorials/hybrid-applications-with-webpack/using-your-library.html