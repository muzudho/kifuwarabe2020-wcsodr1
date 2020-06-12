print('trace   | Start.')

# 配列へのアクセスは遅い気がするので、定数で作っておいて……☆（＾～＾）
i = 0
for file in range(1, 10):
    for rank in range(1, 10):
        print(
            f'const UNIFIED_SQ_{i}_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D = AbsoluteAddress2D {{ file: {file}, rank: {rank} }};')
        i += 1

# 配列アクセスは遅い気がするので、match構文で書こうぜ☆（＾～＾）
enums = ['Sq11', 'Sq12', 'Sq13', 'Sq14', 'Sq15', 'Sq16', 'Sq17', 'Sq18', 'Sq19', 'Sq21', 'Sq22', 'Sq23', 'Sq24', 'Sq25', 'Sq26', 'Sq27', 'Sq28', 'Sq29', 'Sq31', 'Sq32', 'Sq33', 'Sq34', 'Sq35', 'Sq36', 'Sq37', 'Sq38', 'Sq39', 'Sq41', 'Sq42', 'Sq43', 'Sq44', 'Sq45', 'Sq46', 'Sq47', 'Sq48', 'Sq49', 'Sq51', 'Sq52', 'Sq53', 'Sq54',
         'Sq55', 'Sq56', 'Sq57', 'Sq58', 'Sq59', 'Sq61', 'Sq62', 'Sq63', 'Sq64', 'Sq65', 'Sq66', 'Sq67', 'Sq68', 'Sq69', 'Sq71', 'Sq72', 'Sq73', 'Sq74', 'Sq75', 'Sq76', 'Sq77', 'Sq78', 'Sq79', 'Sq81', 'Sq82', 'Sq83', 'Sq84', 'Sq85', 'Sq86', 'Sq87', 'Sq88', 'Sq89', 'Sq91', 'Sq92', 'Sq93', 'Sq94', 'Sq95', 'Sq96', 'Sq97', 'Sq98', 'Sq99', ]
i = 0
print('use crate::cosmic::toy_box::SquareType::*;')
print('let addr_pos = match self {')
for phase in range(0, 81):
    print(
        f'    {enums[i]} => UNIFIED_SQ_{i}_TO_ABSOLUTE_ADDRESS_2D,')
    i += 1
print(f'    _ => panic!("（＾～＾）"),')
print('};')

print('trace   | Finished.')
