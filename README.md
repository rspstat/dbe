# MCP와 커스텀 DB 엔진에 기반한 커스텀 RDBMS

Rust로 직접 구현한 관계형 데이터베이스 엔진과 MCP를 결합하여,
자연어만으로 누구나 데이터베이스를 조작할 수 있는 RDBMS입니다.

졸업작품 | 2025


---


## 핵심 기능

| 분류 | 내용 |
|------|------|
| DB 엔진 | B+Tree, Buffer Pool, WAL, 트랜잭션 직접 구현 |
| SQL 지원 | DDL / DML / JOIN / 트랜잭션 |
| MCP | 자연어 입력 → SQL 자동 생성 → 실행 |
| DBMS | TCP 서버, 다중 클라이언트 동시 접속 |
| 언어 | Rust (2021 edition) |


---


## 실행 예시

SQL 직접 입력 (고급 사용자)
```sql
rustdb> CREATE TABLE users (id INT, name TEXT, age INT)
rustdb> INSERT INTO users VALUES (1, Alice, 25)
rustdb> SELECT * FROM users WHERE age > 24
+----+-------+-----+
| id | name  | age |
+----+-------+-----+
| 1  | Alice | 25  |
+----+-------+-----+
```

자연어 입력 (초급 사용자)
```
rustdb> \ai users 테이블에서 25살 이상 사용자 보여줘
생성된 SQL: SELECT * FROM users WHERE age >= 25;
"25살 이상인 사용자는 Alice(25살) 총 1명입니다."
```
