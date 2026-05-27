---
name: project-notes
description: Quy trình bắt buộc sử dụng pnotes trước khi implement, trước khi khám phá source code diện rộng, và làm cổng kiểm soát hoàn thành (completion gate) sau mọi thay đổi code. Đảm bảo tính liên tục của dự án (continuity) mà không cần quét toàn bộ repository.
---

# SKILL: project-notes

## 1. Mục đích và Ranh giới

> **Nguyên tắc cốt lõi:** Sử dụng project-local notes để tái hồi phục context, cô lập rủi ro và chuyển giao trạng thái làm việc (handoff) hiệu quả giữa các Agent hoặc giữa Agent và Con người.

* **Nhiệm vụ chính:**
    * Tái gọi (recall) context quan trọng trước khi can thiệp source code.
    * Ngăn chặn việc lặp lại các vết xe đổ (traps/dead ends).
    * Bảo toàn các quyết định (decisions), bất biến (invariants), rủi ro (risks) và lỗ hổng kiểm thử (missing tests).
    * Giảm thiểu chi phí duyệt (scan) các file không liên quan.
    * Lưu lại ghi chú liên tục (continuity notes) phục vụ phiên làm việc kế tiếp.

* **Hệ thống phân định lưu trữ:**

| Thành phần | Đường dẫn / Công cụ | Vai trò cấu trúc |
| :--- | :--- | :--- |
| **Tool mặc định** | `./bin/pnotes` | Lớp CLI điều hướng, truy xuất và hỗ trợ nhanh. |
| **Source of Truth** | `.project-notes/notes/*.md` | Định dạng lưu trữ gốc, toàn vẹn dữ liệu. |
| **Skill Artifacts** | `.agents/skills/project-notes/self-improvement/` | Nơi lưu trữ artifacts tự cải tiến của chính skill này. |

* **Giới hạn nghiêm ngặt:** Không sử dụng thư mục dự án `.project-notes/` để lưu trữ log/backup/matrix thuộc về nội bộ cơ chế tự cải tiến của chính skill `project-notes`.

---

## 2. Trigger bắt buộc (Mandatory Triggers)

Agent bắt buộc phải kích hoạt skill này tại 3 thời điểm chiến lược sau:

### 2.1 Trước khi Thực hiện (Pre-implementation)
Áp dụng trước khi sửa đổi source code cho feature, bugfix, refactor, migration, hoặc thay đổi hành vi hệ thống (behavior change).

* **Chiến lược ưu tiên câu lệnh:**
    1.  *Lựa chọn 1 (Ưu tiên cao nhất):* `./bin/pnotes brief --area <path> --limit 5`
    2.  *Lựa chọn 2 (Fallback nếu brief trống/lỗi):* `./bin/pnotes recall --area <path> --limit 5`
* **Bộ lọc nâng cao (Khi xác định rõ task/tag):**
    * `./bin/pnotes brief --area <path> --tag <tag> --limit 5`
    * `./bin/pnotes recall --task <slug>`
    * `./bin/pnotes recall --tag <tag>`

> **Ranh giới:** Phải đọc kỹ brief/notes trả về trước khi can thiệp code. Nghiêm cấm hành vi tự động quét (scan) toàn bộ thư mục `.project-notes/notes/` theo mặc định.

### 2.2 Trước khi Khám phá Source Code diện rộng (Wide-scope Exploration)
Kích hoạt bắt buộc khi:
* Tìm kiếm vị trí implement một tính năng chưa rõ định vị.
* Điều tra (investigate) bug chạy xuyên suốt qua nhiều file/module.
* Chuẩn bị can thiệp vào một module hoặc phân hệ (subsystem) chưa quen thuộc.

> **Mục tiêu:** Tối ưu hóa hiệu năng bằng cách loại bỏ việc đọc mã nguồn không liên quan và tận dụng triệt để những ngữ cảnh (context) đã được xử lý trước đó.

