## 2026-05-26 — continuity-note-quality-gate

Changed:
- Tighten hướng dẫn schema cho continuity note: `id`/`task` slug-safe, `areas` cụ thể, `decisions` bền vững, `risks` explicit, `tests.covers` theo behavior, `missing_tests` cho known gaps, `traps` không placeholder, và `next_agent_hint` cụ thể.
- Viết lại note quality gate hiện có để coi note là project memory cho future `pnotes brief`, không chỉ là completion checkbox.

Why:
- Một số note gần đây có file tồn tại nhưng vẫn dùng placeholder yếu, `areas` quá rộng, test coverage mơ hồ, và identifier khó dùng với CLI.

Expected effect:
- Continuity note sau này dễ recall hơn, an toàn hơn khi agent sau implement, và ít che mất missing coverage hoặc non-obvious contracts.

Risk:
- Checklist mới có thể tăng nhẹ friction nếu agent over-apply optional fields.

Follow-up review:
- Review sau 5-10 continuity notes mới.
- Quyết định: keep, amend, rollback, hoặc inconclusive.
