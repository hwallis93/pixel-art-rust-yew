# Pixel Art (Rust + Yew)
This repo is one of several in which I'm writing the same app using different languages. See [frontends-comparison](https://github.com/hwallis93/frontends-comparison) for more info.

## TODO

- How to pass variable to CSS?
- Everything

## Thoughts

- Had to pick a WASM tool, went for Trunk
- Yew doesn't have an equivalent of Styled Components yet, just CSS component libraries
- Functional components in progress, no hooks, lifecycle method based liked React classes
- State management:
  - One redux-y solution: Yewdux
  - Allows mutable state updates in reducers out the box 👍
