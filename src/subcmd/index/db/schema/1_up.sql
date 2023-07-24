-- NB: sqlite i64 means token atomics field get wonky comparisons when amount >9M SOL.
-- If a transaction contains multiple stakedex program invocations, multiple rows with same sig will be created.
CREATE TABLE IF NOT EXISTS invocations (
    sig TEXT NOT NULl, -- transaction signature
    signer TEXT NOT NULL, -- note: not transaction fee payer. stakedex instruction signer (e.g. user of SwapViaStake)
    ix INTEGER NOT NULL, -- instruction discriminant
    unix_timestamp INTEGER NOT NULL, -- block timestamp
    cpi_prog TEXT NOT NULL DEFAULT '', -- program id of program that CPI'd stakedex program. "" if NA
    amount_in INTEGER NOT NULL DEFAULT 0, -- amount of token atomics in, according to instruction data. 0 if NA. Always able to determine from ix data.
    amount_out INTEGER NOT NULL DEFAULT 0, -- amount of token atomics out received by destination token account. 0 if NA or unable to determine.
    mint_in TEXT NOT NULL DEFAULT '', -- mint of input token. "" if NA
    mint_out TEXT NOT NULL DEFAULT '' -- mint of output token. "" if NA
);
