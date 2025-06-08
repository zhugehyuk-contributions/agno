# Agno 주요 기능 및 사용법

Agno는 강력한 멀티 에이전트 시스템을 구축하기 위한 다양한 기능을 제공합니다. 이 문서는 `cookbook` 디렉토리의 예제 코드를 바탕으로 Agno의 핵심 기능들을 사용하는 방법을 설명합니다.

## 1. 기본 에이전트 생성 및 사용법

Agno의 가장 기본적인 단위는 `Agent` 클래스입니다. 에이전트는 특정 모델 (LLM)을 기반으로 작동하며, 지시사항 (instructions)을 통해 특정 성격이나 역할을 부여받을 수 있습니다.

**예제: `cookbook/getting_started/01_basic_agent.py`**

```python
from textwrap import dedent

from agno.agent import Agent
from agno.models.openai import OpenAIChat

# OpenAIChat 모델을 사용하는 에이전트 생성
agent = Agent(
    model=OpenAIChat(id="gpt-4o"),  # 사용할 LLM 모델 지정
    instructions=dedent("""\
        당신은 열정적인 뉴스 기자이며 스토리텔링에 재능이 있습니다! 🗽
        재치 있는 코미디언과 날카로운 저널리스트의 조합이라고 생각하세요.

        당신의 스타일 가이드:
        - 이모티콘을 사용하여 주목을 끄는 헤드라인으로 시작하세요.
        - 열정과 뉴욕의 태도로 뉴스를 공유하세요.
        - 답변은 간결하지만 재미있게 유지하세요.
        - 적절한 경우 현지 참조 및 뉴욕 속어를 사용하세요.
        - '스튜디오로 다시 연결합니다!' 또는 '빅 애플에서 생방송으로 보도합니다!'와 같은 눈에 띄는 마무리로 끝내세요.

        뉴욕의 에너지를 높게 유지하면서 모든 사실을 확인하는 것을 잊지 마세요!\
    """),  # 에이전트의 역할 및 행동 지침
    markdown=True,  # 응답을 마크다운 형식으로 출력
)

# 에이전트에게 질문하고 응답을 스트리밍으로 출력
agent.print_response(
    "타임스퀘어에서 발생한 주요 뉴스에 대해 알려주세요.", stream=True
)
```

**설명:**

*   `Agent` 클래스를 임포트하고, 사용할 LLM 모델 (예: `OpenAIChat`)을 `model` 파라미터에 전달하여 에이전트를 초기화합니다.
*   `instructions` 파라미터에 문자열로 에이전트의 역할, 성격, 응답 스타일 등을 상세히 기술하여 원하는 행동을 유도할 수 있습니다.
*   `markdown=True`로 설정하면 에이전트의 응답이 마크다운 형식으로 보기 좋게 출력됩니다.
*   `agent.print_response()` 메소드를 사용하여 에이전트에게 작업을 요청하고 응답을 받을 수 있습니다. `stream=True`는 응답을 실시간 스트리밍 형태로 받아볼 때 사용합니다.

## 2. 도구(Tools) 활용 방법

에이전트는 도구를 사용하여 외부 세계와 상호작용하거나 특정 작업을 수행할 수 있습니다. Agno는 다양한 사전 정의된 도구를 제공하며, 사용자 정의 도구를 만들어 통합할 수도 있습니다.

### 2.1. 사전 정의된 도구 사용

**예제: `cookbook/getting_started/02_agent_with_tools.py`**

```python
from textwrap import dedent

from agno.agent import Agent
from agno.models.openai import OpenAIChat
from agno.tools.duckduckgo import DuckDuckGoTools # 웹 검색 도구 임포트

# DuckDuckGo 웹 검색 도구를 사용하는 에이전트 생성
agent = Agent(
    model=OpenAIChat(id="gpt-4o"),
    instructions=dedent("""\
        당신은 열정적인 뉴스 기자이며 스토리텔링에 재능이 있습니다! 🗽
        # ... (이전과 동일한 지시사항) ...
        기억하세요: 항상 웹 검색을 통해 사실을 확인하고 그 뉴욕 특유의 에너지를 유지하세요!\
    """),
    tools=[DuckDuckGoTools()],  # 에이전트에 DuckDuckGoTools 추가
    show_tool_calls=True,  # 도구 호출 과정을 출력
    markdown=True,
)

agent.print_response(
    "타임스퀘어에서 발생한 주요 뉴스에 대해 알려주세요.", stream=True
)
```

**설명:**

