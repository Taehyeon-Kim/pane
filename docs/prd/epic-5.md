# Epic 5: Installation & Distribution System

## Epic Goal

사용자가 Pane을 표준 패키지 관리자(Homebrew, Cargo)를 통해 쉽게 설치하고, 번들된 스킬(Claude Tips Viewer)을 즉시 사용할 수 있도록 완전한 배포 인프라를 구축한다.

## Epic Scope

- 설치/제거 스크립트 작성 (install.sh, uninstall.sh)
- 빌드 및 릴리스 자동화 스크립트 (build-release.sh)
- Homebrew formula 및 tap 구성
- GitHub Actions CI/CD 파이프라인 (ci.yml, release.yml, security-audit.yml)
- 스킬 번들링 메커니즘
- End-to-end 설치 검증 및 테스트

## Success Criteria

- 사용자가 `brew install pane`으로 Pane 설치 가능
- 설치 후 `pane` 실행 시 Claude Tips Viewer 스킬 자동 발견
- `cargo install pane`으로도 설치 가능
- GitHub Releases에서 사전 빌드된 바이너리 다운로드 가능
- CI/CD 파이프라인이 자동으로 릴리스 생성
- 모든 플랫폼(macOS x86_64/ARM64, Linux x86_64)에서 설치 검증 완료

---

## Story 5.1: Installation & Uninstallation Scripts

### Story Statement

**As a** user,
**I want** simple installation and uninstallation scripts,
**so that** I can easily install Pane and its bundled skills without manual configuration.

### Acceptance Criteria

1. `scripts/install.sh` 스크립트 생성
2. 스크립트가 `/usr/local/bin/pane` 및 `/usr/local/bin/claude-tips` 바이너리 설치
3. `/usr/local/share/pane/skills/claude-tips/` 디렉토리 생성 및 manifest 복사
4. `skills/claude-tips/data/claude-tips.yaml` 데이터 파일 복사
5. 설치 과정 진행 메시지 출력 (progress feedback)
6. 설치 실패 시 명확한 에러 메시지와 롤백
7. `scripts/uninstall.sh` 스크립트 생성
8. 제거 스크립트가 모든 설치 파일 및 디렉토리 삭제
9. macOS 및 Linux에서 테스트 통과
10. sudo 권한 필요 시 명확한 안내 메시지

### Technical Notes

- Bash 스크립트 사용 (POSIX 호환)
- `$PREFIX` 변수로 설치 경로 커스터마이징 지원 (기본: `/usr/local`)
- 기존 설치 감지 및 업그레이드 지원
- Dry-run 모드 지원 (`--dry-run` 플래그)

---

## Story 5.2: Build & Release Automation Scripts

### Story Statement

**As a** developer,
**I want** automated build scripts for release binaries,
**so that** I can consistently produce optimized binaries for distribution.

### Acceptance Criteria

1. `scripts/build-release.sh` 스크립트 생성
2. 스크립트가 workspace의 모든 바이너리 빌드 (`pane`, `claude-tips`)
3. Release profile 사용 (opt-level=3, lto=true, strip=true)
4. 크로스 컴파일 지원 (macOS x86_64, macOS ARM64, Linux x86_64)
5. 빌드 아티팩트를 `dist/` 디렉토리에 정리
6. 바이너리 압축 (tar.gz 형식)
7. SHA256 체크섬 파일 생성
8. 빌드 메타데이터 포함 (version, git commit, build date)
9. 빌드 실패 시 명확한 에러 메시지
10. CI 환경에서 실행 가능 (GitHub Actions 호환)

### Technical Notes

- `cross` 도구 사용 (크로스 컴파일)
- 환경 변수로 타겟 플랫폼 지정
- 빌드 캐싱 지원 (CI 성능 최적화)

---

## Story 5.3: Homebrew Formula & Tap

### Story Statement

**As a** macOS/Linux user,
**I want** to install Pane via Homebrew,
**so that** I can easily install and update Pane using a familiar package manager.

### Acceptance Criteria

1. Homebrew tap 리포지토리 생성 (`homebrew-pane`)
2. `pane.rb` formula 파일 작성
3. Formula가 GitHub Releases에서 바이너리 다운로드
4. Formula가 두 바이너리 설치 (`pane`, `claude-tips`)
5. Formula가 스킬 디렉토리 및 manifest 설치
6. `brew install pane/pane/pane` 명령으로 설치 가능
7. `brew test pane` 명령으로 설치 검증
8. Homebrew audit 통과 (`brew audit --strict pane`)
9. macOS 및 Linux에서 테스트 통과
10. Formula에 의존성 명시 (없음 예상)

### Technical Notes

- Tap URL: `https://github.com/pane/homebrew-pane`
- Formula template 참고: Homebrew formula 표준
- 버전 업데이트 자동화 (CI에서 formula 업데이트)

---

## Story 5.4: GitHub Actions CI/CD Pipelines

### Story Statement

**As a** developer,
**I want** automated CI/CD pipelines,
**so that** testing, building, and releasing are automated and consistent.

### Acceptance Criteria

1. `.github/workflows/ci.yml` 파일 생성
2. CI workflow가 모든 PR에서 실행
3. CI가 unit tests, integration tests 실행
4. CI가 `cargo clippy -- -D warnings` 실행
5. CI가 `cargo fmt -- --check` 실행
6. `.github/workflows/release.yml` 파일 생성
7. Release workflow가 version tag 푸시 시 실행 (예: `v0.1.0`)
8. Release workflow가 모든 플랫폼 바이너리 빌드
9. Release workflow가 GitHub Release 자동 생성
10. Release workflow가 Homebrew formula 업데이트
11. `.github/workflows/security-audit.yml` 파일 생성 (optional)
12. Security audit workflow가 주간 실행 및 의존성 취약점 스캔

