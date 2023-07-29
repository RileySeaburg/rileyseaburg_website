# Function to make an HTTP request and measure response time
function Measure-ResponseTime {
    param (
        [string]$Url
    )

    $startTime = Get-Date
    $response = Invoke-WebRequest -Uri $Url
    $endTime = Get-Date

    $responseTime = ($endTime - $startTime).TotalMilliseconds
    return $responseTime
}

# Ask user for the URL to benchmark
$Url = Read-Host "Enter the URL of the website you want to benchmark (e.g., https://www.example.com)"

# Validate if the URL is in a proper format
if (-not $Url -or -not [System.Uri]::IsWellFormedUriString($Url, [System.UriKind]::Absolute)) {
    Write-Host "Invalid URL format. Please enter a valid URL (e.g., https://www.example.com)" -ForegroundColor Red
    Exit 1
}

# Ask user for the number of requests to be made
$NumberOfRequests = Read-Host "Enter the number of requests to be made"

# Validate if the input is a valid positive number
if (-not [int]::TryParse($NumberOfRequests, [ref]$null) -or [int]$NumberOfRequests -le 0) {
    Write-Host "Invalid input. Please enter a valid positive number." -ForegroundColor Red
    Exit 1
}

# Array to store response times
$responseTimes = @()

# Loop to make multiple requests and collect response times
for ($i = 1; $i -le $NumberOfRequests; $i++) {
    Write-Host "Sending request $i..."
    $responseTime = Measure-ResponseTime -Url $Url
    $responseTimes += $responseTime
    Start-Sleep -Milliseconds 500  # Optional delay between requests to avoid flooding the server
}

# Calculate stat
istics
$averageResponseTime = ($responseTimes | Measure-Object -Average).Average
$maxResponseTime = ($responseTimes | Measure-Object -Maximum).Maximum

# Output the results
Write-Host "Website Benchmarking Results"
Write-Host "----------------------------------"
Write-Host "Number of requests made: $NumberOfRequests"
Write-Host "Average Response Time (ms): $averageResponseTime"
Write-Host "Maximum Response Time (ms): $maxResponseTime"
