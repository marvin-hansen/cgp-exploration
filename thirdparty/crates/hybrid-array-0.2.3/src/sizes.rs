//! Supported array sizes: [`typenum::Unsigned`] types with an [`ArraySize`] impl.
//!
//! We support the following array sizes by default:
//!
//! - 0-512
//! - 528-1024 (multiples of 16)
//! - 2048, 4096, 8192
//!
//! When the `extra-sizes` feature is enabled: 1040-4064 (multiples of 32)

use super::{ArraySize, AssocArraySize};

#[cfg(feature = "extra-sizes")]
pub use extra_sizes::*;

/// Implement the `ArraySize` and `AssocArraySize` traits for a given list of `N => UN, ...`
/// mappings.
///
/// `N` is used over `UN::USIZE` in order to improve compile times (avoids associated constant
/// resolution)
macro_rules! impl_array_sizes {
    ($($len:expr => $ty:ident),+ $(,)?) => {
        $(
            unsafe impl ArraySize for $ty {
                type ArrayType<T> = [T; $len];
            }

            impl<T> AssocArraySize for [T; $len] {
                type Size = $ty;
            }
        )+
     };
}

/// Implement array sizes, also importing the relevant constants.
macro_rules! impl_array_sizes_with_import {
    ($($len:expr => $ty:ident),+ $(,)?) => {
        $(
            pub use typenum::consts::$ty;
            impl_array_sizes!($len => $ty);
        )+
     };
}

