use serde::Deserialize;
use std::env;
use std::process::{Command, ExitCode};

#[derive(Deserialize)]
struct StsResponse {
    #[serde(rename = "Credentials")]
    credentials: Credentials,
}

#[derive(Deserialize)]
struct Credentials {
    #[serde(rename = "AccessKeyId")]
    access_key_id: String,
    #[serde(rename = "SecretAccessKey")]
    secret_access_key: String,
    #[serde(rename = "SessionToken")]
    session_token: String,
    #[serde(rename = "Expiration")]
    expiration: String,
}

fn get_mfa_arn() -> Result<String, String> {
    let output = Command::new("aws")
        .args(["iam", "list-mfa-devices", "--query", "MFADevices[].SerialNumber", "--output", "text"])
        .output()
        .map_err(|e| format!("Failed to run aws cli: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to list MFA devices: {stderr}"));
    }

    let arn = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if arn.is_empty() {
        return Err("No MFA devices found for this AWS user".to_string());
    }

    // If multiple devices, take the first one
    let arn = arn.lines().next().unwrap_or(&arn).trim().to_string();
    Ok(arn)
}

fn get_session_token(mfa_arn: &str, mfa_code: &str) -> Result<StsResponse, String> {
    let output = Command::new("aws")
        .args([
            "sts", "get-session-token",
            "--serial-number", mfa_arn,
            "--token-code", mfa_code,
        ])
        .output()
        .map_err(|e| format!("Failed to run aws cli: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to get session token: {stderr}"));
    }

    serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse STS response: {e}"))
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: aws-auth <mfa_code>");
        eprintln!();
        eprintln!("Authenticates with AWS using MFA and outputs export commands");
        eprintln!("for temporary session credentials.");
        eprintln!();
        eprintln!("  eval $(aws-auth 123456)");
        return ExitCode::FAILURE;
    }

    let mfa_code = &args[1];

    if mfa_code.len() != 6 || !mfa_code.chars().all(|c| c.is_ascii_digit()) {
        eprintln!("Error: MFA code must be 6 digits");
        return ExitCode::FAILURE;
    }

    eprintln!("Discovering MFA device...");
    let mfa_arn = match get_mfa_arn() {
        Ok(arn) => arn,
        Err(e) => {
            eprintln!("Error: {e}");
            return ExitCode::FAILURE;
        }
    };
    eprintln!("Using MFA device: {mfa_arn}");

    eprintln!("Requesting session token...");
    let response = match get_session_token(&mfa_arn, mfa_code) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {e}");
            return ExitCode::FAILURE;
        }
    };

    // Output export commands to stdout (eval-able)
    println!("export AWS_ACCESS_KEY_ID='{}'", response.credentials.access_key_id);
    println!("export AWS_SECRET_ACCESS_KEY='{}'", response.credentials.secret_access_key);
    println!("export AWS_SESSION_TOKEN='{}'", response.credentials.session_token);

    eprintln!("Authenticated. Session expires: {}", response.credentials.expiration);

    ExitCode::SUCCESS
}