### Technical Notes

- GitHub Actions matrix build (플랫폼별 병렬 빌드)
- Secrets 사용 (HOMEBREW_TAP_TOKEN for formula updates)
- Release assets 업로드 (바이너리 + SHA256 체크섬)
- Caching strategies for faster builds

---

## Story 5.5: Skill Bundling Mechanism

### Story Statement

**As a** developer,
**I want** a standardized skill bundling mechanism,
**so that** bundled skills are consistently packaged and installed with Pane.

### Acceptance Criteria

1. `skills/` 디렉토리 구조 표준화
2. 각 스킬이 독립적인 디렉토리 구조 (`skills/<skill-name>/`)
3. 스킬 빌드 스크립트가 manifest, 바이너리, 데이터 파일 수집
4. 스킬 번들이 설치 스크립트와 통합
5. 스킬 manifest 검증 (설치 전 validation)
6. 여러 스킬 번들 지원 (향후 확장성)
7. 번들된 스킬 목록 자동 생성
8. 설치 시 스킬 바이너리가 `$PATH`에 추가
9. 설치 시 스킬 manifest가 시스템 경로에 복사
10. 번들링 프로세스 문서화

### Technical Notes

- 스킬 메타데이터 파일 (bundled-skills.json)
- 설치 경로: `/usr/local/share/pane/skills/<skill-name>/`
- 바이너리 경로: `/usr/local/bin/<skill-exec>`
- Data 파일 경로: `/usr/local/share/pane/skills/<skill-name>/data/`

---

## Story 5.6: End-to-End Installation Testing

### Story Statement

**As a** developer,
**I want** comprehensive installation tests,
**so that** I can verify the entire installation process works correctly on all platforms.

### Acceptance Criteria

1. 통합 테스트 스크립트 생성 (`tests/install_test.sh`)
2. 테스트가 clean 환경에서 설치 실행 (Docker 컨테이너)
3. 테스트가 설치 후 `pane --version` 실행 검증
4. 테스트가 설치 후 `claude-tips --version` 실행 검증 (if applicable)
5. 테스트가 `pane` 실행 시 스킬 발견 검증
6. 테스트가 제거 스크립트 실행 검증
7. 테스트가 제거 후 파일 남지 않음 검증
8. macOS (x86_64, ARM64) 및 Linux (x86_64) 테스트
9. CI에서 자동 실행
10. 테스트 결과 리포트 생성

### Technical Notes

- Docker 이미지 사용 (ubuntu:latest, debian:latest)
- macOS CI runner 사용 (GitHub Actions)
- 테스트 격리 (임시 디렉토리)
- Smoke tests for installed binaries

---

## Dependencies

- **Epic 1** (Stories 1.1-1.4) - ✅ Complete (Cargo workspace, config, skill discovery)
- **Epic 2** (Stories 2.1-2.7) - ✅ Complete (TUI launcher)
- **Epic 3** (Stories 3.1-3.3) - ✅ Complete (Inline execution)
- **Epic 4** (Stories 4.1-4.3) - ⚠️ Partial (Claude Tips Viewer built, integration pending Epic 5)

**Note**: Epic 5 must complete before Epic 4 can be fully functional.

## Technical Risks

- **Cross-platform compatibility**: Different behaviors on macOS vs Linux
  - Mitigation: Comprehensive testing on all platforms, use POSIX-compliant scripts
- **Homebrew tap maintenance**: Formula updates may lag behind releases
  - Mitigation: Automate formula updates in CI/CD pipeline
- **Installation permissions**: Users may lack sudo access
  - Mitigation: Support custom installation prefix (`--prefix` flag), document requirements
- **Binary size**: Rust binaries can be large (3-5MB per binary)
  - Mitigation: Use release profile with strip=true, consider UPX compression (optional)

## Out of Scope for Epic 5

- Windows support (future Phase 2)
- APT/RPM package managers (Linux distros, future Phase 2)
- Auto-update mechanism (future Phase 2)
- Signed binaries / notarization (macOS, future enhancement)
- Binary distribution via package registries other than Homebrew/Cargo

---

## Implementation Timeline

**Total Estimated Effort**: 6-9 days (1.5-2 sprints)

| Story | Effort | Dependencies |
|-------|--------|--------------|
| 5.1 Installation Scripts | 1-2 days | None |
| 5.2 Build Scripts | 1 day | None |
| 5.3 Homebrew Formula | 1-2 days | 5.2 (build scripts) |
| 5.4 CI/CD Pipelines | 1-2 days | 5.2, 5.3 |
| 5.5 Skill Bundling | 1 day | 5.1 |
| 5.6 E2E Testing | 1 day | 5.1, 5.2, 5.3, 5.4, 5.5 |

**Recommended Execution Order**:
1. Stories 5.1 & 5.2 (parallel) - Foundation
2. Story 5.5 (parallel with 5.1/5.2) - Bundling mechanism
3. Story 5.3 - Homebrew formula
4. Story 5.4 - CI/CD automation
5. Story 5.6 - Final validation

---

## Success Metrics

**Installation Success Rate**: >95% on supported platforms
**Installation Time**: <2 minutes (including download)
**Binary Size**: <10MB total (both binaries)
**CI/CD Pipeline Speed**: <15 minutes for full release build
**Test Coverage**: 100% installation paths tested
