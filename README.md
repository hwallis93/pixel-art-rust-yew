# Pixel Art (Rust + Yew)
This repo is one of several in which I'm writing the same app using different languages. See [frontends-comparison](https://github.com/hwallis93/frontends-comparison) for more info.

## TODO

- State (yewdux)
- CSS (???)
- Everything else

## Thoughts

- Had to pick a WASM tool, went for Trunk
- Yew doesn't have an equivalent of Styled Components yet, just CSS component libraries
- React class style for Components, functional components in progress
  - But can build against Yew's dev branch and get basically-ready functional components, so doing that.
- State management:
  - One redux-y solution: Yewdux
  - Allows mutable state updates in reducers out the box 👍
