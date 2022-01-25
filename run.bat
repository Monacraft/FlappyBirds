@echo off
Echo Welcome to Flappy Bird
Echo.
Echo hit any key to start game.
pause>nul

for /l %%a in (0 0 1) do (
cls
target\release\client.exe >nul

Echo New High Score!
Echo Would you like to play again?
Echo.
Echo hit any key to start game.
pause>nul
) 