### 2.3 Sau khi có Kết quả Thực hiện (Post-implementation Output)
Bao gồm mọi thay đổi liên quan đến: Source code, tests, config, scripts, migrations, generated files, hoặc project behavior.

> **Quy định hoàn thành (Completion Gate):** Task **chưa** được coi là hoàn thành nếu chưa thỏa mãn một trong hai điều kiện:
> 1. Đã khởi tạo thành công Continuity Note đạt chuẩn.
> 2. Đã cung cấp lý do bỏ qua (skip reason) hoàn toàn hợp lệ.
>
> *Nghiêm cấm lý do từ chối dựa trên: task nhỏ, thay đổi đơn giản, git diff đã rõ ràng hoặc phần tổng hợp (final summary) đã đề cập.*

---

## 3. Điều kiện ngoại lệ (Exceptions & Skip Reasons)

### 3.1 Trường hợp không áp dụng skill
* Tác vụ Q&A thuần túy, không tương tác hoặc chỉnh sửa repository.
* Dịch thuật, viết tài liệu hoặc rewrite văn bản (prose).
* Đọc một file độc lập do người dùng cung cấp mà không cần thám hiểm code/thay đổi code.
* Tác vụ ngoài repo không được thiết lập `./bin/pnotes` hoặc `.project-notes/`.
* Thông tin mật, nhạy cảm không được phép lưu vết vào repo.

### 3.2 Lý do bỏ qua hợp lệ (Valid Skip Reasons)
Tác vụ chỉ được phép bỏ qua cổng kiểm soát project notes nếu thuộc một trong các lý do sau:
* Không có bất kỳ thay đổi nào liên quan đến code/config/test/script/behavior.
* Tác vụ là Read-only hoặc Pure Q&A.
* Repository hiện tại chưa cấu hình hoặc không hỗ trợ `pnotes`.
* User trực tiếp đưa ra yêu cầu bằng văn bản: không tạo note.

---

## 4. Danh mục câu lệnh (Command Reference)

# Khởi tạo hệ thống (Chỉ chạy một lần nếu chưa có)
./bin/pnotes init

# Kiểm tra an toàn trước khi xử lý (Pre-work recall/brief)
./bin/pnotes brief --area src/session-manager --limit 5
./bin/pnotes brief --area src/session-manager --tag resume-flow --limit 5
./bin/pnotes recall --area src/session-manager --limit 5
./bin/pnotes recall --task auth-fix
./bin/pnotes recall --tag bug

# Đọc chi tiết một ghi chú cụ thể
./bin/pnotes show <note-id>

# Kiểm tra trạng thái và danh sách ghi chú chưa đánh giá chất lượng (chạy khi có Continuity được tạo mới)
./bin/pnotes quality status

# Ghi nhận kết quả đánh giá chất lượng từ file JSON cấu trúc
./bin/pnotes quality record --from review-result.json

# Tra cứu hướng dẫn sử dụng từ CLI
./bin/pnotes guide

### Khởi tạo Continuity Note bằng CLI

* *Trường hợp CLI hỗ trợ tham số cơ bản:*

    ./bin/pnotes add continuity \
    --task auth-fix \
    --signal "JWT expiry edge case fixed in middleware" \
    --area src/auth \
    --tag bug \
    --handoff auth-fix-execution-handoff.md


* *Trường hợp CLI hỗ trợ đầy đủ Metadata Flags (Khuyến khích dùng trực tiếp):*

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


> **Lưu ý tương thích:** Nếu CLI thiếu flag cho một metadata cụ thể, Agent không được bỏ qua metadata đó. Hãy tạo note với các flag được hỗ trợ trước, sau đó bổ sung thủ công vào phần Frontmatter của file Markdown.

---

## 5. Quy trình thực hiện (Workflows)

### 5.1 Luồng xử lý Trước khi Implement

