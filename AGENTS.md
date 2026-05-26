## Communication defaults

- Always reply in Vietnamese by default.
- Keep technical terms, code, commands, file paths, config keys, and error messages in their original form unless translation is explicitly requested.
- If the user writes in another language and explicitly asks for that language, follow the user's language for that response.
- Prefer concise, operational Vietnamese.
- Do not switch to English unless the user asks, the content is a verbatim quote, or keeping the original wording is technically important.

## 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

Before implementing:
- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them - don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

## 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

## 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

When editing existing code:
- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it - don't delete it.

When your changes create orphans:
- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.

The test: Every changed line should trace directly to the user's request.

## 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

Transform tasks into verifiable goals:
- "Add validation" → "Write tests for invalid inputs, then make them pass"
- "Fix the bug" → "Write a test that reproduces it, then make it pass"
- "Refactor X" → "Ensure tests pass before and after"

For multi-step tasks, state a brief plan:
```
1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]
```

Strong success criteria let you loop independently. Weak criteria ("make it work") require constant clarification.

---

**These guidelines are working if:** fewer unnecessary changes in diffs, fewer rewrites due to overcomplication, and clarifying questions come before implementation rather than after mistakes.

## Project Notes Completion Gate

For any task with implementation output, project notes are mandatory.

Implementation output includes changes to:

- source code
- tests
- config
- scripts
- migrations
- generated project files
- project behavior

Before starting implementation or broad source exploration, use project notes to recall relevant context:

    ./bin/pnotes brief --area <path> --limit 3

If `brief` is unavailable or returns no useful context, use:

    ./bin/pnotes recall --area <path> --limit 3

Before reporting a task as complete, the agent must do one of the following:

1. Create a continuity note:

       ./bin/pnotes add continuity ...

2. Explicitly state why no note was needed.

When a continuity note is created, inspect the generated file before final response.

The note must be readable multiline Markdown and should include high-signal frontmatter when applicable:

- `decisions`
- `invariants`
- `risks`
- `tests`
- `missing_tests`
- `supersedes`

Valid skip reasons:

- no code/config/test/script/behavior changed
- task was pure Q&A or read-only
- repository has no `./bin/pnotes` or `.project-notes/`
- user explicitly requested no note

Invalid skip reasons:

- task was small
- change was simple
- only a few lines changed
- final summary already explains it
- git diff is enough
- generated note was created but not inspected

Final response must include one line:

    Project notes: created <path>

or:

    Project notes: skipped — <valid reason>
