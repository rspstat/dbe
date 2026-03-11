# MCP와 커스텀 DB 엔진에 기반한 커스텀 RDBMS

> Rust로 직접 구현한 커스텀 DB 엔진과 MCP(Model Context Protocol)를 결합하여,
> 자연어만으로 누구나 데이터베이스를 조작할 수 있는 관계형 데이터베이스 관리 시스템
>
> 졸업작품 | 2025

---


## 프로젝트 개요

본 프로젝트는 Rust 언어로 관계형 데이터베이스 엔진을 바닥부터 직접 설계 및 구현하고,
MCP(Model Context Protocol)를 결합하여 자연어 기반 질의 자동 생성을 지원하는 커스텀 RDBMS입니다.

B+Tree 인덱스, 버퍼 풀, WAL(Write-Ahead Logging), 트랜잭션(COMMIT/ROLLBACK)을 포함한
실제 데이터베이스 엔진의 핵심 구성 요소를 직접 구현하며,
SQL을 모르는 초급 사용자도 자연어 입력만으로 데이터베이스를 조작할 수 있는 것을 목표로 합니다.


---


## 프로젝트 배경 및 목적

기존 RDBMS(MySQL, PostgreSQL 등)는 강력하지만 SQL 문법을 알아야만 사용할 수 있어
비전공자나 초급 사용자에게 높은 진입 장벽이 존재합니다.

본 프로젝트는 두 가지 문제를 동시에 해결합니다.

첫째, 데이터베이스 엔진의 내부 구조(B+Tree, Buffer Pool, WAL 등)를 Rust로 직접 구현하여
DB 엔진의 동작 원리를 깊이 있게 탐구합니다.

둘째, MCP를 통해 자연어를 SQL로 자동 변환함으로써
SQL 지식 없이도 누구나 데이터베이스를 사용할 수 있는 인터페이스를 제공합니다.


---


## 시스템 아키텍처

```
┌──────────────────────────────────────────────┐
│               사용자 인터페이스                 │
│         SQL 직접 입력 / 자연어 입력             │
└──────────────┬───────────────────────────────┘
               │
      ┌────────▼────────┐
      │    MCP 레이어    │
      │  자연어 → SQL   │
      │  Claude API     │
      │  스키마 자동주입  │
      └────────┬────────┘
               │  SQL 쿼리
      ┌────────▼────────┐
      │    SQL 파서      │
      │  Lexer + AST    │
      └────────┬────────┘
               │  AST
      ┌────────▼────────┐
      │   쿼리 실행 엔진  │
      │  Planner        │
      │  Executor       │
      └────────┬────────┘
               │
      ┌────────▼────────┐
      │  스토리지 엔진   │
      │  B+Tree 인덱스  │
      │  Buffer Pool    │
      │  WAL            │
      └────────┬────────┘
               │
      ┌────────▼────────┐
      │    디스크 I/O    │
      │   .rdb 파일     │
      └─────────────────┘
```


---


## 목표 및 구현 범위


### 핵심 구현 목표

| 분류 | 기능 | 상태 |
|------|------|------|
| **Storage** | 4KB 페이지 기반 슬롯 구조 | 예정 |
| **Storage** | B+Tree 인덱스 (삽입/삭제/분할/병합) | 예정 |
| **Storage** | Buffer Pool (LRU 캐시) | 예정 |
| **Storage** | 바이너리 디스크 I/O (.rdb) | 예정 |
| **DDL** | `CREATE TABLE` / `DROP TABLE` / `ALTER TABLE` | 완료 |
| **DML** | `SELECT` / `INSERT` / `UPDATE` / `DELETE` | 완료 |
| **DML** | `WHERE` 조건절 / `JOIN` | 완료 |
| **DML** | `GROUP BY` / `ORDER BY` / `LIMIT` | 예정 |
| **집계** | `COUNT` / `SUM` / `AVG` / `MIN` / `MAX` | 예정 |
| **TCL** | `COMMIT` / `ROLLBACK` | 예정 |
| **TCL** | WAL (Write-Ahead Logging) | 예정 |
| **DBMS** | TCP 서버 (클라이언트/서버 구조) | 예정 |
| **DBMS** | 다중 클라이언트 동시 접속 | 예정 |
| **MCP** | 자연어 → SQL 자동 생성 | 예정 |
| **MCP** | 실행 결과 자연어 설명 | 예정 |
| **MCP** | 스키마 컨텍스트 자동 주입 | 예정 |


### 선택 구현 목표

| 분류 | 기능 | 상태 |
|------|------|------|
| **동시성** | Lock Manager (S-Lock / X-Lock) | 선택 |
| **동시성** | 데드락 감지 및 해제 | 선택 |
| **최적화** | 비용 기반 Query Planner | 선택 |
| **SQL** | 서브쿼리 / DISTINCT / LIKE | 선택 |


---


## 주요 모듈 구조

