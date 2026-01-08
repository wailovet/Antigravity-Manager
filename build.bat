@echo off
echo ====================================
echo   Antigravity Manager Build Script
echo ====================================
echo.

cd /d "%~dp0"

echo [1/2] Installing dependencies...
call npm install
if errorlevel 1 (
    echo ERROR: npm install failed!
    pause
    exit /b 1
)

echo.
echo [2/2] Building Tauri application...
call npm run tauri build
if errorlevel 1 (
    echo ERROR: Build failed!
    pause
    exit /b 1
)

echo.
echo ====================================
echo   Build completed successfully!
echo ====================================
echo.
echo Output files:
echo   - src-tauri\target\release\antigravity_tools.exe
echo   - src-tauri\target\release\bundle\msi\Antigravity Tools_*.msi
echo   - src-tauri\target\release\bundle\nsis\Antigravity Tools_*-setup.exe
echo.
pause
