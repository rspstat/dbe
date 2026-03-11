# 🦀 RustDB-MCP

> **Rust로 구현한 커스텀 데이터베이스 엔진 + MCP 기반 자동 쿼리 생성 시스템**  
> 졸업작품 | 2024

---

## 📌 프로젝트 개요

RustDB-MCP는 Rust로 직접 구현한 경량 관계형 데이터베이스 엔진과,  
AI 기반 MCP(Model Context Protocol)를 결합하여 **자연어 → SQL 쿼리 자동 생성**까지 지원하는 시스템입니다.

단순한 CRUD를 넘어 DDL/DML 전반과 JOIN 연산을 지원하며,  
사용자가 SQL을 몰라도 MCP를 통해 데이터베이스를 조작할 수 있는 것을 목표로 합니다.

---

## 🎯 목표 및 범위

### ✅ 핵심 구현 목표

| 분류 | 기능 | 상태 |
|------|------|------|
| **DDL** | `CREATE TABLE` | 🔲 예정 |
| **DDL** | `DROP TABLE` | 🔲 예정 |
| **DDL** | `ALTER TABLE` | 🔲 예정 |
| **DML** | `SELECT` (단일 테이블) | 🔲 예정 |
| **DML** | `SELECT` + `JOIN` | 🔲 예정 |
| **DML** | `INSERT` | 🔲 예정 |
| **DML** | `UPDATE` | 🔲 예정 |
| **DML** | `DELETE` | 🔲 예정 |
| **DML** | `WHERE` 조건절 | 🔲 예정 |
| **MCP** | 자연어 → SQL 자동 생성 | 🔲 예정 |

### 🔶 선택 구현 목표 (시간 여유 시)

| 분류 | 기능 | 상태 |
|------|------|------|
| **TCL** | `COMMIT` | 🔲 선택 |
| **TCL** | `ROLLBACK` | 🔲 선택 |
| **TCL** | WAL (Write-Ahead Logging) | 🔲 선택 |

---

## 🏗️ 시스템 아키텍처

```
┌─────────────────────────────────────────────────┐
│                  사용자 인터페이스                  │
│            CLI REPL / MCP 자연어 입력              │
└────────────────────┬────────────────────────────┘
                     │
          ┌──────────▼──────────┐
          │     MCP 레이어       │
          │  자연어 → SQL 변환   │
          └──────────┬──────────┘
                     │  SQL 쿼리
          ┌──────────▼──────────┐
          │      SQL 파서        │
          │  Lexer + AST 생성   │
          └──────────┬──────────┘
                     │  AST
          ┌──────────▼──────────┐
          │    쿼리 실행 엔진    │
          │  Planner + Executor │
          └──────────┬──────────┘
                     │
          ┌──────────▼──────────┐
          │    스토리지 엔진     │
          │  B-Tree + 페이지    │
          └──────────┬──────────┘
                     │
          ┌──────────▼──────────┐
          │      디스크 I/O      │
          │    .rdb 파일 저장    │
          └─────────────────────┘
```

---

## 🧩 주요 모듈 구조

```
rustdb-mcp/
├── src/
│   ├── main.rs              # 진입점 + REPL
│   ├── parser/
│   │   ├── lexer.rs         # 토크나이저
│   │   ├── parser.rs        # SQL → AST 파서
│   │   └── ast.rs           # AST 노드 정의
│   ├── engine/
│   │   ├── planner.rs       # 실행 계획 생성
│   │   ├── executor.rs      # AST 실행
│   │   └── join.rs          # JOIN 연산
│   ├── storage/
│   │   ├── page.rs          # 페이지 구조 (4KB)
│   │   ├── buffer_pool.rs   # 버퍼 풀 + LRU 캐시
│   │   ├── btree.rs         # B-Tree 인덱스
│   │   └── disk.rs          # 디스크 I/O
│   ├── catalog/
│   │   └── schema.rs        # 테이블 스키마 관리
│   ├── transaction/         # (선택) 트랜잭션
│   │   ├── wal.rs           # Write-Ahead Log
│   │   └── txn_manager.rs   # COMMIT / ROLLBACK
│   └── mcp/
│       └── client.rs        # MCP 연동 + 쿼리 생성
├── data/                    # .rdb 데이터 파일 저장
├── Cargo.toml
└── README.md
```

---

## ⚙️ 기술 스택

| 분류 | 기술 |
|------|------|
| **언어** | Rust (2021 edition) |
| **패키지 매니저** | Cargo |
| **MCP 연동** | Claude API (MCP 프로토콜) |
| **저장 형식** | 커스텀 바이너리 (.rdb) |
| **인덱스 구조** | B-Tree |
| **에디터** | VS Code + rust-analyzer |

---

## 🖥️ 실행 예시

### CLI REPL 모드
```sql
🦀 RustDB v0.1
rustdb> CREATE TABLE users (id INT, name TEXT, age INT);
✅ Table 'users' created.

rustdb> INSERT INTO users VALUES (1, 'Alice', 25);
✅ 1 row inserted.

rustdb> INSERT INTO users VALUES (2, 'Bob', 30);
✅ 1 row inserted.

rustdb> SELECT * FROM users WHERE age > 24;
+----+-------+-----+
| id | name  | age |
+----+-------+-----+
|  1 | Alice |  25 |
|  2 | Bob   |  30 |
+----+-------+-----+
2 rows returned.

rustdb> SELECT u.name, o.item FROM users u JOIN orders o ON u.id = o.user_id;
+-------+---------+
| name  | item    |
+-------+---------+
| Alice | Laptop  |
| Bob   | Monitor |
+-------+---------+
```

### MCP 자동 쿼리 생성 모드
```
💬 자연어 입력: "25살 이상인 사용자 목록 보여줘"
🤖 생성된 SQL: SELECT * FROM users WHERE age >= 25;
✅ 실행 결과:
+----+-------+-----+
| id | name  | age |
+----+-------+-----+
|  1 | Alice |  25 |
|  2 | Bob   |  30 |
+----+-------+-----+
```

---

## 🚀 설치 및 실행

```bash
# 1. 저장소 클론
git clone https://github.com/yourname/rustdb-mcp.git
cd rustdb-mcp

# 2. 빌드
cargo build --release

# 3. 실행 (REPL 모드)
cargo run

# 4. 실행 (MCP 모드)
cargo run -- --mcp
```

---

## 📅 개발 로드맵

```
Phase 1 │ 스토리지 엔진     │ 페이지, 버퍼 풀, B-Tree 구현
Phase 2 │ SQL 파서          │ Lexer, Parser, AST 구현
Phase 3 │ DDL 실행          │ CREATE / DROP / ALTER
Phase 4 │ DML 실행          │ SELECT / INSERT / UPDATE / DELETE
Phase 5 │ JOIN 구현         │ INNER JOIN, LEFT JOIN
Phase 6 │ MCP 연동          │ 자연어 → SQL 자동 생성
Phase 7 │ 트랜잭션 (선택)   │ WAL, COMMIT, ROLLBACK
```

---

## 👤 개발자

| 이름 | 역할 |
|------|------|
| (이름) | 설계 / 전체 구현 |

---

## 📄 라이선스

MIT License © 2024
