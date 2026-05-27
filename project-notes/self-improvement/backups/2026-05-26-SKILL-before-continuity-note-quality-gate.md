---
name: project-notes
description: Bắt buộc dùng pnotes trước khi implement, trước khi khám phá source code diện rộng, và như một completion gate sau mọi implementation output dù task nhỏ hay lớn. Không được báo task complete nếu chưa tạo continuity note hoặc nêu rõ lý do skip hợp lệ. Dùng pnotes để recall decisions, invariants, risks, tests, missing tests, traps, dead ends, và continuity notes của project mà không phải scan toàn bộ repository.
---

# SKILL: project-notes

## 1. Mục đích và ranh giới

Dùng project-local notes để:

- recall context quan trọng trước khi sửa code;
- tránh lặp lại traps/dead ends;
- không bỏ qua decisions, invariants, risks, tests, missing tests đã biết;
- giảm việc scan source files không liên quan;
- lưu lại continuity sau implementation cho agent/human tiếp theo.

Tool mặc định:

    ./bin/pnotes

Storage của project đang làm việc:

    .project-notes/notes/*.md

Markdown notes là source of truth. CLI chỉ là lớp recall/brief/helper.

Không dùng `.project-notes/` để lưu backup/log/matrix của chính skill `project-notes`. Self-improvement artifacts của skill phải nằm cạnh skill, ví dụ:

    .agents/skills/project-notes/self-improvement/

## 2. Trigger bắt buộc

### 2.1 Trước khi implement

Phải dùng skill này trước khi sửa source code cho feature, bugfix, refactor, migration, hoặc behavior change.

Ưu tiên:

    ./bin/pnotes brief --area <path> --limit 5

Fallback nếu `brief` chưa có hoặc không hữu ích:

    ./bin/pnotes recall --area <path> --limit 5

Có thể thêm filter khi biết task/tag:

    ./bin/pnotes brief --area <path> --tag <tag> --limit 5
    ./bin/pnotes recall --task <slug>
    ./bin/pnotes recall --tag <tag>

Trước khi sửa code, phải đọc brief hoặc notes liên quan được trả về. Không scan toàn bộ `.project-notes/notes/` theo mặc định.

### 2.2 Trước khi khám phá source code diện rộng

Phải dùng skill này trước khi:

- tìm feature được implement ở đâu;
- investigate bug qua nhiều file;
- chuẩn bị sửa một module;
- cần hiểu một subsystem chưa quen.

Mục tiêu không phải tránh đọc code. Mục tiêu là tránh đọc code không liên quan và tránh khám phá lại context đã biết.

### 2.3 Sau khi có implementation output

Implementation output bao gồm thay đổi ở:

- source code;
- tests;
- config;
- scripts;
- migrations;
- generated project files;
- project behavior.

Sau implementation, task chưa được xem là complete cho đến khi một trong hai điều đúng:

1. Đã tạo continuity note.
2. Đã nêu rõ lý do skip hợp lệ.

Không được skip chỉ vì task nhỏ, change đơn giản, chỉ sửa vài dòng, final summary đã nói rồi, hoặc git diff đủ rõ.

## 3. Khi không dùng

Không dùng skill này cho:

- Q&A thuần túy, không làm việc với repo;
- dịch thuật hoặc rewrite prose;
- đọc một file user đưa sẵn khi không cần source exploration hoặc code change;
- task ngoài repo không có `./bin/pnotes` hoặc `.project-notes/`;
- thông tin nhạy cảm không nên commit vào repo.

Valid skip reasons:

- không có code/config/test/script/behavior changed;
- task là pure Q&A hoặc read-only;
- repo không có pnotes setup;
- user explicitly yêu cầu không tạo note.

## 4. Command reference

Initialize một lần nếu cần:

    ./bin/pnotes init

Pre-work recall/brief:

    ./bin/pnotes brief --area src/session-manager --limit 5
    ./bin/pnotes brief --area src/session-manager --tag resume-flow --limit 5
    ./bin/pnotes recall --area src/session-manager --limit 5
    ./bin/pnotes recall --task auth-fix
    ./bin/pnotes recall --tag bug

Tạo continuity note:

    ./bin/pnotes add continuity \
      --task auth-fix \
      --signal "JWT expiry edge case fixed in middleware" \
      --area src/auth \
      --tag bug \
      --handoff auth-fix-execution-handoff.md

Nếu CLI hỗ trợ metadata flags, dùng trực tiếp:

    ./bin/pnotes add continuity \
      --task agent-work-cockpit \
      --signal "Session id capture should not rely on arbitrary stdout parsing." \
      --area src/session-manager \
      --tag resume-flow \
      --decision "Use explicit machine-readable session markers." \
      --invariant "Stored session id must map to exactly one agent execution." \
      --risk "CLI stdout format may change between versions." \
      --test-command "npm test -- session-manager" \
      --test-covers "resume existing session" \
      --test-covers "create new session when missing id" \
      --missing-test "No automated test currently protects browser-close-does-not-kill-subprocess."

Đọc full note:

    ./bin/pnotes show <note-id>

In CLI usage guide:

    ./bin/pnotes guide

Nếu CLI hiện tại chưa hỗ trợ một metadata flag nào đó, không bỏ note. Tạo note bằng supported flags trước, rồi enrich frontmatter thủ công.

## 5. Workflow trước khi implement

1. Xác định target area, task slug, hoặc tag.
2. Chạy `pnotes brief` nếu có.
3. Nếu `brief` chưa có hoặc không hữu ích, chạy `pnotes recall`.
4. Chỉ đọc notes liên quan được trả về.
5. Rút ra decisions, invariants, risks, tests, missing tests, traps, dead ends, next-agent hints.
6. Sau đó mới inspect target source files và nearest tests.
7. Giữ code change surgical.

Nếu không có note nào match:

- không fail;
- tiếp tục source inspection bình thường;
- sau implementation, tạo note nếu phát hiện reusable project knowledge mới.

## 6. Workflow sau khi implement

1. Review thay đổi thực tế.
2. So sánh implementation với Handoff/task intent.
3. Xác định project knowledge có giá trị:
   - decisions;
   - invariants;
   - risks;
   - tests đã chạy và behavior chúng cover;
   - missing test coverage;
   - deviations;
   - traps/gotchas;
   - dead ends;
   - validation delta;
   - một next-agent hint quan trọng nhất.
4. Tạo continuity note.
5. Inspect generated note.
6. Reformat/enrich note nếu cần.
7. Không viết changelog.

Final response phải include:

    Project notes: created <path>

hoặc:

    Project notes: skipped — <valid reason>

## 7. Note schema

Mỗi note được lưu tại:

    .project-notes/notes/{YYYY-MM-DD}-{task-slug}.md

Frontmatter cơ bản:

    ---
    id: 2026-05-25-auth-fix
    type: continuity
    task: auth-fix
    created_at: 2026-05-25
    signal: "JWT expiry edge case fixed in middleware"
    areas:
      - src/auth
    tags:
      - bug
    supersedes: []
    ---

Optional frontmatter cho `pnotes brief`:

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

Body sections bắt buộc:

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

Minimum note hợp lệ có thể dùng:

- `None` cho `deviations`, `traps`, `dead_ends`;
- `As expected` cho `validation_delta` chỉ khi validation thật sự đúng như kỳ vọng;
- `Not run — reason: ...` nếu validation chưa chạy;
- `See Handoff. No surprises.` chỉ khi thật sự không có reusable hint.

## 8. Note quality gates

### 8.1 Formatting

Continuity note phải là Markdown nhiều dòng, dễ đọc, dễ diff, dễ review, và dễ cho LLM đọc.

Không được serialize frontmatter hoặc body thành một hoặc hai dòng dài.

Một note hợp lệ phải có:

- `---` mở và đóng frontmatter trên dòng riêng;
- mỗi frontmatter key trên dòng riêng;
- YAML lists dạng multiline;
- mỗi body section bắt đầu trên dòng riêng;
- dòng trống giữa frontmatter và body;
- dòng trống giữa các body sections.

Nếu note hiện ra như một hoặc hai dòng rất dài, note chưa đạt chuẩn. Phải reformat trước final response.

### 8.2 Frontmatter enrichment

Sau khi tạo note, phải inspect frontmatter.

Nếu applicable, điền:

- `decisions`: quyết định bền vững đã chọn/xác nhận;
- `invariants`: behavior constraint không được phá;
- `risks`: failure mode, race condition, fragility, operational risk;
- `tests`: command đã chạy và behavior được bảo vệ;
- `missing_tests`: coverage gap đã biết;
- `supersedes`: exact note id bị thay thế.

Không chỉ ghi structured project memory trong body nếu thông tin đó thuộc về frontmatter.

### 8.3 Anti-garbage

Không thêm các section:

    ## what_i_did
    ## summary
    ## implementation_details
    ## files_changed
    ## changelog

Những thông tin đó thuộc về git diff, commit message, PR description, hoặc execution evidence.

Trước khi viết một dòng vào note, tự hỏi:

    Thông tin này đã rõ từ git diff, Handoff, hoặc evidence chưa?

Nếu có rồi, không viết lại, trừ khi nó cần được ghi dưới dạng decision, invariant, risk, test coverage, hoặc missing test.

### 8.4 next_agent_hint

`next_agent_hint` phải chứa một hint hữu ích nhất cho agent tiếp theo.

Không dùng `See Handoff` nếu task đã introduce, thay đổi, xác nhận, hoặc bảo vệ decision, invariant, risk, parser/scoring/lifecycle behavior, test coverage, missing test, race condition, compatibility constraint, hoặc non-obvious trap.

Weak:

    See Handoff.

Better:

    Do not simplify test metadata parsing to Clap-only; --test-covers must stay attached to the nearest preceding --test-command.

### 8.5 Test coverage wording

`tests.covers` phải mô tả behavior được bảo vệ, không chỉ ghi tên test hoặc implementation detail mơ hồ.

Weak:

    covers:
      - "id filename derivation"

Better:

    covers:
      - "generated note id matches resolved filename when collision suffix is used"

Nếu command đã chạy nhưng coverage không rõ, ghi điều đã validate thủ công trong `validation_delta`, không invent coverage.

## 9. Recall, brief, scoring, supersedes

`pnotes recall` trả về notes liên quan.

`pnotes brief` trả về current change safety context được aggregate từ notes liên quan.

Với `brief`, notes đã bị supersede phải bị loại khỏi aggregation khi superseding note cũng relevant.

Nếu note B khai báo:

    supersedes:
      - note-A

thì `brief` không được aggregate `decisions`, `invariants`, `risks`, `tests`, `missing_tests`, hoặc recent continuity signal từ note-A khi note-B relevant.

`recall` có thể vẫn hiển thị superseded notes nếu CLI chưa hỗ trợ filter riêng.

Scoring nên ưu tiên:

    Area exact match: +5
    Area prefix match: +4
    Task exact match: +3
    Tag match: +2
    Text match trong signal/read_when: +2
    Recency boost: +1

Tie-break: `created_at` DESC.

## 10. Immutability

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

Dùng exact note id, không match mờ theo title/path.

## 11. Security guardrails

Không lưu:

- credentials;
- API keys;
- private tokens;
- passwords;
- personal data;
- customer secrets;
- production secrets.

Nếu note hữu ích nhưng có thông tin nhạy cảm, viết bản đã sanitize.

## 12. Failure handling

Nếu thiếu `./bin/pnotes`:

- báo project-notes unavailable;
- tiếp tục source inspection bình thường;
- nói rõ không thể chạy pnotes recall/brief.

Nếu lỗi permission:

    chmod +x bin/pnotes

Nếu `brief` unavailable nhưng `recall` có:

    ./bin/pnotes recall --area <path> --limit 5

Nếu không có note nào match:

- tiếp tục bình thường;
- không scan toàn bộ notes;
- sau implementation, tạo note nếu phát hiện reusable project knowledge mới.

## 13. Self-improvement discipline

Self-improvement không phải giấy phép để thêm rule vào `SKILL.md` sau mỗi lỗi nhỏ.

Khi phát hiện vấn đề, phải phân loại root cause trước khi sửa skill:

1. Tool capability gap — ưu tiên sửa CLI/tool.
2. Workflow enforcement gap — ưu tiên tighten completion gate trong AGENTS.md hoặc workflow nhỏ nhất.
3. Documentation clarity gap — ưu tiên rewrite/simplify section hiện có.
4. Artifact quality gap — ưu tiên template/checklist/validation.
5. One-off execution mistake — không sửa skill, chỉ log nếu có khả năng tái diễn.
6. Product/design decision — tạo handoff/decision note, không vá skill.

Minimal intervention priority:

1. No change.
2. Improve CLI/tool behavior.
3. Improve template/generated output.
4. Tighten AGENTS.md gate.
5. Rewrite existing SKILL section.
6. Add new SKILL section.
7. Add new process artifact.

Chỉ thêm section mới khi issue recurring/high-impact, rule general, có success metric, và có rollback condition.

Không add rule nếu chỉ hard-code một incident cụ thể.

Every non-trivial self-improvement is an experiment. It must define:

- expected effect;
- trial window;
- review metric;
- rollback condition.

Default trial window:

    Review after 5-10 new continuity notes.

After trial window, choose:

- `keep` nếu note quality/workflow compliance cải thiện rõ mà không tăng noise;
- `amend` nếu có ích nhưng quá nặng/mơ hồ/gây friction;
- `rollback` nếu không cải thiện, tăng noise, giảm fill rate, giảm compliance, hoặc tăng agent friction;
- `inconclusive` nếu chưa đủ notes.

Do not keep a self-improvement only because it sounds reasonable.

## 14. Self-improvement artifacts

Self-improvement artifacts của skill/workflow nằm cạnh skill, không nằm trong `.project-notes/` của project app.

Nếu skill nằm tại:

    .agents/skills/project-notes/SKILL.md

thì dùng:

    .agents/skills/project-notes/self-improvement/
      backups/
      improvement-log.md
      note-quality-matrix.md

### 14.1 Backup

Backup trước khi thay đổi:

- `SKILL.md`;
- AGENTS.md rule liên quan project-notes;
- note schema;
- CLI guide output;
- completion gate;
- formatting/enrichment rules;
- scoring/brief/recall semantics;
- `note-quality-matrix.md` nếu scoring criteria thay đổi.

Backup path:

    <skill-root>/self-improvement/backups/YYYY-MM-DD-{target-name}-before-{task-slug}.md

### 14.2 Improvement log

Mỗi self-improvement task append vào:

    <skill-root>/self-improvement/improvement-log.md

Entry tối thiểu:

    ## YYYY-MM-DD — {task-slug}

    Changed:
    - ...

    Why:
    - ...

    Expected effect:
    - ...

    Risk:
    - ...

    Follow-up review:
    - Review after 5-10 new continuity notes.
    - Decide: keep, amend, rollback, or inconclusive.

### 14.3 Note quality matrix

Khi thay đổi cách đánh giá chất lượng note, backup matrix cũ nếu có rồi ghi lại matrix mới tại:

    <skill-root>/self-improvement/note-quality-matrix.md

Matrix tối thiểu phải chấm 0-2 cho các nhóm:

- Metadata quality: `signal`, `areas`, `tags`, `decisions`, `invariants`, `risks`, `tests`, `missing_tests`, `supersedes`.
- Body quality: `deviations`, `traps`, `dead_ends`, `validation_delta`, `next_agent_hint`.
- Formatting quality: multiline Markdown, valid/readable YAML, diff-friendly lists, separated sections.
- Anti-garbage quality: no changelog duplication, no invented coverage/decisions, no secrets, useful for `pnotes brief`.

Score guide:

- 0 = missing or harmful
- 1 = present but weak
- 2 = clear, useful, actionable

Recommendation:

- Keep as-is
- Amend mechanical formatting only
- Create follow-up note
- Update skill/pnotes rule
- Improve CLI support

### 14.4 Self-improvement completion

Self-improvement task chưa complete cho đến khi applicable items được xử lý:

- backup created hoặc skip reason recorded;
- improvement log updated;
- note-quality-matrix backed up before rewrite nếu matrix đã tồn tại và scoring criteria changed;
- score matrix created/updated nếu note quality scoring rules changed;
- final response includes changed self-improvement artifact paths.

Final response phải include:

    Self-improvement decision: keep / amend / rollback / inconclusive / not-reviewed
    Backup: created <path> / skipped — <reason>
    Improvement log: updated <path>
    Score matrix backup: created <path> / skipped — <reason>
    Score matrix: created-or-updated <path> / unchanged — <reason>

## 15. Completion requirement

Một code implementation task chưa hoàn tất cho đến khi một trong các điều sau đúng:

1. Đã tạo continuity note cho implementation output.
2. Agent nói rõ vì sao không cần note:
   - không có code/config/test/script behavior changed;
   - task là pure Q&A hoặc read-only;
   - repo không có pnotes setup;
   - user explicitly yêu cầu không tạo note.

Không được dùng “task nhỏ” làm lý do skip project-notes.

Final response phải include:

    Project notes: created <path>

hoặc:

    Project notes: skipped — <valid reason>