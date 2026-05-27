---
name: party-mode
description: Mô phỏng một team nhỏ gồm nhiều vai trò chuyên môn độc lập cùng họp, phản biện và chốt một Execution Handoff artifact đủ rõ để executor ở bước sau có thể implement.
---

# SKILL: Party Mode Decision Council (party-mode)

## 1. Skill Identity & Absolute Boundaries

- **Skill name:** `party-mode`
- **Mục đích:** Dùng một team subagent nhỏ để thảo luận, phản biện và chốt một `Execution Handoff` artifact đủ chi tiết sau khi intent đã được làm rõ (thường là sau `grill-me`).
- **Bản chất:** Đây là phiên họp của nhiều vai trò chuyên môn độc lập dưới sự điều phối của Orchestrator, không phải một cá nhân tự quyết định rồi chuyển qua review.
- **Output cuối:** Một file/artifact `Execution Handoff.md` hoặc trạng thái `BLOCKED` nếu không đủ dữ kiện để tạo handoff đạt chuẩn.

### 🛑 CRITICAL GUARDRAILS (ZERO TOLERANCE)
Để đảm bảo an toàn hệ thống và tránh lãng phí token, Agent phải tuân thủ nghiêm ngặt 2 vùng biên giới sau:
1. **Tuyệt đối KHÔNG thực thi (Non-Execution):** Skill này CHỈ tạo tài liệu handoff. Không implement, không vá file source, không chạy command gây thay đổi hệ thống, không cài package, không commit, không gọi executor hoặc tự ý chuyển phase. Sau khi xuất artifact, agent PHẢI DỪNG LẠI.
2. **Tuyệt đối KHÔNG giả lập một mình (No Solo Roleplay):** Mỗi vai trò tranh luận (PO, Tech Lead, QA) phải là một subagent độc lập được spawn từ hệ thống. Điều phối viên (Orchestrator) không được phép tự đóng thế tất cả vai trò trong một response hoặc tự "bịa" ra sự đồng thuận/bất đồng giữa các bên.

## 2. When to Activate

### NÊN kích hoạt khi
Dùng skill này khi user muốn một team nhỏ cùng chốt hướng và tạo handoff đủ rõ cho execution ở bước sau.
- User đã có clarified intent từ `grill-me`.
- User muốn các vai trò cốt lõi (Product, Tech, QA) cùng phản biện một feature, change request, bug fix, UX flow, hoặc architecture direction.
- User nói các câu như: *"Mở party-mode để team chốt execution handoff"*, *"Cho các role họp rồi xuất handoff"*.

### KHÔNG kích hoạt khi
- Yêu cầu của user còn mơ hồ (Cần quay lại `grill-me` để làm rõ trước).
- User chỉ hỏi một câu factual đơn giản hoặc yêu cầu implement trực tiếp vào source code ngay lập tức.

## 3. Relationship With grill-me

unclear request → grill-me (clarifies intent) → user confirms
→ party-mode runs (subagent discussion & creates Execution Handoff)
→ party-mode returns summary → STOP.

- `grill-me` làm rõ ý định; `party-mode` chốt cách làm và tạo bàn giao.
- `party-mode` không restart lại quá trình khám phá yêu cầu từ đầu trừ khi phát hiện ambiguity thực sự blocking.

## 4. Subagent Execution Requirement

Mỗi role tranh luận phải được spawn như một subagent riêng nhận cùng một context summary ngắn gọn. Nếu runtime của `gemini-cli` không hỗ trợ hoặc không thể gọi subagent đúng cách, `party-mode` phải dừng lại ngay lập tức và báo lỗi:
> `BLOCKED: party-mode yêu cầu subagent độc lập cho từng role. Runtime hiện tại không hỗ trợ.`

## 5. Team Roster

