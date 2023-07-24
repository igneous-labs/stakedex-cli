use rusqlite::Connection;

pub struct Invocation {
    pub sig: String,
    pub signer: String,
    pub ix: u8,
    pub unix_timestamp: i64,
    pub cpi_prog: String,
    pub amount_in: u64,
    pub amount_out: u64,
    pub mint_in: String,
    pub mint_out: String,
}

impl Invocation {
    pub fn save(&self, conn: &Connection) -> rusqlite::Result<()> {
        let mut stmt = conn.prepare_cached(
            "INSERT INTO invocations (sig, signer, ix, unix_timestamp, cpi_prog, amount_in, amount_out, mint_in, mint_out) VALUES (:sig, :signer, :ix, :unix_timestamp, :cpi_prog, :amount_in, :amount_out, :mint_in, :mint_out)"
        )?;
        stmt.execute(&[
            (":sig", &self.sig),
            (":signer", &self.signer),
            (":ix", &self.ix.to_string()),
            (":unix_timestamp", &self.unix_timestamp.to_string()),
            (":cpi_prog", &self.cpi_prog),
            (":amount_in", &self.amount_in.to_string()),
            (":amount_out", &self.amount_out.to_string()),
            (":mint_in", &self.mint_in.to_string()),
            (":mint_out", &self.mint_out.to_string()),
        ])?;
        Ok(())
    }
}
