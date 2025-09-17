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

$POSTGRES_USER = [Environment]::GetEnvironmentVariable("POSTGRES_USER")
$POSTGRES_PASSWORD = [Environment]::GetEnvironmentVariable("POSTGRES_PASSWORD")
$POSTGRES_PORT = [Environment]::GetEnvironmentVariable("POSTGRES_PORT")
$POSTGRES_DB = [Environment]::GetEnvironmentVariable("POSTGRES_DB")

Set-Location ./api
docker compose -f "../docker-compose.dev.yml" down
docker compose -f "../docker-compose.dev.yml" up -d --build 'db'
Start-Sleep -Seconds 3
diesel migration run --database-url postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@127.0.0.1:${POSTGRES_PORT}/${POSTGRES_DB}
docker compose -f "../docker-compose.dev.yml" down
docker compose -f "../docker-compose.dev.yml" up -d --build
Set-Location ..
