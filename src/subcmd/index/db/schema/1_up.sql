-- NB: sqlite i64 means token atomics field get wonky comparisons when amount >9M SOL.
-- If a transaction contains multiple stakedex program invocations, multiple rows with same sig will be created.
CREATE TABLE IF NOT EXISTS invocations (
    sig TEXT NOT NULl, -- transaction signature
    signer TEXT NOT NULL, -- transaction payer
    ix INTEGER NOT NULL, -- instruction discriminant
    cpi_prog TEXT NOT NULL DEFAULT '', -- program id of program that CPI'd stakedex program. "" if NA
    amount_in INTEGER NOT NULL DEFAULT 0, -- amount of token atomics in. 0 if NA
    amount_out INTEGER NOT NULL DEFAULT 0, -- amount of token atomics out. 0 if NA
    mint_in TEXT NOT NULL DEFAULT '', -- mint of input token. "" if NA
    mint_out TEXT NOT NULL DEFAULT '' -- mint of output token. "" if NA
);
