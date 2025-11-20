# Security

## Input Validation

**Validation Library:** Native Rust type system + `serde` validation

**Validation Location:** At API boundaries (CLI parsing, manifest loading, config loading)

**Required Rules:**
- All external inputs MUST be validated
- Validation at parsing time before any processing
- Whitelist approach preferred over blacklist

**Specific Validations:**
1. **Skill Manifest Fields:**
   - `id`: Alphanumeric + hyphens only, max 64 chars
   - `exec`: Must not contain shell metacharacters (`;`, `|`, `&`, etc.)
   - `args`: Array of strings, no shell expansion
   - `tags`: Lowercase alphanumeric + hyphens, max 32 chars each

2. **Config File:**
   - `max_recent_skills`: 1-100 range
   - File paths: Valid UTF-8, no null bytes
   - Favorites/recent: Skill IDs validated against loaded skills

3. **User Input (Search Query):**
   - Max 256 characters
   - UTF-8 validation

## Authentication & Authorization

**N/A** - Local-only application, no authentication required

**Required Patterns:**
- Skills execute with user's permissions (no privilege escalation)
- File system permissions enforced by OS

## Secrets Management

**Development:** No secrets in code or config

**Production:** N/A - No external services requiring secrets

**Code Requirements:**
- NEVER hardcode paths to sensitive files (`~/.ssh/`, `~/.aws/`, etc.)
- No secrets in logs or error messages
- Environment variables sandboxed (only `PANE_*` passed)

## API Security

**N/A** - No HTTP server or IPC endpoints

## Data Protection

**Encryption at Rest:** Not required - config and manifests contain no sensitive data

**Encryption in Transit:** N/A - Local-only application

**PII Handling:**
- No PII collection
- User paths logged as basenames only
- Git context is project-related only

**Logging Restrictions:**
- Never log: File contents, environment variables (except `PANE_*`), full user paths, skill output
- Safe to log: Skill IDs, skill names, error types, component names, correlation IDs

## Dependency Security

**Scanning Tool:** `cargo audit` (integrated in CI)

**Update Policy:**
- Critical vulnerabilities: Patch within 24 hours, release hotfix
- High severity: Patch within 7 days, include in next release
- Medium/Low: Patch in regular release cycle

**Approval Process:**
- New dependencies require architectural review
- Check crates.io for download count (>100K preferred), recent updates (<6 months), license compatibility
- Avoid dependencies with security issues or abandoned maintenance

## Security Testing

**SAST Tool:** `cargo clippy` with security lints enabled

**DAST Tool:** Not applicable

**Penetration Testing:** Not required for MVP

**Security Review Checklist:**
- No `unsafe` code blocks (except in well-justified, audited cases)
- No shell command injection vectors
- Input validation at all boundaries
- Error messages don't leak sensitive information
- Dependencies scanned with `cargo audit`
- File operations use canonical paths (no symlink attacks)
