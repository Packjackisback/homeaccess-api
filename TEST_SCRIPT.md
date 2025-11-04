# API Endpoint Testing Script

This bash script (`test_endpoints.sh`) automatically calls all Home Access Center API endpoints and saves the responses to a text file.

## Features

- Tests all 11 authenticated API endpoints
- Tests both regular and short-name variants where applicable
- Saves responses to a timestamped text file
- Shows color-coded success/failure status
- Includes HTTP status codes
- Supports custom HAC base URLs
- 30-second timeout per request

## Requirements

- `bash` shell
- `curl` command-line tool
- Access to the API server (locally or remote)
- Valid Home Access Center credentials

## Usage

### Basic Usage

```bash
./test_endpoints.sh <username> <password>
```

This will:
- Connect to `http://localhost:3000` (default API server)
- Use `https://homeaccess.katyisd.org` (default HAC URL)
- Save results to `api_responses_YYYYMMDD_HHMMSS.txt`

### Custom HAC URL

```bash
./test_endpoints.sh <username> <password> https://homeaccess.yourschool.org
```

### Custom API Server

```bash
./test_endpoints.sh <username> <password> https://homeaccess.katyisd.org http://localhost:3000
```

### Remote API Server

```bash
./test_endpoints.sh <username> <password> https://homeaccess.katyisd.org https://hac.packjack.dev
```

## Examples

### Local Development

```bash
# Start the API server
cargo run

# In another terminal, run the test script
./test_endpoints.sh student123 mypassword
```

### Testing Production

```bash
./test_endpoints.sh student123 mypassword https://homeaccess.katyisd.org https://hac.packjack.dev
```

## Endpoints Tested

The script tests the following endpoints:

1. `/` - Root endpoint (no auth)
2. `/api/name` - Student name
3. `/api/info` - Student information
4. `/api/classes` - List of classes (regular and short)
5. `/api/averages` - Class averages (regular and short)
6. `/api/assignments` - Detailed assignments (regular and short)
7. `/api/gradebook` - Complete gradebook (regular and short)
8. `/api/weightings` - Grade weightings (regular and short)
9. `/api/reportcard` - Report card
10. `/api/ipr` - Interim progress report
11. `/api/transcript` - Full transcript
12. `/api/rank` - GPA rank and quartile

## Output Format

The script creates a timestamped file (e.g., `api_responses_20251104_143022.txt`) with:

```
========================================================================
  HOME ACCESS CENTER API - ENDPOINT TEST RESULTS
========================================================================

Test Date: 2025-11-04 14:30:22
API URL: http://localhost:3000
HAC Base URL: https://homeaccess.katyisd.org
Username: student123

This file contains responses from all API endpoints.

================================================================================
ENDPOINT: /api/name
DESCRIPTION: Get student name
URL: http://localhost:3000/api/name?user=student123&pass=...
TIMESTAMP: 2025-11-04 14:30:25
HTTP STATUS: 200
================================================================================

{"name": "John Doe"}

...
```

## Viewing Results

```bash
# View the entire file
cat api_responses_20251104_143022.txt

# View with pagination
less api_responses_20251104_143022.txt

# Search for a specific endpoint
grep -A 20 "ENDPOINT: /api/name" api_responses_20251104_143022.txt

# View only successful responses (HTTP 200)
grep -B 5 "HTTP STATUS: 200" api_responses_20251104_143022.txt
```

## Troubleshooting

### Connection Refused

If you get connection errors:
1. Make sure the API server is running
2. Check if the API URL is correct
3. Verify the port is accessible

### Authentication Errors (HTTP 401)

If you get unauthorized errors:
1. Verify your username and password are correct
2. Check that the HAC base URL is correct for your school district
3. Ensure your account has access to Home Access Center

### Timeout Errors

If requests timeout:
1. The HAC server might be slow - this is normal
2. Your credentials might be invalid (causes slow rejection)
3. Network connectivity issues

## Security Notes

⚠️ **Important**: This script passes credentials as command-line arguments and in URLs. 

- Don't run this script on shared systems
- Don't commit the output files to version control (they contain your data)
- The output files are added to `.gitignore` automatically
- Consider deleting output files after reviewing them

## Customization

You can modify the script to:
- Add more endpoints
- Change the output format
- Add JSON formatting with `jq`
- Filter specific fields from responses
- Add retry logic for failed requests

Example with `jq` formatting:

```bash
# Pretty-print JSON responses
cat api_responses_20251104_143022.txt | grep -v "^===" | grep -v "^ENDPOINT" | grep -v "^DESCRIPTION" | grep -v "^URL" | grep -v "^TIMESTAMP" | grep -v "^HTTP STATUS" | jq '.' 2>/dev/null
```