1. **Định vị:** Xác định rõ target area, task slug, hoặc tag liên quan.
2. **Truy vấn bước 1:** Thực thi `pnotes brief` để lấy tổng quan an toàn.
3. **Truy vấn bước 2:** Nếu `brief` không khả dụng hoặc rỗng, thực thi ngay `pnotes recall`.
4. **Hấp thụ:** Đọc cô đọng các notes liên quan được trả về.
5. **Trích xuất:** Định hình rõ: *decisions, invariants, risks, tests, missing tests, traps, dead ends, next-agent hints*.
6. **Thực thi cục bộ:** Inspect các file source mục tiêu và các tầng test gần nhất.
7. **Sửa đổi mã nguồn:** Giữ cho các thay đổi mang tính chính xác cao và khu trú (surgical change).

*Lưu ý:* Nếu không tìm thấy ghi chú nào khớp, Agent không được báo lỗi, tiếp tục quy trình dò code bình thường và chuẩn bị tinh thần thu thập kiến thức mới cho bước sau.

### 5.2 Luồng xử lý Sau khi Implement

1. **Kiểm tra thực tế:** Review toàn bộ thay đổi (diff) vừa tạo ra.
2. **Đánh giá mục tiêu:** Đối chiếu mã nguồn mới với Handoff/Task Intent ban đầu.
3. **Cô đọng tri thức:** Chắt lọc các giá trị tái sử dụng: *decisions, invariants, risks, tests, missing_tests, deviations, traps, dead_ends, validation_delta, next_agent_hint*.
4. **Tạo Note:** Khởi tạo continuity note thông qua CLI hoặc chỉnh sửa thủ công theo đúng Schema.
5. **Inspect cục bộ (Quality Gate):** Tự kiểm tra lại file vừa tạo dựa trên các tiêu chí nhanh ở Section 7 (Frontmatter đầy đủ, ID slug-safe, không chứa changelog rác). *Lưu ý nghiêm ngặt:* Bước này chỉ để đảm bảo tính hợp lệ cục bộ của chính ghi chú vừa tạo. Tuyệt đối **KHÔNG** chạy các lệnh chấm điểm diện rộng (`pnotes quality status/record`) tại đây để tránh gây nhiễu pattern và phá vỡ tính surgical của task code.
6. **Chuẩn hóa:** Tối ưu hóa cấu trúc/reformat file để phục vụ tốt cho cơ chế `pnotes brief` tương lai.
7. **Giới hạn nội dung:** Tuyệt đối không biến ghi chú thành một file changelog trùng lặp.
8. **Phản hồi:** Trả về trạng thái ghi chú bắt buộc trong final response.

---

## 6. Cấu trúc Note Schema chuẩn chỉnh

Mỗi ghi chú được định danh cố định tại đường dẫn: `.project-notes/notes/{YYYY-MM-DD}-{task-slug}.md`

### 6.1 Cấu trúc Frontmatter (YAML)

```yaml
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
# Optional Frontmatter bổ sung cho pnotes brief
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
---

```

#### Quy chuẩn dữ liệu Frontmatter:

* `id` & `task`: Bắt buộc phải là dạng slug-safe (chỉ chứa chữ thường, số, dấu gạch ngang). Không chứa khoảng trắng, dấu hai chấm, dấu gạch chéo, dấu ngoặc hoặc các ký tự đặc biệt.
* *Đúng:* `id: 2026-05-26-story-3-5a-session-summary`
* *Sai:* `id: '2026-05-26-Story 3.5a: Session Summary'`


* `areas`: Phải là một YAML list chứa các đường dẫn repo riêng biệt, tường minh. Không dùng nhãn chung chung, không gộp nhiều area thành một chuỗi phân cách bởi dấu phẩy.
* *Đúng:* Khai báo rõ ràng từng dòng `- frontend/src/hooks`
* *Sai:* `- backend, frontend` hoặc các thư mục gốc quá rộng nếu thay đổi chỉ nằm ở module nhỏ.


