
from packer import Packer

from PIL import Image, ImageDraw

from argparse import ArgumentParser
import os

# File extension whileist
EXTENSIONS = ['.png', '.jpeg']

def parse_args():

    parser = ArgumentParser(description="Sprite packer")

    parser.add_argument('directory', help='Directory containing image files')
    parser.add_argument('-p', dest='padding', type=int, default=0, help='Padded pixels around images')

    return parser.parse_args()

def validate_filename(filename):
    for ext in EXTENSIONS:
        if filename.endswith(ext):
            return True
    return False

def load(packer, directory):
    for root, dirnames, filenames in os.walk(directory):
        for filename in filenames:
            if not validate_filename(filename):
                continue
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

    def paste_into(img, node):

        if not node:
            return

        if node.empty:
            return

        img.paste(node.data, (node.x1, node.y1))
        paste_into(img, node.right)
        paste_into(img, node.bottom)

    img = Image.new('RGBA', (packer.width, packer.height), color='white')
    draw = ImageDraw.ImageDraw(img)
    paste_into(img, packer.root)
    draw_debug_rect(draw, packer.root)
    with open(outfile, 'wb') as fp:
        img.save(fp)

def main():
    args = parse_args()
    packer = Packer(512, 512, padding=args.padding)
    load(packer, args.directory)
    packer.pack()
    save(packer, 'test.png')

if __name__ == '__main__':
    main()