*   사용하려는 도구 (예: `DuckDuckGoTools`)를 임포트합니다.
*   `Agent` 초기화 시 `tools` 파라미터에 사용할 도구의 인스턴스를 리스트 형태로 전달합니다.
*   `show_tool_calls=True`로 설정하면 에이전트가 어떤 도구를 사용하고 어떤 결과를 받았는지 과정을 상세히 볼 수 있어 디버깅에 유용합니다.
*   이제 에이전트는 지시사항에 따라 필요시 `DuckDuckGoTools`를 사용하여 웹에서 정보를 검색하고 답변을 생성합니다.

### 2.2. 사용자 정의 도구 생성 및 사용

**예제: `cookbook/getting_started/07_write_your_own_tool.py`**

```python
import json
from textwrap import dedent
import httpx # HTTP 요청을 위한 라이브러리
from agno.agent import Agent
from agno.models.openai import OpenAIChat

# Hacker News 상위 기사를 가져오는 사용자 정의 함수 (도구)
def get_top_hackernews_stories(num_stories: int = 10) -> str:
    """Hacker News에서 상위 기사를 가져오려면 이 함수를 사용하세요.

    Args:
        num_stories (int): 반환할 기사 수. 기본값은 10입니다.

    Returns:
        str: 상위 기사의 JSON 문자열입니다.
    """
    response = httpx.get("https://hacker-news.firebaseio.com/v0/topstories.json")
    story_ids = response.json()
    stories = []
    for story_id in story_ids[:num_stories]:
        story_response = httpx.get(
            f"https://hacker-news.firebaseio.com/v0/item/{story_id}.json"
        )
        story = story_response.json()
        if "text" in story: # 기사 본문은 제외 (요약 정보만)
            story.pop("text", None)
        stories.append(story)
    return json.dumps(stories) # 결과를 JSON 문자열로 반환

# 사용자 정의 도구를 사용하는 에이전트 생성
agent = Agent(
    model=OpenAIChat(id="gpt-4o"),
    instructions=dedent("""\
        당신은 모든 기술에 대한 열정을 가진 기술에 정통한 Hacker News 기자입니다! 🤖
        # ... (관련 지시사항) ...
    """),
    tools=[get_top_hackernews_stories], # 생성한 함수를 도구로 전달
    show_tool_calls=True,
    markdown=True,
)

agent.print_response("Hacker News 상위 5개 기사를 요약해줘.", stream=True)
```

**설명:**

*   일반 파이썬 함수를 정의하여 사용자 정의 도구를 만들 수 있습니다.
*   함수의 독스트링(docstring)은 LLM이 해당 도구의 기능과 사용법(파라미터 등)을 이해하는 데 사용되므로 명확하게 작성하는 것이 중요합니다. 타입 힌트도 LLM의 이해를 돕습니다.
*   만들어진 함수를 `Agent` 초기화 시 `tools` 리스트에 추가해주면 에이전트가 이 함수를 도구로 사용할 수 있게 됩니다.

## 3. 지식(Knowledge) 및 RAG 활용 방법

에이전트는 지식 베이스(Knowledge Base)를 통해 특정 도메인에 대한 전문 지식을 갖출 수 있습니다. 이는 RAG(Retrieval Augmented Generation, 검색 증강 생성) 기술을 활용하여, 외부 문서나 데이터로부터 관련 정보를 검색하고 이를 기반으로 답변을 생성하는 방식입니다.

### 3.1. URL 기반 PDF 지식 베이스 활용

**예제: `cookbook/getting_started/03_agent_with_knowledge.py`**

```python
from textwrap import dedent
from agno.agent import Agent
from agno.embedder.openai import OpenAIEmbedder # 텍스트 임베딩 모델
from agno.knowledge.pdf_url import PDFUrlKnowledgeBase # URL에서 PDF 로드
from agno.models.openai import OpenAIChat
from agno.tools.duckduckgo import DuckDuckGoTools
from agno.vectordb.lancedb import LanceDb, SearchType # 벡터 데이터베이스

# 타이 레시피 PDF를 지식 베이스로 사용하는 요리 전문가 에이전트
agent = Agent(
    model=OpenAIChat(id="gpt-4o"),
    instructions=dedent("""\
        당신은 열정적이고 지식이 풍부한 태국 요리 전문가입니다! 🧑‍🍳
        # ... (요리 관련 지시사항) ...
        기억하세요:
        - 항상 지식 베이스에서 레시피 진위 여부를 확인하세요.
        - 정보가 웹 출처에서 나온 경우 명확히 표시하세요.
    """),
    knowledge=PDFUrlKnowledgeBase( # PDF 기반 지식 베이스 설정
        urls=["https://agno-public.s3.amazonaws.com/recipes/ThaiRecipes.pdf"], # PDF 파일 URL
        vector_db=LanceDb( # LanceDB를 벡터 저장소로 사용
            uri="tmp/lancedb", # 데이터베이스 저장 경로
            table_name="recipe_knowledge", # 테이블 이름
            search_type=SearchType.hybrid, # 하이브리드 검색 사용
            embedder=OpenAIEmbedder(id="text-embedding-3-small"), # OpenAI 임베딩 모델 사용
        ),
    ),
    tools=[DuckDuckGoTools()], # 웹 검색 도구도 함께 사용
    show_tool_calls=True,
    markdown=True,
)

# 지식 베이스 로드 (최초 실행 시 또는 업데이트 시 필요)
# if agent.knowledge is not None:
# agent.knowledge.load()

agent.print_response(
    "코코넛 밀크 수프에 닭고기와 갈랑갈을 어떻게 만드나요?", stream=True
)
```

