print('trace   | Start.')
print('pub enum UnifiedAddress {')
for file in range(1, 10):
    for rank in range(1, 10):
        print(f'    Sq{file}{rank},')
for phase in range(1, 3):
    for hand in ['King', 'Rook', 'Bishop', 'Gold', 'Silver', 'Knight', 'Lance', 'Pawn']:
        print(f'    {hand}{phase},')
print('}')
print('trace   | Finished.')
