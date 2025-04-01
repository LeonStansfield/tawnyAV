# Tawny AV

## Description
Tawny AV is an audio-visual software built in Rust that generates dynamic visualizations based on live audio captured from a microphone. The visuals are powered by GLSL fragment shaders, which dynamically respond to detected beats in the audio input.

# Features
 - Real-time Audio Processing using the device’s microphone to detect beats and trigger events dynamically.
 - Detected beats are automatically aligned with visual effects for a responsive audiovisual experience.
 - GLSL fragment shaders to generate real-time visuals with smooth performance using the GPU.
 - Users can modify shaders, color palettes, and background images to create their own visuals.
 - All visual assets (images, colors, and shaders) are interchangeable and can be modified by the user
 - Users with GLSL knowledge can implement their own shader effects to expand on the visuals.

## Examples
![Gif showing 4 shaders running in TawnyAV with the beat detection turned off](resources/docs/demo1.gif)

![Screenshot showing UI and melt scene](resources/docs/melt.png)

![Screenshot showing UI and parallax scene](resources/docs/parallax.png)

![Screenshot showing UI and the kbmarcher scene](resources/docs/marcher.png)

## Usage Guide

### Basic Usage

Download the program from the releases section.
Run the main executable.
Basic settings (Beat detection, audio processing sensitivity, scene selection and image loading) can be modified using the GUI.
For live AV shows, you may want to modify these without showing the GUI, I reccomend learning the following shortcuts to modify basic settings on the fly:

Keyboard Shortcuts:
H - Toggle the UI
F11 - Toggle fullscreen
1-9 and Left/Right Arrows - Switch between different visualizations
Space - Toggle beat detection
Up/Down Arrows - Adjust beat detection sensitivity

### Advanced Usage

#### Customising your visuals:
Custom backgroung image:
     - To add your own logo/image to the visuals first ensure the image file is in the correct format:
          - Image file formatting guide here (image must be 1920x1080, white on transparrent background, png).
     - Ensure the image is somewhere where you are able to locate it relative to the programs location (I reccomend inside the resources/images directory)
     - In the GUI, modify the file path to point towards your images location
     - Click 'reload scenes'. This will reload the scenes with your custom image applied.
     - If you want this to be the default logo used on starting the program, simply modify
Custom colour palette:
      - Colour palletes for each shader are stored in each individual shader file. These colours can be modified to your liking, but for your reference, I have left a copy of the default colour palette in the shader in the resources folder.

Custom shaders:
      - Users can add their own custom shaders.
      - Simply add a working fragment shader contained in a .glsl file to the resources/shaders directory.
      - WARNINGL The shader must be a working fragment shader, if the shader does not compile, the program will not start.

## Developers

### Installation
- If you havent got rust installed already, install rust by following the instructions at [rustup.rs](https://rustup.rs/)
- Clone the repository with `git clone https://github.com/LeonStansfield/tawnyAV.git`
- Run the program with `cargo run --release`

## Project Plan
- Scenes:
     - Create 10 full scenes
        - Game of life scene
     - Make scenes dynamically loaded from resources folder (with names), so users can add their own scenes
- Improved functionality of the UI:
    - Chose audio input device
    - Modify and change colour palettes
    - Save settings to be automatically loaded next time.
    - Add warning to UI (warnings will show in the window instead of console, and will not crash the program)
- Left and Right arrow keys to change scenes as well as numbers

- Add support to play videos and apply shader effects to videos

## Known Bugs
 - When pressing f11 for fullscreen, the window goes fullscreen but always on the primary monitor. This should be on the monitor the window is currently on.
