## Communication defaults

- Always reply in Vietnamese by default.
- Keep technical terms, code, commands, file paths, config keys, and error messages in their original form unless translation is explicitly requested.
- If the user writes in another language and explicitly asks for that language, follow the user's language for that response.
- Prefer concise, operational Vietnamese.
- Do not switch to English unless the user asks, the content is a verbatim quote, or keeping the original wording is technically important.

---

## Coding Guidelines

### 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

Before implementing:

* State your assumptions explicitly. If uncertain, ask.
* If multiple interpretations exist, present them - don't pick silently.
* If a simpler approach exists, say so. Push back when warranted.
* If something is unclear, stop. Name what's confusing. Ask.

### 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

* No features beyond what was asked.
* No abstractions for single-use code.
* No "flexibility" or "configurability" that wasn't requested.
* No error handling for impossible scenarios.
* If you write 200 lines and it could be 50, rewrite it.

> **Self-Check:** "Would a senior engineer say this is overcomplicated?" If yes, simplify.

### 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

When editing existing code:

* Don't "improve" adjacent code, comments, or formatting.
* Don't refactor things that aren't broken.
* Match existing style, even if you'd do it differently.
* If you notice unrelated dead code, mention it - don't delete it.
* Remove imports/variables/functions that YOUR changes made unused.
* Don't remove pre-existing dead code unless asked.

> **The Test:** Every changed line in the diff should trace directly to the user's request.

### 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

Transform tasks into verifiable goals:

* **"Add validation"** → Write tests for invalid inputs, then make them pass.
* **"Fix the bug"** → Write a test that reproduces it, then make it pass.
* **"Refactor X"** → Ensure tests pass before and after.

For multi-step tasks, state a brief plan:

```text
1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]

```

> **Note:** Strong success criteria let you loop independently. Weak criteria ("make it work") require constant clarification.

---

## Quy định: Ghi chú Dự án (Project Notes Completion Gate)

### 1. Truy xuất Ngữ cảnh

Bắt buộc chạy lệnh sau để nắm bắt thông tin dự án trước khi triển khai:

    ./bin/pnotes brief --area <path> --limit 10

### 2. Tiêu chuẩn Hoàn thành Tác vụ

Trước khi đóng tác vụ, người thực hiện (Agent) **phải** hoàn thành một trong hai phương án sau:

#### Lựa chọn A: Tạo Ghi chú Tiếp nối (Continuity Note)

Thực thi lệnh:

    ./bin/pnotes add continuity {YYYY-MM-DD}-{task-slug}.md

**Yêu cầu bắt buộc:**

* Kiểm tra (inspect) tệp vừa tạo trước khi phản hồi cuối cùng.
* Viết bằng định dạng Markdown nhiều dòng, rõ ràng.
* Bổ sung siêu dữ liệu (frontmatter) phù hợp: `decisions`, `invariants`, `risks`, `tests`, `missing_tests`, `supersedes`.

#### Lựa chọn B: Bỏ qua (Kèm lý do hợp lệ)

**Được phép bỏ qua (Hợp lệ):**

* Không thay đổi code, config, test, script hay hành vi hệ thống.
* Tác vụ chỉ là Hỏi & Đáp (Q&A) hoặc chỉ đọc (read-only).
* Không tìm thấy `./bin/pnotes` hoặc `.project-notes/` trong kho lưu trữ.
* Người dùng yêu cầu rõ ràng việc không tạo ghi chú.

**Tuyệt đối không bỏ qua (Không hợp lệ):**

* Viện cớ tác vụ nhỏ, thay đổi đơn giản hoặc sửa ít dòng code.
* Cho rằng phần tóm tắt hoặc git diff đã cung cấp đủ thông tin.
* Đã chạy lệnh tạo ghi chú nhưng chưa kiểm tra (inspect) lại tệp.

### 3. Cú pháp Phản hồi Cuối cùng

Đính kèm **duy nhất một dòng** ở cuối thông điệp phản hồi theo đúng cú pháp sau:

*Nếu đã tạo ghi chú:*

    Project notes: created <path>


*Nếu bỏ qua (kèm lý do hợp lệ):*

    Project notes: skipped — <valid reason>
