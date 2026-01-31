#!/usr/bin/env python3
import struct
import os

def analyze_graphic_object(filename):
    """Analyze the Graphic object in detail"""
    with open(filename, 'rb') as f:
        data = f.read()
    
    print(f"=== Analyzing Graphic Object in {filename} ===\n")
    
    # Find Graphic object at 0x001112
    pos = 0x001112
    
    # Read tag and length
    tag = struct.unpack('<H', data[pos:pos+2])[0]
    length = struct.unpack('<I', data[pos+2:pos+6])[0]
    
    print(f"Graphic Object at [0x{pos:06x}]:")
    print(f"  Tag: 0x{tag:04x}")
    print(f"  Length: {length} bytes")
    print(f"  Data ends at: 0x{pos+6+length:06x}\n")
    
    # Print the object data
    print("Object data (hex dump):")
    obj_data = data[pos:pos+6+length]
    for i in range(0, len(obj_data), 16):
        hex_str = ' '.join(f'{b:02x}' for b in obj_data[i:i+16])
        ascii_str = ''.join(chr(b) if 32 <= b < 127 else '.' for b in obj_data[i:i+16])
        print(f"  {pos+i:06x}: {hex_str:48s} {ascii_str}")
    
    # Now parse the properties inside
    print("\n=== Properties in Graphic Object ===")
    
    prop_pos = pos + 6  # Skip object header
    prop_end = pos + 6 + length
    
    while prop_pos < prop_end - 6:
        try:
            prop_tag = struct.unpack('<H', data[prop_pos:prop_pos+2])[0]
            prop_length = struct.unpack('<I', data[prop_pos+2:prop_pos+6])[0]
            
            if prop_tag >= 0x8000:
                # It's a nested object
                print(f"  [0x{prop_pos:06x}] Nested Object 0x{prop_tag:04x}, length={prop_length}")
            else:
                # It's a property
                print(f"  [0x{prop_pos:06x}] Property 0x{prop_tag:04x}, length={prop_length}")
                
                # Show the value in hex
                if prop_length <= 50:
                    val_hex = ' '.join(f'{b:02x}' for b in data[prop_pos+6:prop_pos+6+prop_length])
                    print(f"        Value: {val_hex}")
            
            prop_pos += 6 + prop_length
        except Exception as e:
            print(f"  Error at 0x{prop_pos:06x}: {e}")
            break

# Analyze
if os.path.exists('sample_cdx/Reaction.cdx'):
    analyze_graphic_object('sample_cdx/Reaction.cdx')
