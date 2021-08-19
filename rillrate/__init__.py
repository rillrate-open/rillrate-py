from .rillrate import install, uninstall
from .rillrate import Board, Counter, Gauge, Histogram, Pulse, Table
from .rillrate import Click, Selector, Slider, Switch

from enum import Enum

class Activity(Enum):
    SUSPEND = 0
    AWAKE = 1
    DISCONNECTED = 2
    CONNECTED = 3
    ACTION = 4

class ClickAction:
    pass

class SelectorAction:
    new_selected = None

    def __init__(self, value):
        self.new_selected = value

class SliderAction:
    pass

class SwitchAction:
    pass
