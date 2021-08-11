use crate::{Algorithm, Endian};
use core::mem;

/// Cyclic redundancy check.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CRC<T> {
    pub algorithm: Algorithm<T>,
    pub value: T,
    pub table: [T; 256],
}

macro_rules! crc_impl {
    ( $( $t:ty ),* ) => ($(
        impl CRC<$t> {
            /// Create your own CRC.
            pub const fn new(
                endian: Endian,
                poly: $t,
                init: $t,
                refin: bool,
                refout: bool,
                xorout: $t,
            ) -> Self {
                let (algorithm, value, table) = Algorithm::<$t>::new(
                    endian,
                    poly,
                    init,
                    refin,
                    refout,
                    xorout,
                );

                Self {
                    algorithm,
                    value,
                    table,
                }
            }

            /// The algorithm initializes the value and creates the table.
            ///
            /// # Safety
            /// [`Algorithm`] information must be correct.
            pub const fn from_algorithm(algorithm: Algorithm<$t>) -> Self {
                let value = Algorithm::<$t>::initialize(algorithm.init, algorithm.refin);
                let table = Algorithm::<$t>::create_table(algorithm.poly, algorithm.refin);
                Self {
                    algorithm,
                    value,
                    table,
                }
            }

            /// Initialize value.
            pub fn initialize(&mut self) -> &mut Self {
                self.value = Algorithm::<$t>::initialize(self.algorithm.init, self.algorithm.refin);
                self
            }

            /// Caluculate bytes.
            pub fn calc_bytes(&mut self, bytes: &[u8]) -> &mut Self {
                self.value = Algorithm::<$t>::calc_bytes_with_values(self.algorithm.refin, self.value, bytes, &self.table);
                self
            }

            /// Optional reflection.
            pub const fn optional_reflection(&self) -> $t {
                Algorithm::<$t>::optional_reflection(self.algorithm.refin, self.algorithm.refout, self.value)
            }

            /// Finalize value.
            /// Change value to checksum.
            pub const fn finalize(&self) -> $t {
                Algorithm::<$t>::finalize(self.algorithm.refin, self.algorithm.refout, self.algorithm.xorout, self.value)
            }

            /// Finalize to endian bytes.
            pub const fn finalize_to_endian_bytes(&self) -> [u8; mem::size_of::<$t>()] {
                Algorithm::<$t>::finalize_to_endian_bytes(self.algorithm.endian, self.algorithm.refin, self.algorithm.refout, self.algorithm.xorout, self.value)
            }

            /// Checksum function.
            pub fn checksum(&mut self, bytes: &[u8]) -> $t {
                self.initialize().calc_bytes(bytes).finalize()
            }

            /// Checksum to endian bytes.
            pub fn checksum_to_endian_bytes(&mut self, bytes: &[u8]) -> [u8; mem::size_of::<$t>()] {
                self.initialize().calc_bytes(bytes).finalize_to_endian_bytes()
            }

            /// Check if `value` is error-free.
            /// Returns `true` if error-free.
            pub fn is_error_free(&mut self) -> bool {
                if self.optional_reflection() == self.algorithm.residue {
                    true
                } else {
                    false
                }
            }

            /// Check if bytes [message + checksum] are error-free.
            /// Returns `true` if error-free.
            pub fn is_error_free_bytes(&mut self, bytes: &[u8]) -> bool {
                self.initialize().calc_bytes(bytes).is_error_free()
            }
        }
    )*)
}

crc_impl!(u16, u32, u64, u128);

#[cfg(test)]
mod tests {
    use super::*;

    const CHECK_BYTES: &[u8] = b"123456789";

