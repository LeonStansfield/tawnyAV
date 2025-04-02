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

The [`cellular_automata_scene`](../src/scenes/cellular_automata_scene.rs) provides an implementation of the `Scene` trait that runs cellular automata simulations, such as Conway's Game of Life. This scene runs entirely on the CPU and dynamically responds to beat detection by randomizing the grid, the loaded image will be used to influence the initial state of the grid, so your image can be seen through the simulation.

### Creating a New Cellular Automata Scene

To create a new cellular automata scene, you need to create a new `.txt` file in the `resources/cellular_automata` directory. The file should contain the rules for the cellular automata simulation.

### Text File Format

Each `.txt` file in the `resources/cellular_automata` directory must follow this format:

B36/S23/L5
A:R185,G91,B137
D:R78,G10,B10
SC:0.47

Where:
- The first line contains the rules for the cellular automata simulation in the format `B{birth rules}/S{survival rules}/L{lifetime}`.
- The second line contains the color for the alive cells in the format `A:R{red},G{green},B{blue}`.
- The third line contains the color for the dead cells in the format `D:R{red},G{green},B{blue}`.
- The fourth line contains the spawn chance in the format `SC: {spawn chance}`. This can be a float value between 0 and 1, where 0 is no chance and 1 is a guaranteed spawn.

The lifetime is the number of frames the cell will remain alive before dying.
If the lifetime is more than 1, the two colors will be blended together to create a gradient effect.
