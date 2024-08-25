import pygame
import pygame_gui

class GUIManager:
    def __init__(self, screen_size, scenes):
        self.manager = pygame_gui.UIManager(screen_size)
        self.threshold_slider, self.cooldown_slider = self.create_gui_elements()
        self.scene_dropdown = self.create_scene_dropdown(scenes)
        self.current_scene = scenes[0]
        self.is_fullscreen = False
        self.show_ui = True

    def create_gui_elements(self):
        threshold_label = pygame_gui.elements.UILabel(
            relative_rect=pygame.Rect((40, 10), (200, 20)),
            text='Threshold Multiplier',
            manager=self.manager
        )
        threshold_slider = pygame_gui.elements.UIHorizontalSlider(
            relative_rect=pygame.Rect((40, 40), (200, 20)),
            start_value=1.0,
            value_range=(0.1, 3.5),
            manager=self.manager
        )
        cooldown_label = pygame_gui.elements.UILabel(
            relative_rect=pygame.Rect((300, 10), (200, 20)),
            text='Cooldown Time',
            manager=self.manager
        )
        cooldown_slider = pygame_gui.elements.UIHorizontalSlider(
            relative_rect=pygame.Rect((300, 40), (200, 20)),
            start_value=0.3,
            value_range=(0.1, 1.0),
            manager=self.manager
        )
        return threshold_slider, cooldown_slider

    def create_scene_dropdown(self, scenes):
        return pygame_gui.elements.UIDropDownMenu(
            options_list=[scene.name for scene in scenes],
            starting_option=scenes[0].name,
            relative_rect=pygame.Rect((40, 80), (150, 20)),
            manager=self.manager
        )

    def handle_events(self, scenes):
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                return False
            elif event.type == pygame.KEYDOWN:
                if event.key == pygame.K_F11:
                    self.is_fullscreen = not self.is_fullscreen
                    pygame.display.set_mode((0, 0), pygame.FULLSCREEN if self.is_fullscreen else 0)
                elif event.key == pygame.K_h:
                    self.show_ui = not self.show_ui
                elif pygame.K_1 <= event.key <= pygame.K_9:
                    index = event.key - pygame.K_1
                    if index < len(scenes):
                        self.current_scene = scenes[index]
                elif event.key == pygame.K_UP:
                    new_value = min(self.threshold_slider.get_current_value() + 0.1, self.threshold_slider.value_range[1])
                    self.threshold_slider.set_current_value(new_value)
                elif event.key == pygame.K_DOWN:
                    new_value = max(self.threshold_slider.get_current_value() - 0.1, self.threshold_slider.value_range[0])
                    self.threshold_slider.set_current_value(new_value)

            self.manager.process_events(event)

            if event.type == pygame_gui.UI_DROP_DOWN_MENU_CHANGED and event.ui_element == self.scene_dropdown:
                selected_scene_name = event.text
                self.current_scene = next(scene for scene in scenes if scene.name == selected_scene_name)

        return True
