print('trace   | Start.')
print('const MAP: [usize; 178] = [')
for phase in range(1, 3):
    for file in range(1, 10):
        for rank in range(1, 10):
            print(
                f'    {file}{rank},')
for phase in range(1, 3):
    for hand in ['King', 'Rook', 'Bishop', 'Gold', 'Silver', 'Knight', 'Lance', 'Pawn']:
        print(f'    0,')
print('];')
print('trace   | Finished.')
