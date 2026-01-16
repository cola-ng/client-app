#!/usr/bin/env pwsh
# Initialize Colang English Learning Database
# This script creates the SQLite database and populates it with migrations and seed data

param(
    [switch]$Reset = $false,
    [string]$DbPath = "$PSScriptRoot\learning.db"
)

$ErrorActionPreference = "Stop"

$migrationsPath = Join-Path (Split-Path $PSScriptRoot -Parent) "migrations"
$seedDataPath = Join-Path $PSScriptRoot "seed_data.sql"

Write-Host "==================================" -ForegroundColor Cyan
Write-Host "Colang Database Initialization" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""

# Check if database exists
$dbExists = Test-Path $DbPath

if ($dbExists -and $Reset) {
    Write-Host "⚠️  Removing existing database..." -ForegroundColor Yellow
    Remove-Item $DbPath -Force
    $dbExists = $false
    Write-Host "✓ Database removed" -ForegroundColor Green
    Write-Host ""
}

if ($dbExists) {
    Write-Host "ℹ️  Database already exists at: $DbPath" -ForegroundColor Yellow
    Write-Host "   Use -Reset flag to recreate it." -ForegroundColor Yellow
    Write-Host ""
    $response = Read-Host "Apply migrations to existing database? (y/N)"
    if ($response -ne 'y' -and $response -ne 'Y') {
        Write-Host "Aborted." -ForegroundColor Yellow
        exit 0
    }
}

# Check if sqlite3 is available
$sqliteCmd = Get-Command sqlite3 -ErrorAction SilentlyContinue
if (-not $sqliteCmd) {
    Write-Host "❌ Error: sqlite3 command not found" -ForegroundColor Red
    Write-Host "   Please install SQLite from: https://www.sqlite.org/download.html" -ForegroundColor Yellow
    exit 1
}

# Get migration files
$migrationFiles = Get-ChildItem -Path $migrationsPath -Filter "*.sql" | Sort-Object Name

if ($migrationFiles.Count -eq 0) {
    Write-Host "❌ No migration files found in $migrationsPath" -ForegroundColor Red
    exit 1
}

Write-Host "📂 Found $($migrationFiles.Count) migration file(s)" -ForegroundColor Cyan
Write-Host ""

# Apply each migration
foreach ($file in $migrationFiles) {
    Write-Host "Applying migration: $($file.Name)" -ForegroundColor White
    
    try {
        $sql = Get-Content $file.FullName -Raw
        $output = $sql | sqlite3 $DbPath 2>&1
        
        if ($LASTEXITCODE -ne 0) {
            Write-Host "❌ Failed to apply migration: $($file.Name)" -ForegroundColor Red
            Write-Host "   Error: $output" -ForegroundColor Red
            exit 1
        }
        
        Write-Host "   ✓ Applied successfully" -ForegroundColor Green
    }
    catch {
        Write-Host "❌ Error applying migration: $($file.Name)" -ForegroundColor Red
        Write-Host "   $_" -ForegroundColor Red
        exit 1
    }
}

Write-Host ""

# Ask about sample data
if (-not $dbExists -or $Reset) {
    Write-Host "Would you like to load sample data? (Y/n): " -NoNewline -ForegroundColor Cyan
    $loadData = Read-Host
    
    if ($loadData -ne 'n' -and $loadData -ne 'N') {
        if (Test-Path $seedDataPath) {
            Write-Host ""
            Write-Host "Loading sample data..." -ForegroundColor White
            
            try {
                $sql = Get-Content $seedDataPath -Raw
                $output = $sql | sqlite3 $DbPath 2>&1
                
                if ($LASTEXITCODE -ne 0) {
                    Write-Host "❌ Failed to load sample data" -ForegroundColor Red
                    Write-Host "   Error: $output" -ForegroundColor Red
                    exit 1
                }
                
                Write-Host "✓ Sample data loaded successfully" -ForegroundColor Green
            }
            catch {
                Write-Host "❌ Error loading sample data" -ForegroundColor Red
                Write-Host "   $_" -ForegroundColor Red
                exit 1
            }
        }
        else {
            Write-Host "⚠️  Seed data file not found: $seedDataPath" -ForegroundColor Yellow
        }
    }
}

Write-Host ""
Write-Host "==================================" -ForegroundColor Cyan
Write-Host "✨ Database initialization complete!" -ForegroundColor Green
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Database location: $DbPath" -ForegroundColor White
Write-Host ""

# Show some statistics
Write-Host "Database Statistics:" -ForegroundColor Cyan
$stats = @"
SELECT 
    (SELECT COUNT(*) FROM sqlite_master WHERE type='table') as total_tables,
    (SELECT COUNT(*) FROM scenes) as scenes,
    (SELECT COUNT(*) FROM scene_dialogues) as dialogues,
    (SELECT COUNT(*) FROM classic_dialogue_sources) as classic_sources,
    (SELECT COUNT(*) FROM classic_dialogue_clips) as classic_clips,
    (SELECT COUNT(*) FROM reading_exercises) as reading_exercises,
    (SELECT COUNT(*) FROM issue_words) as issue_words,
    (SELECT COUNT(*) FROM conversations) as conversations,
    (SELECT COUNT(*) FROM learning_sessions) as sessions
"@

try {
    $result = $stats | sqlite3 $DbPath ".mode line"
    Write-Host $result -ForegroundColor White
}
catch {
    Write-Host "⚠️  Could not display statistics" -ForegroundColor Yellow
}

Write-Host ""
