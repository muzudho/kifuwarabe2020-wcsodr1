from library import piece_types

print('trace   | Start.')
print('match self {')
for phase in range(1, 3):
    for piece_type in piece_types:
        print(
            f'    Piece::{piece_type}{phase} => PieceType::{piece_type},')
print('}')
print('trace   | Finished.')
