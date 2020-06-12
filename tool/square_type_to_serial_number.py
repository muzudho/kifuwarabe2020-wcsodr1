print('trace   | Start.')

# 配列アクセスは遅い気がするので、match構文で書こうぜ☆（＾～＾）
enums = ['Sq11', 'Sq12', 'Sq13', 'Sq14', 'Sq15', 'Sq16', 'Sq17', 'Sq18', 'Sq19', 'Sq21', 'Sq22', 'Sq23', 'Sq24', 'Sq25', 'Sq26', 'Sq27', 'Sq28', 'Sq29', 'Sq31', 'Sq32', 'Sq33', 'Sq34', 'Sq35', 'Sq36', 'Sq37', 'Sq38', 'Sq39', 'Sq41', 'Sq42', 'Sq43', 'Sq44', 'Sq45', 'Sq46', 'Sq47', 'Sq48', 'Sq49', 'Sq51', 'Sq52', 'Sq53', 'Sq54',
         'Sq55', 'Sq56', 'Sq57', 'Sq58', 'Sq59', 'Sq61', 'Sq62', 'Sq63', 'Sq64', 'Sq65', 'Sq66', 'Sq67', 'Sq68', 'Sq69', 'Sq71', 'Sq72', 'Sq73', 'Sq74', 'Sq75', 'Sq76', 'Sq77', 'Sq78', 'Sq79', 'Sq81', 'Sq82', 'Sq83', 'Sq84', 'Sq85', 'Sq86', 'Sq87', 'Sq88', 'Sq89', 'Sq91', 'Sq92', 'Sq93', 'Sq94', 'Sq95', 'Sq96', 'Sq97', 'Sq98', 'Sq99', ]
i = 0
print('impl SquareType {')
print('    pub fn to_serial_number(&self) -> usize {')
print('        use crate::cosmic::toy_box::SquareType::*;')
print('        match self {')
for file in range(1, 10):
    for rank in range(1, 10):
        print(
            f'            {enums[i]} => {file}{rank},')
        i += 1
print('        }')
print('    }')
print('}')

print('trace   | Finished.')