* `decisions`: Mỗi item phản ánh một quyết định mang tính bền vững; không gộp nhiều quyết định vào một dòng, không mô tả tiến trình thực hiện.
* `risks`: Ghi rõ kịch bản lỗi hệ thống (failure mode), có thể kèm phương án giảm thiểu, nhưng bản chất rủi ro phải trực diện.
* `tests`: Mỗi command phải đi kèm mảng `covers` không rỗng. Phải mô tả hành vi được bảo vệ, không chỉ ghi tên hàm test.
* `missing_tests`: Liệt kê các lỗ hổng test đã nhận diện (ví dụ: thiếu E2E browser, thiếu kiểm thử vòng đời tiến trình, thiếu CLI validation diện rộng).

### 6.2 Cấu trúc Body Sections (Markdown)

```markdown
## task
{task-slug} — {run-id nếu có} — {handoff link nếu có}

## deviations
None hoặc các điểm thiết kế/thực thi đi chệch khỏi Handoff/Plan ban đầu.

## traps
Các ràng buộc ngầm (hidden constraints), gotchas, hoặc các điều kiện biên không hiển nhiên.

## dead_ends
Các hướng tiếp cận đã thử nghiệm nhưng thất bại hoặc bị loại bỏ, kèm lý do chi tiết.

## validation_delta
Kết quả nghiệm thu: "As expected", hoặc "Not run — reason: ...", hoặc các khác biệt kiểm thử đáng chú ý.

## next_agent_hint
Một chỉ dẫn có mật độ thông tin cao (high-signal) dành riêng cho Agent kế tiếp.

```

> **Tiêu chuẩn điền thông tin tối thiểu:** > * Có thể dùng `None` cho `deviations`, `traps`, `dead_ends`.
> * Có thể dùng `As expected` cho `validation_delta` nếu việc kiểm thử chuẩn xác hoàn toàn.
> * Chỉ dùng `See Handoff. No surprises.` cho mục `next_agent_hint` khi và chỉ khi hệ thống thực sự không có bất kỳ rủi ro ngầm hay kiến thức tái sử dụng nào.
> * **Đặc biệt:** Tuyệt đối không ghi `## traps\nNone` nếu kiến trúc xử lý có dính dáng đến: *async cache timing, rollback logic, subprocess lifecycle, trạng thái chuyển đổi DB, race conditions, parser behavior, hoặc các ràng buộc giao tiếp ngầm giữa frontend/backend.*

---

## 7. Tiêu chuẩn chất lượng nghiêm ngặt (Quality Gate)

Sau khi tạo ghi chú, Agent phải tự động chạy cơ chế kiểm soát chất lượng (Inspect) trước khi đưa ra phản hồi cuối cùng cho người dùng.

### 7.1 Quy định Định dạng (Formatting Discipline)

* Continuity Note bắt buộc phải là Markdown nhiều dòng, tường minh, dễ diff và dễ đọc bởi cả người và LLM.
* Không được gom Frontmatter hoặc Body thành một hoặc hai dòng siêu dài.
* Cặp ký tự định ranh giới frontmatter `---` phải nằm riêng biệt trên một dòng độc lập.
* Giữa Frontmatter và phần Body, cũng như giữa các Section với nhau bắt buộc phải cách nhau bởi ít nhất một dòng trống.

### 7.2 Phòng chống Rác thông tin (Anti-garbage Guidelines)

* **Tuyệt đối không** tự tiện thêm các phân đoạn mang tính chất nhật ký cá nhân như: `## what_i_did`, `## summary`, `## implementation_details`, `## files_changed`, `## changelog`.
* Trước khi viết bất kỳ thông tin nào vào phần body, hãy luôn tự vấn: *"Thông tin này đã hiển thị rõ trong git diff, file Handoff hoặc bằng chứng thực thi (evidence) chưa?"*. Nếu câu trả lời là **Đã rõ**, nghiêm cấm đưa vào note, trừ khi nó được cấu trúc lại dưới dạng một *decision, invariant, risk, hoặc test coverage*.

