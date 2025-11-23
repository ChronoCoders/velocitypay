use anyhow::{Result, anyhow};
use sp_core::crypto::{AccountId32, Ss58Codec};

/// Validate SS58 wallet address format
pub fn validate_wallet_address(address: &str) -> Result<()> {
    // Attempt to parse as SS58 address
    AccountId32::from_ss58check(address)
        .map_err(|_| anyhow!("Invalid wallet address format. Must be a valid SS58 address."))?;

    Ok(())
}

/// Validate email format
pub fn validate_email(email: &str) -> Result<()> {
    // Basic email validation
    if !email.contains('@') || !email.contains('.') {
        return Err(anyhow!("Invalid email format"));
    }

    if email.len() < 5 || email.len() > 255 {
        return Err(anyhow!("Email must be between 5 and 255 characters"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("invalid").is_err());
        assert!(validate_email("@example.com").is_err());
    }
}
