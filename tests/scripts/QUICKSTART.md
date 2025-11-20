# Test Suite Quickstart Guide

Get the build script test suite running in under 5 minutes.

## 🚀 Quick Setup

### 1. Install BATS

**macOS:**
```bash
brew install bats-core
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install bats
```

**npm (any platform):**
```bash
npm install -g bats
```

### 2. Verify Installation

```bash
bats --version
# Expected: Bats 1.x.x
```

### 3. Run Tests

```bash
# From project root
cd /Users/tony/Develop/pane

# Run all implemented tests (28 tests)
bats tests/scripts/*.bats
```

Expected output:
```
✓ 5.2-UNIT-001: Script exists and is executable
✓ 5.2-UNIT-002: Script has proper shebang (#!/usr/bin/env bash)
✓ 5.2-UNIT-003: Script sets strict error handling (set -e -u -o pipefail)
...
28 tests, 0 failures
```

## 📊 What's Tested (28 Tests Implemented)

### ✅ Prerequisites (8 tests) - P0
- Script exists and executable
- Proper shebang and error handling
- Rust toolchain detection
- Git availability
- Error message quality

### ✅ Build Execution (7 tests) - P0/P1
- Workspace builds both binaries
- `--release` flag usage
- `--workspace` flag usage
- Build failure handling
- Progress messages

### ✅ Platform Detection (7 tests) - P0/P1
- Valid target acceptance (3 platforms)
- Invalid target rejection
- Auto-detection
- Error messages

### ✅ Flag Parsing (6 tests) - P0/P1
- `--help` displays usage
- `--dry-run` prevents execution
- `--clean` flag recognition
- `--target` flag parsing
- Unknown flag errors

## 🎯 Test Results Impact

**Current Coverage**: 28/47 planned scenarios (60%)

**Quality Gate Impact:**
- Reliability NFR: CONCERNS → Moving toward PASS
- Automated Coverage: 0% → 60% (P0/P1 complete)
- Technical Debt: High → Medium

## 🔍 Quick Test Examples

### Run Single Test File
```bash
bats tests/scripts/test_prerequisites.bats
```

### Run Specific Test by ID
```bash
bats -f "5.2-UNIT-014" tests/scripts/test_prerequisites.bats
```

### Verbose Output
```bash
bats -t tests/scripts/*.bats
```

## 🐛 Troubleshooting

### "bats: command not found"
```bash
# Install BATS first
brew install bats-core  # macOS
# or
npm install -g bats     # any platform
```

### "No such file or directory: scripts/build-release.sh"
```bash
# Run from project root
cd /Users/tony/Develop/pane
pwd  # Should show: /Users/tony/Develop/pane
```

### Tests Fail with Permission Errors
```bash
# Make scripts executable
chmod +x tests/scripts/*.bats
chmod +x tests/scripts/helpers/*.bash
```

## 📝 Next Steps

### Remaining Implementation (19 tests)

**To reach 80% coverage target:**

1. **AC5: Artifact Organization** (6 tests)
   - File: `test_artifacts.bats`
   - Effort: 3-4 hours

2. **AC6-7: Compression & Checksums** (9 tests)
   - File: `test_compression.bats`
   - Effort: 4-5 hours

3. **AC8: Metadata** (4 tests)
   - File: `test_metadata.bats`
   - Effort: 2-3 hours

**Total Remaining Effort**: ~10-12 hours to reach 80% coverage

### CI Integration

```yaml
# .github/workflows/test-build-script.yml
name: Build Script Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install BATS
        run: npm install -g bats
      - name: Run tests
        run: bats tests/scripts/*.bats
```

## 📚 Resources

- **Test Design**: `docs/qa/assessments/5.2-test-design-20251119.md`
- **Detailed README**: `tests/scripts/README.md`
- **Quality Gate**: `docs/qa/gates/5.2-build-release-automation-scripts.yml`

## ✨ Success Metrics

**Test Suite Acceptance Criteria:**
- [x] P0 tests implemented (21/21 ✅)
- [x] Test execution time <20 minutes (currently <1 minute ✅)
- [ ] ≥80% scenario coverage (currently 60%, need 19 more tests)
- [ ] CI pipeline integrated

**When Complete:**
- Gate Status: CONCERNS (80/100) → PASS (95/100)
- Story 5.2 ready for production deployment
- Regression protection in place
- Technical debt resolved

---

**Current Status**: 🟡 **60% Complete** - P0/P1 tests done, P2 remaining
**Time to 80%**: ~10-12 hours additional implementation
**Time to 100%**: ~2-3 days total (as estimated)
