
HORIZONTAL = True
VERTICLE = False

class BinPackingException(Exception): pass
class SplitException(BinPackingException): pass
class NoSpaceException(BinPackingException): pass

class RectangleNode:

    def __init__(self, x1, y1, x2, y2, parent=None, level=0):
        self.x1 = x1
        self.y1 = y1
        self.x2 = x2
        self.y2 = y2
        self._splitted = False
        self.right = None
        self.bottom = None
        self.parent = parent
        self.level = level
        self.leaf = False
        self._data = None
        self._content_width = 0
        self._content_height = 0
        self.padding = 0

    @property
    def splitted(self):
        return self._splitted

    @property
    def x(self):
        return self.x1 + (self.padding / 2)

    @property
    def y(self):
        return self.y1 + (self.padding / 2)

    @property
    def width(self):
        return self.x2 - self.x1

    @property
    def height(self):
        return self.y2 - self.y1

    def fits(self, width, height):
        return (width <= self.width) and (height <= self.height)

    @property
    def data(self):
        return self._data

    @property
    def empty(self):
        return self._data == None

    def insert(self, datum, width, height, padding=0):

        if self.empty:
            self._data = datum
            self._content_width = width
            self._content_height = height
            self.split()
            return True
        else:
            raise BinPackingException('Cannot insert, node occupied')

    def split(self):

        if self._splitted:
            raise SplitException('Node is already split')

        width = self._content_width
        height = self._content_height
        self.right = RectangleNode(self.x1 + width, self.y1, self.x2, self.y1 + height, parent=self, level=self.level+1)
        self.bottom = RectangleNode(self.x1, self.y1 + height, self.x2, self.y2, parent=self, level=self.level+1)
        self._splitted = True

    def resize(self, width, height):
        raise NotImplementedError()

    def __repr__(self):
        return '<l:{0} RectangleNode ({1}, {2}) ({3}, {4})>'.format(self.level, self.x1, self.y1, self.x2, self.y2)

class Packer:

    def __init__(self, width, height, padding=0):
        self.images = []
        self.root = RectangleNode(0, 0, width, height)
        self.padding = padding

    @property
    def width(self):
        return self.root.width

    @property
    def height(self):
        return self.root.height

    def add(self, image):
        self.images.append(image)

    def sort(self):
        self.images.sort(key=lambda im: -im.height)

    def pack(self):
        self.sort()
        right_list = [self.root]
        bottom_list = []

        for image in self.images:
            w, h = image.width + self.padding, image.height + self.padding
            inserted = False
            for node in right_list:
                if node.fits(w, h):
                    node.insert(image, w, h)
                    right_list.append(node.right)
                    bottom_list.append(node.bottom)
                    inserted = True
                    right_list = list(filter(lambda el: el != node, right_list))
                    break
            if not inserted:
                for node in bottom_list:
                    if node.fits(w, h):
                        node.insert(image, w, h)
                        right_list.append(node.right)
                        bottom_list.append(node.bottom)
                        inserted = True
                        bottom_list = list(filter(lambda el: el != node, bottom_list))
                        break
            if not inserted:
                raise NoSpaceException("Image couldn't be inserted")
