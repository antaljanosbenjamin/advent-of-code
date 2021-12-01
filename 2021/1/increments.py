
with open('input.txt') as f:
    measurements = [int(x) for x in f.readlines()]

increments = 0
for i in range(1, len(measurements)):
    if measurements[i-1] < measurements[i]:
        increments += 1

print(increments)