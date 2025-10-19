# Home Access Center API

API for accessing Home Access Center student information including grades, assignments, transcripts, and more.

## API Documentation

Full OpenAPI 3.0 specification is available in [`openapi.yaml`](./openapi.yaml).

You can view the interactive API documentation by:
- Opening [`docs.html`](./docs.html) in your browser for local Swagger UI interface
- Opening `openapi.yaml` in [Swagger Editor](https://editor.swagger.io/)
- Using tools like Redoc, Swagger UI, or Stoplight
- Visiting the full documentation at https://homeaccesscenterapi-docs.vercel.app/

## Available Endpoints

### Student Information
- `/api/name` - Get student name
- `/api/info` - Get student profile (grade, school, DOB, counselor, language, cohort year)

### Classes & Grades
- `/api/classes` - Get list of classes
- `/api/averages` - Get class averages
- `/api/assignments` - Get detailed assignments with grades
- `/api/weightings` - Get grade category weightings
- `/api/gradebook` - Get complete gradebook (averages + assignments + weightings)

### Reports
- `/api/reportcard` - Get report card tables
- `/api/ipr` - Get interim progress report
- `/api/transcript` - Get full transcript with GPA and semesters
- `/api/rank` - Get GPA rank and quartile

## Query Parameters

All endpoints require authentication and support the following parameters:

**Required:**
- `user` - Home Access Center username
- `pass` - Home Access Center password

**Optional:**
- `link` - Home Access Center portal URL (defaults to `https://homeaccess.katyisd.org`)
- `short` - Use shortened class names (boolean, for class-related endpoints)
- `six_weeks` - Specific six weeks period to retrieve (for assignment endpoints)
- `no_cache` - Bypass cache and force fresh data retrieval (boolean)

## Example Usage

```bash
# Get student name
curl "http://localhost:3000/api/name?user=USERNAME&pass=PASSWORD"

# Get class averages with shortened names
curl "http://localhost:3000/api/averages?user=USERNAME&pass=PASSWORD&short=true"

# Get assignments for specific six weeks period, bypassing cache
curl "http://localhost:3000/api/assignments?user=USERNAME&pass=PASSWORD&six_weeks=3&no_cache=true"
```

## Implementation Status

All endpoints implemented ✓
