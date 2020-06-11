print('trace   | Start.')
print('match *drop {')
for phase in range(1, 3):
    for hand in ['King', 'Rook', 'Bishop', 'Gold', 'Silver', 'Knight', 'Lance', 'Pawn']:
        print(
            f'    DoubleFacedPiece::{hand}{phase} => UnifiedAddress::{hand}{phase},')
print('}')
print('trace   | Finished.')
