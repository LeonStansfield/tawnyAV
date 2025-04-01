## `image_scene`
The [`image_scene`](../src/scenes/image_scene.rs) module provides an implementation of the `Scene` trait that displays a static image. 
This scene mostly exists just to show how a simple scene can be implemented.

## `shader_scene`

The [`shader_scene`](../src/shader_scene.rs) module provides an implementation of the `Scene` trait that uses GLSL fragment shaders to render dynamic visualizations. This scene is designed to respond to live audio input by updating shader uniforms based on detected beats.

- ***How It Works***:
  - The `ShaderScene` struct initializes a GLSL fragment shader and sets up a `Material` with uniforms such as `Time`, `Seed`, and `TextureSize`.
  - The `update` method updates the `Time` and `Seed` uniforms based on the current frame time and beat detection. When a beat is detected, the `Seed` is randomized, and the `Time` is adjusted to create dynamic effects.
  - The `draw` method renders the shader output to a render target and then displays it on the screen.

### GLSL Shaders

GLSL fragment shaders are used to define the visual effects in shader_scene`. 
The shaders are loaded from the `resources/shaders` directory at runtime. The `ShaderScene` sets up the following uniforms for the shaders:

- `Time`: A float value representing the elapsed time, which can be used to create animations.
- `Seed`: A random float value that changes on each detected beat, allowing for randomized effects.
- `TextureSize`: A vector representing the dimensions of the render target.

### Adding a New Shader

If you want to add new shaders, TawnyAV will automatically create a new scene for any shaders in resources/shaders.

1. Place your GLSL fragment shader in the `resources/shaders` directory.
2. The shader will be automatically loaded and added as a scene when the application starts.

## `cellular_automata_scene`

The [`cellular_automata_scene`](../src/scenes/cellular_automata_scene.rs) module provides an implementation of the `Scene` trait that runs Conway's Game of Life. This scene runs entirely on the CPU and dynamically responds to beat detection by randomizing the grid.