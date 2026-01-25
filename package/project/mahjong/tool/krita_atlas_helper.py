from krita import *

def create_tiled_layers_with_masks(n, tile_size, x_offset=0, y_offset=0, padding=0):
    """
    n = "n x n tiles" (sqrt(number of tiles))
    tile_size = individual size of tile (pixels)
    x_offset = start x coordinate
    y_offset = start y coordinate
    padding = padding between tiles and origin
    """
    doc = Krita.instance().activeDocument()

    if not doc:
        raise Exception("No active document found")

    width = doc.width()
    height = doc.height()

    available_width = width - x_offset - padding * (tile_size - 1)
    available_height = height - y_offset - padding * (tile_size - 1)

    if available_width <= 0 or available_height <= 0:
        raise Exception("Offset and padding too large for canvas size")

    if tile_size <= 0:
        raise Exception("Resulting tile size is zero or negative")

    root = doc.rootNode()
    group = doc.createGroupLayer(f"Tiles (offset {x_offset},{y_offset}, padding {padding})")
    root.addChildNode(group, None)

    for row in range(n):
        for col in range(n):
            layer = doc.createNode(f"Tile_{row}_{col}", "grouplayer")
            group.addChildNode(layer, None)

            mask = doc.createTransparencyMask(f"TileMask_{row}_{col}")
            layer.addChildNode(mask, None)

            # Clear mask (make black)
            # 1 Byte per pixel due to mask channel being just single byte
            a = 0
            pixel = bytes([a])
            pixel_data = pixel * (width * height) # Fill all pixels
            qba = QByteArray(pixel_data) # Convert to QByteArray
            mask.setPixelData(qba, 0, 0, width, height)

            x = x_offset + col * (tile_size + padding)
            y = y_offset + row * (tile_size + padding)

            # 1 Byte per pixel due to mask channel being just single byte
            a = 255
            pixel = bytes([a])
            pixel_data = pixel * (tile_size * tile_size) # Fill all pixels
            qba = QByteArray(pixel_data) # Convert to QByteArray
            mask.setPixelData(qba, x, y, tile_size, tile_size)

# Usage
create_tiled_layers_with_masks(3, 256)
