@echo off
echo Resetting VeloPay database...
echo.

echo Step 1: Dropping existing database...
psql -U postgres -c "DROP DATABASE IF EXISTS velopay;"
if errorlevel 1 (
    echo ERROR: Failed to drop database
    pause
    exit /b 1
)

echo Step 2: Creating fresh database...
psql -U postgres -c "CREATE DATABASE velopay;"
if errorlevel 1 (
    echo ERROR: Failed to create database
    pause
    exit /b 1
)

echo Step 3: Granting privileges...
psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE velopay TO altug;"
if errorlevel 1 (
    echo ERROR: Failed to grant privileges
    pause
    exit /b 1
)

echo Step 4: Granting schema permissions...
psql -U postgres -d velopay -c "GRANT ALL ON SCHEMA public TO altug;"
if errorlevel 1 (
    echo ERROR: Failed to grant schema permissions
    pause
    exit /b 1
)

echo.
echo Database reset complete!
echo.
echo Now run:
echo   set DATABASE_URL=postgresql://altug:velopay123@localhost/velopay
echo   sqlx migrate run
echo   cargo sqlx prepare
echo.
pause