impl_array_sizes_with_import! {
    0 => U0,
    1 => U1,
    2 => U2,
    3 => U3,
    4 => U4,
    5 => U5,
    6 => U6,
    7 => U7,
    8 => U8,
    9 => U9,
    10 => U10,
    11 => U11,
    12 => U12,
    13 => U13,
    14 => U14,
    15 => U15,
    16 => U16,
    17 => U17,
    18 => U18,
    19 => U19,
    20 => U20,
    21 => U21,
    22 => U22,
    23 => U23,
    24 => U24,
    25 => U25,
    26 => U26,
    27 => U27,
    28 => U28,
    29 => U29,
    30 => U30,
    31 => U31,
    32 => U32,
    33 => U33,
    34 => U34,
    35 => U35,
    36 => U36,
    37 => U37,
    38 => U38,
    39 => U39,
    40 => U40,
    41 => U41,
    42 => U42,
    43 => U43,
    44 => U44,
    45 => U45,
    46 => U46,
    47 => U47,
    48 => U48,
    49 => U49,
    50 => U50,
    51 => U51,
    52 => U52,
    53 => U53,
    54 => U54,
    55 => U55,
    56 => U56,
    57 => U57,
    58 => U58,
    59 => U59,
    60 => U60,
    61 => U61,
    62 => U62,
    63 => U63,
    64 => U64,
    65 => U65,
    66 => U66,
    67 => U67,
    68 => U68,
    69 => U69,
    70 => U70,
    71 => U71,
    72 => U72,
    73 => U73,
    74 => U74,
    75 => U75,
    76 => U76,
    77 => U77,
    78 => U78,
    79 => U79,
    80 => U80,
    81 => U81,
    82 => U82,
    83 => U83,
    84 => U84,
    85 => U85,
    86 => U86,
    87 => U87,
    88 => U88,
    89 => U89,
    90 => U90,
    91 => U91,
    92 => U92,
    93 => U93,
    94 => U94,
    95 => U95,
    96 => U96,
    97 => U97,
    98 => U98,
    99 => U99,
    100 => U100,
    101 => U101,
    102 => U102,
    103 => U103,
    104 => U104,
    105 => U105,
    106 => U106,
    107 => U107,
    108 => U108,
    109 => U109,
    110 => U110,
    111 => U111,
    112 => U112,
    113 => U113,
    114 => U114,
    115 => U115,
    116 => U116,
    117 => U117,
    118 => U118,
    119 => U119,
    120 => U120,
    121 => U121,
    122 => U122,
    123 => U123,
    124 => U124,
    125 => U125,
    126 => U126,
    127 => U127,
    128 => U128,
    129 => U129,
    130 => U130,
    131 => U131,
    132 => U132,
    133 => U133,
    134 => U134,
    135 => U135,
    136 => U136,
    137 => U137,
    138 => U138,
    139 => U139,
    140 => U140,
    141 => U141,
    142 => U142,
    143 => U143,
    144 => U144,
    145 => U145,
    146 => U146,
    147 => U147,
    148 => U148,
    149 => U149,
    150 => U150,
    151 => U151,
    152 => U152,
    153 => U153,
    154 => U154,
    155 => U155,
    156 => U156,
    157 => U157,
    158 => U158,
    159 => U159,
    160 => U160,
    161 => U161,
    162 => U162,
    163 => U163,
    164 => U164,
    165 => U165,
    166 => U166,
    167 => U167,
    168 => U168,
    169 => U169,
    170 => U170,
    171 => U171,
    172 => U172,
    173 => U173,
    174 => U174,
    175 => U175,
    176 => U176,
    177 => U177,
    178 => U178,
    179 => U179,
    180 => U180,
    181 => U181,
    182 => U182,
    183 => U183,
    184 => U184,
    185 => U185,
    186 => U186,
    187 => U187,
    188 => U188,
    189 => U189,
    190 => U190,
    191 => U191,
    192 => U192,
    193 => U193,
    194 => U194,
    195 => U195,
    196 => U196,
    197 => U197,
    198 => U198,
    199 => U199,
    200 => U200,
    201 => U201,
    202 => U202,
    203 => U203,
    204 => U204,
    205 => U205,
    206 => U206,
    207 => U207,
    208 => U208,
    209 => U209,
    210 => U210,
    211 => U211,
    212 => U212,
    213 => U213,
    214 => U214,
    215 => U215,
    216 => U216,
    217 => U217,
    218 => U218,
    219 => U219,
    220 => U220,
    221 => U221,
    222 => U222,
    223 => U223,
    224 => U224,
    225 => U225,
    226 => U226,
    227 => U227,
    228 => U228,
    229 => U229,
    230 => U230,
    231 => U231,
    232 => U232,
    233 => U233,
    234 => U234,
    235 => U235,
    236 => U236,
    237 => U237,
    238 => U238,
    239 => U239,
    240 => U240,
    241 => U241,
    242 => U242,
    243 => U243,
    244 => U244,
    245 => U245,
    246 => U246,
    247 => U247,
    248 => U248,
    249 => U249,
    250 => U250,
    251 => U251,
    252 => U252,
    253 => U253,
    254 => U254,
    255 => U255,
    256 => U256,
    257 => U257,
    258 => U258,
    259 => U259,
    260 => U260,
    261 => U261,
    262 => U262,
    263 => U263,
    264 => U264,
    265 => U265,
    266 => U266,
    267 => U267,
    268 => U268,
    269 => U269,
    270 => U270,
    271 => U271,
    272 => U272,
    273 => U273,
    274 => U274,
    275 => U275,
    276 => U276,
    277 => U277,
    278 => U278,
    279 => U279,
    280 => U280,
    281 => U281,
    282 => U282,
    283 => U283,
    284 => U284,
    285 => U285,
    286 => U286,
    287 => U287,
    288 => U288,
    289 => U289,
    290 => U290,
    291 => U291,
    292 => U292,
    293 => U293,
    294 => U294,
    295 => U295,
    296 => U296,
    297 => U297,
    298 => U298,
    299 => U299,
    300 => U300,
    301 => U301,
    302 => U302,
    303 => U303,
    304 => U304,
    305 => U305,
    306 => U306,
    307 => U307,
    308 => U308,
    309 => U309,
    310 => U310,
    311 => U311,
    312 => U312,
    313 => U313,
    314 => U314,
    315 => U315,
    316 => U316,
    317 => U317,
    318 => U318,
    319 => U319,
    320 => U320,
    321 => U321,
    322 => U322,
    323 => U323,
    324 => U324,
    325 => U325,
    326 => U326,
    327 => U327,
    328 => U328,
    329 => U329,
    330 => U330,
    331 => U331,
    332 => U332,
    333 => U333,
    334 => U334,
    335 => U335,
    336 => U336,
    337 => U337,
    338 => U338,
    339 => U339,
    340 => U340,
    341 => U341,
    342 => U342,
    343 => U343,
    344 => U344,
    345 => U345,
    346 => U346,
    347 => U347,
    348 => U348,
    349 => U349,
    350 => U350,
    351 => U351,
    352 => U352,
    353 => U353,
    354 => U354,
    355 => U355,
    356 => U356,
    357 => U357,
    358 => U358,
    359 => U359,
    360 => U360,
    361 => U361,
    362 => U362,
    363 => U363,
    364 => U364,
    365 => U365,
    366 => U366,
    367 => U367,
    368 => U368,
    369 => U369,
    370 => U370,
    371 => U371,
    372 => U372,
    373 => U373,
    374 => U374,
    375 => U375,
    376 => U376,
    377 => U377,
    378 => U378,
    379 => U379,
    380 => U380,
    381 => U381,
    382 => U382,
    383 => U383,
    384 => U384,
    385 => U385,
    386 => U386,
    387 => U387,
    388 => U388,
    389 => U389,
    390 => U390,
    391 => U391,
    392 => U392,
    393 => U393,
    394 => U394,
    395 => U395,
    396 => U396,
    397 => U397,
    398 => U398,
    399 => U399,
    400 => U400,
    401 => U401,
    402 => U402,
    403 => U403,
    404 => U404,
    405 => U405,
    406 => U406,
    407 => U407,
    408 => U408,
    409 => U409,
    410 => U410,
    411 => U411,
    412 => U412,
    413 => U413,
    414 => U414,
    415 => U415,
    416 => U416,
    417 => U417,
    418 => U418,
    419 => U419,
    420 => U420,
    421 => U421,
    422 => U422,
    423 => U423,
    424 => U424,
    425 => U425,
    426 => U426,
    427 => U427,
    428 => U428,
    429 => U429,
    430 => U430,
    431 => U431,
    432 => U432,
    433 => U433,
    434 => U434,
    435 => U435,
    436 => U436,
    437 => U437,
    438 => U438,
    439 => U439,
    440 => U440,
    441 => U441,
    442 => U442,
    443 => U443,
    444 => U444,
    445 => U445,
    446 => U446,
    447 => U447,
    448 => U448,
    449 => U449,
    450 => U450,
    451 => U451,
    452 => U452,
    453 => U453,
    454 => U454,
    455 => U455,
    456 => U456,
    457 => U457,
    458 => U458,
    459 => U459,
    460 => U460,
    461 => U461,
    462 => U462,
    463 => U463,
    464 => U464,
    465 => U465,
    466 => U466,
    467 => U467,
    468 => U468,
    469 => U469,
    470 => U470,
    471 => U471,
    472 => U472,
    473 => U473,
    474 => U474,
    475 => U475,
    476 => U476,
    477 => U477,
    478 => U478,
    479 => U479,
    480 => U480,
    481 => U481,
    482 => U482,
    483 => U483,
    484 => U484,
    485 => U485,
    486 => U486,
    487 => U487,
    488 => U488,
    489 => U489,
    490 => U490,
    491 => U491,
    492 => U492,
    493 => U493,
    494 => U494,
    495 => U495,
    496 => U496,
    497 => U497,
    498 => U498,
    499 => U499,
    500 => U500,
    501 => U501,
    502 => U502,
    503 => U503,
    504 => U504,
    505 => U505,
    506 => U506,
    507 => U507,
    508 => U508,
    509 => U509,
    510 => U510,
    511 => U511,
    512 => U512,
    528 => U528,
    544 => U544,
    560 => U560,
    576 => U576,
    592 => U592,
    608 => U608,
    624 => U624,
    640 => U640,
    656 => U656,
    672 => U672,
    688 => U688,
    704 => U704,
    720 => U720,
    736 => U736,
    752 => U752,
    768 => U768,
    784 => U784,
    800 => U800,
    816 => U816,
    832 => U832,
    848 => U848,
    864 => U864,
    880 => U880,
    896 => U896,
    912 => U912,
    928 => U928,
    944 => U944,
    960 => U960,
    976 => U976,
    992 => U992,
    1008 => U1008,
    1024 => U1024,
    2048 => U2048,
    4096 => U4096,
    8192 => U8192,
}

