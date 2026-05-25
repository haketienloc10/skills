---
name: project-notes
description: Bắt buộc dùng pnotes trước khi implement, trước khi khám phá source code diện rộng, và sau khi có implementation output. Dùng pnotes để recall decisions, invariants, risks, tests, missing tests, traps, dead ends, và continuity notes của project mà không phải scan toàn bộ repository.
---

# SKILL: project-notes

## 1. Nhận diện skill

- Tên skill: `project-notes`
- Mục đích: dùng project-local notes để recall context liên quan trước khi sửa code, và lưu lại continuity có giá trị sau khi sửa code.
- Tool: `./bin/pnotes` — Rust CLI binary được cài vào repo đích từ skill này.
- Storage: `.project-notes/notes/*.md` — Markdown file có YAML frontmatter.
- Source of truth: Markdown notes. CLI chỉ là lớp recall/brief/helper.

Skill này tồn tại để tránh agent:

- tự khám phá lại trap đã biết;
- lặp lại approach đã bị loại;
- bỏ qua decisions hoặc invariants đã chốt;
- bỏ sót known test gaps;
- scan source files không liên quan theo mặc định;
- implement xong nhưng không để lại continuity hữu ích cho lần sau.

## 2. Bootstrap repo đích

Nếu repo đang làm việc chưa có `./bin/pnotes`, nhưng user muốn dùng `project-notes` cho repo đó, cài bằng bundled script của skill:

    sh <skill-dir>/scripts/install.sh <repo-dir>

Trong đó:

- `<skill-dir>` là thư mục chứa skill `project-notes`;
- `<repo-dir>` mặc định là current directory nếu bỏ trống;
- script build Rust CLI từ `pnotes-cli/`, copy binary vào `<repo-dir>/bin/pnotes`, và tạo `<repo-dir>/.project-notes/notes/`.

Sau khi cài, verify:

    ./bin/pnotes guide

Không tự cài vào repo đích nếu user chỉ hỏi Q&A hoặc repo đó không nên nhận thêm file local tool.

## 3. Trigger bắt buộc

### 3.1 Trước khi implement

Phải dùng skill này trước khi sửa source code cho feature, bugfix, refactor, migration, hoặc behavior change.

Ưu tiên dùng lệnh này nếu CLI hỗ trợ:

    ./bin/pnotes brief --area <path> --limit 3

Fallback:

    ./bin/pnotes recall --area <path> --limit 3

Nếu biết task/tag thì dùng thêm filter:

    ./bin/pnotes brief --area <path> --tag <tag> --limit 3
    ./bin/pnotes recall --task <slug>
    ./bin/pnotes recall --tag <tag>

Trước khi sửa code, phải đọc brief trả về hoặc các note liên quan được trả về.

Không scan toàn bộ `.project-notes/notes/` theo mặc định.

### 3.2 Trước khi khám phá source code diện rộng

Phải dùng skill này trước khi khám phá một vùng source code diện rộng.

Ví dụ:

- tìm feature được implement ở đâu;
- investigate bug qua nhiều file;
- chuẩn bị sửa một module;
- cần hiểu một subsystem chưa quen.

Chạy:

    ./bin/pnotes brief --area <path> --limit 3

Hoặc nếu chưa có `brief`:

    ./bin/pnotes recall --area <path> --limit 3

Dùng kết quả để thu hẹp source files và notes cần đọc.

Mục tiêu không phải tránh đọc code. Mục tiêu là tránh đọc code không liên quan và tránh khám phá lại context đã biết.

### 3.3 Sau khi có implementation output

Phải dùng skill này sau khi có implementation output.

Implementation output nghĩa là có thay đổi ở source code, tests, config, scripts, migrations, hoặc behavior của project.

Sau khi implement, tạo continuity note nếu công việc tạo ra bất kỳ thông tin nào sau đây:

- decision;
- invariant;
- risk;
- test evidence;
- missing test;
- deviation so với Handoff hoặc plan;
- trap/gotcha;
- rejected approach/dead end;
- validation delta;
- next-agent hint.

Nếu task đến từ Execution Handoff và đã có implementation output, tạo ít nhất một minimum continuity note.

Minimum note hợp lệ có thể dùng:

