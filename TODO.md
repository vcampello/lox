# Next steps

## Phase 1 — Fix known bugs

- [ ] Fix `or()` right-recursion in `src/frontend/parser.rs:104` — `self.or()` → `self.and()`
- [ ] Remove `dbg!(&token)` from `src/frontend/parser.rs:400`
- [ ] Fix misleading comment in `src/frontend/scanner.rs:182` ("Multi-line comment" → multi-line string handling)
- [ ] Fix `FunctionStmt.body` type in `src/ast/stmt.rs` — `Box<Stmt>` → `BlockStmt`

## Phase 2 — Complete scanner

- [ ] Add block comments (`/* ... */`) to `handle_comment` in scanner
- [ ] Add string escape sequences (`\"`, `\n`, `\t`, `\\`) to `handle_string` in scanner

## Phase 3 — Port book tests + test harness

- [ ] Design test harness that:
    - Parses `// expect: <value>` lines from `.lox` files
    - Parses `// expect runtime error: <message>` lines
    - Parses `// [line N] Error at ...` lines
    - Strips expectation comments before running
- [ ] Add output capture to interpreter — inject a `Write`-based writer replacing `println!`
- [ ] Add thin wrapper in `Lox` that exposes `run_with_output(source: &str) -> Result<String>`
- [ ] Clone test files from `github.com/munificent/craftinginterpreters` into `tests/lox_suite/`
- [ ] Write integration tests that auto-discover `.lox` files under `tests/lox_suite/`
- [ ] Write project-specific tests for extra features (continue/break, logical operators in README)

## Phase 4 — Resolver (Chapter 11)

- [ ] Build `src/backend/resolver.rs` — second pass over the AST
- [ ] Resolve variable binding depths into a side table (`HashMap<Span, usize>`) on the interpreter
- [ ] Add `Env::get_at(depth, name)` and `Env::assign_at(depth, name, value)` for depth-based lookup
- [ ] Modify variable read/assign in interpreter to use resolved depth when available
- [ ] Validate: returning from top-level, using `this` outside class, self-referencing initializers
- [ ] Add resolver errors to `LoxError`
- [ ] `closure.lox` passes

## Phase 5 — Classes (Chapters 12–13)

- [ ] Class declarations — parser (`ClassStmt`), AST node, interpreter
- [ ] Class instances — `Value::Instance` with fields map
- [ ] Property access — `GetExpr` and `SetExpr` in parser + interpreter
- [ ] Methods — store in class, bind `this` on invocation
- [ ] Constructors (`init`) — special return handling (always returns instance)
- [ ] Inheritance — `class Foo < Bar`, `super` calls
- [ ] `this` resolution in resolver

## Phase 6 — Polish

- [ ] Native functions — `Value::NativeFunction(fn(&[Value]) -> Value)`, add `clock()`
- [ ] Remove remaining `TODO`/`FIXME` markers
- [ ] Consolidate error reporting in `src/main.rs` (two identical blocks)
- [ ] Stretch: ternary operator (`? :`) — tokens exist, add parser + interpreter support
- [ ] Update `README.md` to reflect completed features
