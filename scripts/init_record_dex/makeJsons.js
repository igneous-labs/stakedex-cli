/**
 * Create and save the initial batch of known stakedexes to file
 */

const fs = require("fs");

const DEST_DIR = `${__dirname}/jsons`;

const WITHDRAW_STAKE = [
  {
    mint: "7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj",
    mainAccount: "49Yi1TKkNyYjPAFdR9LBvoHcUjuPX4Df5T5yv39w2XTn",
    ty: "Lido"
  }
];

const DEPOSIT_STAKE = [
  {
    mint: "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So",
    mainAccount: "8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC",
    ty: "Marinade"
  },
  {
    mint: "So11111111111111111111111111111111111111112",
    mainAccount: "3rBnnH9TTgd3xwu48rnzGsaQkSr1hR64nY71DrDt6VrQ",
    ty: "Unstakeit"
  }
];

const DEPOSIT_WITHDRAW_STAKE = [
  {
    mint: "Hg35Vd8K3BS2pLB3xwC2WqQV8pmpCm3oNRGYP1PEpmCM",
    mainAccount: "GUAMR8ciiaijraJeLDEDrFVaueLm9YzWWY9R7CBPL9rA",
    ty: "Eversol"
  },
  {
    mint: "5oVNBeEEQvYi1cX3ir8Dx5n1P7pdxydbGF2X4TxVusJm",
    mainAccount: "5oc4nmbNTda9fx8Tw57ShLD132aqDK65vuHH4RU1K4LZ",
    ty: "Socean"
  },
  {
    mint: "GEJpt3Wjmr628FqXxTgxMce1pLntcPV4uFi8ksxMyPQh",
    mainAccount: "7ge2xKsZXmqPxa3YmXxXmzCp9Hc2ezrTxh6PECaxCwrL",
    ty: "Spl"
  },
  {
    mint: "J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn",
    mainAccount: "Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb",
    ty: "Spl"
  },
  {
    mint: "7Q2afV64in6N6SeZsAAB81TJzwDoD6zpqmHkzi9Dcavn",
    mainAccount: "CtMyWsrUtAwXWiGr9WjHT5fC3p3fgV8cyGpLTo2LJzG1",
    ty: "Spl"
  },
  {
    mint: "LAinEtNLgpmCP9Rvsf5Hn8W6EhNiKLZQti1xfWMLy6X",
    mainAccount: "2qyEeSAWKfU18AFthrF7JA8z8ZCi1yt76Tqs917vwQTV",
    ty: "Spl"
  },
  {
    mint: "CgnTSoL3DgY9SFHxcLj6CgCgKKoTBr6tp4CPAEWy25DE",
    mainAccount: "CgntPoLka5pD5fesJYhGmUCF8KU1QS1ZmZiuAuMZr2az",
    ty: "Spl"
  },
]

function makeDepositSolJsons() {
  // everything but unstakeit
  const allDepositSols = [
    ...WITHDRAW_STAKE,
    DEPOSIT_STAKE[0],
    ...DEPOSIT_WITHDRAW_STAKE,
  ];
  allDepositSols.forEach((depositSol, i) => {
    fs.writeFileSync(
      `${DEST_DIR}/depositSol${i}.json`,
      JSON.stringify(depositSol, undefined, 2),
    );
  })
}

function makeOneWayPoolPairJsons() {
  let i = 0;
  const depositStakes = [
    ...DEPOSIT_STAKE,
    ...DEPOSIT_WITHDRAW_STAKE,
  ]
  for (const withdrawStake of WITHDRAW_STAKE) {
    for (const depositStake of depositStakes) {
      fs.writeFileSync(
        `${DEST_DIR}/oneWayPoolPair${i}.json`,
        JSON.stringify({
          withdrawStakeTy: withdrawStake.ty,
          depositStakeTy: depositStake.ty,
          withdrawStakeMint: withdrawStake.mint,
          depositStakeMint: depositStake.mint,
          withdrawStakeMainAccount: withdrawStake.mainAccount,
          depositStakeMainAccount: depositStake.mainAccount,
        }, undefined, 2),
      );
      i += 1;
    }
  }
  // all the WITHDRAW_STAKE have been paired above
  for (const depositStake of DEPOSIT_STAKE) {
    for (const withdrawStake of DEPOSIT_WITHDRAW_STAKE) {
      fs.writeFileSync(
        `${DEST_DIR}/oneWayPoolPair${i}.json`,
        JSON.stringify({
          withdrawStakeTy: withdrawStake.ty,
          depositStakeTy: depositStake.ty,
          withdrawStakeMint: withdrawStake.mint,
          depositStakeMint: depositStake.mint,
          withdrawStakeMainAccount: withdrawStake.mainAccount,
          depositStakeMainAccount: depositStake.mainAccount,
        }, undefined, 2),
      );
      i += 1;
    }
  }
}

function makeTwoWayPoolPairJsons() {
  let i = 0;
  for (let j = 0; j < DEPOSIT_WITHDRAW_STAKE.length - 1; j++) {
    for (let k = j + 1; k < DEPOSIT_WITHDRAW_STAKE.length; k++) {
      const a = DEPOSIT_WITHDRAW_STAKE[j];
      const b = DEPOSIT_WITHDRAW_STAKE[k];
      fs.writeFileSync(
        `${DEST_DIR}/twoWayPoolPair${i}.json`,
        JSON.stringify({
          aTy: a.ty,
          bTy: b.ty,
          aMint: a.mint,
          bMint: b.mint,
          aMainAccount: a.mainAccount,
          bMainAccount: b.mainAccount,
        }, undefined, 2),
      );
      i += 1;
    }
  }
}

makeDepositSolJsons();
makeOneWayPoolPairJsons();
makeTwoWayPoolPairJsons();
