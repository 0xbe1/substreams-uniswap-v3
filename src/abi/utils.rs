use bigdecimal::{BigDecimal, Zero};

const WHITELIST_TOKENS: [&str; 20] = [
    "0x6b175474e89094c44da98b954eedeac495271d0f", // DAI
    "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48", // USDC
    "0xdac17f958d2ee523a2206206994597c13d831ec7", // USDT
    "0x0000000000085d4780b73119b644ae5ecd22b376", // TUSD
    "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599", // WBTC
    "0x5d3a536e4d6dbd6114cc1ead35777bab948e3643", // cDAI
    "0x39aa39c021dfbae8fac545936693ac917d5e7563", // cUSDC
    "0x86fadb80d8d2cff3c3680819e4da99c10232ba0f", // EBASE
    "0x57ab1ec28d129707052df4df418d58a2d46d5f51", // sUSD
    "0x9f8f72aa9304c8b593d555f12ef6589cc3a579a2", // MKR
    "0xc00e94cb662c3520282e6f5717214004a7f26888", // COMP
    "0x514910771af9ca656af840dff83e8264ecf986ca", // LINK
    "0xc011a73ee8576fb46f5e1c5751ca3b9fe0af2a6f", // SNX
    "0x0bc529c00c6401aef6d220be8c6ea1667f6ad93e", // YFI
    "0x111111111117dc0aa78b770fa6a738034120c302", // 1INCH
    "0xdf5e0e81dff6faf3a7e52ba697820c5e32d806a8", // yCurv
    "0x956f47f50a910163d8bf957cf5846d573e7f87ca", // FEI
    "0x7d1afa7b718fb893db30a3abc0cfc608aacfebb0", // MATIC
    "0x7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9", // AAVE
    "0xfe2e637202056d30016725477c5da089ab0a043a" // sETH2
];

const STABLE_COINS: [&str; 6] = [
    "0x6b175474e89094c44da98b954eedeac495271d0f", // DAI
    "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48", // USDC
    "0xdac17f958d2ee523a2206206994597c13d831ec7", // USDT
    "0x0000000000085d4780b73119b644ae5ecd22b376", // TUSD
    "0x956f47f50a910163d8bf957cf5846d573e7f87ca", // FEI
    "0x4dd28568d05f09b02220b09c2cb307bfd837cb95"  // PRINTS
];

fn zero_big_decimal() -> BigDecimal {
    BigDecimal::zero().with_prec(100)
}