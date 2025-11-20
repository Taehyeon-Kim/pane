# Build Script Test Suite

Automated test suite for `scripts/build-release.sh` using BATS (Bash Automated Testing System).

## Prerequisites

### Install BATS

**Option 1: npm** (Recommended)
```bash
npm install -g bats
```

**Option 2: Homebrew** (macOS)
```bash
brew install bats-core
```

**Option 3: From source**
```bash
git clone https://github.com/bats-core/bats-core.git
cd bats-core
./install.sh /usr/local
```

### Verify Installation

```bash
bats --version
# Should output: Bats 1.x.x
```

## Running Tests

### Run All Tests
```bash
bats tests/scripts/*.bats
```

### Run Specific Test File
```bash
bats tests/scripts/test_prerequisites.bats
bats tests/scripts/test_build.bats
bats tests/scripts/test_platform.bats
bats tests/scripts/test_flags.bats
```

### Run with Verbose Output
```bash
bats -t tests/scripts/*.bats
```

### Run Specific Test by Name
```bash
bats -f "5.2-UNIT-014" tests/scripts/test_prerequisites.bats
```

## Test Structure

```
tests/scripts/
├── README.md                    # This file
├── test_prerequisites.bats      # 8 tests - P0 prerequisite validation
├── test_build.bats              # 7 tests - P0/P1 build execution
├── test_platform.bats           # 7 tests - P0/P1 platform detection
├── test_flags.bats              # 6 tests - P0/P1 flag parsing
└── helpers/
    ├── setup_suite.bash         # Shared test setup
    ├── teardown_suite.bash      # Shared test cleanup
    ├── mock_cargo.bash          # Mock cargo commands
    ├── mock_git.bash            # Mock git commands
    └── assertions.bash          # Custom assertions
```

## Test Coverage

### Current Implementation

**Test Files Created**: 4
**Total Tests**: 28 P0/P1 tests implemented
**Coverage**: ~60% of 47 planned scenarios

### Priority Breakdown
- **P0 Tests**: 21 tests (critical path)
- **P1 Tests**: 7 tests (core functionality)
- **Not Yet Implemented**: P2/P3 tests (19 scenarios)

### Coverage by Acceptance Criteria

| AC | Description | Tests | Status |
|----|-------------|-------|--------|
| AC1 | Script creation | 3 | ✅ Implemented |
| AC2 | Build workspace | 3 | ✅ Implemented |
| AC3 | Release profile | 3 | ✅ Implemented |
| AC4 | Cross-compilation | 7 | ✅ Implemented |
| AC5 | Organize dist/ | 0 | ⏳ Planned |
| AC6 | Compression | 0 | ⏳ Planned |
| AC7 | Checksums | 0 | ⏳ Planned |
| AC8 | Metadata | 0 | ⏳ Planned |
| AC9 | Error messages | 6 | ✅ Implemented |
| AC10 | CI compatibility | 0 | ⏳ Planned |

## Test IDs Reference

All tests follow the naming convention: `5.2-{LEVEL}-{SEQ}`

Examples:
- `5.2-UNIT-001`: Unit test #1
- `5.2-INT-001`: Integration test #1
- `5.2-E2E-001`: E2E test #1

See `docs/qa/assessments/5.2-test-design-20251119.md` for complete test design.

## Mocking Strategy

### Unit-Style Tests
- Mock all external commands (cargo, git, tar)
- Test individual script functions in isolation
- Fast execution (<1 second per test)

### Integration-Style Tests
- Mock external commands but test full script flow
- Validate file system changes
- Moderate execution time (1-5 seconds per test)

### E2E Tests (Not Yet Implemented)
- Use real Rust toolchain, git, tar
- Run on actual platforms
- Slower execution (minutes per test)

## CI Integration

### GitHub Actions Example

```yaml
name: Test Build Script

on: [push, pull_request]

jobs:
  test-build-script:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Install BATS
        run: npm install -g bats

      - name: Run build script tests
        run: bats tests/scripts/*.bats

      - name: Upload test results
        if: always()
        uses: actions/upload-artifact@v3
        with:
          name: test-results
          path: test-results/
```

## Writing New Tests

### Basic Test Structure

```bash
#!/usr/bin/env bats
# Test Suite: Description
# Priority: P0/P1/P2/P3
# Test IDs: 5.2-XXX-NNN

load 'helpers/setup_suite'
load 'helpers/mock_cargo'
load 'helpers/assertions'

setup() {
    export PATH="$MOCK_BIN_DIR:$ORIGINAL_PATH"
    cd "$BATS_TEST_TMPDIR"
}

@test "5.2-XXX-NNN: Test description" {
    # Arrange
    mock_cargo_success

    # Act
    run scripts/build-release.sh

    # Assert
    assert_exit_status 0
    assert_output_contains "expected string"
}
```

### Available Assertions

- `assert_file_exists <path>`
- `assert_dir_exists <path>`
- `assert_output_contains <string>`
- `assert_file_contains <file> <string>`
- `assert_exit_status <code>`

### Available Mocks

- `mock_cargo_success` - Successful cargo build
- `mock_cargo_failure` - Failed cargo build
- `mock_cargo_missing` - Cargo not installed
- `mock_git_success` - Successful git commands
- `mock_git_missing` - Git not installed

## Troubleshooting

### Tests Fail with "command not found"
- Ensure BATS is installed: `bats --version`
- Check PATH includes BATS installation

### Tests Fail with "No such file or directory"
- Run tests from project root directory
- Verify script path: `scripts/build-release.sh`

### Mocks Not Working
- Check `$MOCK_BIN_DIR` is in PATH
- Verify mock scripts have execute permissions
- Review `setup()` function in test file

### Tests Hang or Timeout
- Check for infinite loops in mocked commands
- Verify cleanup in teardown functions
- Use `bats -t` for trace output

## Next Steps

### Remaining Implementation (19 tests)

**AC5: Artifact Organization** (6 tests)
- Directory structure validation
- Version extraction
- Multi-target organization

**AC6-7: Compression & Checksums** (9 tests)
- Archive creation
- Checksum generation
- Integrity validation

**AC8: Metadata** (6 tests)
- JSON schema validation
- Field completeness
- Format compliance

**AC10: CI Compatibility** (5 tests)
- Non-interactive execution
- Environment variable handling
- GitHub Actions integration

### Future Enhancements

- [ ] Add test coverage reporting
- [ ] Implement E2E tests with real builds
- [ ] Add performance benchmarks
- [ ] Create CI pipeline integration
- [ ] Add mutation testing for robustness

## Resources

- [BATS Documentation](https://bats-core.readthedocs.io/)
- [Test Design Document](../../docs/qa/assessments/5.2-test-design-20251119.md)
- [Quality Gate](../../docs/qa/gates/5.2-build-release-automation-scripts.yml)
- [Story 5.2](../../docs/stories/5.2.story.md)

## Support

For questions or issues with the test suite:
1. Review test design document
2. Check BATS documentation
3. Examine existing test examples
4. Review helper function implementations
