print('trace   | Start.')
print('pub enum UnifiedAddress {')
for file in range(1, 10):
    for rank in range(1, 10):
        print(f'    sq{file}{rank},')
for phase in range(1, 3):
    for hand in ['king', 'rook', 'bishop', 'gold', 'silver', 'knight', 'lance', 'pawn']:
        print(f'    {hand}{phase},')
print('}')
print('trace   | Finished.')
