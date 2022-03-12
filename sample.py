import sys
import os
import pandas as pd


def main():
    """
    Main function
    """
    print("Hello World")
    print(sys.argv)
    print(os.getcwd())
    print(os.listdir())
    print(pd.__version__)