### 7.3 Chuẩn hóa ngôn từ Test Coverage (`tests.covers`)

Mô tả trực diện hành vi nghiệp vụ được bảo toàn, không ghi chung chung tên file hay tiến trình kỹ thuật mơ hồ.

* *Yếu (Weak):* `covers: - "id filename derivation"`
* *Mạnh (Better):* `covers: - "generated note id matches resolved filename when collision suffix is used"`
* Nếu thực hiện nhiều lệnh kiểm thử trên các môi trường khác nhau (backend, frontend, CLI), hãy phân tách thành các entry riêng biệt trong mảng `tests`, không nối chuỗi dài bằng dấu `&&`.

---

## 8. Cơ chế Recall, Brief & Scoring

### 8.1 Logic Aggregate và Quyền ghi đè (Supersedes)

* `pnotes brief` tổng hợp context an toàn từ các note có liên quan, tự động loại trừ các dữ liệu cũ đã bị ghi đè (superseded).
* Nếu **Ghi chú B** khai báo trường thuộc tính:
```yaml
supersedes:
  - note-A

```

Khi Ghi chú B được xác định là có liên quan (relevant), hệ thống `brief` sẽ lập tức cô lập và loại bỏ toàn bộ dữ liệu từ `note-A` (bao gồm decisions, invariants, risks, tests, và signal) ra khỏi kết quả tổng hợp cuối cùng nhằm tránh gây nhiễu context.

### 8.2 Ma trận Trọng số Tìm kiếm (Scoring System)

Khi thực thi tìm kiếm, các ghi chú được chấm điểm tự động để sắp xếp độ ưu tiên theo bảng sau:

| Tiêu chí khớp dữ liệu (Match Criteria) | Điểm số cộng thêm (Score) |
| --- | --- |
| Khớp chính xác Area (`Area exact match`) | `+5` |
| Khớp tiền tố đường dẫn Area (`Area prefix match`) | `+4` |
| Khớp chính xác mã định danh Task (`Task exact match`) | `+3` |
| Khớp thẻ phân loại (`Tag match`) | `+2` |
| Khớp từ khóa trong chuỗi `signal` hoặc `read_when` | `+2` |
| Ưu tiên ghi chú mới nhất (`Recency boost`) | `+1` |
| **Tiêu chí gỡ hòa (Tie-break)** | Sắp xếp theo thứ tự `created_at` **DESC** (Mới xếp trước) |

---

## 9. Tính bất biến & Bảo mật (Immutability & Security)

### 9.1 Tính bất biến (Immutability)

Continuity notes hoạt động như những biên bản thực thi bất biến (immutable execution records).

* *Hành vi ĐƯỢC PHÉP:* Sửa lỗi chính tả, liên kết bị gãy, hiệu chỉnh đường dẫn tương đối hoặc reformat hiển thị mà không thay đổi ngữ nghĩa.
* *Hành vi CẤM:* Khởi tạo lại kết luận, xóa bỏ phân đoạn lỗi (deviations), chèn thêm các bẫy (traps) mới phát hiện vào một note cũ đã đóng, thay đổi kết quả kiểm thử (validation result) hoặc biến đổi nội dung `next_agent_hint`.
* *Giải pháp thay thế:* Nếu thông tin cốt lõi thay đổi, bắt buộc phải tạo một note mới hoàn toàn và sử dụng cơ chế `supersedes` để chỉ định chính xác ID của note cũ.

### 9.2 Rào chắn Bảo mật (Security Guardrails)

Tuyệt đối không ghi nhận các thông tin nhạy cảm vào project notes dưới mọi hình thức, bao gồm: Credentials, API keys, private tokens, passwords, personal data, customer secrets, production secrets. Mọi dữ liệu đưa vào bắt buộc phải được làm sạch (sanitize) hoàn toàn.

---

## 10. Xử lý sự cố (Failure Handling)

