from library import piece_types, demoted_piece_types

print('trace   | Start.')
print('match self {')
for phase in range(1, 3):
    for i, piece_type in enumerate(piece_types):
        print(
            f'    Piece::{piece_type}{phase} => Piece::{demoted_piece_types[i]}{phase},')
print('}')
print('trace   | Finished.')
