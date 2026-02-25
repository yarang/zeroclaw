# ZeroClaw Development Team

**팀 이름:** zeroclaw-dev
**생성일:** 2026-02-25
**목적:** ZeroClaw Rust agent development for code implementation and modification

## 팀 구성

**모델:** 모든 에이전트는 `glm-5` 사용

| 멤버 | 역할 | 타입 | 상태 |
|------|------|------|------|
| team-lead | 전체 조정 및 아키텍처 결정 | general-purpose | 활성 |
| researcher | 코드베이스 탐색 및 분석 | Explore | 활성 |
| docs-writer | 문서화 | general-purpose | 활성 |
| developer-security | 보안 관련 개발 | general-purpose | 활성 |

## 담당 영역

### developer-core
- 핵심 기능 구현
- `src/agent/` 모듈 담당
- 아키텍처 설계 및 구현

### developer-tools
- Tool trait 구현 (`src/tools/`)
- DelegateTool 개발
- Tool 레지스트리 관리

### developer-security
- `src/security/` 모듈 담당
- `src/gateway/` 보안 기능
- 접근 제어 및 정책 구현

### researcher
- 코드베이스 구조 파악
- 기술 조사 및 분석
- 문서 검색 및 정보 수집

### tester
- 테스트 코드 작성
- `cargo test` 실행
- 코드 검증

### docs-writer
- README 및 docs/ 파일 업데이트
- 다국어 문서 동기화 (en, zh-CN, ja, ru, fr, vi)
- API 문서화

## 완료된 작업

### 2026-02-25

1. **파일 기반 멀티 에이전트 활성화** (Task #1)
   - AgentRegistry 구현 (`src/agent/registry.rs`)
   - AgentWatcher 구현 (`src/agent/watcher.rs`)
   - DelegateTool-Registry 연동
   - 빌드 성공

2. **컴파일 오류 수정** (Task #7, #8)
   - Default trait 구현 추가
   - 포맷 문자열 수정
   - 중복 함수 제거

## 진행 중인 작업

- Task #10: 테스트 실패 10개 수정

## 대기 중인 작업

- Task #3: 베트남어 문서 정리 및 i18n 구조 개선
- Task #4: 새로운 문서 리뷰 및 통합
- Task #5: CLI 에이전트 테스트 구현

## 설정 파일 위치

```
~/.claude/teams/zeroclaw-dev/
├── config.json    # 팀 설정
└── tasks/         # 태스크 저장소
```

## 통신 방식

- 메시지: `SendMessage` tool 사용
- 브로드캐스트: 긴급 공지 시에만 사용
- 태스크: `TaskCreate`, `TaskUpdate` 사용
