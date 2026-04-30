@echo off
REM Reinit PostgreSQL Database Script

echo Starting database reinitialization process...
echo.

REM Stopping and removing existing containers and volumes
echo Removing existing containers and volumes...
docker-compose down -v

if %errorlevel% neq 0 (
    echo Error: Failed to stop and remove containers and volumes
    exit /b 1
)

echo.
echo Starting new containers and initializing database...
docker-compose up -d

if %errorlevel% neq 0 (
    echo Error: Failed to start new containers
    exit /b 1
)

echo.
echo Waiting for the database to initialize...
timeout /t 5 /nobreak

echo.
echo ? Checking container status...
echo.
docker-compose ps