    const CRC_32_AIXM: Algorithm<u32> = Algorithm {
        endian: Endian::Native,
        poly: 0x814141ab,
        init: 0x00000000,
        refin: false,
        refout: false,
        xorout: 0x00000000,
        residue: 0x00000000,
    };
    const CRC_32_AUTOSAR: Algorithm<u32> = Algorithm {
        endian: Endian::Little,
        poly: 0xf4acfb13,
        init: 0xffffffff,
        refin: true,
        refout: true,
        xorout: 0xffffffff,
        residue: 0x904cddbf,
    };
    const CRC_32_BASE91_D: Algorithm<u32> = Algorithm {
        endian: Endian::Little,
        poly: 0xa833982b,
        init: 0xffffffff,
        refin: true,
        refout: true,
        xorout: 0xffffffff,
        residue: 0x45270551,
    };
    const CRC_32_BZIP2: Algorithm<u32> = Algorithm {
        endian: Endian::Big,
        poly: 0x04c11db7,
        init: 0xffffffff,
        refin: false,
        refout: false,
        xorout: 0xffffffff,
        residue: 0xc704dd7b,
    };
    const CRC_32_CD_ROM_EDC: Algorithm<u32> = Algorithm {
        endian: Endian::Native,
        poly: 0x8001801b,
        init: 0x00000000,
        refin: true,
        refout: true,
        xorout: 0x00000000,
        residue: 0x00000000,
    };
    const CRC_32_CKSUM: Algorithm<u32> = Algorithm {
        endian: Endian::Big,
        poly: 0x04c11db7,
        init: 0x00000000,
        refin: false,
        refout: false,
        xorout: 0xffffffff,
        residue: 0xc704dd7b,
    };
    const CRC_32_ISCSI: Algorithm<u32> = Algorithm {
        endian: Endian::Little,
        poly: 0x1edc6f41,
        init: 0xffffffff,
        refin: true,
        refout: true,
        xorout: 0xffffffff,
        residue: 0xb798b438,
    };
    const CRC_32_ISO_HDLC: Algorithm<u32> = Algorithm {
        endian: Endian::Little,
        poly: 0x04c11db7,
        init: 0xffffffff,
        refin: true,
        refout: true,
        xorout: 0xffffffff,
        residue: 0xdebb20e3,
    };
    const CRC_32_JAMCRC: Algorithm<u32> = Algorithm {
        endian: Endian::Native,
        poly: 0x04c11db7,
        init: 0xffffffff,
        refin: true,
        refout: true,
        xorout: 0x00000000,
        residue: 0x00000000,
    };
    const CRC_32_MPEG_2: Algorithm<u32> = Algorithm {
        endian: Endian::Native,
        poly: 0x04c11db7,
        init: 0xffffffff,
        refin: false,
        refout: false,
        xorout: 0x00000000,
        residue: 0x00000000,
    };
    const CRC_32_XFER: Algorithm<u32> = Algorithm {
        endian: Endian::Native,
        poly: 0x000000af,
        init: 0x00000000,
        refin: false,
        refout: false,
        xorout: 0x00000000,
        residue: 0x00000000,
    };

