import pyaudio
import numpy as np
import scipy.fftpack
import time
from collections import deque

CHUNK = 1024
FORMAT = pyaudio.paInt16
CHANNELS = 1
RATE = 44100
LOW_FREQ = 20
HIGH_FREQ = 150
ROLLING_WINDOW = 50

class AudioProcessor:
    def __init__(self):
        self.p = pyaudio.PyAudio()
        self.stream = self.p.open(format=FORMAT, channels=CHANNELS, rate=RATE, input=True, frames_per_buffer=CHUNK)
        self.energy_queue = deque(maxlen=ROLLING_WINDOW)
        self.last_beat_time = 0

    def read_data(self):
        return self.stream.read(CHUNK, exception_on_overflow=False)

    def detect_beat(self, data, threshold_multiplier, cooldown_time):
        samples = np.frombuffer(data, dtype=np.int16)
        fft_spectrum = np.abs(scipy.fftpack.fft(samples))[:CHUNK // 2]
        freqs = np.fft.fftfreq(len(samples), 1.0 / RATE)[:CHUNK // 2]
        low_freq_spectrum = fft_spectrum[(freqs >= LOW_FREQ) & (freqs <= HIGH_FREQ)]
        energy = np.sum(low_freq_spectrum)
        self.energy_queue.append(energy)
        
        if len(self.energy_queue) == ROLLING_WINDOW:
            avg_energy = np.mean(self.energy_queue)
            if energy > threshold_multiplier * avg_energy:
                current_time = time.time()
                if current_time - self.last_beat_time > cooldown_time:
                    self.last_beat_time = current_time
                    return True
        return False

    def close(self):
        self.stream.stop_stream()
        self.stream.close()
        self.p.terminate()