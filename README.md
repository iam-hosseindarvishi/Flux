# Flux Agent Kit

این بسته شامل فایل‌های پایه برای کار با Antigravity/Gemini است.

- `AGENTS.md`: قوانین دائمی پروژه
- `master_prompt.md`: پرامپت اصلی ایجنت
- `tasks/`: تعریف کامل هر تسک
- `tasks.json`: رجیستری وضعیت تسک‌ها
- `scripts/task-status.mjs`: ابزار امن تغییر وضعیت
- `.gitignore`: فایل‌های محلی و حساس که نباید commit شوند

اجرای وضعیت‌ها:

```bash
node scripts/task-status.mjs show
node scripts/task-status.mjs start 001
node scripts/task-status.mjs complete 001
```
