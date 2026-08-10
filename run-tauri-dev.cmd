@echo off
set "PATH=%~dp0node_modules\.bin;%PATH%"
cd /d "%~dp0"
"%~dp0node_modules\.bin\pnpm.CMD" tauri dev