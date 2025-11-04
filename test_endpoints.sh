#!/bin/bash

# Test script to call all Home Access Center API endpoints
# Usage: ./test_endpoints.sh <username> <password> [base_url]
#
# Examples:
#   ./test_endpoints.sh myuser mypass
#   ./test_endpoints.sh myuser mypass https://homeaccess.katyisd.org

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if required arguments are provided
if [ $# -lt 2 ]; then
    echo -e "${RED}Error: Missing required arguments${NC}"
    echo "Usage: $0 <username> <password> [base_url] [api_url]"
    echo ""
    echo "Arguments:"
    echo "  username  - Home Access Center username"
    echo "  password  - Home Access Center password"
    echo "  base_url  - (Optional) Home Access Center base URL (default: https://homeaccess.katyisd.org)"
    echo "  api_url   - (Optional) API server URL (default: http://localhost:3000)"
    echo ""
    echo "Example:"
    echo "  $0 student123 mypassword"
    echo "  $0 student123 mypassword https://homeaccess.katyisd.org http://localhost:3000"
    exit 1
fi

# Parse arguments
USERNAME="$1"
PASSWORD="$2"
HAC_BASE_URL="${3:-https://homeaccess.katyisd.org}"
API_URL="${4:-http://localhost:3000}"

# Output file
OUTPUT_FILE="api_responses_$(date +%Y%m%d_%H%M%S).txt"

# Create separator function
print_separator() {
    echo "================================================================================"
}

# Function to make API call and save response
call_endpoint() {
    local endpoint="$1"
    local description="$2"
    local extra_params="$3"
    
    echo -e "${YELLOW}Testing: $endpoint - $description${NC}"
    
    # Build URL with parameters
    local url="${API_URL}${endpoint}?user=${USERNAME}&pass=${PASSWORD}&link=${HAC_BASE_URL}"
    
    # Add extra parameters if provided
    if [ -n "$extra_params" ]; then
        url="${url}&${extra_params}"
    fi
    
    # Make request and capture response with timeout
    local http_code
    local response
    
    response=$(curl -s -m 30 -w "\nHTTP_STATUS:%{http_code}" "$url" 2>&1)
    http_code=$(echo "$response" | grep "HTTP_STATUS:" | cut -d: -f2)
    response=$(echo "$response" | sed '/HTTP_STATUS:/d')
    
    # Write to output file
    {
        print_separator
        echo "ENDPOINT: $endpoint"
        echo "DESCRIPTION: $description"
        echo "URL: $url"
        echo "TIMESTAMP: $(date '+%Y-%m-%d %H:%M:%S')"
        echo "HTTP STATUS: $http_code"
        print_separator
        echo ""
        echo "$response"
        echo ""
        echo ""
    } >> "$OUTPUT_FILE"
    
    # Print status
    if [ "$http_code" -eq 200 ]; then
        echo -e "${GREEN}✓ Success (HTTP $http_code)${NC}"
    else
        echo -e "${RED}✗ Failed (HTTP $http_code)${NC}"
    fi
    echo ""
}

# Start test
echo "========================================================"
echo "  Home Access Center API Endpoint Test"
echo "========================================================"
echo ""
echo "API URL: $API_URL"
echo "HAC URL: $HAC_BASE_URL"
echo "Username: $USERNAME"
echo "Output file: $OUTPUT_FILE"
echo ""
echo "Starting tests..."
echo ""

# Initialize output file
{
    echo "========================================================================"
    echo "  HOME ACCESS CENTER API - ENDPOINT TEST RESULTS"
    echo "========================================================================"
    echo ""
    echo "Test Date: $(date '+%Y-%m-%d %H:%M:%S')"
    echo "API URL: $API_URL"
    echo "HAC Base URL: $HAC_BASE_URL"
    echo "Username: $USERNAME"
    echo ""
    echo "This file contains responses from all API endpoints."
    echo ""
} > "$OUTPUT_FILE"

# Test root endpoint (no auth required)
echo -e "${YELLOW}Testing: / - Root endpoint${NC}"
response=$(curl -s -m 10 "${API_URL}/" 2>&1)
{
    print_separator
    echo "ENDPOINT: /"
    echo "DESCRIPTION: Root endpoint (no authentication)"
    echo "URL: ${API_URL}/"
    echo "TIMESTAMP: $(date '+%Y-%m-%d %H:%M:%S')"
    print_separator
    echo ""
    echo "$response"
    echo ""
    echo ""
} >> "$OUTPUT_FILE"
echo -e "${GREEN}✓ Success${NC}"
echo ""

# Test all authenticated endpoints
call_endpoint "/api/name" "Get student name"
call_endpoint "/api/info" "Get student information"
call_endpoint "/api/classes" "Get list of classes"
call_endpoint "/api/classes" "Get list of classes (short names)" "short=true"
call_endpoint "/api/averages" "Get class averages"
call_endpoint "/api/averages" "Get class averages (short names)" "short=true"
call_endpoint "/api/assignments" "Get detailed assignments"
call_endpoint "/api/assignments" "Get detailed assignments (short names)" "short=true"
call_endpoint "/api/gradebook" "Get complete gradebook"
call_endpoint "/api/gradebook" "Get complete gradebook (short names)" "short=true"
call_endpoint "/api/weightings" "Get grade weightings"
call_endpoint "/api/weightings" "Get grade weightings (short names)" "short=true"
call_endpoint "/api/reportcard" "Get report card"
call_endpoint "/api/ipr" "Get interim progress report"
call_endpoint "/api/transcript" "Get full transcript"
call_endpoint "/api/rank" "Get GPA rank and quartile"

# Summary
echo "========================================================"
echo "  Test Complete!"
echo "========================================================"
echo ""
echo "All responses have been saved to: $OUTPUT_FILE"
echo ""
echo "You can view the results with:"
echo "  cat $OUTPUT_FILE"
echo "  less $OUTPUT_FILE"
echo ""