- `None` cho `deviations`, `traps`, `dead_ends`;
- `As expected` cho `validation_delta` chỉ khi validation thật sự đúng như kỳ vọng;
- `Not run — reason: ...` nếu validation chưa chạy;
- `See Handoff. No surprises.` cho `next_agent_hint` chỉ khi thật sự không có surprise.

## 4. Khi không dùng

Không dùng skill này cho:

- Q&A thuần túy, không làm việc với repo;
- dịch thuật hoặc rewrite prose;
- đọc một file user đưa sẵn khi không cần source exploration hoặc code change;
- task ngoài repo không có `./bin/pnotes` hoặc `.project-notes/`, trừ khi user muốn bootstrap project-notes vào repo đó;
- thông tin nhạy cảm không nên commit vào repo.

Không được skip skill chỉ vì code task nhỏ.

Task nhỏ vẫn có thể tạo ra decision, risk, missing test, hoặc trap quan trọng.

## 5. Command reference

Initialize một lần nếu cần:

    ./bin/pnotes init

Recall notes liên quan trước khi làm:

    ./bin/pnotes recall --area src/session-manager --limit 3
    ./bin/pnotes recall --task auth-fix
    ./bin/pnotes recall --tag bug

Tạo change safety brief trước khi làm, nếu CLI hỗ trợ:

    ./bin/pnotes brief --area src/session-manager --limit 3
    ./bin/pnotes brief --area src/session-manager --tag resume-flow --limit 3

Tạo continuity note sau khi implement:

    ./bin/pnotes add continuity \
      --task auth-fix \
      --signal "JWT expiry edge case fixed in middleware" \
      --area src/auth \
      --tag bug \
      --handoff auth-fix-execution-handoff.md

Nếu CLI hỗ trợ metadata flags, thêm các thông tin high-value ngay khi tạo note:

    ./bin/pnotes add continuity \
      --task agent-work-cockpit \
      --signal "Session id capture should not rely on arbitrary stdout parsing." \
      --area src/session-manager \
      --tag resume-flow \
      --decision "Use explicit machine-readable session markers." \
      --invariant "Stored session id must map to exactly one agent execution." \
      --risk "CLI stdout format may change between versions." \
      --missing-test "No automated test currently protects browser-close-does-not-kill-subprocess."

Đọc full note:

    ./bin/pnotes show 2026-05-25-auth-fix

In CLI usage guide:

    ./bin/pnotes guide

## 6. Workflow trước khi implement

Trước khi sửa code:

1. Xác định target area, task slug, hoặc tag.
2. Chạy `pnotes brief` nếu có.
3. Nếu `brief` chưa có hoặc không có kết quả, chạy `pnotes recall`.
4. Chỉ đọc các note liên quan được trả về.
5. Rút ra:
   - decisions;
   - invariants;
   - risks;
   - tests;
   - missing tests;
   - traps;
   - rejected approaches;
   - next-agent hints.
6. Sau đó mới inspect target source files và nearest tests.
7. Giữ code change surgical.

Nếu không có note nào match:

- không fail;
- tiếp tục source inspection bình thường;
- sau implementation, tạo note nếu phát hiện reusable project knowledge mới.

## 7. Workflow sau khi implement

Sau khi code thay đổi:

1. Review những gì đã đổi.
2. So sánh implementation với Handoff hoặc task intent.
3. Xác định project knowledge có giá trị:
   - decisions đã chốt;
   - invariants phát hiện;
   - risks gặp phải;
   - tests đã chạy và chúng cover gì;
   - missing test coverage;
   - deviations so với plan;
   - traps/gotchas;
   - dead ends;
   - validation delta;
   - một next-agent hint quan trọng nhất.
4. Tạo continuity note.
5. Điền các body sections.
6. Không viết changelog.

Note phải giúp agent tiếp theo tránh rediscovery. Note không phải implementation summary.

## 8. Cấu trúc note

Mỗi note được lưu tại:

    .project-notes/notes/{YYYY-MM-DD}-{task-slug}.md

Base frontmatter:

    id: "2026-05-25-auth-fix"
    type: "continuity"
    task: "auth-fix"
    created_at: "2026-05-25"
    signal: "JWT expiry edge case fixed in middleware"
    areas:
      - src/auth
    tags:
      - bug
    supersedes: []