**설명:**

*   `PDFUrlKnowledgeBase`를 사용하여 특정 URL에 있는 PDF 파일을 지식 베이스로 로드합니다.
*   `vector_db`에는 문서를 벡터로 변환하여 저장하고 검색할 벡터 데이터베이스를 설정합니다. 예제에서는 `LanceDb`를 사용하며, `OpenAIEmbedder`를 통해 텍스트를 벡터로 변환합니다.
*   `agent.knowledge.load()`를 호출하여 지식 베이스를 실제로 로드하고 임베딩을 생성합니다. (이미 생성된 경우 건너뛸 수 있도록 주석 처리 가능)
*   에이전트는 이제 `knowledge`에 설정된 PDF 문서의 내용을 우선적으로 참조하여 답변하며, 필요시 `tools`에 있는 웹 검색 도구를 보조적으로 사용합니다.

### 3.2. 로컬 PDF 파일 지식 베이스 활용

**예제: `cookbook/agent_concepts/knowledge/pdf_kb.py`** (핵심 부분 발췌)

```python
from agno.agent import Agent
from agno.knowledge.pdf import PDFKnowledgeBase, PDFReader # 로컬 PDF 로드
from agno.vectordb.pgvector import PgVector # PostgreSQL 기반 벡터 데이터베이스

# 로컬 'data/pdfs' 디렉토리의 PDF들을 지식 베이스로 사용
knowledge_base = PDFKnowledgeBase(
    path="data/pdfs",  # PDF 파일들이 있는 로컬 디렉토리 경로
    vector_db=PgVector(
        table_name="pdf_documents",
        db_url="postgresql+psycopg://ai:ai@localhost:5532/ai", # PostgreSQL 연결 정보
    ),
    reader=PDFReader(chunk=True), # PDF 리더 설정 (청크 단위로 읽기)
)
# 지식 베이스 로드 (recreate=False는 기존 데이터가 있으면 새로 만들지 않음)
knowledge_base.load(recreate=False)

# 지식 베이스를 사용하는 에이전트 생성
agent = Agent(
    knowledge=knowledge_base,
    search_knowledge=True, # 지식 베이스 검색 활성화
)

agent.print_response("지식 베이스에 있는 내용에 대해 질문해주세요.", markdown=True)
```

**설명:**

*   `PDFKnowledgeBase`를 사용하여 로컬 파일 시스템의 특정 디렉토리(`path`)에 있는 PDF 파일들을 지식 베이스로 구성할 수 있습니다.
*   이 예제에서는 `PgVector`를 벡터 데이터베이스로 사용합니다.
*   `reader=PDFReader(chunk=True)`는 PDF 내용을 적절한 크기의 청크로 나누어 처리하도록 설정합니다.
*   `search_knowledge=True`는 에이전트가 질문에 답변할 때 지식 베이스를 적극적으로 검색하도록 합니다.

## 4. 메모리(Memory) 및 저장소(Storage) 활용 방법

에이전트는 메모리 기능을 통해 이전 대화의 맥락을 기억하고, 저장소 기능을 통해 이러한 상태를 영구적으로 보존할 수 있습니다.

### 4.1. 메모리 및 SQLite 저장소 활용

**예제: `cookbook/agent_concepts/memory/06_agent_with_memory.py`** (핵심 부분 발췌)

