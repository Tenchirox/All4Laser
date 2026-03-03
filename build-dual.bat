@echo off
setlocal

set SCRIPT_DIR=%~dp0
set PS_SCRIPT=%SCRIPT_DIR%scripts\build-dual.ps1

if not exist "%PS_SCRIPT%" (
  echo [error] Missing script: %PS_SCRIPT%
  exit /b 1
)

powershell -NoProfile -ExecutionPolicy Bypass -File "%PS_SCRIPT%" %*
set EXIT_CODE=%ERRORLEVEL%

exit /b %EXIT_CODE%
