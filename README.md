# run server

Watches the assets and runs tailwind tree shaking

## Build
Watches the html templates and generates minified tailwind css file at assets/main.css
```
pnpm tailwindcss
```

Formats html code
```
pnpm format
```

## Run server
```
cargo install cargo-shuttle
cargo shuttle run
```

or

```
cargo install cargo-watch
cargo watch -x 'shuttle run'
```

## Deploy
```
cargo shuttle deploy
```