/// Additional typenum size aliases beyond what are normally provided.
///
/// These are defined using their component bits rather than `Add` to avoid conflicting impls.
#[cfg(feature = "extra-sizes")]
#[allow(missing_docs)]
mod extra_sizes {
    use super::{ArraySize, AssocArraySize};
    use typenum::{
        consts::{B0, B1},
        UInt, UTerm,
    };

    // This macro constructs a UInt type from a sequence of bits.  The bits are interpreted as the
    // little-endian representation of the integer in question.  For example, uint!(1 1 0 1 0 0 1) is
    // U75 (not U105).
    macro_rules! uint {
        () => { UTerm };
        (0 $($bs:tt)*) => { UInt< uint!($($bs)*), B0 > };
        (1 $($bs:tt)*) => { UInt< uint!($($bs)*), B1 > };
    }

    pub type U1040 = uint!(0 0 0 0 1 0 0 0 0 0 1);
    pub type U1056 = uint!(0 0 0 0 0 1 0 0 0 0 1);
    pub type U1072 = uint!(0 0 0 0 1 1 0 0 0 0 1);
    pub type U1088 = uint!(0 0 0 0 0 0 1 0 0 0 1);
    pub type U1104 = uint!(0 0 0 0 1 0 1 0 0 0 1);
    pub type U1120 = uint!(0 0 0 0 0 1 1 0 0 0 1);
    pub type U1136 = uint!(0 0 0 0 1 1 1 0 0 0 1);
    pub type U1152 = uint!(0 0 0 0 0 0 0 1 0 0 1);
    pub type U1168 = uint!(0 0 0 0 1 0 0 1 0 0 1);
    pub type U1184 = uint!(0 0 0 0 0 1 0 1 0 0 1);
    pub type U1200 = uint!(0 0 0 0 1 1 0 1 0 0 1);
    pub type U1216 = uint!(0 0 0 0 0 0 1 1 0 0 1);
    pub type U1232 = uint!(0 0 0 0 1 0 1 1 0 0 1);
    pub type U1248 = uint!(0 0 0 0 0 1 1 1 0 0 1);
    pub type U1264 = uint!(0 0 0 0 1 1 1 1 0 0 1);
    pub type U1280 = uint!(0 0 0 0 0 0 0 0 1 0 1);
    pub type U1296 = uint!(0 0 0 0 1 0 0 0 1 0 1);
    pub type U1312 = uint!(0 0 0 0 0 1 0 0 1 0 1);
    pub type U1328 = uint!(0 0 0 0 1 1 0 0 1 0 1);
    pub type U1344 = uint!(0 0 0 0 0 0 1 0 1 0 1);
    pub type U1360 = uint!(0 0 0 0 1 0 1 0 1 0 1);
    pub type U1376 = uint!(0 0 0 0 0 1 1 0 1 0 1);
    pub type U1392 = uint!(0 0 0 0 1 1 1 0 1 0 1);
    pub type U1408 = uint!(0 0 0 0 0 0 0 1 1 0 1);
    pub type U1424 = uint!(0 0 0 0 1 0 0 1 1 0 1);
    pub type U1440 = uint!(0 0 0 0 0 1 0 1 1 0 1);
    pub type U1456 = uint!(0 0 0 0 1 1 0 1 1 0 1);
    pub type U1472 = uint!(0 0 0 0 0 0 1 1 1 0 1);
    pub type U1488 = uint!(0 0 0 0 1 0 1 1 1 0 1);
    pub type U1504 = uint!(0 0 0 0 0 1 1 1 1 0 1);
    pub type U1520 = uint!(0 0 0 0 1 1 1 1 1 0 1);
    pub type U1536 = uint!(0 0 0 0 0 0 0 0 0 1 1);
    pub type U1552 = uint!(0 0 0 0 1 0 0 0 0 1 1);
    pub type U1568 = uint!(0 0 0 0 0 1 0 0 0 1 1);
    pub type U1584 = uint!(0 0 0 0 1 1 0 0 0 1 1);
    pub type U1600 = uint!(0 0 0 0 0 0 1 0 0 1 1);
    pub type U1616 = uint!(0 0 0 0 1 0 1 0 0 1 1);
    pub type U1632 = uint!(0 0 0 0 0 1 1 0 0 1 1);
    pub type U1648 = uint!(0 0 0 0 1 1 1 0 0 1 1);
    pub type U1664 = uint!(0 0 0 0 0 0 0 1 0 1 1);
    pub type U1680 = uint!(0 0 0 0 1 0 0 1 0 1 1);
    pub type U1696 = uint!(0 0 0 0 0 1 0 1 0 1 1);
    pub type U1712 = uint!(0 0 0 0 1 1 0 1 0 1 1);
    pub type U1728 = uint!(0 0 0 0 0 0 1 1 0 1 1);
    pub type U1744 = uint!(0 0 0 0 1 0 1 1 0 1 1);
    pub type U1760 = uint!(0 0 0 0 0 1 1 1 0 1 1);
    pub type U1776 = uint!(0 0 0 0 1 1 1 1 0 1 1);
    pub type U1792 = uint!(0 0 0 0 0 0 0 0 1 1 1);
    pub type U1808 = uint!(0 0 0 0 1 0 0 0 1 1 1);
    pub type U1824 = uint!(0 0 0 0 0 1 0 0 1 1 1);
    pub type U1840 = uint!(0 0 0 0 1 1 0 0 1 1 1);
    pub type U1856 = uint!(0 0 0 0 0 0 1 0 1 1 1);
    pub type U1872 = uint!(0 0 0 0 1 0 1 0 1 1 1);
    pub type U1888 = uint!(0 0 0 0 0 1 1 0 1 1 1);
    pub type U1904 = uint!(0 0 0 0 1 1 1 0 1 1 1);
    pub type U1920 = uint!(0 0 0 0 0 0 0 1 1 1 1);
    pub type U1936 = uint!(0 0 0 0 1 0 0 1 1 1 1);
    pub type U1952 = uint!(0 0 0 0 0 1 0 1 1 1 1);
    pub type U1968 = uint!(0 0 0 0 1 1 0 1 1 1 1);
    pub type U1984 = uint!(0 0 0 0 0 0 1 1 1 1 1);
    pub type U2000 = uint!(0 0 0 0 1 0 1 1 1 1 1);
    pub type U2016 = uint!(0 0 0 0 0 1 1 1 1 1 1);
    pub type U2032 = uint!(0 0 0 0 1 1 1 1 1 1 1);
    pub type U2064 = uint!(0 0 0 0 1 0 0 0 0 0 0 1);
    pub type U2080 = uint!(0 0 0 0 0 1 0 0 0 0 0 1);
    pub type U2096 = uint!(0 0 0 0 1 1 0 0 0 0 0 1);
    pub type U2112 = uint!(0 0 0 0 0 0 1 0 0 0 0 1);
    pub type U2128 = uint!(0 0 0 0 1 0 1 0 0 0 0 1);
    pub type U2144 = uint!(0 0 0 0 0 1 1 0 0 0 0 1);
    pub type U2160 = uint!(0 0 0 0 1 1 1 0 0 0 0 1);
    pub type U2176 = uint!(0 0 0 0 0 0 0 1 0 0 0 1);
    pub type U2192 = uint!(0 0 0 0 1 0 0 1 0 0 0 1);
    pub type U2208 = uint!(0 0 0 0 0 1 0 1 0 0 0 1);
    pub type U2224 = uint!(0 0 0 0 1 1 0 1 0 0 0 1);
    pub type U2240 = uint!(0 0 0 0 0 0 1 1 0 0 0 1);
    pub type U2256 = uint!(0 0 0 0 1 0 1 1 0 0 0 1);
    pub type U2272 = uint!(0 0 0 0 0 1 1 1 0 0 0 1);
    pub type U2288 = uint!(0 0 0 0 1 1 1 1 0 0 0 1);
    pub type U2304 = uint!(0 0 0 0 0 0 0 0 1 0 0 1);
    pub type U2320 = uint!(0 0 0 0 1 0 0 0 1 0 0 1);
    pub type U2336 = uint!(0 0 0 0 0 1 0 0 1 0 0 1);
    pub type U2352 = uint!(0 0 0 0 1 1 0 0 1 0 0 1);
    pub type U2368 = uint!(0 0 0 0 0 0 1 0 1 0 0 1);
    pub type U2384 = uint!(0 0 0 0 1 0 1 0 1 0 0 1);
    pub type U2400 = uint!(0 0 0 0 0 1 1 0 1 0 0 1);
    pub type U2416 = uint!(0 0 0 0 1 1 1 0 1 0 0 1);
    pub type U2432 = uint!(0 0 0 0 0 0 0 1 1 0 0 1);
    pub type U2448 = uint!(0 0 0 0 1 0 0 1 1 0 0 1);
    pub type U2464 = uint!(0 0 0 0 0 1 0 1 1 0 0 1);
    pub type U2480 = uint!(0 0 0 0 1 1 0 1 1 0 0 1);
    pub type U2496 = uint!(0 0 0 0 0 0 1 1 1 0 0 1);
    pub type U2512 = uint!(0 0 0 0 1 0 1 1 1 0 0 1);
    pub type U2528 = uint!(0 0 0 0 0 1 1 1 1 0 0 1);
    pub type U2544 = uint!(0 0 0 0 1 1 1 1 1 0 0 1);
    pub type U2560 = uint!(0 0 0 0 0 0 0 0 0 1 0 1);
    pub type U2576 = uint!(0 0 0 0 1 0 0 0 0 1 0 1);
    pub type U2592 = uint!(0 0 0 0 0 1 0 0 0 1 0 1);
    pub type U2608 = uint!(0 0 0 0 1 1 0 0 0 1 0 1);
    pub type U2624 = uint!(0 0 0 0 0 0 1 0 0 1 0 1);
    pub type U2640 = uint!(0 0 0 0 1 0 1 0 0 1 0 1);
    pub type U2656 = uint!(0 0 0 0 0 1 1 0 0 1 0 1);
    pub type U2672 = uint!(0 0 0 0 1 1 1 0 0 1 0 1);
    pub type U2688 = uint!(0 0 0 0 0 0 0 1 0 1 0 1);
    pub type U2704 = uint!(0 0 0 0 1 0 0 1 0 1 0 1);
    pub type U2720 = uint!(0 0 0 0 0 1 0 1 0 1 0 1);
    pub type U2736 = uint!(0 0 0 0 1 1 0 1 0 1 0 1);
    pub type U2752 = uint!(0 0 0 0 0 0 1 1 0 1 0 1);
    pub type U2768 = uint!(0 0 0 0 1 0 1 1 0 1 0 1);
    pub type U2784 = uint!(0 0 0 0 0 1 1 1 0 1 0 1);
    pub type U2800 = uint!(0 0 0 0 1 1 1 1 0 1 0 1);
    pub type U2816 = uint!(0 0 0 0 0 0 0 0 1 1 0 1);
    pub type U2832 = uint!(0 0 0 0 1 0 0 0 1 1 0 1);
    pub type U2848 = uint!(0 0 0 0 0 1 0 0 1 1 0 1);
    pub type U2864 = uint!(0 0 0 0 1 1 0 0 1 1 0 1);
    pub type U2880 = uint!(0 0 0 0 0 0 1 0 1 1 0 1);
    pub type U2896 = uint!(0 0 0 0 1 0 1 0 1 1 0 1);
    pub type U2912 = uint!(0 0 0 0 0 1 1 0 1 1 0 1);
    pub type U2928 = uint!(0 0 0 0 1 1 1 0 1 1 0 1);
    pub type U2944 = uint!(0 0 0 0 0 0 0 1 1 1 0 1);
    pub type U2960 = uint!(0 0 0 0 1 0 0 1 1 1 0 1);
    pub type U2976 = uint!(0 0 0 0 0 1 0 1 1 1 0 1);
    pub type U2992 = uint!(0 0 0 0 1 1 0 1 1 1 0 1);
    pub type U3008 = uint!(0 0 0 0 0 0 1 1 1 1 0 1);
    pub type U3024 = uint!(0 0 0 0 1 0 1 1 1 1 0 1);
    pub type U3040 = uint!(0 0 0 0 0 1 1 1 1 1 0 1);
    pub type U3056 = uint!(0 0 0 0 1 1 1 1 1 1 0 1);
    pub type U3072 = uint!(0 0 0 0 0 0 0 0 0 0 1 1);
    pub type U3088 = uint!(0 0 0 0 1 0 0 0 0 0 1 1);
    pub type U3104 = uint!(0 0 0 0 0 1 0 0 0 0 1 1);
    pub type U3120 = uint!(0 0 0 0 1 1 0 0 0 0 1 1);
    pub type U3136 = uint!(0 0 0 0 0 0 1 0 0 0 1 1);
    pub type U3152 = uint!(0 0 0 0 1 0 1 0 0 0 1 1);
    pub type U3168 = uint!(0 0 0 0 0 1 1 0 0 0 1 1);
    pub type U3184 = uint!(0 0 0 0 1 1 1 0 0 0 1 1);
    pub type U3200 = uint!(0 0 0 0 0 0 0 1 0 0 1 1);
    pub type U3216 = uint!(0 0 0 0 1 0 0 1 0 0 1 1);
    pub type U3232 = uint!(0 0 0 0 0 1 0 1 0 0 1 1);
    pub type U3248 = uint!(0 0 0 0 1 1 0 1 0 0 1 1);
    pub type U3264 = uint!(0 0 0 0 0 0 1 1 0 0 1 1);
    pub type U3280 = uint!(0 0 0 0 1 0 1 1 0 0 1 1);
    pub type U3296 = uint!(0 0 0 0 0 1 1 1 0 0 1 1);
    pub type U3312 = uint!(0 0 0 0 1 1 1 1 0 0 1 1);
    pub type U3328 = uint!(0 0 0 0 0 0 0 0 1 0 1 1);
    pub type U3344 = uint!(0 0 0 0 1 0 0 0 1 0 1 1);
    pub type U3360 = uint!(0 0 0 0 0 1 0 0 1 0 1 1);
    pub type U3376 = uint!(0 0 0 0 1 1 0 0 1 0 1 1);
    pub type U3392 = uint!(0 0 0 0 0 0 1 0 1 0 1 1);
    pub type U3408 = uint!(0 0 0 0 1 0 1 0 1 0 1 1);
    pub type U3424 = uint!(0 0 0 0 0 1 1 0 1 0 1 1);
    pub type U3440 = uint!(0 0 0 0 1 1 1 0 1 0 1 1);
    pub type U3456 = uint!(0 0 0 0 0 0 0 1 1 0 1 1);
    pub type U3472 = uint!(0 0 0 0 1 0 0 1 1 0 1 1);
    pub type U3488 = uint!(0 0 0 0 0 1 0 1 1 0 1 1);
    pub type U3504 = uint!(0 0 0 0 1 1 0 1 1 0 1 1);
    pub type U3520 = uint!(0 0 0 0 0 0 1 1 1 0 1 1);
    pub type U3536 = uint!(0 0 0 0 1 0 1 1 1 0 1 1);
    pub type U3552 = uint!(0 0 0 0 0 1 1 1 1 0 1 1);
    pub type U3568 = uint!(0 0 0 0 1 1 1 1 1 0 1 1);
    pub type U3584 = uint!(0 0 0 0 0 0 0 0 0 1 1 1);
    pub type U3600 = uint!(0 0 0 0 1 0 0 0 0 1 1 1);
    pub type U3616 = uint!(0 0 0 0 0 1 0 0 0 1 1 1);
    pub type U3632 = uint!(0 0 0 0 1 1 0 0 0 1 1 1);
    pub type U3648 = uint!(0 0 0 0 0 0 1 0 0 1 1 1);
    pub type U3664 = uint!(0 0 0 0 1 0 1 0 0 1 1 1);
    pub type U3680 = uint!(0 0 0 0 0 1 1 0 0 1 1 1);
    pub type U3696 = uint!(0 0 0 0 1 1 1 0 0 1 1 1);
    pub type U3712 = uint!(0 0 0 0 0 0 0 1 0 1 1 1);
    pub type U3728 = uint!(0 0 0 0 1 0 0 1 0 1 1 1);
    pub type U3744 = uint!(0 0 0 0 0 1 0 1 0 1 1 1);
    pub type U3760 = uint!(0 0 0 0 1 1 0 1 0 1 1 1);
    pub type U3776 = uint!(0 0 0 0 0 0 1 1 0 1 1 1);
    pub type U3792 = uint!(0 0 0 0 1 0 1 1 0 1 1 1);
    pub type U3808 = uint!(0 0 0 0 0 1 1 1 0 1 1 1);
    pub type U3824 = uint!(0 0 0 0 1 1 1 1 0 1 1 1);
    pub type U3840 = uint!(0 0 0 0 0 0 0 0 1 1 1 1);
    pub type U3856 = uint!(0 0 0 0 1 0 0 0 1 1 1 1);
    pub type U3872 = uint!(0 0 0 0 0 1 0 0 1 1 1 1);
    pub type U3888 = uint!(0 0 0 0 1 1 0 0 1 1 1 1);
    pub type U3904 = uint!(0 0 0 0 0 0 1 0 1 1 1 1);
    pub type U3920 = uint!(0 0 0 0 1 0 1 0 1 1 1 1);
    pub type U3936 = uint!(0 0 0 0 0 1 1 0 1 1 1 1);
    pub type U3952 = uint!(0 0 0 0 1 1 1 0 1 1 1 1);
    pub type U3968 = uint!(0 0 0 0 0 0 0 1 1 1 1 1);
    pub type U3984 = uint!(0 0 0 0 1 0 0 1 1 1 1 1);
    pub type U4000 = uint!(0 0 0 0 0 1 0 1 1 1 1 1);
    pub type U4016 = uint!(0 0 0 0 1 1 0 1 1 1 1 1);
    pub type U4032 = uint!(0 0 0 0 0 0 1 1 1 1 1 1);
    pub type U4048 = uint!(0 0 0 0 1 0 1 1 1 1 1 1);
    pub type U4064 = uint!(0 0 0 0 0 1 1 1 1 1 1 1);
    pub type U4080 = uint!(0 0 0 0 1 1 1 1 1 1 1 1);

