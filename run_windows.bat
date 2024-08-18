@echo off

:: Define the name of the virtual environment directory
set VENV_DIR=venv

:: Activate the virtual environment
call %VENV_DIR%\Scripts\activate

:: Run the main Python file
python main.py

:: Deactivate the virtual environment
deactivate