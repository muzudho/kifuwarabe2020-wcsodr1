from library import double_faced_piece_types


print('trace   | Start.')
print('match self {')
for phase in range(1, 3):
    for double_faced_piece_type in double_faced_piece_types:
        print(
            f'    DoubleFacedPiece::{double_faced_piece_type}{phase} => Piece::{double_faced_piece_type}{phase}, ')
print('}')
print('trace   | Finished.')
