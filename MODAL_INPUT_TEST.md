# Modal Input System - Testing Guide

## Story 6.1 구현 완료

Vim 스타일의 Normal/Insert 모드 시스템이 구현되었습니다.

## 빠른 시작

```bash
# 앱 실행
./target/release/pane
```

## 테스트 시나리오

### 1. Normal 모드 (초기 상태)

앱을 실행하면 **Normal 모드**로 시작합니다.

**확인 사항**:
- Footer에 "j/k Move | / Search | Enter Run | Esc Quit" 표시됨
- `j` 키를 누르면 → 선택이 아래로 이동
- `k` 키를 누르면 → 선택이 위로 이동

### 2. Insert 모드 진입

Normal 모드에서 `/` 키를 누르세요.

**확인 사항**:
- Footer가 **"-- INSERT --  Type to search | Esc Normal mode"**로 변경됨
- Secondary 색상 (기본: Yellow)으로 "-- INSERT --" 표시

### 3. 검색어 입력

Insert 모드에서 "jkjk"를 입력해보세요.

**확인 사항**:
- `j`, `k` 키가 검색어로 입력됨 (네비게이션 동작 안 함)
- 검색 바에 "jkjk" 표시됨
- 검색 결과가 필터링됨

### 4. Normal 모드로 복귀

Insert 모드에서 `Esc` 키를 누르세요.

**확인 사항**:
- Footer가 다시 "j/k Move | / Search | ..." 로 변경됨
- 검색어 "jkjk"는 그대로 유지됨
- 검색 결과 필터링도 유지됨

### 5. 검색어 삭제

Normal 모드에서 (검색어가 있을 때) `Esc` 키를 누르세요.

**확인 사항**:
- 검색어가 삭제됨
- 모든 스킬이 다시 표시됨
- 앱은 계속 실행 중 (종료 안 됨)

### 6. 앱 종료

Normal 모드에서 (검색어가 없을 때) `Esc` 키를 누르세요.

**확인 사항**:
- 앱이 정상 종료됨

## 전체 워크플로우 예시

```
1. 앱 시작 → Normal 모드
2. j, k로 네비게이션
3. / 눌러서 Insert 모드 진입
4. "claude" 입력 (검색)
5. Esc로 Normal 모드 복귀 (검색어 유지)
6. j, k로 필터링된 결과에서 네비게이션
7. Esc로 검색어 삭제
8. Esc로 앱 종료
```

## 키 바인딩 요약

### Normal 모드
| 키 | 동작 |
|----|------|
| `j` | 아래로 이동 |
| `k` | 위로 이동 |
| `/` | Insert 모드 진입 (검색 시작) |
| `↑`/`↓` | 위/아래 이동 (화살표 키) |
| `Enter` | 선택한 스킬 실행 |
| `Tab` | View 모드 전환 (All/Favorites/Recent) |
| `Esc` | 검색어 삭제 또는 앱 종료 |

### Insert 모드
| 키 | 동작 |
|----|------|
| 모든 문자 | 검색어에 입력 (j, k 포함) |
| `Backspace` | 검색어 마지막 문자 삭제 |
| `Esc` | Normal 모드로 복귀 |
| `↑`/`↓` | 위/아래 이동 (검색 중에도 가능) |
| `Enter` | 선택한 스킬 실행 |
| `Tab` | View 모드 전환 |

## 구현 세부사항

**코드 변경**:
- `src/state.rs`: InputMode enum 추가
- `src/input.rs`: 모드별 키 매핑
- `src/app.rs`: 모드 전환 로직
- `src/ui/components/footer.rs`: 모드 표시

**테스트**:
- ✅ 31개 새로운 테스트 추가
- ✅ 168개 전체 테스트 통과
- ✅ 릴리즈 빌드 성공

## 문제 발생 시

문제가 발생하면 다음을 확인하세요:

1. **빌드가 최신인가?**
   ```bash
   cargo build --release
   ```

2. **테스트가 통과하는가?**
   ```bash
   cargo test --lib
   ```

3. **버전 확인**
   ```bash
   ./target/release/pane --version
   # 출력: pane 0.1.0
   ```

## 알려진 제한사항

- `f` 키 (Toggle Favorite): Placeholder만 구현됨 (향후 구현 예정)
- `?` 키 (Show Help): Story 6.3에서 구현 예정

## 다음 단계

Story 6.1 완료 후 추천하는 다음 스토리:
- **Story 6.4**: 시각적 개선 (아이콘, 하이라이트, 색상)
- **Story 6.3**: 도움말 시스템
- **Story 6.2**: 마우스 지원
