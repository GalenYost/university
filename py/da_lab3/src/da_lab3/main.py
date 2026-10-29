import sys
import tkinter as tk
from .ui import StatsApp

def run():
    root = tk.Tk()
    app = StatsApp(root)
    if len(sys.argv) > 1:
        app.load_data_from_path(sys.argv[1])
    root.mainloop()

if __name__ == "__main__":
    run()
