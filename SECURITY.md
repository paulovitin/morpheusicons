# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 0.1.x   | ✅ Current          |

## Reporting a Vulnerability

If you discover a security vulnerability in MorpheusIcons, please report it responsibly.

**Do NOT open a public GitHub issue for security vulnerabilities.**

Instead, please email:

📧 **paulo@example.com**

### What to include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if you have one)

### Response timeline

- **Acknowledgment:** within 48 hours
- **Initial assessment:** within 7 days
- **Fix & disclosure:** coordinated with reporter, typically within 30 days

### Scope

MorpheusIcons is a client-side library that processes SVG path data. Security concerns most likely relate to:

- **SVG parsing** — malformed input causing panics, infinite loops, or excessive memory allocation
- **WASM builds** — potential issues in the browser context
- **Dependency supply chain** — compromised optional dependencies

### Out of Scope

- Issues in upstream framework crates (gpui, egui, iced, etc.) — report those to their respective maintainers
- Denial of service via intentionally large inputs (we accept this as a known limitation for untrusted input)

## Recognition

We appreciate responsible disclosure and will credit reporters in release notes (unless anonymity is preferred).
