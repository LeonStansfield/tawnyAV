from abc import ABC, abstractmethod

class Scene(ABC):
    @abstractmethod
    def draw(self, screen, data):
        pass

    def handle_beat(self):
        pass