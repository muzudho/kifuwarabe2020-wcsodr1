from library import piece_types, pieces_to_double_faced_pieces


print('trace   | Start.')
print('match self {')
for phase in range(1, 3):
    for i, piece_type in enumerate(piece_types):
        print(
            f'    Piece::{piece_type}{phase} = > DoubleFacedPiece: : {pieces_to_double_faced_pieces[i]}{phase}, ')
print('}')
print('trace   | Finished.')
