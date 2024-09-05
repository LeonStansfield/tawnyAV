# Tawny AV

## Description
Tawny AV is a rust based audio visual software to display dynamic visualizations based live audio captured by a microphone. Tawny dynamically triggers effects based on the beat of the audio input in order to create procedural visualizations.


## Installation
- If you havent got rust installed already, install rust by following the instructions at [rustup.rs](https://rustup.rs/)
- Clone the repository with `git clone https://github.com/LeonStansfield/tawnyAV.git`
- Run the program with `cargo run --release`

## Usage
- Run the program with `cargo run --release`
- Press `F11` to toggle fullscreen
- Press the number keys `1-9` to switch between different visualizations
- Press `Space` to stop the beat detection
- Press the `Up` and `Down` arrow keys to adjust the sensitivity of the beat detection --  Not implemented yet

## Todo
- Modify the paper melt scene so that it is more visually appealing, with the logo more integrated, fits a set colour scheme and adapts to the beat of the music
- Modify the cloud scene so that it fits a set colour scheme, with the logo integrated and adapts to the beat of the music
- Add support to play videos and apply shader effects to videos

## Bugs
 - When pressing f11 for fullscreen, the window goes fullscreen but always on the primary monitor. This should be on the monitor the window is currently on. Also fullscreen doesnt work on linux.