```python
from uuid import uuid4
from agno.agent.agent import Agent
from agno.memory.v2.db.sqlite import SqliteMemoryDb # SQLite 기반 메모리 DB
from agno.memory.v2.memory import Memory # 메모리 관리 클래스
from agno.models.openai import OpenAIChat
from agno.storage.sqlite import SqliteStorage # SQLite 기반 세션 저장소

# SQLite를 사용하는 메모리 데이터베이스 설정
memory_db = SqliteMemoryDb(table_name="memory", db_file="tmp/memory.db")
memory = Memory(db=memory_db) # 메모리 객체 생성

# 세션 ID 및 사용자 ID 정의
session_id = str(uuid4())
john_doe_id = "john_doe@example.com"

agent = Agent(
    model=OpenAIChat(id="gpt-4o-mini"),
    memory=memory, # 생성한 메모리 객체를 에이전트에 전달
    storage=SqliteStorage( # SQLite를 세션 저장소로 사용
        table_name="agent_sessions", db_file="tmp/persistent_memory.db"
    ),
    enable_user_memories=True, # 사용자별 메모리 생성 활성화
)

# 사용자 정보와 함께 첫 번째 메시지 전송
agent.print_response(
    "제 이름은 John Doe이고 주말에는 산에서 하이킹하는 것을 좋아합니다.",
    stream=True,
    user_id=john_doe_id, # 사용자 ID 전달
    session_id=session_id, # 세션 ID 전달
)

# 이전 대화 내용을 기억하고 답변하는지 확인
agent.print_response(
    "제 취미는 무엇인가요?", stream=True, user_id=john_doe_id, session_id=session_id
)

# John Doe의 메모리 내용 확인
memories = agent.get_user_memories(user_id=john_doe_id)
print("John Doe's memories:")
# ... (메모리 출력) ...
```

**설명:**

*   `agno.memory.v2.memory.Memory` 클래스와 데이터베이스 백엔드(예: `SqliteMemoryDb`)를 사용하여 에이전트의 메모리를 관리합니다.
*   `agno.storage.sqlite.SqliteStorage` (또는 다른 스토리지 옵션)를 사용하여 에이전트 세션 정보를 영구적으로 저장합니다.
*   `Agent` 초기화 시 `memory`와 `storage` 객체를 전달합니다.
*   `enable_user_memories=True`로 설정하면 사용자별로 개인화된 정보를 기억하고 활용할 수 있습니다.
*   `agent.print_response()` 호출 시 `user_id`와 `session_id`를 전달하여 특정 사용자의 특정 대화 세션에 대한 메모리를 관리할 수 있습니다.
*   `agent.get_user_memories()`를 통해 특정 사용자에 대해 에이전트가 기억하고 있는 내용을 확인할 수 있습니다.

**참고:** `cookbook/getting_started/04_agent_with_storage.py` 예제는 `read_chat_history=True` 또는 `add_history_to_messages=True` 옵션을 사용하여 대화 기록을 LLM에 전달하는 방식을 보여줍니다. 이는 `Memory` 객체를 직접 사용하는 것보다 간단한 내장 메모리 활용 방식입니다.

## 5. 에이전트 팀(Agent Teams) 구성 및 활용 방법

복잡한 작업을 수행하기 위해 여러 에이전트가 협력하는 팀을 구성할 수 있습니다. 각 에이전트는 특정 역할을 담당하며, 팀 조정자(coordinator)가 이들의 작업을 조율합니다.

**예제: `cookbook/getting_started/05_agent_team.py`** (핵심 부분 발췌)

```python
from textwrap import dedent
from agno.agent import Agent
from agno.models.openai import OpenAIChat
from agno.team.team import Team # 팀 클래스 임포트
from agno.tools.duckduckgo import DuckDuckGoTools
from agno.tools.yfinance import YFinanceTools

# 1. 웹 검색 담당 에이전트 정의
web_agent = Agent(
    name="Web Agent",
    role="웹에서 정보 검색",
    model=OpenAIChat(id="gpt-4o"),
    tools=[DuckDuckGoTools()],
    instructions=dedent("""\
        당신은 숙련된 웹 연구원이자 뉴스 분석가입니다! 🔍
        # ... (웹 검색 관련 지시사항) ...
    """),
    show_tool_calls=True,
    markdown=True,
)

# 2. 금융 데이터 분석 담당 에이전트 정의
finance_agent = Agent(
    name="Finance Agent",
    role="금융 데이터 가져오기",
    model=OpenAIChat(id="gpt-4o"),
    tools=[
        YFinanceTools(stock_price=True, analyst_recommendations=True, company_info=True)
    ],
    instructions=dedent("""\
        당신은 시장 데이터에 대한 전문 지식을 갖춘 숙련된 금융 분석가입니다! 📊
        # ... (금융 분석 관련 지시사항) ...
    """),
    show_tool_calls=True,
    markdown=True,
)

# 3. 에이전트 팀 구성
agent_team = Team(
    members=[web_agent, finance_agent], # 팀 멤버로 위에서 정의한 에이전트들 추가
    model=OpenAIChat(id="gpt-4o"), # 팀 조정자(coordinator)가 사용할 모델
    mode="coordinate", # 팀 운영 모드 (coordinate: 조정자가 멤버 조율)
    success_criteria=dedent("""\
        명확한 섹션과 데이터 기반 통찰력을 갖춘 포괄적인 금융 뉴스 보고서.
    """), # 팀 작업의 성공 기준
    instructions=dedent("""\
        당신은 유명 금융 뉴스 데스크의 편집장입니다! 📰
        # ... (편집장 역할 및 최종 보고서 작성 관련 지시사항) ...
    """), # 팀 조정자의 지시사항
    add_datetime_to_instructions=True,
    show_tool_calls=True,
    markdown=True,
    enable_agentic_context=True, # 에이전트 간 컨텍스트 공유 활성화
    show_members_responses=False, # 멤버 에이전트의 개별 응답은 숨김 (조정자 최종 응답만 표시)
)

# 팀에게 작업 요청
agent_team.print_response(
    message="NVDA에 대한 분석가 추천을 요약하고 최신 뉴스를 공유해주세요.",
    stream=True,
)
```

