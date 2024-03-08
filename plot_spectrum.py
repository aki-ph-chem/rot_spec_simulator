from os import name
import pandas as pd
import matplotlib.pyplot as plt
import sys

def main() -> None:
    if sys.argv.__len__() < 2:
        print("Error: invalid args")
        exit(1)

    path_to_file = sys.argv[1]
    data = pd.read_csv(path_to_file, header=None, names = ["x", "y"])
    data_pos_y = data.query("y > 0")

    offset = 1500
    data_x = data_pos_y["x"] + offset 
    data_y = data_pos_y["y"]

    int_max = data_y.max()
    int_max = 1

    plt.scatter(data_x, data_y / int_max)
    plt.show()

if __name__ == "__main__":
    main()
