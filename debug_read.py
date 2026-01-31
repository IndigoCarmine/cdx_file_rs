#!/usr/bin/env python3
import struct

def analyze_cdx(filepath):
    with open(filepath, 'rb') as f:
        data = f.read()
    
    print(f"File: {filepath}")
    print(f"Total size: {len(data)} bytes\n")
    
    # First 22 bytes are header
    header = data[:22]
    print(f"Header (22 bytes): {header.hex()}")
    print(f"Header as string: {header[:8]}")  # Should be "VjCD0100"
    
    offset = 22
    
    # Root object tag
    tag = struct.unpack_from('<H', data, offset)[0]
    print(f"\nRoot tag at offset {offset}: 0x{tag:04x}")
    print(f"  Tag & 0x8000: {tag & 0x8000} (should be non-zero)")
    print(f"  Is object: {(tag & 0x8000) != 0}")
    
    offset += 2
    
    # Root object ID
    obj_id = struct.unpack_from('<I', data, offset)[0]
    print(f"\nRoot object ID at offset {offset}: {obj_id}")
    
    offset += 4
    
    # Next tag (properties or children)
    next_tag = struct.unpack_from('<H', data, offset)[0]
    print(f"\nNext tag at offset {offset}: 0x{next_tag:04x}")
    print(f"  Is property: {(next_tag & 0x8000) == 0}")
    print(f"  Is object: {(next_tag & 0x8000) != 0}")
    
    if next_tag != 0x0000:
        if (next_tag & 0x8000) == 0:
            # Property
            offset += 2
            size_or_marker = struct.unpack_from('<H', data, offset)[0]
            print(f"  Property size/marker at offset {offset}: 0x{size_or_marker:04x} ({size_or_marker})")

if __name__ == '__main__':
    import sys
    
    files = [
        'sample_cdx/benzene.cdx',
        'sample_cdx/Reaction.cdx',
        'sample_cdx/Analysis.cdx',
    ]
    
    for f in files:
        try:
            analyze_cdx(f)
            print("\n" + "="*60 + "\n")
        except Exception as e:
            print(f"Error: {e}\n")
