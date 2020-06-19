import sys

def print_aliases():
    if len(sys.argv) < 2:
        print("Please supply the path to a .gitconfig file")
        return

    arg = sys.argv[1] 
    config = open(arg)
    contents = config.read()
    result = contents.split('[alias]\n', 1)[1]
    result = result.split('[', 1)[0]

    lines = result.split('\n')
    del lines[-1]
    for line in lines:
        if line.startswith('#'):
            print(f"{OutputColors.HEADER}{line}{OutputColors.END}")
        else:
            parts = line.partition('=')
            print(f"{OutputColors.ALIAS}{parts[0]}{OutputColors.END}=", end='')
            print(f"{OutputColors.COMMAND}{parts[2]}{OutputColors.END}")


class OutputColors:
    HEADER = '\033[31m'
    ALIAS = '\033[32m'
    COMMAND = '\033[34m'
    END = '\033[0m'

if __name__ == '__main__':
    print_aliases()