```
rustdb-mcp/
├── src/
│   ├── main.rs                  # 진입점 + REPL
│   ├── parser/
│   │   ├── lexer.rs             # 토크나이저
│   │   ├── parser.rs            # SQL → AST 파서
│   │   └── ast.rs               # AST 노드 정의
│   ├── engine/
│   │   ├── planner.rs           # 실행 계획 생성
│   │   ├── executor.rs          # AST 실행
│   │   └── join.rs              # JOIN 연산
│   ├── storage/
│   │   ├── page.rs              # 페이지 구조 (4KB)
│   │   ├── buffer_pool.rs       # 버퍼 풀 + LRU 캐시
│   │   ├── btree.rs             # B+Tree 인덱스
│   │   └── disk.rs              # 바이너리 디스크 I/O
│   ├── catalog/
│   │   └── schema.rs            # 테이블 스키마 관리
│   ├── transaction/
│   │   ├── wal.rs               # Write-Ahead Log
│   │   └── txn_manager.rs       # COMMIT / ROLLBACK
│   ├── server/
│   │   ├── tcp_server.rs        # TCP 서버
│   │   └── protocol.rs          # 커스텀 통신 프로토콜
│   └── mcp/
│       └── client.rs            # MCP 연동 + 자연어 쿼리
├── data/                        # .rdb 데이터 파일
├── Cargo.toml
└── README.md
```


---


## 기술 스택

| 분류 | 기술 |
|------|------|
| **언어** | Rust (2021 edition) |
| **패키지 매니저** | Cargo |
| **MCP 연동** | Claude API (MCP 프로토콜) |
| **인덱스 구조** | B+Tree |
| **저장 형식** | 커스텀 바이너리 (.rdb) |
| **직렬화** | serde / serde_json |
| **에디터** | VS Code + rust-analyzer |


---


## 실행 예시


### SQL 직접 입력 모드 (고급 사용자)

```sql
rustdb> CREATE TABLE users (id INT, name TEXT, age INT)
Table 'users' created.

rustdb> INSERT INTO users VALUES (1, Alice, 25)
1 row inserted.

rustdb> INSERT INTO users VALUES (2, Bob, 30)
1 row inserted.

rustdb> SELECT * FROM users WHERE age > 24
+----+-------+-----+
| id | name  | age |
+----+-------+-----+
| 1  | Alice | 25  |
| 2  | Bob   | 30  |
+----+-------+-----+
2 row(s) returned.

rustdb> SELECT * FROM users JOIN orders ON id = user_id
+----+-------+---------+---------+
| id | name  | user_id | item    |
+----+-------+---------+---------+
| 1  | Alice | 1       | Laptop  |
| 2  | Bob   | 2       | Monitor |
+----+-------+---------+---------+
2 row(s) returned.
```


### MCP 자연어 모드 (초급 사용자)

```
rustdb> \ai users 테이블에서 25살 이상 사용자 보여줘

생성된 SQL: SELECT * FROM users WHERE age >= 25;
실행하시겠어요? (y/n): y

+----+-------+-----+
| id | name  | age |
+----+-------+-----+
| 1  | Alice | 25  |
| 2  | Bob   | 30  |
+----+-------+-----+

"25살 이상인 사용자는 Alice(25살), Bob(30살) 총 2명입니다."
```


---


## 설치 및 실행

```bash
# 1. 저장소 클론
git clone https://github.com/yourname/rustdb-mcp.git
cd rustdb-mcp

# 2. 빌드
cargo build --release

# 3. SQL 직접 입력 모드 실행
cargo run

# 4. MCP 자연어 모드 실행
cargo run -- --mcp
```


---


## 개발 로드맵

```
Phase 1  │ SQL 파서 + 기본 실행 엔진  │ Lexer, Parser, AST, Executor     완료
Phase 2  │ JSON 기반 영속성          │ 재시작 후 데이터 유지              완료
Phase 3  │ B+Tree 인덱스             │ 삽입, 삭제, 분할, 병합, 범위 탐색  진행 예정
Phase 4  │ 페이지 + Buffer Pool      │ 4KB 페이지, LRU 캐시              진행 예정
Phase 5  │ 바이너리 디스크 I/O       │ .rdb 파일 포맷 직접 설계           진행 예정
Phase 6  │ WAL + 트랜잭션            │ COMMIT, ROLLBACK, 충돌 복구       진행 예정
Phase 7  │ TCP 서버                  │ 클라이언트/서버 구조, 다중 접속    진행 예정
Phase 8  │ MCP 연동                  │ 자연어 → SQL, 결과 자연어 설명    진행 예정
Phase 9  │ 벤치마크                  │ SQLite 대비 성능 비교              진행 예정
Phase 10 │ 문서화 + 발표 준비        │ 보고서, 발표 자료                  진행 예정
```


---


## 개발자

| 이름 | 역할 |
|------|------|
| (이름) | 설계 / 전체 구현 |


---


## 라이선스

MIT License © 2025
