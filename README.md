# `lighter`
### A small pixel shader editor that is easy on your hardware

## Features
* Write your GLSL fragment shader in your favourite code editor: `lighter` will automatically reload when the file changes
* Static mode by default: only rerender on window size changes and shader file changes, vsynced live mode included
* GLSL compilation errors are printed to stdout without interrupting the program

## Installation & use
Install Cargo for Rust compilation. The preferred method for this is with [Rustup](https://rustup.rs).
```
git clone https://github.com/Four-Griffins/lighter
cd lighter
cargo install --path .
...
lighter <path_to_fragment_shader>
```

## Why not just use Shadertoy?
[Shadertoy](https://shadertoy.com) is nice, but I like making very computationally heavy, static image shaders, all on integrated graphics.
Shadertoy and other tools make my PC slow to a crawl by rendering the same image over and over again.
`lighter` goes out of its way to keep redrawing to a minimum, only redrawing on window resizes and shader file changes.
Plus, it's a single binary with no dependencies, so I have a small and snappy environment with whatever code editor I want, without having to install a shader development IDE that I won't use 90% of the features of.