**설명:**

*   각각 특정 역할과 도구를 가진 개별 `Agent`들을 먼저 정의합니다.
*   `Team` 클래스를 사용하여 이 에이전트들을 `members`로 하는 팀을 구성합니다.
*   `model` 파라미터는 팀 전체의 작업을 조율하는 조정자(coordinator) LLM을 지정합니다.
*   `mode="coordinate"`는 조정자 모델이 멤버 에이전트들의 작업을 지시하고 결과를 종합하는 방식입니다. (다른 모드도 존재)
*   `success_criteria`는 팀 작업의 완료 조건을 명시합니다.
*   `instructions`는 팀 조정자에게 주어지는 지시사항으로, 멤버들의 결과를 어떻게 종합하여 최종 결과물을 만들지에 대한 내용입니다.
*   `enable_agentic_context=True`는 팀 내 에이전트들이 서로의 작업 내용이나 컨텍스트를 공유할 수 있도록 합니다.

## 6. 워크플로우(Workflows) 구성 및 활용 방법

워크플로우는 여러 에이전트 또는 팀의 작업을 순차적 또는 병렬적으로 연결하여 자동화된 프로세스를 정의하고 실행하는 기능입니다. 복잡한 다단계 작업을 체계적으로 관리할 수 있습니다.

**예제: `cookbook/getting_started/09_research_workflow.py`** (핵심 부분 발췌, 클래스 정의)

```python
from textwrap import dedent
from typing import Dict, Iterator, Optional
from agno.agent import Agent
from agno.models.openai import OpenAIChat
from agno.storage.sqlite import SqliteStorage
from agno.tools.duckduckgo import DuckDuckGoTools
from agno.tools.newspaper4k import Newspaper4kTools # 웹 기사 내용 추출 도구
from agno.workflow import RunEvent, RunResponse, Workflow # 워크플로우 클래스
from pydantic import BaseModel, Field

# 데이터 모델 정의 (Pydantic 사용)
class Article(BaseModel): # 검색된 기사 정보
    title: str = Field(..., description="기사 제목.")
    url: str = Field(..., description="기사 링크.")
    summary: Optional[str] = Field(..., description="가능한 경우 기사 요약.")

class SearchResults(BaseModel): # 검색 결과
    articles: list[Article]

class ScrapedArticle(BaseModel): # 스크랩된 기사 내용
    # ... (Article과 유사, content 필드 추가)
    content: Optional[str] = Field(..., description="마크다운 형식의 기사 내용.")

# 워크플로우 클래스 정의
class ResearchReportGenerator(Workflow):
    description: str = "학문적 엄밀함과 매력적인 스토리텔링을 결합한 포괄적인 연구 보고서를 생성합니다."

    # 1단계: 웹 검색 에이전트
    web_searcher: Agent = Agent(
        model=OpenAIChat(id="gpt-4o-mini"),
        tools=[DuckDuckGoTools()],
        instructions="# ... (권위 있는 학술 자료 검색 지침) ...",
        response_model=SearchResults, # 이 에이전트의 출력 형식 지정
    )

    # 2단계: 기사 스크랩 에이전트
    article_scraper: Agent = Agent(
        model=OpenAIChat(id="gpt-4o-mini"),
        tools=[Newspaper4kTools()],
        instructions="# ... (학술 내용 정확히 추출 및 마크다운 변환 지침) ...",
        response_model=ScrapedArticle, # 이 에이전트의 출력 형식 지정
    )

    # 3단계: 보고서 작성 에이전트
    writer: Agent = Agent(
        model=OpenAIChat(id="gpt-4o"),
        instructions="# ... (학문적 보고서 작성 및 인용 지침) ...",
        expected_output="# ... (보고서의 기대 형식 상세 기술) ...",
        markdown=True,
    )

    # 워크플로우 실행 로직
    def run(
        self,
        topic: str,
        use_search_cache: bool = True,
        use_scrape_cache: bool = True,
        use_cached_report: bool = True,
    ) -> Iterator[RunResponse]:
        # ... (캐시 확인 로직) ...

        # 1. 웹 검색 실행
        search_results: Optional[SearchResults] = self.web_searcher.run(topic).content
        if not search_results or not search_results.articles:
            # ... (결과 없음 처리) ...
            return

        # 2. 기사 스크랩 실행 (검색된 각 기사에 대해)
        scraped_articles: Dict[str, ScrapedArticle] = {}
        for article_info in search_results.articles:
            # ... (캐시 확인) ...
            scraped_content: Optional[ScrapedArticle] = self.article_scraper.run(article_info.url).content
            if scraped_content:
                scraped_articles[article_info.url] = scraped_content

        # ... (스크랩 결과 캐시 저장) ...

        # 3. 보고서 작성 실행
        writer_input = {"topic": topic, "articles": [a.model_dump() for a in scraped_articles.values()]}
        yield from self.writer.run(json.dumps(writer_input, indent=4), stream=True)

        # ... (최종 보고서 캐시 저장) ...

# 워크플로우 실행 부분
if __name__ == "__main__":
    # ... (사용자로부터 토픽 입력 받기) ...
    research_workflow = ResearchReportGenerator(
        session_id=f"research-report-on-{topic.replace(' ', '-')}", # 세션 ID 설정
        storage=SqliteStorage( # 워크플로우 상태 저장을 위한 스토리지
            table_name="research_report_workflow",
            db_file="tmp/workflows.db",
        ),
    )
    report_stream = research_workflow.run(topic=topic)
    # ... (결과 출력) ...
```