    // ML-DSA sizes
    //
    // Includes the public key, private key, and signature sizes not covered elsewhere, as well as
    // some intermediate value sizes.
    //
    // U3293 is not required for ML-DSA, but was included in an early iteration of this section
    // (before the `ml_dsa` crate was created).  So it is included here for backward compatibility.
    pub type U2420 = uint!(0 0 1 0 1 1 1 0 1 0 0 1);
    pub type U3293 = uint!(1 0 1 1 1 0 1 1 0 0 1 1);
    pub type U3309 = uint!(1 0 1 1 0 1 1 1 0 0 1 1);
    pub type U4480 = uint!(0 0 0 0 0 0 0 1 1 0 0 0 1);
    pub type U4544 = uint!(0 0 0 0 0 0 1 1 1 0 0 0 1);
    pub type U4595 = uint!(1 1 0 0 1 1 1 1 1 0 0 0 1);
    pub type U4627 = uint!(1 1 0 0 1 0 0 0 0 1 0 0 1);
    pub type U4896 = uint!(0 0 0 0 0 1 0 0 1 1 0 0 1);

    // SLH-DSA sizes
    pub type U7856 = uint!(0 0 0 0 1 1 0 1 0 1 1 1 1);
    pub type U16224 = uint!(0 0 0 0 0 1 1 0 1 1 1 1 1 1);
    pub type U17088 = uint!(0 0 0 0 0 0 1 1 0 1 0 0 0 0 1);
    pub type U29792 = uint!(0 0 0 0 0 1 1 0 0 0 1 0 1 1 1);
    pub type U35664 = uint!(0 0 0 0 1 0 1 0 1 1 0 1 0 0 0 1);
    pub type U49856 = uint!(0 0 0 0 0 0 1 1 0 1 0 0 0 0 1 1);

