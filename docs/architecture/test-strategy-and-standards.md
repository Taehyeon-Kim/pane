# Test Strategy and Standards

## Testing Philosophy

**Approach:** Test-Driven Development (TDD) encouraged but not mandatory

**Coverage Goals:**
- Unit Tests: ≥80% code coverage for core logic
- Integration Tests: ≥70% coverage of critical user workflows
- End-to-End Tests: Manual for MVP, automated in Phase 2

**Test Pyramid:**
- 70% Unit Tests - Fast, isolated, comprehensive coverage
- 20% Integration Tests - Component interactions
- 10% E2E Tests - Complete user journeys

## Test Types and Organization

### Unit Tests

**Framework:** `cargo test` + `rstest` 0.18.0 for parametric tests

**File Convention:** Co-located with source files using `#[cfg(test)] mod tests { ... }`

**Location:** Same file as the code being tested

**Mocking Library:** `mockall` 0.12.0 for mocking traits (where needed)

**Coverage Requirement:** ≥80% for core modules

**AI Agent Requirements:**
- Generate tests for all public methods
- Cover edge cases and error paths
- Follow AAA pattern (Arrange, Act, Assert)
- Mock all external dependencies

### Integration Tests

**Scope:** Test interactions between multiple components

**Location:** `tests/integration/` directory

**Test Infrastructure:**
- File System: Use `tempfile` crate for temporary test directories
- Git Repositories: Use `git2` to create temporary git repos
- Config Files: Use fixtures in `tests/fixtures/configs/`

### End-to-End Tests

**Framework:** Manual testing for MVP

**Scope:** Complete user workflows from `pane` command to skill execution and return

**Environment:** Actual terminal environment with real skill executables

**Test Data:** Use bundled Claude Code Tips Viewer skill as primary E2E test target

## Test Data Management

**Strategy:** Fixtures with version control

**Fixtures Location:** `tests/fixtures/`

**Structure:**
```
tests/fixtures/
├── skills/          # Valid and invalid skill manifests
├── configs/         # Config file variations
└── tips/            # Sample tips data
```

**Factories:** Use builder pattern for creating test data programmatically

**Cleanup Strategy:**
- Unit tests: No cleanup needed (in-memory)
- Integration tests: `tempfile::TempDir` auto-cleanup via `Drop`
- E2E tests: Manual cleanup or temporary user directories

## Continuous Testing

**CI Integration:**
- On every PR: Run all unit and integration tests
- On merge to main: Run full test suite + coverage report
- On release tag: Run smoke tests on built binaries

**Performance Tests:** Not in MVP, consider in Phase 2

**Security Tests:**
- `cargo audit` in CI (weekly + on every release)
- Dependency vulnerability scanning via GitHub Dependabot
