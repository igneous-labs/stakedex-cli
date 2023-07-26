/*
/// Slot where mainnet stakedex program started becoming active
pub const EARLIEST_SLOT: u64 = 203_129_826;

/// Transaction in slot EARLIEST_SLOT that marks when mainnet stakedex program started becoming active
pub const EALIEST_SIGNATURE: &str =
    "3xFLbx6aBqeAm8z8ZY8gQbx5PFX8gVU1FJbUVvy7Eo4q6bxCyxRTBcG7FU4mRi7qiTQ8KXpCZvQWaozADvmKTmNm";

/// Slot where payer was removed
pub const PAYER_REMOVED_SLOT: u64 = 206_388_868;

/// Transaction in slot PAYER_REMOVED_SLOT where program was updated
pub const PAYER_REMOVED_SIGNATURE: &str =
    "4csG1SzE9WZepZdCv1dXfQaD4SAQ7mZXHjBJizP2hZLLkqnVprpq3tPAKrqTBaYEFbJJFx2oSEXTmSwrsjvyLbYX";

pub const FIRST_NON_ADMIN_SLOT_SINCE_PAYER_REMOVED: u64 = 206_389_107;
*/

pub const FIRST_NON_ADMIN_SIGNATURE_SINCE_PAYER_REMOVED: &str =
    "2scJfbaU4VbPDiiLqPB9NGT9U6LWXcxNRW2zw2qRNoKUR2UqmeiYJZ18VzmohBMxtrVwyyd6rGPi6VCrRN2SpFrs";

/// RPC limit on `limit` paramter for getSignaturesForAddress
pub const MAX_SIGNATURES_FOR_ADDRESS_LIMIT: usize = 1_000;
