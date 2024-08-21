@echo off

:: Define the name of the virtual environment directory
set VENV_DIR=venv

:: Check if the virtual environment directory exists
if not exist %VENV_DIR% (
    echo Virtual environment not found. Creating one...
    python -m venv %VENV_DIR%
)

:: Activate the virtual environment
call %VENV_DIR%\Scripts\activate

:: Check if requirements.txt exists and install/update dependencies
if exist requirements.txt (
    pip install -r requirements.txt
) else (
    echo requirements.txt not found. Please provide a requirements file.
)

:: Run the main Python file
python main.py

:: Deactivate the virtual environment
deactivate