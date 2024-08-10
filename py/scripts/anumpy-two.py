import numpy as np

items = np.array([1, 2, 3, 4])
rolls = 0
max_ones = 0

# Precompute the roll matrix (space complex)
roll_matrix = np.random.choice(items, size=(1_000_000, 231))

for roll_set in roll_matrix:
    rolls += 1
    ones_count = np.sum(roll_set == 1)
    
    if ones_count > max_ones:
        max_ones = ones_count

    if ones_count >= 177:
        break

print("Highest Ones Roll:", max_ones)
print("Number of Roll Sessions: ", rolls)
