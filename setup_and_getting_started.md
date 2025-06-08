# Agno 개발 환경 설정 및 시작하기

이 문서는 Agno 프레임워크를 사용하여 개발을 시작하기 위한 환경 설정 방법과 기본적인 시작 가이드를 제공합니다.

## 1. Python 가상 환경 설정

Agno 프로젝트를 위한 격리된 Python 환경을 만드는 것이 좋습니다. 이를 위해 `uv` 또는 Python에 내장된 `venv` 모듈을 사용할 수 있습니다.

### 1.1. `uv` 사용 (권장)

`uv`는 빠른 Python 패키지 설치 및 관리를 위한 도구입니다. `uv`가 설치되어 있다면 다음 명령어를 사용하세요.

```bash
# Python 3.12 (또는 호환되는 버전)으로 가상 환경 생성
uv venv --python 3.12 .venv

# 가상 환경 활성화 (Linux/macOS)
source .venv/bin/activate

# 가상 환경 활성화 (Windows - PowerShell)
# .venv\Scripts\Activate.ps1
# 가상 환경 활성화 (Windows - CMD)
# .venv\Scripts\activate.bat
```

### 1.2. `python3 -m venv` 사용

Python에 내장된 `venv` 모듈을 사용할 수도 있습니다.

```bash
# 가상 환경 생성
python3 -m venv .venv

# 가상 환경 활성화 (Linux/macOS)
source .venv/bin/activate

# 가상 환경 활성화 (Windows - PowerShell)
# .venv\Scripts\Activate.ps1
# 가상 환경 활성화 (Windows - CMD)
# .venv\Scripts\activate.bat
```

가상 환경이 활성화되면 프롬프트 앞에 `(.venv)`와 같이 가상 환경 이름이 표시됩니다.

## 2. Agno 설치

가상 환경이 활성화된 상태에서 다음 명령어를 사용하여 최신 버전의 Agno 라이브러리를 설치합니다.

```bash
pip install -U agno
```

또는 `uv`를 사용한다면:

```bash
uv pip install -U agno
```

`-U` 옵션은 이미 설치된 경우 최신 버전으로 업그레이드합니다.

## 3. 필수 의존성 설치

Agno는 핵심 기능 외에도 다양한 모델 제공자, 도구, 데이터베이스 등과 연동하기 위해 추가적인 라이브러리가 필요할 수 있습니다. `README.md`의 예제 또는 `cookbook`의 특정 예제를 실행하기 전에 필요한 의존성을 설치해야 합니다.

예를 들어, OpenAI 모델, Anthropic 모델, YFinance 도구, DuckDuckGo 검색 도구를 사용하려면 다음 라이브러리들을 설치합니다.

```bash
# pip 사용 시
pip install openai anthropic yfinance duckduckgo-search

# uv 사용 시
uv pip install openai anthropic yfinance duckduckgo-search
```

각 `cookbook` 예제 파일 상단이나 `README.md`의 예제 설명 부분에서 필요한 `pip install` 명령어를 확인할 수 있습니다. 일반적으로 다음과 같은 형태로 안내됩니다.

```
Run `pip install openai duckduckgo-search agno` to install dependencies.
```

또는

```
uv pip install agno anthropic yfinance
```

## 4. API 키 설정

Agno를 통해 다양한 LLM 서비스(OpenAI, Anthropic, Google AI 등)나 외부 API(YFinance, Google Maps 등)를 사용하려면 해당 서비스의 API 키를 발급받아 환경 변수로 설정해야 합니다.

자주 사용되는 API 키 환경 변수 이름은 다음과 같습니다:

*   `OPENAI_API_KEY`
*   `ANTHROPIC_API_KEY`
*   `GOOGLE_API_KEY` (또는 `GEMINI_API_KEY`)
*   기타 서비스별 API 키 (예: `YFINANCE_API_KEY`, `DUCKDUCKGO_API_KEY` 등은 일반적으로 필요하지 않으나, 특정 도구가 유료 플랜을 요구할 경우 필요할 수 있습니다.)

### Linux / macOS 에서 환경 변수 설정

터미널에서 다음 명령어를 사용하여 현재 세션에 대한 환경 변수를 설정할 수 있습니다.

```bash
export OPENAI_API_KEY="sk-your_openai_api_key_here"
export ANTHROPIC_API_KEY="sk-ant-api03-your_anthropic_api_key_here"
export GOOGLE_API_KEY="your_google_api_key_here"
```

매번 터미널을 열 때마다 설정하지 않으려면, 쉘 설정 파일(예: `~/.bashrc`, `~/.zshrc`)에 위 `export` 명령어들을 추가하고 파일을 다시 로드하거나 새 터미널 세션을 시작하세요.

### Windows 에서 환경 변수 설정

**CMD (명령 프롬프트):**

```cmd
set OPENAI_API_KEY=sk-your_openai_api_key_here
set ANTHROPIC_API_KEY=sk-ant-api03-your_anthropic_api_key_here
```

**PowerShell:**

```powershell
$Env:OPENAI_API_KEY="sk-your_openai_api_key_here"
$Env:ANTHROPIC_API_KEY="sk-ant-api03-your_anthropic_api_key_here"
```

영구적으로 환경 변수를 설정하려면 '시스템 환경 변수 편집' 메뉴를 사용하세요.

1.  '내 PC' 또는 '내 컴퓨터'에서 마우스 오른쪽 버튼을 클릭하고 '속성'을 선택합니다.
2.  '고급 시스템 설정'을 클릭합니다.
3.  '환경 변수...' 버튼을 클릭합니다.
4.  '새로 만들기...'를 클릭하여 변수 이름과 값을 입력합니다.

## 5. Cookbook 예제 실행 방법

Agno의 다양한 기능과 사용법을 익히는 가장 좋은 방법은 `cookbook` 디렉토리에 있는 예제들을 실행해보는 것입니다.

1.  Agno 리포지토리를 클론하거나 다운로드하여 `cookbook` 디렉토리로 이동합니다.
2.  실행하려는 예제 파일에 필요한 의존성을 먼저 설치합니다 (위 "필수 의존성 설치" 섹션 참조).
3.  필요한 API 키가 환경 변수로 설정되어 있는지 확인합니다.
4.  터미널에서 다음 명령어를 사용하여 Python 스크립트를 실행합니다.

```bash
python cookbook/getting_started/01_basic_agent.py
```

또는 `cookbook` 디렉토리 내부에서 상대 경로로 실행할 수도 있습니다.

```bash
# cookbook 디렉토리로 이동했다고 가정
cd cookbook
python getting_started/01_basic_agent.py
```

## 6. Agno Telemetry 설정 (선택 사항)

Agno는 가장 많이 사용되는 모델 제공자를 파악하여 업데이트 우선순위를 정하기 위해 에이전트가 사용한 모델 정보를 로깅합니다. 이 원격 측정(Telemetry) 기능은 기본적으로 활성화되어 있습니다.

이 기능을 비활성화하고 싶다면, 다음과 같이 `AGNO_TELEMETRY` 환경 변수를 `false`로 설정하세요.

**Linux / macOS:**

```bash
export AGNO_TELEMETRY=false
```

**Windows (CMD):**

```cmd
set AGNO_TELEMETRY=false
```

**Windows (PowerShell):**

```powershell
$Env:AGNO_TELEMETRY="false"
```

이 설정을 통해 Agno 사용에 대한 어떠한 정보도 외부로 전송되지 않도록 할 수 있습니다.

이제 Agno 개발을 시작할 준비가 되었습니다! `cookbook`의 다양한 예제들을 탐색하고 자신만의 강력한 AI 에이전트를 구축해보세요.
