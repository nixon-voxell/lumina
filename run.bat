@echo off
setlocal

rem Check if an argument is provided
if "%~1"=="" (
    echo Please provide the number of client instances as an argument.
    exit /b
)

echo Building...
cargo build

set count=%~1

echo Starting the server...
start /B cargo run --bin lumina_server
timeout 1

echo Starting %count% client instances...
for /L %%i in (1,1,%count%) do (
    echo Starting client instance %%i...
    start /B cargo run --bin lumina_client
    timeout 1
)

echo All client instances are running.
