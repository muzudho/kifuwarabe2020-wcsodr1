print('trace   | Start.')

# 配列へのアクセスは遅い気がするので、定数で作っておいて……☆（＾～＾）
i = 0
for phase in range(1, 3):
    for file in range(1, 10):
        for rank in range(1, 10):
            print(
                f'const UNIFIED_ADDRESS_{i}_TO_ADDRESS_POS1: AddressPos1 = AddressPos1::Board(SquareType::Sq{file}{rank});')
            i += 1
for phase in range(1, 3):
    for hand in ['King', 'Rook', 'Bishop', 'Gold', 'Silver', 'Knight', 'Lance', 'Pawn']:
        turn = "Phase::First" if phase == 1 else "Phase::Second"
        print(
            f'const UNIFIED_ADDRESS_{i}_TO_ADDRESS_POS1: AddressPos1 = AddressPos1::Hand(({turn},DoubleFacedPieceType::{hand}));')
        i += 1

# 配列アクセスは遅い気がするので、match構文で書こうぜ☆（＾～＾）
enums = ['Sq11_1', 'Sq12_1', 'Sq13_1', 'Sq14_1', 'Sq15_1', 'Sq16_1', 'Sq17_1', 'Sq18_1', 'Sq19_1', 'Sq21_1', 'Sq22_1', 'Sq23_1', 'Sq24_1', 'Sq25_1', 'Sq26_1', 'Sq27_1', 'Sq28_1', 'Sq29_1', 'Sq31_1', 'Sq32_1', 'Sq33_1', 'Sq34_1', 'Sq35_1', 'Sq36_1', 'Sq37_1', 'Sq38_1', 'Sq39_1', 'Sq41_1', 'Sq42_1', 'Sq43_1', 'Sq44_1', 'Sq45_1', 'Sq46_1', 'Sq47_1', 'Sq48_1', 'Sq49_1', 'Sq51_1', 'Sq52_1', 'Sq53_1', 'Sq54_1', 'Sq55_1', 'Sq56_1', 'Sq57_1', 'Sq58_1', 'Sq59_1', 'Sq61_1', 'Sq62_1', 'Sq63_1', 'Sq64_1', 'Sq65_1', 'Sq66_1', 'Sq67_1', 'Sq68_1', 'Sq69_1', 'Sq71_1', 'Sq72_1', 'Sq73_1', 'Sq74_1', 'Sq75_1', 'Sq76_1', 'Sq77_1', 'Sq78_1', 'Sq79_1', 'Sq81_1', 'Sq82_1', 'Sq83_1', 'Sq84_1', 'Sq85_1', 'Sq86_1', 'Sq87_1', 'Sq88_1', 'Sq89_1', 'Sq91_1', 'Sq92_1', 'Sq93_1', 'Sq94_1', 'Sq95_1', 'Sq96_1', 'Sq97_1', 'Sq98_1', 'Sq99_1', 'Sq11_2', 'Sq12_2', 'Sq13_2', 'Sq14_2', 'Sq15_2', 'Sq16_2', 'Sq17_2', 'Sq18_2',
         'Sq19_2', 'Sq21_2', 'Sq22_2', 'Sq23_2', 'Sq24_2', 'Sq25_2', 'Sq26_2', 'Sq27_2', 'Sq28_2', 'Sq29_2', 'Sq31_2', 'Sq32_2', 'Sq33_2', 'Sq34_2', 'Sq35_2', 'Sq36_2', 'Sq37_2', 'Sq38_2', 'Sq39_2', 'Sq41_2', 'Sq42_2', 'Sq43_2', 'Sq44_2', 'Sq45_2', 'Sq46_2', 'Sq47_2', 'Sq48_2', 'Sq49_2', 'Sq51_2', 'Sq52_2', 'Sq53_2', 'Sq54_2', 'Sq55_2', 'Sq56_2', 'Sq57_2', 'Sq58_2', 'Sq59_2', 'Sq61_2', 'Sq62_2', 'Sq63_2', 'Sq64_2', 'Sq65_2', 'Sq66_2', 'Sq67_2', 'Sq68_2', 'Sq69_2', 'Sq71_2', 'Sq72_2', 'Sq73_2', 'Sq74_2', 'Sq75_2', 'Sq76_2', 'Sq77_2', 'Sq78_2', 'Sq79_2', 'Sq81_2', 'Sq82_2', 'Sq83_2', 'Sq84_2', 'Sq85_2', 'Sq86_2', 'Sq87_2', 'Sq88_2', 'Sq89_2', 'Sq91_2', 'Sq92_2', 'Sq93_2', 'Sq94_2', 'Sq95_2', 'Sq96_2', 'Sq97_2', 'Sq98_2', 'Sq99_2', 'King1', 'Rook1', 'Bishop1', 'Gold1', 'Silver1', 'Knight1', 'Lance1', 'Pawn1', 'King2', 'Rook2', 'Bishop2', 'Gold2', 'Silver2', 'Knight2', 'Lance2', 'Pawn2', ]
i = 0
print('use crate::cosmic::toy_box::UnifiedAddress::*;')
print('let addr_pos1 = match self {')
for phase in range(0, 178):
    print(
        f'    {enums[i]} => UNIFIED_ADDRESS_{i}_TO_ADDRESS_POS1,')
    i += 1
print('};')

print('trace   | Finished.')
