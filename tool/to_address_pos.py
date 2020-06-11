print('trace   | Start.')
print('const MAP: [AddressPos; 178] = [')
for phase in range(1, 3):
    for file in range(1, 10):
        for rank in range(1, 10):
            print(
                f'    AddressPos::Board(AbsoluteAddress2D {{ file: {file}, rank: {rank} }}),')
for phase in range(1, 3):
    for hand in ['King', 'Rook', 'Bishop', 'Gold', 'Silver', 'Knight', 'Lance', 'Pawn']:
        print(f'    AddressPos::Hand(DoubleFacedPiece::{hand}{phase}),')
print('];')
print('trace   | Finished.')
