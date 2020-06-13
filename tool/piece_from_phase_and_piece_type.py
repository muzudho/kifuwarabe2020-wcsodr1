print('trace   | Start.')

print('impl Piece {')
print(
    '    pub fn from_phase_and_piece_tpye(friend: Phase, piece_type: PieceType) -> Self {')
print('        use crate::cosmic::toy_box::UnifiedAddress::*;')
print('        match self {')
hands = ['King', 'Rook', 'Bishop', 'Gold', 'Silver', 'Knight', 'Lance', 'Pawn',
         'Dragon', 'Horse', 'PromotedSilver', 'PromotedKnight', 'PromotedLance', 'PromotedPawn']

i = 0
for phase in range(1, 3):
    for hand in hands:
        print(
            f'                PieceType::{hand} => Piece::{hand}{phase},')
        i += 1
print('        }')
print('    }')
print('}')

print('trace   | Finished.')
