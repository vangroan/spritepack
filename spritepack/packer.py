
HORIZONTAL = True
VERTICLE = False

class RectangleNode:

    def __init__(self, x1, y1, x2, y2, parent=None, level=0):
        self.x1 = x1
        self.y1 = y1
        self.x2 = x2
        self.y2 = y2
        self.split = False
        self.right = None
        self.bottom = None
        self.parent = parent
        self.level = level
        self.leaf = False
        self._data = None

    @property
    def x(self):
        return self.x1

    @property
    def y(self):
        return self.y1

    @property
    def width(self):
        return self.x2 - self.x1

    @property
    def height(self):
        return self.y2 - self.y1

    @property
    def data(self):
        return self._data

    @property
    def empty(self):
        return self._data == None

    def insert(self, datum, width, height):
        self._data = datum
        self.right = RectangleNode(self.x1 + width, self.y1, self.x2, self.y1 + height, parent=self, level=self.level+1)
        self.bottom = RectangleNode(self.x1, self.y1 + height, self.x2, self.y2, parent=self, level=self.level+1)

    def resize(self, width, height):
        raise NotImplementedError()

    def __repr__(self):
        return '<l:{0} RectangleNode ({1}, {2}) ({3}, {4})>'.format(self.level, self.x1, self.y1, self.x2, self.y2)

class Packer:

    def __init__(self, width, height):
        self.images = []
        self.root = RectangleNode(0, 0, width, height)

    @property
    def width(self):
        return self.root.width

    @property
    def height(self):
        return self.root.height

    def add(self, image):
        self.images.append(image)

    def pack(self):

        right = self.root
        bottom = None

        for image in self.images:
            if right.empty:
                right.insert(image, image.width, image.height)
                bottom = right.bottom
                right = right.right
            elif bottom != None and bottom.empty:
                bottom.insert(image, image.width, image.height)
                right = bottom.right
                bottom = bottom.bottom
