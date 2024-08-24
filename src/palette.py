
class Palette:
    def __init__(self, file_path):
        self.file_path = file_path
        self.colors = self._load_palette()

    def _load_palette(self):
        self.colors = []
        with open(self.file_path, 'r') as file:
            line = file.readline().strip()
            if line:
                # Split the line by ', ' and convert each RGB string to a tuple of integers
                rgb_strings = line.split('), (')
                for rgb_string in rgb_strings:
                    # Remove any leading/trailing parentheses and whitespace
                    rgb_string = rgb_string.strip('() ')
                    # Convert the string to a tuple of integers
                    self.colors.append(tuple(map(int, rgb_string.split(','))))
        return self.colors

    def get_colors(self):
        return self.colors
