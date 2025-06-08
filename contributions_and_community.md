# Agno 기여 및 커뮤니티

Agno는 오픈소스 프로젝트이며, 커뮤니티의 모든 기여를 환영합니다. 이 문서는 Agno 프로젝트에 기여하는 방법과 다양한 커뮤니티 지원 채널에 대해 안내합니다.

## 1. Agno 프로젝트에 기여하는 방법

Agno에 기여하는 가장 일반적인 방법은 GitHub의 'fork and pull request' 워크플로우를 따르는 것입니다.

### 1.1. 일반적인 기여 절차

1.  **리포지토리 포크 (Fork)**: Agno GitHub 리포지토리 우측 상단의 'Fork' 버튼을 클릭하여 자신의 계정으로 리포지토리를 복제합니다.
2.  **새 브랜치 생성 (New Branch)**: 로컬 머신에 포크한 리포지토리를 클론하고, 새로운 기능 추가나 버그 수정을 위한 브랜치를 생성합니다. (예: `git checkout -b my-new-feature`)
3.  **기능 추가/개선 (Add/Improve)**: 해당 브랜치에서 코드를 수정하거나 새로운 기능을 추가합니다.
4.  **PR 가이드라인 준수**: 아래의 Pull Request 가이드라인에 따라 커밋 메시지와 PR 내용을 작성합니다.
5.  **PR 전송 (Send Pull Request)**: 작업이 완료되면 자신의 포크된 리포지토리에서 원본 Agno 리포지토리로 Pull Request(PR)를 보냅니다.

### 1.2. Pull Request (PR) 가이드라인

명확하고 정리된 프로젝트 히스토리를 유지하기 위해 PR 제출 시 다음 가이드라인을 준수해주세요.

*   **PR 제목 형식**: PR 제목은 반드시 대괄호로 묶인 타입 태그로 시작해야 하며, 한 칸 띄고 간결한 제목을 작성합니다.
    *   예시: `[feat] 사용자 인증 기능 추가`
    *   유효한 타입 태그: `[feat]`, `[fix]`, `[docs]`, `[test]`, `[refactor]`, `[build]`, `[ci]`, `[chore]`, `[perf]`, `[style]`, `[revert]`
*   **이슈 연결**: PR 본문 설명에는 해당 PR이 해결하는 이슈를 키워드(예: `fixes #<이슈_번호>`, `closes #<이슈_번호>`, `resolves #<이슈_번호>`)를 사용하여 연결하는 것이 좋습니다.
    *   예시: `이 PR은 새로운 로그인 흐름을 구현하여 #42 이슈를 해결합니다.`

이러한 가이드라인은 PR Lint 워크플로우를 통해 자동으로 검사됩니다.

### 1.3. 개발 환경 설정

로컬에서 Agno 개발을 시작하기 위한 환경 설정 방법은 다음과 같습니다.

1.  Agno 리포지토리를 클론합니다.
2.  `uv` 설치 여부를 확인합니다 (`uv --version`). 설치되어 있지 않다면 `pip install uv`로 설치합니다.
3.  가상 환경을 생성하고 필요한 패키지를 설치합니다.
    *   Unix 계열: `./scripts/dev_setup.sh` 스크립트를 실행합니다.
    *   Windows: `.\scripts\dev_setup.bat` 스크립트를 실행합니다.
    *   이 스크립트들은 현재 디렉토리에 `.venv` 가상 환경을 만들고, 필요한 패키지를 설치하며, `agno` 패키지를 수정 가능 모드(editable mode)로 설치합니다.
4.  생성된 가상 환경을 활성화합니다.
    *   Unix 계열: `source .venv/bin/activate`
    *   Windows: `.venv\Scripts\activate`

이후에는 `uv pip install`을 사용하여 필요한 추가 패키지를 설치합니다.

### 1.4. 코드 포맷팅 및 유효성 검사

코드를 제출하기 전에 프로젝트의 품질 표준을 충족하는지 확인하기 위해 다음 스크립트를 실행해야 합니다.

*   Unix 계열:
    *   `./scripts/format.sh` (Ruff를 사용한 코드 포맷팅)
    *   `./scripts/validate.sh` (MyPy를 사용한 정적 타입 검사)
*   Windows:
    *   `.\scripts\format.bat`
    *   `.\scripts\validate.bat`

### 1.5. 로컬 테스트

PR을 제출하기 전에 모든 테스트가 로컬 환경에서 통과하는지 확인해야 합니다.

1.  위의 개발 환경 설정을 완료합니다.
2.  전체 테스트 스위트를 실행합니다: `./scripts/test.sh`
3.  특정 테스트 파일이나 케이스를 실행하려면: `pytest ./libs/agno/tests/unit/utils/test_string.py` (또는 테스트하려는 다른 파일 경로)

새로운 기능을 추가하는 경우, 적절한 테스트 코드를 함께 포함해야 합니다.

### 1.6. 새로운 기능 추가 가이드

`CONTRIBUTING.md` 파일에는 새로운 벡터 데이터베이스, 모델 제공자, 또는 도구를 Agno에 추가하는 방법에 대한 상세한 단계별 가이드가 포함되어 있습니다. 새로운 구성 요소를 추가하고자 한다면 해당 문서를 반드시 참조하시기 바랍니다.

## 2. 문서 및 커뮤니티 지원

Agno 사용 및 개발에 도움이 될 수 있는 다양한 자료와 커뮤니티 채널이 마련되어 있습니다.

*   **공식 문서 (Official Documentation)**:
    *   Agno의 기능, API, 사용법 등에 대한 상세한 정보를 제공합니다.
    *   링크: [https://docs.agno.com](https://docs.agno.com)
*   **Cookbook (예제 코드)**:
    *   Agno의 다양한 기능을 실제 코드로 보여주는 예제 모음입니다. `getting_started`부터 특정 `agent_concepts`, `models`, `tools` 사용법까지 다양한 예제를 포함하고 있습니다.
    *   GitHub 링크: [https://github.com/agno-agi/agno/tree/main/cookbook](https://github.com/agno-agi/agno/tree/main/cookbook)
*   **커뮤니티 포럼 (Community Forum)**:
    *   질문을 하고, 아이디어를 공유하며, 다른 Agno 사용자와 토론할 수 있는 공간입니다. (Discourse 기반)
    *   링크: [https://community.agno.com/](https://community.agno.com/)
*   **Discord**:
    *   실시간으로 다른 개발자들과 소통하고 도움을 주고받을 수 있는 Discord 채널입니다.
    *   초대 링크: [https://discord.gg/4MtYHHrgA8](https://discord.gg/4MtYHHrgA8)

궁금한 점이 있거나 도움이 필요하면 언제든지 Discord나 커뮤니티 포럼을 통해 문의해주세요.

## 3. 행동 강령 (Code of Conduct)

Agno 프로젝트는 모든 참여자가 서로 존중하고 협력적인 환경을 만들기 위해 노력합니다. 프로젝트의 루트 디렉토리에는 `CODE_OF_CONDUCT.md` 파일이 있으며, 모든 기여자 및 커뮤니티 참여자는 이 행동 강령을 숙지하고 준수해야 합니다.

## 4. 라이선스

Agno 프로젝트는 `MPL-2.0 (Mozilla Public License 2.0)` 라이선스 하에 배포됩니다. 라이선스 전문은 프로젝트 루트 디렉토리의 `LICENSE` 파일에서 확인할 수 있습니다.
