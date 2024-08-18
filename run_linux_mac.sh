#!/bin/bash

# Define the name of the virtual environment directory
VENV_DIR="venv"

# Activate the virtual environment
source $VENV_DIR/bin/activate

# Run the main Python file
python main.py

# Deactivate the virtual environment
deactivate