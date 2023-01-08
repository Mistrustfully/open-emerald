#!/usr/bin/python3
# A small helper script for graphic assets.

from PIL import Image
import sys
import os

def get_pal_from_jasc(jasc_location):
    jasc_file = open(jasc_location)
    jasc_str = jasc_file.read()
    colors = jasc_str.split()
    assert(colors[0] == "JASC-PAL")
    assert(colors[1] == "0100")
    palette = []

    for color in colors[3:]:
        print(int(color))
        palette.append(int(color))

    return palette

def swap_image_palette(image_location, palette_location):

    image = Image.open(image_location)
    image.putpalette(get_pal_from_jasc(palette_location)) 
    image.save(open(image_location).name.split(".")[-2] + "-" + os.path.basename(palette_location).split(".")[-2] + ".png")

def change_first_index(image_location):
    image = Image.open(image_location)
    image.save(image_location, transparency=0)

def main():
    if sys.argv[1] == "swap":
        swap_image_palette(sys.argv[2], sys.argv[3])
    elif sys.argv[1] == "fix":
        for file_to_fix in sys.argv[2:]:
            change_first_index(file_to_fix)

if __name__ == "__main__":
    main()
