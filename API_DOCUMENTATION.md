# API Documentation Guide

This repository includes comprehensive OpenAPI 3.0 documentation for the Home Access Center API.

## Quick Start

### View Documentation Locally

1. **HTML Documentation** (Recommended for quick reference)
   ```bash
   # Open docs.html in your browser
   open docs.html  # macOS
   xdg-open docs.html  # Linux
   start docs.html  # Windows
   ```

2. **OpenAPI Specification**
   - The complete OpenAPI 3.0 spec is in `openapi.yaml`
   - Can be imported into any OpenAPI-compatible tool

### Use with Popular Tools

#### Swagger Editor (Online)
1. Visit [editor.swagger.io](https://editor.swagger.io/)
2. File → Import File → Select `openapi.yaml`
3. View interactive documentation and test endpoints

#### VS Code
1. Install "OpenAPI (Swagger) Editor" extension
2. Open `openapi.yaml`
3. Right-click → "Preview Swagger"

#### Postman
1. Open Postman
2. Import → Upload Files → Select `openapi.yaml`
3. All endpoints will be imported as a collection

#### Insomnia
1. Open Insomnia
2. Create New → Import From → File → Select `openapi.yaml`
3. All endpoints ready to use

#### Redocly CLI
```bash
# Install Redocly CLI
npm install -g @redocly/cli

# Generate beautiful HTML documentation
redocly build-docs openapi.yaml --output docs-preview.html

# Start a local preview server
redocly preview-docs openapi.yaml
```

## What's Documented

### 13 Endpoints Across 4 Categories

**Student Information**
- `GET /api/name` - Student's full name
- `GET /api/info` - Complete student profile

**Classes**
- `GET /api/classes` - List of enrolled classes

**Grades**
- `GET /api/averages` - Current class averages
- `GET /api/assignments` - Detailed assignments
- `GET /api/weightings` - Grade category weightings
- `GET /api/gradebook` - Complete gradebook

**Reports**
- `GET /api/reportcard` - Official report cards
- `GET /api/ipr` - Interim progress reports
- `GET /api/transcript` - Full academic transcript
- `GET /api/rank` - Class rank and quartile

### Authentication

All endpoints require query parameters:
- `user` - Home Access Center username (required)
- `pass` - Home Access Center password (required)
- `link` - Portal URL (optional, defaults to Katy ISD)

### Optional Parameters

- `short` - Use shortened class names (boolean)
- `six_weeks` - Specific grading period (string)
- `no_cache` - Bypass cache (boolean)

## Examples

### Basic Request
```bash
curl "http://localhost:3000/api/name?user=student123&pass=mypassword"
```

### With Optional Parameters
```bash
curl "http://localhost:3000/api/averages?user=student123&pass=mypassword&short=true&no_cache=true"
```

### Complete Gradebook
```bash
curl "http://localhost:3000/api/gradebook?user=student123&pass=mypassword" | jq
```

## Response Formats

All endpoints return JSON with appropriate status codes:
- `200` - Success
- `401` - Invalid credentials
- `500` - Server error

## Caching

The API implements intelligent caching:
- Login sessions: 30 minutes
- Page data: 5 minutes
- Override with `no_cache=true` parameter

## Additional Resources

- [OpenAPI Specification](https://spec.openapis.org/oas/v3.0.3)
- [Swagger Tools](https://swagger.io/tools/)
- [Redocly Documentation](https://redocly.com/docs/)
- [Full Documentation Site](https://homeaccesscenterapi-docs.vercel.app/)

## Contributing

When adding new endpoints, please update:
1. `openapi.yaml` - Add endpoint definition
2. `docs.html` - Add to appropriate section
3. `README.md` - Update endpoint list
4. Validate with: `swagger-cli validate openapi.yaml`
