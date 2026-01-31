#!/usr/bin/env python3
import struct
import os

def read_cdx_header(filename):
    """Read CDX file and analyze arrow objects"""
    with open(filename, 'rb') as f:
        # Read CDXML header
        header = f.read(2)
        print(f"File Header: {header}")
        
        # Read rest of file
        data = f.read()
    
    return data

def parse_cdx_objects(data, filename):
    """Parse CDX objects and find arrows"""
    print(f"\n=== Analyzing {filename} ===")
    print(f"File size: {len(data)} bytes\n")
    
    pos = 0
    object_count = {}
    arrow_objects = []
    graphic_objects = []
    
    while pos < len(data) - 6:
        try:
            # Read tag (u16 little-endian)
            tag = struct.unpack('<H', data[pos:pos+2])[0]
            # Read length (u32 little-endian)
            length = struct.unpack('<I', data[pos+2:pos+6])[0]
            
            # Known object tags
            if tag == 0x8027:  # Arrow object
                print(f"[{pos:06x}] ARROW OBJECT (0x8027): length={length}")
                arrow_objects.append((pos, length))
                object_count[0x8027] = object_count.get(0x8027, 0) + 1
            elif tag == 0x8007:  # Graphic object
                print(f"[{pos:06x}] GRAPHIC OBJECT (0x8007): length={length}")
                graphic_objects.append((pos, length))
                object_count[0x8007] = object_count.get(0x8007, 0) + 1
            elif tag == 0x8003:  # Fragment
                object_count[0x8003] = object_count.get(0x8003, 0) + 1
            elif tag == 0x8004:  # Node
                object_count[0x8004] = object_count.get(0x8004, 0) + 1
            elif tag == 0x8005:  # Bond
                object_count[0x8005] = object_count.get(0x8005, 0) + 1
            elif tag == 0x8006:  # Text
                object_count[0x8006] = object_count.get(0x8006, 0) + 1
            elif tag == 0x8001:  # Page
                object_count[0x8001] = object_count.get(0x8001, 0) + 1
            elif tag == 0x8000:  # Document
                object_count[0x8000] = object_count.get(0x8000, 0) + 1
            elif tag < 0x8000:  # It's a property
                pass
            else:
                # Unknown object tag
                pass
            
            pos += 6 + length
        except Exception as e:
            print(f"Error at position {pos:06x}: {e}")
            break
    
    print(f"\n=== Summary ===")
    tag_names = {
        0x8000: "Document",
        0x8001: "Page",
        0x8003: "Fragment",
        0x8004: "Node",
        0x8005: "Bond",
        0x8006: "Text",
        0x8007: "Graphic",
        0x8027: "Arrow",
    }
    
    for tag, count in sorted(object_count.items()):
        name = tag_names.get(tag, f"Unknown {tag:04x}")
        print(f"{name:20} (0x{tag:04x}): {count}")
    
    print(f"\nArrow objects found: {len(arrow_objects)}")
    print(f"Graphic objects found: {len(graphic_objects)}")
    
    return arrow_objects, graphic_objects

# Analyze Reaction.cdx
if os.path.exists('sample_cdx/Reaction.cdx'):
    data = read_cdx_header('sample_cdx/Reaction.cdx')
    arrows, graphics = parse_cdx_objects(data, 'Reaction.cdx')
    
    if arrows:
        print("\n=== ARROW OBJECT DETAILS ===")
        for pos, length in arrows[:1]:  # Show first arrow in detail
            print(f"Arrow at offset {pos:06x}, length {length}")
            # Print hex dump
            start = pos
            end = min(pos + length + 100, len(data))
            hex_data = data[start:end]
            for i in range(0, min(len(hex_data), 200), 16):
                hex_str = ' '.join(f'{b:02x}' for b in hex_data[i:i+16])
                ascii_str = ''.join(chr(b) if 32 <= b < 127 else '.' for b in hex_data[i:i+16])
                print(f"  {start+i:06x}: {hex_str:48s} {ascii_str}")
else:
    print("sample_cdx/Reaction.cdx not found")
