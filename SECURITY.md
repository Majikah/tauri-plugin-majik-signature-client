# Security Policy

This policy covers the Tauri v2 plugin and its webview-to-Rust IPC boundary. It does not replace the security policies of the Rust client, cryptographic core or host application.

## Supported versions

| Version | Status |
| --- | --- |
| 1.6.x | Current supported release line; use the latest patch |
| Earlier versions | Upgrade to the latest supported release |

This project is independently maintained. Security response and fixes are best-effort, not a contractual SLA.

## Reporting a vulnerability

Please do not disclose exploitable issues in public GitHub issues. Use GitHub Private Vulnerability Reporting:

- https://github.com/Majikah/tauri-plugin-majik-signature-client/security/advisories/new
- Or email business@majikah.solutions with SECURITY in the subject.

Include the affected plugin version, host OS, Tauri version, relevant command/event, reproducible steps and potential impact. Redact private keys, mnemonic phrases, passphrases, personal files and production database contents.

## Security boundary and assumptions

- The webview-to-Rust command interface is an untrusted input boundary. TypeScript checks provide fast feedback only; the Rust plugin repeats validation before processing native paths.
- Native paths are interpreted with the host process's filesystem privileges. The plugin does not implement a filesystem sandbox or grant/revoke filesystem capabilities. Host applications must configure Tauri capabilities and OS permissions to restrict what a webview can access.
- Signing keys and decrypted stamp material remain inside the Rust client process. The plugin serializes command inputs/results and forwards events; it does not expose private key bytes.
- Stream cancellation is cooperative and checked between bounded work steps. It cannot guarantee immediate interruption of OS file I/O or recover from the operating system terminating the process under severe memory pressure.
- Streaming MJKSMAP verification requires a trusted account/contact/address key. It intentionally does not enable self-reported-key verification.
- The plugin does not provide confidentiality. Signing and verification do not encrypt file content.
- The host must configure its shared SQLite connection securely, including applying a SQLCipher key before queries when SQLCipher is used.
- The default capability permission set enables the full command surface. Prefer individual command permissions where practical and expose only commands required by each webview.
- No independent third-party security audit or certification is claimed.

## Enforced IPC bounds

| Input | Limit |
| --- | --- |
| Streaming operation ID | 1–128 restricted ASCII bytes |
| Native path | 4,096 UTF-16 code units |
| Streaming chunk size | 64 KiB–64 MiB (matches Rust client 1.6.0) |
| Active or queued streaming operations | 4 |
| Native-path batch entries | 1–10,000 |
| Manifest/native path | 4,096 UTF-16 code units per value |
| MIME metadata | 1,024 UTF-16 code units |
| Aggregate batch path/MIME metadata | 8 MiB |
| MJKSMAP verification file | 64 MiB |
| Inline detached envelope | 16 MiB before base64 decode |

Optional maxBytes and maxBytesPerFile settings are enforced by the Rust streaming client against actual canonical input bytes. Omitting them does not impose an arbitrary total-size cap. These bounds reduce resource-exhaustion risk; they do not eliminate all denial-of-service risks.

## Test expectations

Plugin CI includes adapter-specific boundary tests for oversized and malformed IPC metadata, invalid paths, operation IDs, oversized chunk requests, excess batch counts and oversized inline envelopes. Cryptographic vectors and core parser/adversarial tests belong in the underlying Rust client and majik-signature repositories; this plugin validates the Tauri adapter boundary rather than duplicating those suites.