    // Kemeleon ML-KEM Encoding sizes
    pub type U749 = uint!(1 0 1 1 0 1 1 1 0 1);
    pub type U781 = uint!(1 0 1 1 0 0 0 0 1 1);
    pub type U877 = uint!(1 0 1 1 0 1 1 0 1 1);
    pub type U1124 = uint!(0 0 1 0 0 1 1 0 0 0 1);
    pub type U1156 = uint!(0 0 1 0 0 0 0 1 0 0 1);
    pub type U1252 = uint!(0 0 1 0 0 1 1 1 0 0 1);
    pub type U1498 = uint!(0 1 0 1 1 0 1 1 1 0 1);
    pub type U1530 = uint!(0 1 0 1 1 1 1 1 1 0 1);
    pub type U1658 = uint!(0 1 0 1 1 1 1 0 0 1 1);

    // LMS sizes
    pub type U2047 = uint!(1 1 1 1 1 1 1 1 1 1 1);
    pub type U2180 = uint!(0 0 1 0 0 0 0 1 0 0 0 1);
    pub type U4292 = uint!(0 0 1 0 0 0 1 1 0 0 0 0 1);
    pub type U8516 = uint!(0 0 1 0 0 0 1 0 1 0 0 0 0 1);

    // FrodoKEM640 sizes

