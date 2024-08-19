# Tawny AV

## Description
Tawny AV is a python based audio visual software to display dynamic visualizations based live audio captured by a microphone. Tawny dynamically triggers effects based on the beat of the audio input in order to create procedural visualizations.


## Installation
1. Ensure the latest version of Python is installed on your system. You can download the latest version of Python from the official website: https://www.python.org/downloads/

2. Clone the repository:
    ```sh
    git clone https://github.com/LeonStansfield/tawnyAV.git
    ```

3. Run the correct program for your operating system. If the virtual environment isnt already created, it will set this up along with the dependencies. EG run_linux_mac.sh or run_windows.bat

## Todo
 - I would like to modify TawnyAV to use GPU based shaders instead of CPU based visuals. This will essentially turn TawnyAV into a audio reactive shader toy - meaning it could support a large range of visuals, similar to those found on www.shadertoy.com
 - I would like to be able to support many visuals being loaded and switched between in run time. I think this would require a refactor of the overall architecture to make audio processing independent from the visuals. The visuals can then be instantiated 'scene' objects which can be switched between in real time. 'Scene' objects can recieve a 'signal' when the audio processor detects a beat, indicating them to complete an action.
