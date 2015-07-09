#! /usr/env/python
"""Simple script to generate random images
"""

from PIL import Image, ImageDraw

from argparse import ArgumentParser
import os
import random

def parse_args():

    parser = ArgumentParser(description=__doc__)
    parser.add_argument('outdir', help='directory to output files')
    parser.add_argument('-c', dest='count', type=int,
        default=5, help='Number of images to create (default is 5)')
    parser.add_argument('--max-w', default=50, type=int, help='maximum width (default is 50)')
    parser.add_argument('--max-h', default=50, type=int, help='maximum height (default is 50)')
    parser.add_argument('--min-w', default=25, type=int, help='minimum width (default is 25)')
    parser.add_argument('--min-h', default=25, type=int, help='minimum height (default is 25)')
    return parser.parse_args()

def random_colour():
    red = random.randint(0, 255)
    green = random.randint(0, 255)
    blue = random.randint(0, 255)
    return (red << 16) | (green << 8) | blue

def generate_images(count, outdir, constraints):

    outpath = os.path.abspath(outdir)

    for i in range(count):

        width = random.randint(constraints[0], constraints[2])
        height = random.randint(constraints[1], constraints[3])
        colour = random_colour()
        img = Image.new('RGB', (width, height), colour)

        filename = '{0:003d}.png'.format(i)
        filepath = os.path.join(outpath, filename)

        print('{0} ({1}, {2}) colour:0x{3:06x}'.format(filepath, width, height, colour))

        with open(filepath, 'wb') as fp:
            img.save(fp)

def main():

    args = parse_args()
    generate_images(args.count, args.outdir,
        (args.min_w, args.min_h, args.max_w, args.max_h))

if __name__ == '__main__':
    main()
