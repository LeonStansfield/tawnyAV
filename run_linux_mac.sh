#!/bin/bash

# Define the name of the virtual environment directory
VENV_DIR="venv"

# Check if the virtual environment directory exists
if [ ! -d "$VENV_DIR" ]; then
    echo "Virtual environment not found. Creating one..."
    python3 -m venv $VENV_DIR
    source $VENV_DIR/bin/activate
    if [ -f "requirements.txt" ]; then
        pip install -r requirements.txt
    else
        echo "requirements.txt not found. Please provide a requirements file."
    fi
else
    # Activate the virtual environment
    source $VENV_DIR/bin/activate
fi

# Run the main Python file
python main.py

# Deactivate the virtual environment
deactivate