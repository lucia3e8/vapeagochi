from PIL import Image
import numpy as np
import argparse
from pathlib import Path

def parse_c_array(text):
    """Parse a C-style array of hex values into a list of integers.
    
    Example input: "{0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF}"
    Returns: [255, 255, 255, 255, 255, 255, 255]
    """
    # Remove curly braces and whitespace
    text = text.strip('{}').strip()
    
    # Split on commas and process each value
    values = []
    if text:
        for val in text.split(','):
            # Clean up whitespace and convert hex to int
            val = val.strip()
            if val.startswith('0x'):
                values.append(int(val, 16))
            else:
                values.append(int(val))
                
    return values


def read_bmp_to_c_array(filepath):
    """Read a BMP file and convert it to a C-style array string of hex values.
    
    Args:
        filepath: Path to the BMP file
        
    Returns:
        String containing C-style array of hex values representing the image data
    """
    img = Image.open(filepath).convert('1')  # Convert to black and white
    
    # Check image dimensions
    if img.size != (128, 64):
        print(f"ඞඞඞ image size {img.size} is sus! should be 128x64 pixels")
    
    # Convert to numpy array and flatten
    pixels = np.array(img, dtype=np.uint8).flatten()

    # each non-zero pixel becomes a 1, otherwise 0
    bits = (pixels > 0)

    # pack 8 bits, representing 8 consecutive pixels, into each byte
    bits = np.packbits(bits)
    
    return bits

def main():
    # Set up argument parser
    parser = argparse.ArgumentParser(description='Convert BMP image to C array header file')
    parser.add_argument('-i', '--input', required=True,
                       help='Path to input BMP file')
    
    args = parser.parse_args()
    inpath = Path(args.input)

    hex_values = []
    arr = read_bmp_to_c_array(inpath)
    if len(arr) != 1024:
        print(f"ඞඞඞ bitpacked array length {len(arr)} is sus! should be 1024 (128 high * 64 wide / 8 pixels per byte)")
    
    for i, x in enumerate(arr):
        hex_values.append(f"0x{x:02X}")
    
    c_array = ",\n    ".join([", ".join(hex_values[i:i+16]) for i in range(0, len(hex_values), 16)])

    filename = inpath.stem
    c_const = f"const unsigned char image_data_{filename}[] = {{\n    {c_array}\n}};"

    print(c_const)

if __name__ == '__main__':
    main()

    