**설명:**

*   `Workflow` 클래스를 상속받아 사용자 정의 워크플로우를 만듭니다.
*   워크플로우 내부에 여러 단계의 `Agent`들을 정의합니다. 각 에이전트는 특정 작업을 수행하고, `response_model`을 통해 출력 형식을 명시할 수 있습니다.
*   `run` 메소드에 워크플로우의 전체 실행 로직을 구현합니다. 이 로직에는 각 에이전트를 순차적으로 또는 조건부로 호출하고, 그 결과를 다음 에이전트의 입력으로 전달하는 과정이 포함됩니다.
*   예제에서는 웹 검색 -> 기사 스크랩 -> 보고서 작성의 3단계로 연구 보고서를 생성합니다.
*   워크플로우는 세션 관리(`session_id`)와 상태 저장(`storage`)을 지원하여, 중간 결과 캐싱 등을 통해 효율성을 높일 수 있습니다. `get_cached_report`, `add_report_to_cache` 등의 헬퍼 함수를 만들어 캐싱 로직을 구현합니다.
*   `cookbook/workflows/blog_post_generator.py`도 유사한 구조로 블로그 게시물 생성 워크플로우를 보여줍니다.

## 7. 멀티모달 에이전트 활용 방법

Agno 에이전트는 텍스트뿐만 아니라 이미지, 오디오 등 다양한 형식의 데이터를 처리할 수 있는 멀티모달 기능을 지원합니다.

### 7.1. 이미지 입력 처리

**예제: `cookbook/getting_started/10_image_agent.py`**

```python
from textwrap import dedent
from agno.agent import Agent
from agno.media import Image # 이미지 처리를 위한 Image 클래스
from agno.models.openai import OpenAIChat
from agno.tools.duckduckgo import DuckDuckGoTools

# 이미지 분석 및 관련 뉴스 보고 에이전트
agent = Agent(
    model=OpenAIChat(id="gpt-4o"), # 멀티모달 지원 모델 사용
    description="이미지를 생생하게 전달하는 세계적 수준의 비주얼 저널리스트이자 문화 특파원입니다!",
    instructions=dedent("""\
        이미지 분석 및 뉴스 보고 시 다음 원칙을 따르세요.
        1. 시각적 분석: ...
        2. 뉴스 통합: ...
        3. 스토리텔링 스타일: ...
    """),
    tools=[DuckDuckGoTools()],
    show_tool_calls=True,
    markdown=True,
)

# 이미지 URL을 사용하여 에이전트에게 질문
agent.print_response(
    "이 이미지에 대해 설명하고 최신 관련 뉴스를 공유해주세요.",
    images=[ # images 파라미터에 Image 객체 리스트 전달
        Image(
            url="https://upload.wikimedia.org/wikipedia/commons/0/0c/GoldenGateBridge-001.jpg"
        )
    ],
    stream=True,
)
```

