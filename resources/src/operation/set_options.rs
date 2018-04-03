use asset::Flag;

/// Use “Set Options” operation to set following options to your account:
///
/// Set/clear account flags:
/// AUTH_REQUIRED_FLAG (0x1) - if set, TrustLines are created with authorized set to false
/// requiring the issuer to set it for each TrustLine.
/// AUTH_REVOCABLE_FLAG (0x2) - if set, the authorized flag in TrustLines can be cleared.
/// Otherwise, authorization cannot be revoked.
/// Set the account’s inflation destination.
/// Add new signers to the account.
/// Set home domain.
#[derive(Debug, Clone)]
pub struct SetOptions {
    signer_key: String,
    signer_weight: u8,
    master_key_weight: u8,
    low_threshold: u32,
    med_threshold: u32,
    high_threshold: u32,
    home_domain: String,
    set_flags: Option<Flag>,
    clear_flags: Option<Flag>,
}

impl SetOptions {
    /// Creates a new SetOptions
    pub fn new(
        signer_key: String,
        signer_weight: u8,
        master_key_weight: u8,
        low_threshold: u32,
        med_threshold: u32,
        high_threshold: u32,
        home_domain: String,
        set_flags: Option<Flag>,
        clear_flags: Option<Flag>,
    ) -> SetOptions {
        SetOptions {
            signer_key: signer_key,
            signer_weight: signer_weight,
            master_key_weight: master_key_weight,
            low_threshold: low_threshold,
            med_threshold: med_threshold,
            high_threshold: high_threshold,
            home_domain: home_domain,
            set_flags: set_flags,
            clear_flags: clear_flags,
        }
    }

    /// The public key of the new signer.
    pub fn signer_key<'a>(&'a self) -> &'a str {
        &self.signer_key
    }

    /// The weight of the new signer (1-255).
    pub fn signer_weight(&self) -> u8 {
        self.signer_weight
    }

    /// The weight of the master key (1-255).
    pub fn master_key_weight(&self) -> u8 {
        self.master_key_weight
    }

    /// The sum weight for the low threshold.
    pub fn low_threshold(&self) -> u32 {
        self.low_threshold
    }

    /// The sum weight for the medium threshold.
    pub fn med_threshold(&self) -> u32 {
        self.med_threshold
    }

    /// The sum weight for the high threshold.
    pub fn high_threshold(&self) -> u32 {
        self.high_threshold
    }

    /// The home domain used for reverse federation lookup
    pub fn home_domain<'a>(&'a self) -> &'a str {
        &self.home_domain
    }

    /// The flags that have been set in this operation
    pub fn set_flags(&self) -> Option<Flag> {
        self.set_flags
    }

    /// The flags that have been cleared in this operation
    pub fn clear_flags(&self) -> Option<Flag> {
        self.clear_flags
    }
}
