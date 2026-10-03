# Browser Extension Security Contract

## Trust model
- Extension, desktop app, and browser are separate trust boundaries.
- URLs, headers, cookies, page metadata, and messages are untrusted input.
- The extension must never expose cookies, tokens, passwords, or full request headers to web pages.
- The desktop app must never accept download commands from arbitrary websites.

## Native Messaging
- Native Messaging is the default transport.
- The host manifest uses an exact extension ID allowlist.
- Messages are JSON objects with a `version`, `request_id`, `action`, and `payload`.
- Maximum message size: 1 MB.
- Allowed actions: `ping`, `get_status`, `inspect_media`, `add_download`, `cancel_request`.
- The app validates schema, URL scheme, filename, headers, and payload size before processing.
- Only `https://` and `http://` URLs are allowed by default. `file://`, `data:`, `javascript:`, localhost, private IPs, and cloud metadata IPs are blocked unless explicitly enabled.
- Cookies and authorization headers are forwarded only for the exact source origin and only after user consent.
- Never persist browser cookies or authorization headers after the download is created.
- Logs must redact URLs containing tokens, cookies, Authorization headers, and query parameters marked as secrets.

## WebSocket fallback
- Disabled by default and enabled only in Settings.
- Bind exclusively to `127.0.0.1` or `::1`, never `0.0.0.0`.
- Use a random 256-bit per-installation token stored in the OS credential store.
- Require token authentication before every session and rotate the token after reinstall or reset.
- Validate `Origin` against the exact configured extension origins, but do not treat Origin as the only authentication factor.
- Reject cross-origin, unauthenticated, malformed, oversized, replayed, or expired messages.
- Use a nonce, timestamp, and request_id. Reject timestamps older than 30 seconds and duplicate request IDs.
- Apply rate limits: 30 requests per minute per session, with stricter limits for `add_download`.
- Show a desktop consent prompt for the first request from every new browser profile.
- Never allow the WebSocket to read files, execute commands, change settings, or access arbitrary local paths.

## User consent
- `capture_all_downloads` is off by default.
- Single downloads require visible confirmation unless the user explicitly enabled an approved domain rule.
- Domain rules are exact-origin matches, never wildcard by default.
- Consent text must show domain, filename, size if known, destination category, and whether cookies/authentication will be used.
- The user can revoke a domain rule and all extension access from Settings.

## Download validation
- Normalize and validate URLs before enqueueing.
- Prevent SSRF: block loopback, link-local, private, multicast, broadcast, and cloud metadata ranges after DNS resolution.
- Re-check redirects and validate every redirect destination.
- Limit redirects to 10 and reject protocol downgrades from HTTPS to HTTP unless explicitly approved.
- Sanitize filenames and prevent path traversal, reserved device names, and control characters.
- The extension cannot choose an arbitrary destination path; the app resolves paths from trusted category rules.
- Never execute downloaded files automatically.

## Error handling and audit
- Fail closed on authentication, validation, consent, or schema errors.
- Return generic errors to the extension; keep sensitive details only in redacted local logs.
- Log security events: rejected origin, invalid token, blocked URL, consent decision, and revoked rule.
- Provide a local “Reset browser integration” action that revokes tokens, removes domain rules, and disables WebSocket fallback.
- Add automated tests for origin spoofing, token theft attempts, replay, malformed JSON, oversized payloads, SSRF, redirects, path traversal, and cookie leakage.
