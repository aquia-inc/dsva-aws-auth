# aws-auth

Simple CLI tool for AWS MFA session authentication. Automates the multi-step process of getting temporary session credentials via `aws sts get-session-token`.

## Prerequisites

- [AWS CLI](https://aws.amazon.com/cli/) installed and configured with your permanent access key
- An MFA device associated with your IAM user
- `jq` is **not** required — this tool handles JSON parsing natively

## Install

```bash
cargo install --path .
```

Or build manually:

```bash
cargo build --release
# binary is at ./target/release/aws-auth
```

## Usage

Add a shell function to your `~/.zshrc` or `~/.bashrc`:

```bash
aws-auth() { eval $(command aws-auth "$@"); }
```

Then authenticate with your 6-digit MFA code:

```bash
aws-auth 123456
```

This will:
1. Auto-detect your MFA device ARN via `aws iam list-mfa-devices`
2. Request temporary session credentials from AWS STS
3. Export `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, and `AWS_SESSION_TOKEN` into your current shell

Status messages go to stderr, export commands go to stdout — so `eval` captures only the exports.