    #[test]
    fn check_refin_true_table() {
        let crc32c_table: [u32; 256] = [
            0, 4067132163, 3778769143, 324072436, 3348797215, 904991772, 648144872, 3570033899,
            2329499855, 2024987596, 1809983544, 2575936315, 1296289744, 3207089363, 2893594407,
            1578318884, 274646895, 3795141740, 4049975192, 51262619, 3619967088, 632279923,
            922689671, 3298075524, 2592579488, 1760304291, 2075979607, 2312596564, 1562183871,
            2943781820, 3156637768, 1313733451, 549293790, 3537243613, 3246849577, 871202090,
            3878099393, 357341890, 102525238, 4101499445, 2858735121, 1477399826, 1264559846,
            3107202533, 1845379342, 2677391885, 2361733625, 2125378298, 820201905, 3263744690,
            3520608582, 598981189, 4151959214, 85089709, 373468761, 3827903834, 3124367742,
            1213305469, 1526817161, 2842354314, 2107672161, 2412447074, 2627466902, 1861252501,
            1098587580, 3004210879, 2688576843, 1378610760, 2262928035, 1955203488, 1742404180,
            2511436119, 3416409459, 969524848, 714683780, 3639785095, 205050476, 4266873199,
            3976438427, 526918040, 1361435347, 2739821008, 2954799652, 1114974503, 2529119692,
            1691668175, 2005155131, 2247081528, 3690758684, 697762079, 986182379, 3366744552,
            476452099, 3993867776, 4250756596, 255256311, 1640403810, 2477592673, 2164122517,
            1922457750, 2791048317, 1412925310, 1197962378, 3037525897, 3944729517, 427051182,
            170179418, 4165941337, 746937522, 3740196785, 3451792453, 1070968646, 1905808397,
            2213795598, 2426610938, 1657317369, 3053634322, 1147748369, 1463399397, 2773627110,
            4215344322, 153784257, 444234805, 3893493558, 1021025245, 3467647198, 3722505002,
            797665321, 2197175160, 1889384571, 1674398607, 2443626636, 1164749927, 3070701412,
            2757221520, 1446797203, 137323447, 4198817972, 3910406976, 461344835, 3484808360,
            1037989803, 781091935, 3705997148, 2460548119, 1623424788, 1939049696, 2180517859,
            1429367560, 2807687179, 3020495871, 1180866812, 410100952, 3927582683, 4182430767,
            186734380, 3756733383, 763408580, 1053836080, 3434856499, 2722870694, 1344288421,
            1131464017, 2971354706, 1708204729, 2545590714, 2229949006, 1988219213, 680717673,
            3673779818, 3383336350, 1002577565, 4010310262, 493091189, 238226049, 4233660802,
            2987750089, 1082061258, 1395524158, 2705686845, 1972364758, 2279892693, 2494862625,
            1725896226, 952904198, 3399985413, 3656866545, 731699698, 4283874585, 222117402,
            510512622, 3959836397, 3280807620, 837199303, 582374963, 3504198960, 68661723,
            4135334616, 3844915500, 390545967, 1230274059, 3141532936, 2825850620, 1510247935,
            2395924756, 2091215383, 1878366691, 2644384480, 3553878443, 565732008, 854102364,
            3229815391, 340358836, 3861050807, 4117890627, 119113024, 1493875044, 2875275879,
            3090270611, 1247431312, 2660249211, 1828433272, 2141937292, 2378227087, 3811616794,
            291187481, 34330861, 4032846830, 615137029, 3603020806, 3314634738, 939183345,
            1776939221, 2609017814, 2295496738, 2058945313, 2926798794, 1545135305, 1330124605,
            3173225534, 4084100981, 17165430, 307568514, 3762199681, 888469610, 3332340585,
            3587147933, 665062302, 2042050490, 2346497209, 2559330125, 1793573966, 3190661285,
            1279665062, 1595330642, 2910671697,
        ];

        let crc32c = CRC::<u32>::from_algorithm(CRC_32_ISCSI);
        assert_eq!(crc32c_table, crc32c.table);
    }

    #[test]
    fn check_refin_false_table() {
        let crc32_table: [u32; 256] = [
            0, 79764919, 159529838, 222504665, 319059676, 398814059, 445009330, 507990021,
            638119352, 583659535, 797628118, 726387553, 890018660, 835552979, 1015980042,
            944750013, 1276238704, 1221641927, 1167319070, 1095957929, 1595256236, 1540665371,
            1452775106, 1381403509, 1780037320, 1859660671, 1671105958, 1733955601, 2031960084,
            2111593891, 1889500026, 1952343757, 2552477408, 2632100695, 2443283854, 2506133561,
            2334638140, 2414271883, 2191915858, 2254759653, 3190512472, 3135915759, 3081330742,
            3009969537, 2905550212, 2850959411, 2762807018, 2691435357, 3560074640, 3505614887,
            3719321342, 3648080713, 3342211916, 3287746299, 3467911202, 3396681109, 4063920168,
            4143685023, 4223187782, 4286162673, 3779000052, 3858754371, 3904687514, 3967668269,
            881225847, 809987520, 1023691545, 969234094, 662832811, 591600412, 771767749,
            717299826, 311336399, 374308984, 453813921, 533576470, 25881363, 88864420, 134795389,
            214552010, 2023205639, 2086057648, 1897238633, 1976864222, 1804852699, 1867694188,
            1645340341, 1724971778, 1587496639, 1516133128, 1461550545, 1406951526, 1302016099,
            1230646740, 1142491917, 1087903418, 2896545431, 2825181984, 2770861561, 2716262478,
            3215044683, 3143675388, 3055782693, 3001194130, 2326604591, 2389456536, 2200899649,
            2280525302, 2578013683, 2640855108, 2418763421, 2498394922, 3769900519, 3832873040,
            3912640137, 3992402750, 4088425275, 4151408268, 4197601365, 4277358050, 3334271071,
            3263032808, 3476998961, 3422541446, 3585640067, 3514407732, 3694837229, 3640369242,
            1762451694, 1842216281, 1619975040, 1682949687, 2047383090, 2127137669, 1938468188,
            2001449195, 1325665622, 1271206113, 1183200824, 1111960463, 1543535498, 1489069629,
            1434599652, 1363369299, 622672798, 568075817, 748617968, 677256519, 907627842,
            853037301, 1067152940, 995781531, 51762726, 131386257, 177728840, 240578815, 269590778,
            349224269, 429104020, 491947555, 4046411278, 4126034873, 4172115296, 4234965207,
            3794477266, 3874110821, 3953728444, 4016571915, 3609705398, 3555108353, 3735388376,
            3664026991, 3290680682, 3236090077, 3449943556, 3378572211, 3174993278, 3120533705,
            3032266256, 2961025959, 2923101090, 2868635157, 2813903052, 2742672763, 2604032198,
            2683796849, 2461293480, 2524268063, 2284983834, 2364738477, 2175806836, 2238787779,
            1569362073, 1498123566, 1409854455, 1355396672, 1317987909, 1246755826, 1192025387,
            1137557660, 2072149281, 2135122070, 1912620623, 1992383480, 1753615357, 1816598090,
            1627664531, 1707420964, 295390185, 358241886, 404320391, 483945776, 43990325,
            106832002, 186451547, 266083308, 932423249, 861060070, 1041341759, 986742920,
            613929101, 542559546, 756411363, 701822548, 3316196985, 3244833742, 3425377559,
            3370778784, 3601682597, 3530312978, 3744426955, 3689838204, 3819031489, 3881883254,
            3928223919, 4007849240, 4037393693, 4100235434, 4180117107, 4259748804, 2310601993,
            2373574846, 2151335527, 2231098320, 2596047829, 2659030626, 2470359227, 2550115596,
            2947551409, 2876312838, 2788305887, 2733848168, 3165939309, 3094707162, 3040238851,
            2985771188,
        ];

        let crc32 = CRC::<u32>::from_algorithm(CRC_32_BZIP2);
        assert_eq!(crc32_table, crc32.table);
    }