    pub type U9616 = uint!(0 0 0 0 1 0 0 1 1 0 1 0 0 1);
    pub type U19888 = uint!(0 0 0 0 1 1 0 1 1 0 1 1 0 0 1);
    pub type U9720 = uint!(0 0 0 1 1 1 1 1 1 0 1 0 0 1);
    pub type U9752 = uint!(0 0 0 1 1 0 0 0 0 1 1 0 0 1);

    // FrodoKEM976 sizes
    pub type U15632 = uint!(0 0 0 0 1 0 0 0 1 0 1 1 1 1);
    pub type U31296 = uint!(0 0 0 0 0 0 1 0 0 1 0 1 1 1 1);
    pub type U15744 = uint!(0 0 0 0 0 0 0 1 1 0 1 1 1 1);
    pub type U15792 = uint!(0 0 0 0 1 1 0 1 1 0 1 1 1 1);

    // FrodoKEM1344 sizes
    pub type U21520 = uint!(0 0 0 0 1 0 0 0 0 0 1 0 1 0 1);
    pub type U43088 = uint!(0 0 0 0 1 0 1 0 0 0 0 1 0 1 0 1);
    pub type U21632 = uint!(0 0 0 0 0 0 0 1 0 0 1 0 1 0 1);
    pub type U21696 = uint!(0 0 0 0 0 0 1 1 0 0 1 0 1 0 1);

