# Walrus Hosting Site

TBD

## Tests

Unit tests are co-located next to the modules that they're testing. When deeper tests are needed they will be in a global `/tests` folder.

## Environment Variables

All required environment variables (and their requirements) are listed in `.env.example`.

### For Development
- Your IDE or language server uses the values in `.env` to provide autocomplete and query checking for sqlx.
- If your development database runs locally, set its URL in `.env` using `localhost` (e.g. `postgres://user@localhost:5432/walrus`).
- For the container runtime, use `.env.local` to override any values that need the container's perspective, such as the database host (`postgres://user@db:5432/walrus`) as per the docker compose.

This ensures your IDE and/or language serve works correctly while the Docker container connects to the database correctly.