    #[test]
    fn check() {
        // (Algorithm, check)
        let algos = [
            (CRC_32_AIXM, 0x3010bf7f),
            (CRC_32_AUTOSAR, 0x1697d06a),
            (CRC_32_BASE91_D, 0x87315576),
            (CRC_32_BZIP2, 0xfc891918),
            (CRC_32_CD_ROM_EDC, 0x6ec2edc4),
            (CRC_32_CKSUM, 0x765e7680),
            (CRC_32_ISCSI, 0xe3069283),
            (CRC_32_ISO_HDLC, 0xcbf43926),
            (CRC_32_JAMCRC, 0x340bc6d9),
            (CRC_32_MPEG_2, 0x0376e6e7),
            (CRC_32_XFER, 0xbd0be338),
        ];
        for algo in algos {
            let mut crc32 = CRC::<u32>::from_algorithm(algo.0);
            assert_eq!(crc32.checksum(CHECK_BYTES), algo.1);
        }
    }

    #[test]
    fn residue() {
        // True if little-endian.
        let algos = [
            (CRC_32_AUTOSAR, 0x1697d06a_u32),
            (CRC_32_BASE91_D, 0x87315576),
            (CRC_32_BZIP2, 0xfc891918),
            (CRC_32_CKSUM, 0x765e7680),
            (CRC_32_ISCSI, 0xe3069283),
            (CRC_32_ISO_HDLC, 0xcbf43926),
        ];

        for algo in algos {
            let mut crc32 = CRC::<u32>::from_algorithm(algo.0);

            // message b"123456789"
            let check = Algorithm::<u32>::to_endian_bytes(algo.1, algo.0.endian);

            crc32.calc_bytes(CHECK_BYTES);
            crc32.calc_bytes(&check);
            assert!(crc32.is_error_free());

            // message []
            let checksum = crc32.checksum_to_endian_bytes(&[]);
            assert!(crc32.is_error_free_bytes(&checksum));

            // Check if `CRC::from_algorithm` algo is equal to `CRC::new` algo.
            let algo = crc32.algorithm;
            let crc_new = CRC::<u32>::new(
                algo.endian,
                algo.poly,
                algo.init,
                algo.refin,
                algo.refout,
                algo.xorout,
            );
            assert_eq!(algo, crc_new.algorithm);
        }
    }
}
