#!/usr/bin/python3

# SPDX-FileCopyrightText: 2023 Christian Fletcher <mistrustfully@gmail.com>
#
# SPDX-License-Identifier: GPL-3.0-or-later

# A small helper script for graphic assets.

from PIL import Image
import sys
import os
import json

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

def generate_font_config(font_location):
    image = Image.open(font_location)
    font_config = {
        "layout": [],
        "spacing": {}
    }

    try:
        i = -1
        while True:
            i += 1
            char = input("Input a character (Control + C to finish)\n> ")

            if char == " ":
                font_config["spacing"][char] = 16
                continue

            font_config["layout"].append(char)
            
            pos_x = (i % 16) * 16
            pos_y = (i // 16)* 16

            print("top left", pos_x, pos_y)
            print("bottom right", pos_x + 16, pos_y + 16)

            max_x = 0
            for y in range(pos_y, pos_y + 16):
                for x in range(pos_x, pos_x + 16):
                    if image.getpixel((x, y))[3] != 0:
                        max_x = x % 16
                        print(x, max_x)
                        continue

            font_config["spacing"][char] = max_x + 1
    except KeyboardInterrupt:
        json_object = json.dumps(font_config, indent=4)
        output = open(open(font_location).name.split(".")[-2] + ".font_config", "w")
        output.write(json_object)
        print("\nDone!")
     

def main():
    if sys.argv[1] == "swap":
        swap_image_palette(sys.argv[2], sys.argv[3])
    elif sys.argv[1] == "fix":
        for file_to_fix in sys.argv[2:]:
            change_first_index(file_to_fix)
    elif sys.argv[1] == "font":
        generate_font_config(sys.argv[2])

if __name__ == "__main__":
    main()
