import sys

def print_aliases():
    incorrectFileErrorMsg = "Please supply the path to a .gitconfig file"
    if len(sys.argv) < 2:
        print(incorrectFileErrorMsg)
        return

    fileName = sys.argv[1] 

    if fileName != ".gitconfig":
        print(incorrectFileErrorMsg)
        return

    config = open(fileName)
    contents = config.read()
    result = contents.split('[alias]\n', 1)[1]
    result = result.split('[', 1)[0]

    lines = result.split('\n')
    del lines[-1]

    outputLines = []
    maxAliasLength = 0
    for line in lines:
        if line.startswith('#'):
            outputLines.append((line, ""))
        else:
            lineParts = line.partition('=')
            alias = lineParts[0].strip()
            command = lineParts[2].strip()
            outputLines.append((alias,command))
            maxAliasLength = max(maxAliasLength, len(alias))

    for (alias, command) in outputLines:
        if command == "":
            print(f"{OutputColors.HEADER}{alias}{OutputColors.END}")
        else:
            postAliasSpaces = ''.join([char * (maxAliasLength - len(alias) + 1) for char in " "])
            print(f"{OutputColors.ALIAS}  {alias}{postAliasSpaces}{OutputColors.END}= ", end='')
            print(f"{OutputColors.COMMAND}{command}{OutputColors.END}")


class OutputColors:
    HEADER = '\033[31m'
    ALIAS = '\033[32m'
    COMMAND = '\033[34m'
    END = '\033[0m'

if __name__ == '__main__':
    print_aliases()