### Default roles (The Core Triangle)
- **Product Owner (PO):** Đóng vai trò tích hợp của BA và Customer Advocate. Chuyển intent thành requirement, business rules, scope boundaries; đại diện cho end-user để bảo vệ giá trị sử dụng thật; đồng thời ưu tiên Must-have/Should-have và nghiêm ngặt chặn scope creep để bảo vệ kỷ luật MVP.
- **Technical Lead:** Review tính khả thi, kiến trúc, rủi ro tích hợp, technical trade-off. Bảo vệ tính thực tế kỹ thuật.
- **QA (Quality Analyst):** Xác định acceptance criteria, edge cases, rủi ro hồi quy (regression) và điều kiện nghiệm thu. Bảo vệ khả năng kiểm chứng.

*Lưu ý: Vai trò thư ký (Scribe/Secretary) sẽ do chính Orchestrator kiêm nhiệm để tối ưu token, không spawn subagent riêng.*

### Conditional roles (Chỉ dùng khi task thực sự cần)
- **UX / Workflow Designer:** Dùng khi task liên quan UI/UX, user journey, interaction flow phức tạp.
- **Risk / Ops Reviewer:** Dùng khi liên quan tới production data, database migration, permission, auth hoặc các thay đổi hệ thống nguy hiểm.

## 6. Role Selection Rules
Giới hạn số role subagent tham gia tranh luận từ **2 đến 5** tùy theo tính chất task:
- *Normal Feature:* PO, Tech Lead, QA.
- *Bug Fix:* Tech Lead, QA (có thể thêm PO nếu ảnh hưởng đến business logic gốc).
- *Architecture/Refactor:* PO, Tech Lead, QA, Risk/Ops.

## 7. Core Principles
- **Independent perspectives:** Mỗi role phát biểu từ chuyên môn riêng, không đồng ý chung chung. Đồng ý phải có lý do/điều kiện đi kèm.
- **Controlled disagreement:** Khuyến khích phản biện thẳng thắn nhưng có kiểm soát.
- **Context Discovery:** Ưu tiên đọc thụ động các file sẵn có (README, spec, source code liên quan) trước khi bắt user phải copy-paste lại nội dung.

## 8. Discussion Context Compression
Trước khi spawn các subagent, Orchestrator phải nén context thành một bản tóm tắt **dưới 400 từ** gồm: User intent, Desired outcome, Confirmed constraints, và Target area. Không đưa toàn bộ lịch sử chat dài dòng vào subagent.

## 9. Session Flow & Circuit Breaker

### Step 1: Orchestrator mở phiên họp
Nêu rõ clarified intent, mục tiêu cần chốt và danh sách các subagent role tham gia.

### Step 2: Initial Position Round
Mỗi debating subagent phát biểu ngắn gọn về: Quan điểm chính, lý do và điều kiện để role đó chấp thuận (approval conditions).

### Step 3: Orchestrator ghi nhận sơ bộ
Ghi nhanh các điểm đồng thuận và các điểm đang có xung đột (conflict) giữa các bên.

### Step 4: Challenge Round (Vòng phản biện & Cắt đuôi)
Các role đối thoại trực tiếp theo lượt thoại (turn-based).
- ⚡ **Cơ chế Cắt đuôi (Circuit Breaker):** Mỗi xung đột/issue chỉ được tranh luận **tối đa 2 lượt thoại** giữa các bên liên quan.
- Nếu sau 2 lượt vẫn không tìm được tiếng nói chung, **Orchestrator bắt buộc phải can thiệp**, cắt đuôi tranh luận, đánh dấu điểm đó là `BLOCKED` và đưa thẳng vào chat summary cuối để user chốt. Không tạo handoff ready nếu blocker đó ảnh hưởng trực tiếp đến implementation.

### Step 5: Orchestrator tổng hợp sau phản biện
Ghi lại những gì đã thống nhất, những trade-off và những điểm nghẽn (nếu có) cần User quyết định.

