import pandas as pd
import matplotlib.pyplot as plt
import sys

def main() -> None:
    if sys.argv.__len__() < 2:
        print("Error: invalid args")
        exit(1)

    path_to_file = sys.argv[1]
    data = pd.read_csv(path_to_file)
    data_pos_y = data.query("y > 0")

    offset = 100
    data_x = data_pos_y["x"] + offset 
    data_y = data_pos_y["y"]

    plt.scatter(data_x, data_y)
    plt.show()

if __name__ == "__main__":
    main()
