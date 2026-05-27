# Note Quality Matrix

Dùng matrix này khi review continuity notes được tạo theo hướng dẫn `project-notes` đã cập nhật.

Score:

- 0 = thiếu, gây hại, misleading, hoặc chỉ là placeholder.
- 1 = có nhưng rộng, mơ hồ, khó recall, hoặc yếu về actionability.
- 2 = rõ, cụ thể, bền vững, và hữu ích cho future `pnotes brief`.

## Metadata Quality

- `id` / `task`: slug-safe, stable, CLI-friendly; không có spaces, human title nhiều punctuation, quotes, `/`, `:`, hoặc `&`.
- `signal`: summary ngắn, human-readable, nêu project memory có thể reuse.
- `areas`: repo paths đủ cụ thể để recall; không dùng broad labels hoặc comma-joined strings.
- `tags`: relevant và không thay thế cho `areas` cụ thể.
- `decisions`: mỗi item là một durable choice; không phải implementation summary.
- `invariants`: behavior hoặc contract mà change sau này phải giữ.
- `risks`: explicit failure mode hoặc fragility; mitigation alone không đủ.
- `tests`: commands được tách khi bảo vệ behavior khác nhau; `covers` non-empty và behavior-focused.
- `missing_tests`: known coverage gaps explicit khi applicable.
- `supersedes`: chỉ dùng exact note ids.

## Body Quality

- `deviations`: khác biệt meaningful so với handoff/plan, hoặc `None` chỉ khi đúng.
- `traps`: capture non-obvious async, rollback, lifecycle, parser, cache, integration, timing, hoặc hidden-contract behavior; không ghi `None` khi behavior đó tồn tại.
- `dead_ends`: ghi approach đã loại bỏ khi hữu ích cho agent sau, hoặc `None` chỉ khi đúng.
- `validation_delta`: ghi đúng expected validation, not-run reason, hoặc validation difference.
- `next_agent_hint`: guidance cụ thể, high-signal; tránh `See Handoff` khi có reusable context.

## Formatting Quality

- Readable multiline Markdown.
- YAML frontmatter valid/readable.
- YAML lists diff-friendly.
- Body sections được tách bằng blank lines.
- Không serialize frontmatter/body thành một dòng dài.

## Anti-Garbage Quality

- Không duplicate changelog, file list, hoặc generic implementation summary.
- Không invent decisions, risks, test coverage, hoặc validation.
- Không chứa secrets hoặc sensitive data.
- Hữu ích cho future `pnotes brief`, không chỉ tồn tại để complete.

## Recommendation

- Keep as-is.
- Amend mechanical formatting only.
- Create follow-up note.
- Update skill/pnotes rule.
- Improve CLI support.
