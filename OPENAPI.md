# OpenAPI Documentation

This directory contains the OpenAPI 3.0 specification for the Home Access Center API.

## Files

- **openapi.yaml** - OpenAPI specification in YAML format (recommended for editing)
- **openapi.json** - OpenAPI specification in JSON format (auto-generated from YAML)

## Viewing the Documentation

### Interactive Documentation (Recommended)

Visit the hosted Swagger UI interface:
```
https://hac.packjack.dev/docs
```

This provides:
- Interactive API explorer with "Try it out" functionality
- Complete parameter descriptions and examples
- Response schemas and examples
- No downloads or setup required
- Search and filter capabilities

### Raw Specification Files

The OpenAPI specification files are also available:
- YAML: https://hac.packjack.dev/openapi.yaml
- JSON: https://hac.packjack.dev/openapi.json

### Using with API Tools

You can use these specification files with various OpenAPI tools:

#### Swagger Editor
Visit https://editor.swagger.io/ and import the URL:
```
https://hac.packjack.dev/openapi.yaml
```

#### Redoc
Visit https://redocly.github.io/redoc/ and import the URL:
```
https://hac.packjack.dev/openapi.yaml
```

#### Postman
1. Open Postman
2. Click "Import"
3. Select "Link" tab
4. Paste: `https://hac.packjack.dev/openapi.json`
5. Click "Import"

#### curl (download locally)
```bash
curl -O https://hac.packjack.dev/openapi.yaml
curl -O https://hac.packjack.dev/openapi.json
```

## Updating the Documentation

If you modify `openapi.yaml`, regenerate the JSON file:

```bash
npm install -g js-yaml
npx js-yaml openapi.yaml > openapi.json
```

To validate the OpenAPI spec:

```bash
npm install -g @apidevtools/swagger-cli
swagger-cli validate openapi.yaml
```

## API Overview

The Home Access Center API provides 11 endpoints:

### Student Information
- `GET /api/name` - Get student name
- `GET /api/info` - Get student profile information

### Classes
- `GET /api/classes` - Get list of classes
- `GET /api/averages` - Get class averages
- `GET /api/weightings` - Get grade category weightings

### Assignments & Grades
- `GET /api/assignments` - Get detailed assignments
- `GET /api/gradebook` - Get complete gradebook with grades and weightings

### Reports
- `GET /api/reportcard` - Get report card
- `GET /api/ipr` - Get interim progress report
- `GET /api/transcript` - Get full transcript with GPA
- `GET /api/rank` - Get GPA rank and quartile

All endpoints (except `/` and `/api/`) require authentication via query parameters:
- `user` - Home Access Center username
- `pass` - Home Access Center password

Optional parameters:
- `link` - Home Access Center base URL (defaults to https://homeaccess.katyisd.org)
- `short` - Return shortened class names (boolean)
- `six_weeks` - Specific six weeks period (for assignment endpoints)
- `no_cache` - Bypass cache and fetch fresh data (boolean)
