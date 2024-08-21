#!/usr/bin/env pwsh
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# Function to check if a command exists
function Test-Command {
    param (
        [string]$Command
    )
    return Get-Command $Command -ErrorAction SilentlyContinue
}

# Check if sqlx is installed
if (-not (Test-Command "sqlx")) {
    Write-Error "Error: sqlx is not installed."
    Write-Error "Use:"
    Write-Error "    cargo install --version='~0.7' sqlx-cli --no-default-features --features rustls,postgres"
    Write-Error "to install it."
    exit 1
}

# Check if a custom user has been set, otherwise default to 'postgres'
$DB_USER = $Env:POSTGRES_USER
if (-not $DB_USER) { $DB_USER = "postgres" }

# Check if a custom password has been set, otherwise default to 'password'
$DB_PASSWORD = $Env:POSTGRES_PASSWORD
if (-not $DB_PASSWORD) { $DB_PASSWORD = "19991029csy" }

# Check if a custom database name has been set, otherwise default to 'newsletter'
$DB_NAME = $Env:POSTGRES_DB
if (-not $DB_NAME) { $DB_NAME = "newsletter" }

# Check if a custom port has been set, otherwise default to '5432'
$DB_PORT = $Env:POSTGRES_PORT
if (-not $DB_PORT) { $DB_PORT = "5432" }

# Check if a custom host has been set, otherwise default to 'localhost'
$DB_HOST = $Env:POSTGRES_HOST
if (-not $DB_HOST) { $DB_HOST = "nas.fermimix.com" }

# Set the DATABASE_URL environment variable
$Env:DATABASE_URL = "postgres://$DB_USER`:$DB_PASSWORD@$DB_HOST`:$DB_PORT/$DB_NAME"

# Run migrations
sqlx database create | Write-Host
sqlx migrate run | Write-Host

Write-Host "Postgres has been migrated, ready to go!"