    impl_array_sizes! {
        1040 => U1040,
        1056 => U1056,
        1072 => U1072,
        1088 => U1088,
        1104 => U1104,
        1120 => U1120,
        1136 => U1136,
        1152 => U1152,
        1168 => U1168,
        1184 => U1184,
        1200 => U1200,
        1216 => U1216,
        1232 => U1232,
        1248 => U1248,
        1264 => U1264,
        1280 => U1280,
        1296 => U1296,
        1312 => U1312,
        1328 => U1328,
        1344 => U1344,
        1360 => U1360,
        1376 => U1376,
        1392 => U1392,
        1408 => U1408,
        1424 => U1424,
        1440 => U1440,
        1456 => U1456,
        1472 => U1472,
        1488 => U1488,
        1504 => U1504,
        1520 => U1520,
        1536 => U1536,
        1552 => U1552,
        1568 => U1568,
        1584 => U1584,
        1600 => U1600,
        1616 => U1616,
        1632 => U1632,
        1648 => U1648,
        1664 => U1664,
        1680 => U1680,
        1696 => U1696,
        1712 => U1712,
        1728 => U1728,
        1744 => U1744,
        1760 => U1760,
        1776 => U1776,
        1792 => U1792,
        1808 => U1808,
        1824 => U1824,
        1840 => U1840,
        1856 => U1856,
        1872 => U1872,
        1888 => U1888,
        1904 => U1904,
        1920 => U1920,
        1936 => U1936,
        1952 => U1952,
        1968 => U1968,
        1984 => U1984,
        2000 => U2000,
        2016 => U2016,
        2032 => U2032,
        2064 => U2064,
        2080 => U2080,
        2096 => U2096,
        2112 => U2112,
        2128 => U2128,
        2144 => U2144,
        2160 => U2160,
        2176 => U2176,
        2192 => U2192,
        2208 => U2208,
        2224 => U2224,
        2240 => U2240,
        2256 => U2256,
        2272 => U2272,
        2288 => U2288,
        2304 => U2304,
        2320 => U2320,
        2336 => U2336,
        2352 => U2352,
        2368 => U2368,
        2384 => U2384,
        2400 => U2400,
        2416 => U2416,
        2432 => U2432,
        2448 => U2448,
        2464 => U2464,
        2480 => U2480,
        2496 => U2496,
        2512 => U2512,
        2528 => U2528,
        2544 => U2544,
        2560 => U2560,
        2576 => U2576,
        2592 => U2592,
        2608 => U2608,
        2624 => U2624,
        2640 => U2640,
        2656 => U2656,
        2672 => U2672,
        2688 => U2688,
        2704 => U2704,
        2720 => U2720,
        2736 => U2736,
        2752 => U2752,
        2768 => U2768,
        2784 => U2784,
        2800 => U2800,
        2816 => U2816,
        2832 => U2832,
        2848 => U2848,
        2864 => U2864,
        2880 => U2880,
        2896 => U2896,
        2912 => U2912,
        2928 => U2928,
        2944 => U2944,
        2960 => U2960,
        2976 => U2976,
        2992 => U2992,
        3008 => U3008,
        3024 => U3024,
        3040 => U3040,
        3056 => U3056,
        3072 => U3072,
        3088 => U3088,
        3104 => U3104,
        3120 => U3120,
        3136 => U3136,
        3152 => U3152,
        3168 => U3168,
        3184 => U3184,
        3200 => U3200,
        3216 => U3216,
        3232 => U3232,
        3248 => U3248,
        3264 => U3264,
        3280 => U3280,
        3296 => U3296,
        3312 => U3312,
        3328 => U3328,
        3344 => U3344,
        3360 => U3360,
        3376 => U3376,
        3392 => U3392,
        3408 => U3408,
        3424 => U3424,
        3440 => U3440,
        3456 => U3456,
        3472 => U3472,
        3488 => U3488,
        3504 => U3504,
        3520 => U3520,
        3536 => U3536,
        3552 => U3552,
        3568 => U3568,
        3584 => U3584,
        3600 => U3600,
        3616 => U3616,
        3632 => U3632,
        3648 => U3648,
        3664 => U3664,
        3680 => U3680,
        3696 => U3696,
        3712 => U3712,
        3728 => U3728,
        3744 => U3744,
        3760 => U3760,
        3776 => U3776,
        3792 => U3792,
        3808 => U3808,
        3824 => U3824,
        3840 => U3840,
        3856 => U3856,
        3872 => U3872,
        3888 => U3888,
        3904 => U3904,
        3920 => U3920,
        3936 => U3936,
        3952 => U3952,
        3968 => U3968,
        3984 => U3984,
        4000 => U4000,
        4016 => U4016,
        4032 => U4032,
        4048 => U4048,
        4064 => U4064,
        4080 => U4080,
    }

