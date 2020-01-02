#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
Module doc
"""
from argparse import ArgumentParser



def main():
    """
    Entry Point
    """
    parser = ArgumentParser()
    parser.add_argument('file')
    args = parser.parse_args()

    with open(args.file, 'rb') as handle:
        content = handle.read()

        for i in range(0, len(content), 2):
            if i + 1 >= len(content):
                break
            instruction = content[i] << 8 | content[i + 1]
            nibbles = (
                (instruction & 0xF000) >> 12,
                (instruction & 0x0F00) >> 8,
                (instruction & 0x00F0) >> 4,
                (instruction & 0x000F),
            )

            nnn = instruction & 0x0FFF
            kk = instruction & 0x00FF
            fam = nibbles[0]
            x = nibbles[1]
            y = nibbles[2]
            n = nibbles[3]
            output = "{:04x}: {:04x}: ".format(0x200 + i, instruction)

            if fam == 0:
                if nnn == 0x0E0:
                    output += "CLS"
                if nnn == 0x0EE:
                    output += "RET"
            if fam == 1:
                output += "JP 0x{:x}".format(nnn)
            if fam == 2:
                output += "CALL 0x{:x}".format(nnn)
            if fam == 3:
                output += "SE V{:X}, 0x{:02x}".format(x, kk)
            if fam == 4:
                output += "SNE V{:X}, 0x{:02x}".format(x, kk)
            if fam == 5:
                output += "SE V{:X}, 0x{:02x}".format(x, y)
            if fam == 6:
                output += "LD V{:X}, 0x{:02x}".format(x, kk)
            if fam == 7:
                output += "ADD V{:X}, 0x{:02x}".format(x, kk)
            if fam == 8:
                if n == 0:
                    output += "LD V{:X}, V{:X}".format(x, y)
                if n == 1:
                    output += "OR V{:X}, V{:X}".format(x, y)
                if n == 2:
                    output += "AND V{:X}, V{:X}".format(x, y)
                if n == 3:
                    output += "XOR V{:X}, V{:X}".format(x, y)
                if n == 4:
                    output += "ADD V{:X}, V{:X}".format(x, y)
                if n == 5:
                    output += "SUB V{:X}, V{:X}".format(x, y)
                if n == 6:
                    output += "SHR V{:X}".format(x)
                if n == 7:
                    output += "SUBN V{:X}, V{:X}".format(x, y)
                if n == 0xE:
                    output += "SHL V{:X}, V{:X}".format(x, y)
            if fam == 9:
                output += "SNE V{:X}, V{:X}".format(x, y)
            if fam == 0xA:
                output += "LD I, 0x{:x}".format(nnn)
            if fam == 0xB:
                output += "JP V0, 0x{:x}".format(nnn)
            if fam == 0xC:
                output += "RND V{:X}, 0x{:02x}".format(x, kk)
            if fam == 0xD:
                output += "DRW V{:X}, V{:X}, 0x{:02x}".format(x, y, n)
            if fam == 0xE:
                if kk == 0x9E:
                    output += "SKP V{:X}".format(x)
                if kk == 0xA1:
                    output += "SKNP V{:X}".format(x)
            if fam == 0xF:
                if kk == 0x07:
                    output += "LD V{:X}, DT".format(x)
                if kk == 0x0A:
                    output += "LD V{:X}, K".format(x)
                if kk == 0x15:
                    output += "LD DT, V{:X}".format(x)
                if kk == 0x18:
                    output += "LD ST, V{:X}".format(x)
                if kk == 0x1E:
                    output += "ADD I, V{:X}".format(x)
                if kk == 0x29:
                    output += "LD F, V{:X}".format(x)
                if kk == 0x33:
                    output += "LD B, V{:X}".format(x)
                if kk == 0x55:
                    output += "LD [I], V{:X}".format(x)
                if kk == 0x65:
                    output += "LD V{:X}, [I]".format(x)


            spaces = 30 - len(output)
            output += ' ' * spaces
            output += "{:08b} {:08b}".format((instruction & 0xFF00) >> 8, instruction & 0x00FF).replace('1', '*').replace('0', '_')

            print(output)


if __name__ == "__main__":
    main()

