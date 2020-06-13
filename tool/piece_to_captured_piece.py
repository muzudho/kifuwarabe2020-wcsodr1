from library import piece_types, captured_pieces

print('trace   | Start.')
print('match self {')
i = 0
for phase in range(1, 3):
    for piece_type in piece_types:
        print(
            f'    Piece::{piece_type}{phase} => Piece::{captured_pieces[i]},')
        i += 1
print('}')
print('trace   | Finished.')
