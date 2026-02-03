# Security Policy

## Reported Vulnerabilities

If you discover a security vulnerability in Rust Koans, please email security@example.com instead of using the issue tracker.

## Security Considerations

Rust Koans is an educational project designed to teach Rust programming. While the project itself doesn't handle sensitive data, here are some best practices:

### For Contributors

- ✅ Never commit sensitive information (API keys, tokens, passwords)
- ✅ Use .gitignore to exclude local configuration files
- ✅ Review code for security vulnerabilities before submitting
- ✅ Keep dependencies up to date

### For Users

- ✅ These koans run locally on your machine
- ✅ No data is sent to external servers
- ✅ Review code before running if concerned
- ✅ Use trusted Rust versions from official sources

## Dependency Security

We use `cargo-audit` to check for known vulnerabilities:

```bash
cargo install cargo-audit
cargo audit
```

## Updates

This project recommends:

- Staying on latest stable Rust (1.75+)
- Keeping VS Code extensions updated
- Reviewing GitHub security advisories

## Reporting

If you find a security issue:

1. **Do NOT open a public issue**
2. Email details to your security contact
3. Include:
   - Description of vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

## Acknowledgments

We thank security researchers who help improve this project.

---

**Last Updated**: 2026-02-03