**설명:**

*   멀티모달을 지원하는 모델(예: `gpt-4o`)을 선택합니다.
*   `agno.media.Image` 클래스를 사용하여 이미지를 표현합니다. 이미지는 URL, 로컬 파일 경로, 또는 바이트 데이터로 제공될 수 있습니다.
*   `agent.print_response()` (또는 `agent.run()`) 호출 시 `images` 파라미터에 `Image` 객체의 리스트를 전달하여 에이전트가 이미지를 입력으로 받도록 합니다.
*   에이전트는 이미지의 내용을 이해하고, 이를 바탕으로 지시사항에 따른 작업을 수행합니다 (예: 이미지 묘사, 관련 정보 검색).

### 7.2. 오디오 입력 및 출력 처리

**예제: `cookbook/agent_concepts/multimodal/audio_input_output.py`** (핵심 부분 발췌)

```python
import requests
from agno.agent import Agent
from agno.media import Audio # 오디오 처리를 위한 Audio 클래스
from agno.models.openai import OpenAIChat
from agno.utils.audio import write_audio_to_file # 오디오 파일 저장 유틸리티

# 오디오 파일 URL에서 데이터 가져오기
url = "https://openaiassets.blob.core.windows.net/$web/API/docs/audio/alloy.wav"
response = requests.get(url)
wav_data = response.content # 오디오 데이터를 바이트로 저장

# 오디오 입출력을 지원하는 모델로 에이전트 설정
agent = Agent(
    model=OpenAIChat(
        id="gpt-4o-audio-preview", # 오디오 특화 모델 ID (예시)
        modalities=["text", "audio"], # 지원하는 모달리티 명시
        audio={"voice": "sage", "format": "wav"}, # 출력 오디오 설정 (목소리, 포맷)
    ),
    markdown=True,
)

# 오디오 데이터를 입력으로 전달
agent.run(
    "이 녹음 파일에 무엇이 있나요?",
    audio=[Audio(content=wav_data, format="wav")] # audio 파라미터에 Audio 객체 리스트 전달
)

# 에이전트의 응답에 오디오가 포함되어 있다면 파일로 저장
if agent.run_response.response_audio is not None:
    write_audio_to_file(
        audio=agent.run_response.response_audio.content, filename="tmp/result.wav"
    )
```

**설명:**

*   오디오 처리를 지원하는 모델 (예: `gpt-4o-audio-preview`)을 선택하고, `modalities`에 "audio"를 포함시킵니다.
*   `model` 설정 시 `audio` 딕셔너리를 통해 출력 오디오의 목소리(`voice`)나 포맷(`format`) 등을 지정할 수 있습니다.
*   `agno.media.Audio` 클래스를 사용하여 오디오 데이터를 표현합니다. 내용은 바이트 데이터, 포맷은 'wav', 'mp3' 등으로 지정합니다.
*   `agent.run()` 호출 시 `audio` 파라미터에 `Audio` 객체 리스트를 전달합니다.
*   에이전트가 오디오를 출력하면 `agent.run_response.response_audio`에서 해당 오디오 데이터에 접근할 수 있으며, `write_audio_to_file`과 같은 유틸리티를 사용하여 파일로 저장할 수 있습니다.

## 8. FastAPI를 이용한 에이전트 배포 방법

개발된 Agno 에이전트는 `FastAPIApp`을 사용하여 쉽게 웹 서비스로 배포할 수 있습니다.

**예제: `cookbook/apps/fastapi/basic.py`**

```python
from agno.agent import Agent
from agno.app.fastapi.app import FastAPIApp # FastAPI 앱 래퍼
from agno.models.openai import OpenAIChat

# 배포할 에이전트 정의
basic_agent = Agent(
    name="Basic Agent",
    model=OpenAIChat(id="gpt-4o"),
    add_history_to_messages=True, # 대화 기록 사용
    num_history_responses=3,
    add_datetime_to_instructions=True,
    markdown=True,
)

# FastAPIApp을 사용하여 에이전트를 FastAPI 앱으로 래핑
fastapi_app = FastAPIApp(
    agent=basic_agent, # 배포할 에이전트 객체
    name="Basic Agent", # 앱 이름
    app_id="basic_agent", # 앱 ID
    description="질문에 답변하고 작업을 도와줄 수 있는 기본 에이전트입니다.", # 앱 설명
)

# FastAPI 앱 인스턴스 가져오기
app = fastapi_app.get_app()

# ASGI 서버 (예: Uvicorn)를 통해 앱 실행
if __name__ == "__main__":
    # fastapi_app.serve()는 내부적으로 uvicorn.run()을 호출
    fastapi_app.serve(app="basic:app", port=8001, reload=True)
```