Optional frontmatter cho change safety brief:

    decisions:
      - "Use explicit machine-readable session markers."

    invariants:
      - "Stored session id must map to exactly one agent execution."

    risks:
      - "CLI stdout format may change between versions."

    tests:
      - command: "npm test -- session-manager"
        covers:
          - "resume existing session"
          - "create new session when missing id"

    missing_tests:
      - "No test for browser-close-does-not-kill-subprocess."

Body sections:

    ## task
    {task-slug} — {run-id nếu có} — {handoff link nếu có}

    ## deviations
    None hoặc những điểm làm khác Handoff/plan

    ## traps
    Hidden constraints, gotchas, hoặc điều kiện không hiển nhiên

    ## dead_ends
    Approach đã thử và bị loại, kèm lý do

    ## validation_delta
    As expected, Not run — reason: ..., hoặc validation difference đáng chú ý

    ## next_agent_hint
    Một high-signal hint cho agent tiếp theo

## 8. Không được đưa gì vào note

Không thêm các section như:

    ## what_i_did
    ## summary
    ## implementation_details
    ## files_changed
    ## changelog

Những thông tin đó thuộc về git diff, commit message, PR description, hoặc execution evidence.

Trước khi viết một dòng vào note, tự hỏi:

    Thông tin này đã rõ từ git diff, Handoff, hoặc evidence chưa?

Nếu có rồi, không viết lại, trừ khi nó cần được ghi dưới dạng decision, invariant, risk, test coverage note, hoặc missing test note.

## 9. Semantics của recall và brief

`pnotes recall` trả về notes liên quan.

`pnotes brief` trả về current change safety context được aggregate từ notes liên quan.

Với `brief`, notes đã bị supersede phải bị loại khỏi aggregation.

Nếu note B khai báo:

    supersedes:
      - note-A

Thì `brief` không được aggregate `decisions`, `invariants`, `risks`, `tests`, `missing_tests`, hoặc recent continuity signal từ note-A khi note-B relevant.

`recall` có thể vẫn hiển thị superseded notes nếu CLI chưa hỗ trợ filter riêng.

## 10. Recall scoring

`pnotes recall` và `pnotes brief` nên ưu tiên notes theo:

    Area exact match: +5
    Area prefix match: +4
    Task exact match: +3
    Tag match: +2
    Text match trong signal/read_when: +2
    Recency boost: +1

Tie-break: `created_at` DESC.

Ví dụ prefix match:

- Note area `src/session-manager`, query `src/` -> match.
- Note area `src/`, query `src/session-manager` -> match.
- Note area `src/session-manager`, query `src/session-manager` -> exact match.

## 11. Immutability và superseding

Continuity notes là immutable execution records.

Được phép sửa:

- typo;
- broken link;
- wrong relative path;
- formatting không đổi nghĩa.

Không được sửa:

- rewrite conclusions;
- remove deviations;
- thêm traps mới phát hiện vào note cũ;
- đổi validation result;
- đổi `next_agent_hint`.

Nếu substantive information thay đổi sau này, tạo note mới và dùng:

    supersedes:
      - old-note-id

Dùng `supersedes` bằng exact note id, không match mờ theo title/path.

## 12. Security guardrails

Không lưu:

- credentials;
- API keys;
- private tokens;
- passwords;
- personal data;
- customer secrets;
- production secrets.

Nếu note hữu ích nhưng có thông tin nhạy cảm, viết bản đã sanitize.

## 13. Failure handling

Nếu thiếu `./bin/pnotes`:

- báo rằng project-notes unavailable;
- tiếp tục source inspection bình thường;
- nói rõ không thể chạy pnotes recall/brief.

Nếu lỗi permission:

    chmod +x bin/pnotes

Nếu chưa có `brief` nhưng có `recall`:

    ./bin/pnotes recall --area <path> --limit 3

Nếu không có note nào match:

- tiếp tục bình thường;
- không scan toàn bộ notes;
- sau implementation, tạo note nếu phát hiện reusable project knowledge mới.

## 14. Completion requirement

Một code implementation task chưa hoàn tất cho đến khi một trong các điều sau đúng:

1. Đã tạo continuity note cho implementation output.
2. Agent nói rõ vì sao không cần note:
   - không có code/config/test/script behavior changed;
   - task là pure Q&A hoặc read-only;
   - repo không có pnotes setup;
   - user explicitly yêu cầu không tạo note.

Không được dùng “task nhỏ” làm lý do skip project-notes.
