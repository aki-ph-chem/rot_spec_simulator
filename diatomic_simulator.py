import numpy as np
import matplotlib.pyplot as plt
from typing import Callable

class DiatomicMolecule:
    def __init__(self,rot_const: float) -> None:
        self.rot_const = rot_const

    def energy(self, j: int) -> float:
        return 2*self.rot_const * j* (j + 1)

class Population:
    def __init__(self, temp: float, j_max: int, rot_const: float) -> None:
        self.temp = temp
        self.j_max = j_max
        self.molecule = DiatomicMolecule(rot_const)

    def spectrum(self) -> tuple[np.ndarray, np.ndarray]:
        x = np.arange(0, self.j_max, 1)
        energy_vec = np.vectorize(self.molecule.energy)

        wn = 2*self.molecule.rot_const *(x + 1)
        '''
        Z: partition function
        boltzaman factor: exp(-E_{lower}/k_BT) /Z - exp(-E_{upper}/k_BT) /Z
        '''

        boltzman_factor = ((x + 1) / (2 * x + 1)) *(np.exp(-energy_vec(x) /self.temp) - np.exp(-energy_vec(x + 1) /self.temp))
        intensity = boltzman_factor / ((2* x + 1) * np.exp(-energy_vec(x) / self.temp)).sum() 

        return (wn, intensity)

class LorentzianShape:
    def __init__(self, width: float) -> None:
        self.width = width

    def lorenth(self, x: float, x_centor: float) -> float:
        return (self.width / (2* np.pi)) / ((x - x_centor)**2 + (self.width / 2)**2)

class GaussianShape:
    def __init__(self, width: float) -> None:
        self.width = width

    def gauss(self, x: float, x_centor: float) -> float:
        coeffcient = np.sqrt(np.log(2) / np.pi) / (self.width /2.0)

        return coeffcient * np.exp(-(x - x_centor)**2 / (self.width / 2.0)) 

def convolute_line_shape(
        x_ini: float,
        x_fin: float,
        x_step: float,
        line_func: Callable[[float, float], float],
        raw_signal: tuple[np.ndarray, np.ndarray]
        ) -> tuple[np.ndarray, np.ndarray]:

    x_signal = np.arange(x_ini, x_fin, x_step)
    y_signal = np.zeros(x_signal.__len__())

    for i in range(raw_signal[0].__len__()):
        for j in range(x_signal.__len__()):
            y_signal[j] += raw_signal[1][i] * line_func(x_signal[j], raw_signal[0][i])

    return (x_signal, y_signal)

def main() -> None:
    temperature = 300 

    pop = Population(temperature, 30, 1.0)
    spectrum = pop.spectrum()

    lorentz_func = LorentzianShape(0.4)
    spectrum_lorentz_width = convolute_line_shape(0.0, 120, 0.01, lorentz_func.lorenth, spectrum)

    gauss_func = GaussianShape(0.4)
    spectrum_gauss_width = convolute_line_shape(0.0, 120, 0.01, gauss_func.gauss, spectrum)

    plt.scatter(spectrum[0], spectrum[1], color = 'blue', label = "spectrum")
    plt.plot(spectrum_lorentz_width[0], spectrum_lorentz_width[1], color = 'green', label = "spectrum_lorentz_width")
    plt.plot(spectrum_gauss_width[0], spectrum_gauss_width[1], color = 'lime', label = "spectrum_gauss_width")

    plt.legend()
    plt.show()

if __name__ == "__main__":
    main()
