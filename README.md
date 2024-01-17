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
cargo run
```

or

```
cargo install cargo-watch
cargo watch -x run
```

## Deploy
```
cargo shuttle deploy
```