* **Trường hợp thiếu tập tin thực thi `./bin/pnotes`:**
* Thông báo tường minh: `project-notes unavailable`.
* Tiếp tục quy trình thám hiểm và chỉnh sửa mã nguồn bình thường.
* Nêu rõ trong báo cáo việc không thể thực thi recall/brief.


* **Trường hợp lỗi phân quyền (Permission Denied):** Thực thi ngay lệnh cấp quyền truy cập: `chmod +x bin/pnotes`
* **Trường hợp lệnh `brief` lỗi nhưng `recall` chạy được:** Chuyển đổi trạng thái sang lệnh dự phòng: `./bin/pnotes recall --area <path> --limit 5`
* **Trường hợp không tìm thấy ghi chú nào khớp (No match):** Tiếp tục thực hiện task bình thường, không quét bừa bãi toàn bộ kho dữ liệu. Khi kết thúc task, nếu phát hiện tri thức mới có khả năng tái sử dụng, tiến hành lập note như quy định.

---

## 11. Kỷ luật Tự cải tiến (Self-improvement Discipline)

> **Nguyên tắc tối cao:** Tự cải tiến không phải là cái cớ để tùy tiện chèn thêm các quy tắc rời rạc vào `SKILL.md` sau mỗi lỗi vận hành nhỏ. Mọi chỉnh sửa đối với skill phải tuân thủ quy trình phân lập nguyên nhân và ưu tiên can thiệp tối thiểu.

### 11.1 Phân loại Nguyên nhân gốc (Root Cause Classification)

Trước khi chỉnh sửa skill, lỗi phải được phân loại chính xác vào 1 trong 6 nhóm:

1. *Tool capability gap:* Thiếu hụt năng lực của công cụ CLI -> Ưu tiên nâng cấp công cụ/CLI.
2. *Workflow enforcement gap:* Lỏng lẻo trong khâu kiểm soát hoàn thành -> Thắt chặt kiểm soát tại `AGENTS.md`.
3. *Documentation clarity gap:* Tài liệu mơ hồ -> Viết lại hoặc tinh giản hóa phân đoạn tài liệu hiện có.
4. *Artifact quality gap:* Chất lượng note kém -> Cải tiến template/checklist hoặc công cụ validation.
5. *One-off execution mistake:* Sai sót nhất thời của Agent -> Không sửa đổi skill, chỉ log lại nếu cần cảnh báo.
6. *Product/design decision:* Thay đổi mang tính thiết kế sản phẩm -> Tạo handoff/decision note, nghiêm cấm vá skill.

### 11.2 Thứ tự Ưu tiên Can thiệp Tối thiểu (Minimal Intervention Priority)

Agent bắt buộc phải duyệt qua thứ tự ưu tiên từ thấp đến cao, chỉ chuyển sang bước sau nếu bước trước không thể giải quyết triệt để vấn đề:

1. Giữ nguyên trạng hệ thống (No change).
2. Cải tiến hành vi của công cụ CLI (`bin/pnotes`).
3. Tối ưu hóa cấu trúc template / định dạng dữ liệu đầu ra.
4. Thắt chặt chẽ hơn cổng kiểm soát tại file cấu hình `AGENTS.md`.
5. Viết lại hoặc làm rõ nội dung của một Section hiện có trong `SKILL.md`.
6. Chỉ thêm Section mới trong `SKILL.md` khi lỗi có tính lặp lại cao, có ảnh hưởng lớn, quy tắc mang tính tổng quát, có chỉ số đo lường thành công và có điều kiện khôi phục (rollback condition).
7. Thêm mới một tệp cấu trúc quy trình (process artifact).

### 11.3 Cơ chế Thử nghiệm (Experimentation Rules)

Mỗi chỉnh sửa phi tiểu tiết (non-trivial self-improvement) đối với cấu trúc skill đều được coi là một cuộc thử nghiệm và bắt buộc phải định nghĩa: *expected effect, trial window, review metric, và rollback condition.*

