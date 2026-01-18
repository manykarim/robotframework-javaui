@echo off
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0lsof.ps1" %*
