import numpy as np

items = np.array([1, 2, 3, 4])
rolls = 0
max_ones = 0

while rolls < 1_000_000_000:
    rolls += 1
    numbers = np.random.choice(items, size=231)
    counts = np.bincount(numbers, minlength=5)[1:]  # minlength=5 to ensure we have 4 slots
    ones_count = counts[0]
    
    if ones_count > max_ones:
        max_ones = ones_count

    if ones_count >= 177:
        break

print("Highest Ones Roll:", max_ones)
print("Number of Roll Sessions: ", rolls)