* **Khung thử nghiệm mặc định (Trial Window):** Đánh giá lại sau khi có thêm từ **5 đến 10** continuity notes mới được khởi tạo.
* **Các trạng thái quyết định sau thử nghiệm:**
* `keep`: Tiếp tục giữ nếu chất lượng ghi chú và tính tuân thủ quy trình tăng tiến rõ rệt mà không sinh nhiễu (noise).
* `amend`: Tinh chỉnh nếu quy tắc có giá trị nhưng gây ma sát vận hành (friction) cao hoặc còn mơ hồ.
* `rollback`: Khôi phục trạng thái cũ ngay lập tức nếu thử nghiệm không mang lại cải tiến, làm tăng nhiễu, giảm tỷ lệ điền ghi chú (fill rate) hoặc tăng độ trễ xử lý của Agent.
* `inconclusive`: Chưa đủ số lượng ghi chú cần thiết để đưa ra kết luận.

---

## 12. Định dạng Tệp Tri thức Tự cải tiến (Self-improvement Artifacts)

Toàn bộ tri thức tự cải tiến phải được lưu biệt lập tại cấu trúc thư mục phân tách: `<skill-root>/self-improvement/` (Ví dụ: `.agents/skills/project-notes/self-improvement/`).

### 12.1 Nhật ký Cải tiến (improvement-log.md)

Mọi hành vi thay đổi cấu trúc skill phải được append vào tệp này theo cấu trúc cố định:

```markdown
## YYYY-MM-DD — {task-slug}

Changed:
- [Mô tả chi tiết các điểm thay đổi]

Why:
- [Nguyên nhân cốt lõi và lập luận]

Expected effect:
- [Hành vi hoặc chỉ số mong đợi sẽ cải thiện]

Risk:
- [Rủi ro hệ thống hoặc độ trễ phát sinh]

Follow-up review:
- Quy trình review: Đánh giá lại sau 5-10 continuity notes mới.
- Quyết định hướng đi: lựa chọn giữa keep, amend, rollback, hoặc inconclusive.

```

### 12.2 Ma trận Chất lượng Ghi chú (note-quality-matrix.md)

Khi có sự thay đổi về tiêu chí đánh giá, Agent phải thực hiện sao lưu matrix cũ trước khi cập nhật dữ liệu mới. Hệ thống chấm điểm bắt buộc áp dụng thang điểm từ `0 đến 2` cho 4 trục chất lượng chính:

| Trục Chất lượng | Tiêu chí Kiểm định dữ liệu |
| --- | --- |
| **Metadata quality** | Kiểm tra độ chuẩn xác của `signal`, `areas`, `tags`, `decisions`, `invariants`, `risks`, `tests`, `missing_tests`, `supersedes`. |
| **Body quality** | Kiểm tra độ sâu thông tin của `deviations`, `traps`, `dead_ends`, `validation_delta`, `next_agent_hint`. |
| **Formatting quality** | Xác thực cú pháp multiline Markdown, tính hợp lệ của YAML, cấu trúc list thân thiện với diff, phân tách section bằng dòng trống. |
| **Anti-garbage quality** | Xác nhận không trùng lặp changelog, không bịa đặt dữ liệu test, không để lộ thông tin mật, có giá trị thực tế cho `pnotes brief`. |

* *Hướng dẫn chấm điểm:* `0` = Thiếu hụt hoặc gây hại; `1` = Có xuất hiện nhưng còn yếu/mơ hồ; `2` = Rõ ràng, hữu ích và mang tính hành động cao (actionable).

### 12.3 Kích hoạt Chấm điểm Chất lượng (Scoring Trigger)

Agent không tự nhẩm đếm số lượng file hoặc mốc ngày giờ thủ công. Quy trình chấm điểm chất lượng và tự đánh giá hiệu quả thực nghiệm bắt buộc phải kích hoạt trong luồng Tự cải tiến khi câu lệnh `./bin/pnotes quality status` trả về trạng thái `review_required: yes`.

