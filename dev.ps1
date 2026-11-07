if (Test-Path -Path ".env") {
    Get-Content .env | ForEach-Object {
        if ($_ -match "^\s*([^#][^=]+)=(.*)$") {
            $key = $matches[1].Trim()
            $value = $matches[2].Trim()
            if ($value -match '^"(.*)"$' -or $value -match "^'(.*)'$") {
                $value = $matches[1]
            }
            [Environment]::SetEnvironmentVariable($key, $value, "Process")
        }
    }
} else {
    Write-Host ".env file not found" -ForegroundColor Red
    exit 1
}

docker compose -f "docker-compose.dev.yml" down
docker compose -f "docker-compose.dev.yml" up -d --build
