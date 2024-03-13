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
        boltzaman factor: exp(-E_{upper}/k_BT) / Z 
        '''
        boltzman_factor = np.exp(-energy_vec(x) /self.temp)
        population_array = (x + 1) * boltzman_factor 
        intensity = population_array / ((2* x + 1) * boltzman_factor).sum() 

        return (wn, intensity)

class Population2:
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
        boltzman_factor = np.exp(-energy_vec(x) /self.temp) - np.exp(-energy_vec(x + 1) /self.temp)

        population_array = (x + 1) * boltzman_factor 
        intensity = population_array / ((2* x + 1) * boltzman_factor).sum() 

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
    temperature = 120 

    pop_1 = Population(temperature, 30, 2.0)
    spectrum_1 = pop_1.spectrum()

    pop_2 = Population2(temperature, 30, 2.0)
    spectrum_2 = pop_2.spectrum()

    lorentz_func = LorentzianShape(0.4)
    spectrum_2_lorentz_width = convolute_line_shape(0.0, 120, 0.01, lorentz_func.lorenth, spectrum_2)

    gauss_func = GaussianShape(0.4)
    spectrum_2_gauss_width = convolute_line_shape(0.0, 120, 0.01, gauss_func.gauss, spectrum_2)

    plt.scatter(spectrum_1[0], spectrum_1[1], color = 'red', label = "spectrum_1")
    plt.scatter(spectrum_2[0], spectrum_2[1], color = 'blue', label = "spectrum_2")
    plt.plot(spectrum_2_lorentz_width[0], spectrum_2_lorentz_width[1], color = 'green', label = "spectrum_2_lorentz_width")
    plt.plot(spectrum_2_gauss_width[0], spectrum_2_gauss_width[1], color = 'lime', label = "spectrum_2_gauss_width")

    plt.legend()
    plt.show()


if __name__ == "__main__":
    main()
