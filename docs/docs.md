# TawnyAV Documentation

## Overview

TawnyAV is an audio-visual software built in Rust that generates dynamic visualizations based on live audio captured from a microphone. The visuals are powered by GLSL fragment shaders, which dynamically respond to detected beats in the audio input. The software is designed to be modular, allowing developers to easily add new scenes and shaders.

## How It Works

### 1. Audio Processing
The audio input is captured using the `cpal` crate in [`audio_processing.rs`](../src/audio_processing.rs). The software calculates the energy of the audio signal and uses a rolling average to detect beats. When a beat is detected, a global flag (`BEAT_DETECTED`) is set, which is used by the scenes to trigger visual changes.

### 2. Scene Management
The [`SceneManager`](../src/scene_manager.rs) is responsible for managing the active scene. It stores a list of scenes and allows switching between them using user input. Scenes implement the `Scene` trait defined in [`scene.rs`](../src/scene.rs), which requires `update`, `draw`, and `get_name` methods.

### 3. Scene Implementations

The program is designed to be modular, allowing developers to implement different types of scenes. Each scene must implement the `Scene` trait defined in [`scene.rs`](../src/scene.rs). This trait requires the following methods:

- `update(&mut self)`: Updates the scene's state, such as responding to beat detection or other events.
- `draw(&mut self)`: Renders the scene to the screen.
- `get_name(&self) -> &str`: Returns the name of the scene.

See [`scene_implementation.md`](scene_implementation.md) for more details on current scene implementations.

### 4. User Input
User input is handled in [`input.rs`](../src/input.rs). Users can toggle beat detection, adjust sensitivity, switch scenes, and toggle fullscreen mode using keyboard shortcuts.

### 5. Graphical User Interface (GUI)
The GUI is implemented in [`ui.rs`](../src/ui.rs) using `macroquad::ui`. It allows users to interact with the application, such as selecting scenes, toggling beat detection, and changing the image used in the visualizations.

### 6. Global Variables
Global variables are defined in [`globals.rs`](../src/globals.rs) using the `lazy_static` crate. These include screen dimensions, beat detection flags, sensitivity, and the path to the current image.

## Adding a New Scene
Tawny AV is designed to be modular, allowing for the implementation of other scene types (for example, if you wanted to expand it to use CPU based visualisations using macroquad). To add a new scene, follow these steps:
1. Create a new Rust file in the `src/scenes/` directory.
2. Implement the `Scene` trait for your new scene.
3. Add your scene to the `get_scenes` function in [`mod.rs`](../src/scenes/mod.rs).