# Tawny AV

## Description
Tawny AV is a rust based audio visual software to display dynamic visualizations based live audio captured by a microphone. Tawny dynamically triggers effects based on the beat of the audio input in order to create procedural visualizations.


## Installation/Usage

## Todo
- Rewrite installation + usage instructions for new rust rewrite
- Add support to play videos and apply shader effects to videos
- Create global variables for global things such as render resolution, resource file paths, ect.

## Bugs
 - Reset function on reaction diffusion is not working. It does not reset the state of the simulation and instead flashes the image then leaves the screen black
 - When pressing f11 for fullscreen, the window goes fullscreen but always on the primary monitor. This should be on the monitor the window is currently on.
 - The reaction diffusion simulation is not working as expected. The simulation does not generate patterns as expected instead fading to black quickly.
 - Game of life doesnt work as expected at all.
 