Quy trình thực thi chấm điểm chu kỳ của Agent:

1. Thực thi câu lệnh `./bin/pnotes quality status` để quét và lấy danh sách các tệp ghi chú chưa đánh giá (pending notes).

2. Nếu CLI trả về `review_required: no` (chưa đạt ngưỡng hoặc thuộc trigger below_threshold), bỏ qua bước chấm điểm, giữ nguyên trạng hệ thống.

3. Nếu CLI trả về `review_required: yes`, Agent tiến hành đọc nội dung toàn bộ các file ghi chú nằm trong danh sách pending.

4. Đối chiếu các file đó với Ma trận chất lượng ghi chú (Section 12.2) để đánh giá chấm điểm từ 0 đến 2 cho từng trục.

5. Kết xuất một file JSON tạm thời `review-result.json` chứa cấu trúc bắt buộc:

```JSON
{
  "id": "YYYY-MM-DD-review-wave-x",
  "reviewed_at": "YYYY-MM-DD HH:MM:SS.mmm",
  "reviewed_until": "[Mốc thời gian tạo của file note mới nhất được duyệt]",
  "notes_reviewed": ["path/to/note1.md", "path/to/note2.md"],
  "average_score": 1.75,
  "decision": "keep"
}
```
*(Thuộc tính decision bắt buộc phải thuộc một trong các giá trị: keep, amend, rollback, inconclusive).*

6. Thực thi lệnh ghi nhận: `./bin/pnotes quality record --from review-result.json` để CLI cập nhật tự động duy nhất phần YAML frontmatter của file `note-quality-review-log.md`.

7. Tiến hành ghi nhận (append) phần nội dung nhận xét chi tiết, các lỗi hệ thống lặp lại (regressions) hoặc điểm tiến bộ vào ngay phía dưới phần nội dung Markdown của file `note-quality-review-log.md` để làm dữ liệu nền tảng cho các đợt improve tiếp theo.

Các trigger khẩn cấp khác (Bắt buộc chạy không phụ thuộc bộ đếm CLI):

- Trước khi đưa ra quyết định chuyển đổi trạng thái thử nghiệm (`keep`, `amend`, hoặc `rollback`) của một quy tắc thực nghiệm.

- Trước khi tiến hành bất kỳ sửa đổi nào tác động trực tiếp vào tệp cấu hình `SKILL.md`, note schema, completion gate, hoặc logic xử lý tổng hợp của `pnotes brief/recall`.

- Khi phát hiện tối thiểu 2 ghi chú gần nhất được tạo liên tiếp mắc cùng một lỗi hệ thống nặng.

- Khi đầu ra của `pnotes brief` trả về kết quả nhiễu, mâu thuẫn trực tiếp hoặc rỗng hoàn toàn.

---

## 13. Định dạng Phản hồi Bắt buộc tại Đầu ra (Output Requirements)

Để vượt qua các cổng kiểm soát tiến trình, Agent bắt buộc phải đính kèm các block trạng thái tiêu chuẩn ở cuối phản hồi cuối cùng (final response) tùy theo loại tác vụ:

### 13.1 Đối với Tác vụ Thực hiện Code (Code Implementation Task Completion)

Project notes: created <path_đến_file_ghi_chú>

*Hoặc trong trường hợp không tạo note hợp lệ:*

Project notes: skipped — <lý_do_bỏ_qua_hợp_lệ>

### 13.2 Đối với Tác vụ Tự cải tiến Skill (Self-improvement Task Completion)

Self-improvement decision: keep / amend / rollback / inconclusive / not-reviewed
Quality Status Check: review_required: yes (processed via CLI) / no (below_threshold)
Backup: created  / skipped — 
Improvement log: updated 
Score matrix backup: created  / skipped — 
Score matrix: created-or-updated  / unchanged — 
Note quality review: updated  / skipped —  (Scoring Recorded At: YYYY-MM-DD HH:MM:SS.mmm)
