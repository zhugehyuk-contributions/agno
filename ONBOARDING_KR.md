# Agno 프로젝트 온보딩 가이드

이 문서는 새로운 개발 팀장이 하루 만에 프로젝트 구조와 개발 흐름을 이해할 수 있도록 작성되었습니다. 모든 내용은 저장소의 README와 예제 코드를 기반으로 정리하였습니다.

## 프로젝트 개요
Agno는 다중 에이전트 시스템(Multi-Agent Systems)을 쉽고 빠르게 구축하기 위한 풀스택 프레임워크입니다. 퍼포먼스를 중시하며, 다양한 모델 제공자와 툴을 아우르는 경량 라이브러리를 목표로 합니다.

주요 특징은 다음과 같습니다.
- 23개 이상의 모델 제공자와 통합된 **모델 독립적 인터페이스**
- **빠른 실행 속도**와 **낮은 메모리 사용량**
- 체계적인 **추론 기능(Reasoning)** 지원
- 텍스트·이미지·오디오·비디오를 모두 다룰 수 있는 **멀티모달 에이전트**
- 지식 기반과 저장소를 통한 **에이전트 메모리** 및 **RAG** 구현
- **팀(Team)** 단위의 협업과 워크플로우 구성 지원
- FastAPI 라우팅과 실시간 모니터링 기능 제공

## 저장소 구조
```
.
├─ cookbook/      # 예제와 데모 코드 모음
├─ libs/
│  ├─ agno/       # 핵심 라이브러리
│  └─ infra/      # Docker, AWS 등 인프라 관련 모듈
├─ scripts/       # 개발 및 테스트 스크립트
└─ README.md      # 프로젝트 전반 소개
```

### 1. `cookbook/`
- `getting_started/` 폴더에는 기본 에이전트 생성부터 이미지·오디오 처리, 에이전트 팀 구성까지 단계별 예제가 포함되어 있습니다.
- `agent_concepts/` 폴더에서는 비동기 처리, 메모리, 툴 사용법 등 주요 개념을 심화 학습할 수 있습니다.
- 기타 `tools`, `workflows`, `observability` 등 실무에 필요한 다양한 예제가 준비되어 있습니다.

### 2. `libs/agno/`
- 에이전트(`agent`), 모델 래퍼(`models`), 툴킷(`tools`), 벡터 DB(`vectordb`), 워크플로우(`workflow`) 등 라이브러리의 핵심 기능이 이곳에 위치합니다.
- `memory`, `knowledge`, `storage` 모듈을 통해 대화 이력 관리와 장기 기억을 지원합니다.
- `team` 모듈은 여러 에이전트를 조합해 협업을 수행할 수 있게 도와줍니다.

### 3. `libs/infra/`
- `agno_docker`와 `agno_aws` 서브패키지로 구성되며, 각각 Docker 및 AWS 관련 리소스를 다룹니다.

### 4. `scripts/`
- `dev_setup.sh` : 개발 환경 구성을 자동화합니다. 가상 환경 생성과 필수 패키지 설치, `agno` 패키지를 편집 모드로 설치합니다.
- `format.sh`, `validate.sh` : 코드 포맷팅과 정적 분석(Mypy)을 실행합니다.
- `test.sh` : 단위 테스트와 통합 테스트를 실행합니다.
- 그 외 성능 측정이나 예제 실행에 필요한 스크립트가 포함되어 있습니다.

## 개발 환경 설정
1. 저장소 클론 후, **`scripts/dev_setup.sh`** 실행으로 가상 환경 및 의존성 설치를 완료합니다.
2. 가상 환경 활성화: `source .venv/bin/activate`
3. 필요한 API 키를 환경 변수로 설정합니다. 예) `export OPENAI_API_KEY=...`
4. 코드 수정 전 `scripts/format.sh`, `scripts/validate.sh`를 통해 포맷팅과 정적 분석을 수행합니다.
5. 변경 사항을 테스트하려면 `scripts/test.sh`를 실행합니다.

## 예제 실행
- `cookbook/getting_started/01_basic_agent.py`부터 차례대로 실행하며 프레임워크의 기본 사용법을 학습할 수 있습니다.
- 예제 실행 시 필요한 의존성은 각 README에 명시되어 있으므로, 누락된 패키지가 있다면 `pip install`로 추가 설치합니다.

## 테스트
- `libs/agno/tests` 디렉터리에는 단위 테스트와 통합 테스트가 분리되어 있습니다.
- `scripts/test.sh`를 실행하면 모든 테스트가 수행됩니다.

## 추가 자료
- 공식 문서: [https://docs.agno.com](https://docs.agno.com)
- 커뮤니티 포럼: [https://community.agno.com](https://community.agno.com)
- 디스코드: [https://discord.gg/4MtYHHrgA8](https://discord.gg/4MtYHHrgA8)

Agno는 멀티 에이전트 시스템 구축에 필요한 다양한 기능을 포괄적으로 제공하므로, 위 폴더 구조와 스크립트 사용법을 숙지하면 빠른 온보딩이 가능합니다.
