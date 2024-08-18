def interpolate_color(color1, color2, factor):
    return (
        int(color1[0] + (color2[0] - color1[0]) * factor),
        int(color1[1] + (color2[1] - color1[1]) * factor),
        int(color1[2] + (color2[2] - color1[2]) * factor)
    )

# Original palette
palette = [
    (255, 255, 255),
    (217, 67, 80),
    (242, 109, 182),
    (141, 65, 191),
    (22, 71, 115),
    (75, 158, 191),
    (0, 0, 0),
]

# Generate a new palette with 20 colors
new_palette = []
num_colors = 100
num_intervals = num_colors - 1
for i in range(len(palette) - 1):
    new_palette.append(palette[i])
    for j in range(1, num_intervals // (len(palette) - 1)):
        factor = j / (num_intervals // (len(palette) - 1))
        new_palette.append(interpolate_color(palette[i], palette[i + 1], factor))
new_palette.append(palette[-1])

# Ensure the palette has exactly 20 colors
while len(new_palette) > num_colors:
    new_palette.pop()

print(new_palette)