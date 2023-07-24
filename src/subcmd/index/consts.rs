/// Slot where mainnet stakedex program started becoming active
pub const EARLIEST_SLOT: u64 = 203_129_826;

/// Transaction in slot EARLIEST_SLOT that marks when mainnet stakedex program started becoming active
pub const EALIEST_SIGNATURE: &str =
    "3xFLbx6aBqeAm8z8ZY8gQbx5PFX8gVU1FJbUVvy7Eo4q6bxCyxRTBcG7FU4mRi7qiTQ8KXpCZvQWaozADvmKTmNm";

/// RPC limit on `limit` paramter for getSignaturesForAddress
pub const MAX_SIGNATURES_FOR_ADDRESS_LIMIT: usize = 1_000;
