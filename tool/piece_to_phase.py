from library import piece_types

print('trace   | Start.')
print('match self {')
for phase in range(1, 3):
    for piece_type in piece_types:
        turn = "Phase::First" if phase == 1 else "Phase::Second"
        print(
            f'    Piece::{piece_type}{phase} => {turn},')
print('}')
print('trace   | Finished.')
