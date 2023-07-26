/*
/// Slot where mainnet stakedex program started becoming active
pub const EARLIEST_SLOT: u64 = 203_129_826;

/// Transaction in slot EARLIEST_SLOT that marks when mainnet stakedex program started becoming active
pub const EALIEST_SIGNATURE: &str =
    "3xFLbx6aBqeAm8z8ZY8gQbx5PFX8gVU1FJbUVvy7Eo4q6bxCyxRTBcG7FU4mRi7qiTQ8KXpCZvQWaozADvmKTmNm";

/// Slot where payer was removed
pub const PAYER_REMOVED_SLOT: u64 = 205_067_324;

/// Transaction in slot PAYER_REMOVED_SLOT where program was updated
pub const PAYER_REMOVED_SIGNATURE: &str =
    "tZL1zdBk5P8Q7V9m3qpZ8w8B5SgkSomAjSiEPm1tRKQJvNvaJPoPCougPF5JegAoEEnnZjMiuLzTryUUmSagxmG";

pub const FIRST_NON_ADMIN_SLOT_SINCE_PAYER_REMOVED: u64 = 205_076_752;
*/

pub const FIRST_NON_ADMIN_SIGNATURE_SINCE_PAYER_REMOVED: &str =
    "3w9f8YnD8G4ktry66qEYJFYmdSGiNviqdJ5CMv35hAhzXHE9Ub1pzWwTFvidnZ9bWgdPBWEgHfhM3ecmSGEwNASP";

/// RPC limit on `limit` paramter for getSignaturesForAddress
pub const MAX_SIGNATURES_FOR_ADDRESS_LIMIT: usize = 1_000;