**설명:**

*   배포하고자 하는 `Agent` (또는 `Team`, `Workflow`)를 먼저 정의합니다.
*   `FastAPIApp` 클래스를 임포트하고, 배포할 에이전트(또는 팀/워크플로우) 객체와 앱 관련 메타데이터(이름, ID, 설명 등)를 전달하여 인스턴스를 생성합니다.
*   `fastapi_app.get_app()`을 호출하여 실제 FastAPI 애플리케이션 객체를 얻습니다.
*   `fastapi_app.serve()` 메소드를 사용하거나 직접 Uvicorn과 같은 ASGI 서버를 사용하여 애플리케이션을 실행할 수 있습니다. 이렇게 배포된 서비스는 HTTP 요청을 통해 에이전트와 상호작용할 수 있는 API 엔드포인트를 제공합니다.

## 9. Agno Playground 사용법

Agno Playground는 개발한 에이전트, 팀, 워크플로우를 웹 UI를 통해 쉽게 테스트하고 상호작용할 수 있는 도구입니다. `README.md`에서는 Playground를 통해 에이전트와 채팅하고 모니터링할 수 있다고 언급합니다.

**예제: `cookbook/apps/playground/basic.py`** (코드 기반 설명)

```python
from agno.agent import Agent
from agno.memory.agent import AgentMemory
from agno.memory.db.postgres import PgMemoryDb # 메모리 저장용 PostgreSQL
from agno.models.openai import OpenAIChat
from agno.playground import Playground, serve_playground_app # Playground 클래스
from agno.storage.postgres import PostgresStorage # 세션 저장용 PostgreSQL

# Playground에서 테스트할 에이전트 정의
basic_agent = Agent(
    name="Basic Agent",
    model=OpenAIChat(id="gpt-4o"),
    memory=AgentMemory( # PostgreSQL 기반 메모리 설정
        db=PgMemoryDb(
            table_name="agent_memory",
            db_url="postgresql+psycopg://ai:ai@localhost:5532/ai",
        ),
        create_user_memories=True,
        update_user_memories_after_run=True,
        create_session_summary=True,
        update_session_summary_after_run=True,
    ),
    storage=PostgresStorage( # PostgreSQL 기반 세션 저장소 설정
        table_name="agent_sessions", db_url="postgresql+psycopg://ai:ai@localhost:5532/ai", auto_upgrade_schema=True
    ),
    add_history_to_messages=True,
    # ... (기타 설정) ...
    markdown=True,
)

# Playground 인스턴스 생성
playground = Playground(
    agents=[basic_agent], # Playground에 포함할 에이전트 (또는 팀/워크플로우) 리스트
    name="Basic Agent Playground", # Playground 이름
    description="기본 에이전트를 위한 플레이그라운드입니다.", # 설명
    app_id="basic-agent-playground", # Playground ID
)

# Playground FastAPI 앱 가져오기
app = playground.get_app()

# Playground 앱 실행
if __name__ == "__main__":
    # playground.serve()는 내부적으로 uvicorn.run()을 호출하며 Playground UI를 제공
    playground.serve(app="basic:app", reload=True) # basic:app은 현재 파일(basic.py)의 app 변수를 의미
```

**설명:**

*   Playground에서 테스트하고 싶은 `Agent`, `Team`, 또는 `Workflow` 객체들을 정의합니다.
*   `Playground` 클래스를 임포트하고, 테스트할 객체들의 리스트와 Playground 메타데이터(이름, 설명, ID)를 전달하여 인스턴스를 생성합니다.
*   `playground.get_app()`으로 FastAPI 앱을 얻고, `playground.serve()`를 호출하여 Playground 웹 애플리케이션을 실행합니다.
*   실행 후 웹 브라우저에서 해당 주소(기본값: `http://localhost:8000`)로 접속하면, 정의된 에이전트들과 상호작용할 수 있는 UI가 나타납니다. 이 UI를 통해 메시지를 보내고, 응답을 확인하며, 세션 관리, 도구 사용 내역, 멀티모달 입력 등을 테스트할 수 있습니다.
*   `README.md`의 "Get Started" 섹션에서는 Playground를 통해 에이전트와 채팅하고 `agno.com`에서 모니터링할 수 있다고 안내하고 있어, Playground가 에이전트 개발 및 테스트의 중요한 부분임을 알 수 있습니다.

이것으로 Agno의 주요 기능과 사용법에 대한 설명을 마칩니다. 각 예제 코드를 직접 실행해보면서 Agno의 강력한 기능들을 경험해보시기 바랍니다.