    // ML-DSA sizes
    impl_array_sizes! {
        2420 => U2420,
        3293 => U3293,
        3309 => U3309,
        4480 => U4480,
        4544 => U4544,
        4595 => U4595,
        4627 => U4627,
        4896 => U4896,
    }

    // SLH-DSA sizes
    impl_array_sizes! {
        7856 => U7856,
        16224 => U16224,
        17088 => U17088,
        29792 => U29792,
        35664 => U35664,
        49856 => U49856,
    }

    // Kemeleon ML-KEM Encoding sizes
    impl_array_sizes! {
        749 => U749,
        781 => U781,
        877 => U877,
        1124 => U1124,
        1156 => U1156,
        1252 => U1252,
        1498 => U1498,
        1530 => U1530,
        1658 => U1658,
    }

    // LMS sizes
    impl_array_sizes! {
        2047 => U2047,
        2180 => U2180,
        4292 => U4292,
        8516 => U8516,
    }

    // Frodo sizes
    impl_array_sizes! {
        9616 => U9616,
        19888 => U19888,
        9720 => U9720,
        9752 => U9752,
        15632 => U15632,
        31296 => U31296,
        15744 => U15744,
        15792 => U15792,
        21520 => U21520,
        43088 => U43088,
        21632 => U21632,
        21696 => U21696,
    }
}