### Step 6: Team Decision Round
PO đưa ra hướng chốt tinh gọn cuối cùng (MVP). Tech Lead và QA xác nhận nhanh góc nhìn của mình dựa trên hướng chốt đó. Nếu còn điểm nghẽn không thể tự quyết, chuyển trạng thái phiên thành `BLOCKED`.

### Step 7: Xuất dữ liệu & Kiểm Tra Readiness Gate
Orchestrator tổng hợp thông tin và tạo `Execution Handoff` artifact. Hệ thống thực hiện checklist nhanh để đảm bảo handoff không bị mơ hồ về mặt kỹ thuật.
Readiness Gate chỉ PASS nếu handoff có đủ:
- final decision rõ, không còn option ngang hàng;
- target files/modules/areas cụ thể;
- in-scope/out-of-scope rõ;
- state/data contract đủ exact nếu có state;
- action/API guard đủ exact nếu có workflow/action;
- UI/backend contract đủ rõ nếu có UI/backend;
- acceptance criteria đo được;
- validation/evidence executor phải trả về;
- không còn blocker kỹ thuật khiến executor phải tự quyết.

### Step 8: Dừng hệ thống (Stop)
Trả chat summary ngắn cho user, tạo/lưu `Execution Handoff` artifact nếu READY, rồi kết thúc phiên.

## 10. Execution Handoff Artifact Template
*(Tài liệu này được xuất ra file `.md` riêng biệt, không dump toàn bộ vào màn hình chat)*
  Format:
  ```markdown
  # Execution Handoff

  ## 1. Objective
  [Mục tiêu implementation ngắn gọn từ 1-3 câu]

  ## 2. Final Decision
  [Quyết định cuối cùng đã chốt, không để các option ngang hàng]

  ## 3. Original Request Alignment
  - Thay đổi/Mở rộng so với yêu cầu gốc: ...
  - Thu hẹp/Trì hoãn (Đẩy vào phase sau): ...

  ## 4. Implementation Scope
  - **In Scope:** ...
  - **Out of Scope:** ...

  ## 5. Target Files / Areas To Inspect Or Modify
  | Area / File Path | Expected Work |
  |---|---|
  | `example/path/to/file` | [Mô tả cụ thể việc cần làm] |

  ## 6. Technical Contracts
  - **Data / State Contract:** exact fields, enum, casing, source of truth, transition rules, legacy mapping.
  - **Action / Guard Contract:** action name, enabled/disabled condition, request/payload, result, error behavior.
  - **UI / UX Contract:** visible state, primary action, empty/loading/error states, labels/copy that must appear.
  - **API / Backend Contract:** endpoint/function/store affected, persistence behavior, event/SSE behavior if any.

  ## 7. Execution Plan For Future Executor
  1. Inspect ...
  2. Implement ...
  3. Add tests & validate ...

  ## 8. Verification & Risks
  - **Acceptance Criteria:** [Checklist có thể đo đạc, kiểm chứng được]
  - **Required Tests:** [Command chạy test cụ thể hoặc phân vùng cần test]
  - **Regression Risks:** [Rủi ro phá vỡ tính năng cũ]
  - **Evidence Required:** [Bằng chứng executor tương lai phải nộp: ảnh, log, test output]

  ## 9. Do Not Do
  - [Những điều cấm executor bước sau tự ý thực hiện]
  ```

## 11. Final Chat Response Template
Khi Handoff READY:
Đã tạo `Execution Handoff` artifact thành công.
- Final decision: [Tóm tắt chốt cuối]
- Scope: [READY]
- Artifact Path: `<path_to_file>`

Khi Handoff BLOCKED:
Chưa thể tạo `Execution Handoff` đạt chuẩn.
- Status: BLOCKED
- Lý do & Thiếu quyết định: [Nêu rõ điểm xung đột chưa giải quyết sau Challenge Round]
- Cần user chốt: [Câu hỏi lựa chọn cho user]
- Artifact Path: None hoặc `<path_to_blocked_log>`