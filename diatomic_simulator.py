import numpy as np
import matplotlib.pyplot as plt

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
        boltzman_factor = np.exp(-energy_vec(x) /self.temp)
        population_array = (x + 1) * boltzman_factor 
        intensity = population_array / ((2* x + 1) * boltzman_factor).sum() 

        return (wn, intensity)

def main() -> None:
    pop_1 = Population(300, 30, 2.0)

    spectrum_1 = pop_1.spectrum()
    plt.scatter(spectrum_1[0], spectrum_1[1], color = 'red')
    plt.show()

if __name__ == "__main__":
    main()
