# AGENTS.md

## Project
Flux is an open-source, cross-platform download manager inspired by IDM. It uses a Rust core with a Tauri desktop shell and a modern UI. The product priorities are reliable multi-connection downloads, resume support, browser integration, and smart organization of series and movies.

## Working rules

1. Read `master_prompt.md`, `docs/PRD.md` if present, and the relevant task file before editing code.
2. Work on exactly one task at a time unless the task explicitly requires a dependency.
3. Inspect the existing repository before choosing filenames, modules, libraries, or patterns. Do not overwrite working code without understanding it.
4. Prefer small, testable changes. Keep public APIs stable unless the task requires a change.
5. Do not invent completed work. A task is complete only when its acceptance criteria pass and the relevant tests/build/checks succeed.
6. Never commit secrets, API keys, cookies, tokens, downloaded media, build artifacts, or local environment files.
7. Keep UI copy simple and user-facing. Avoid fake technical branding and unnecessary jargon.
8. Use the design tokens instead of hard-coded colors in UI components. Preserve RTL support, keyboard navigation, and reduced-motion behavior.
9. Update task status only through `node scripts/task-status.mjs`. Do not edit completion state by hand.
10. Before reporting completion, run the narrowest relevant tests, then the full checks when practical, and summarize files changed, commands run, and any remaining risk.

## Definition of done

- The implementation matches the task acceptance criteria.
- Tests or validation exist for important behavior.
- No unrelated files or dependencies were changed.
- Documentation is updated when behavior, setup, or public interfaces change.
- The task registry is updated with the status command only after validation passes.
