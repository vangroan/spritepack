
from packer import Packer

from PIL import Image, ImageDraw

from argparse import ArgumentParser
import os

def parse_args():

    parser = ArgumentParser(description="Sprite packer")

    parser.add_argument('directory', help='Directory containing image files')

    return parser.parse_args()

def load(packer, directory):
    for root, dirnames, filenames in os.walk(directory):
        for filename in filenames:
            filepath = os.path.join(root, filename)
            with open(filepath, 'rb') as fp:
                img = Image.open(filepath)
                img.load()
                packer.add(img)

def save(packer, outfile):

    rgb = ['red', 'green', 'blue']

    def draw_debug_rect(draw, node):
        nonlocal rgb
        if not node:
            return
        color = rgb[node.level % 3]
        draw.rectangle(((node.x1, node.y1), (node.x2, node.y2)),
            fill=None, outline=color)
        draw_debug_rect(draw, node.right)
        draw_debug_rect(draw, node.bottom)

    img = Image.new('RGBA', (packer.width, packer.height), color='white')
    draw = ImageDraw.ImageDraw(img)

    

    draw_debug_rect(draw, packer.root)
    with open(outfile, 'wb') as fp:
        img.save(fp)

def main():
    args = parse_args()
    packer = Packer(512, 512)
    load(packer, args.directory)
    packer.pack()
    save(packer, 'test.png')

if __name__ == '__main__':
    main()
