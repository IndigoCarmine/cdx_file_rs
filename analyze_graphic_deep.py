#!/usr/bin/env python3
import struct
import os

def find_all_objects_deep(filename):
    """Deep parse CDX to find all objects with their properties"""
    with open(filename, 'rb') as f:
        data = f.read()
    
    print(f"=== Deep Parse of {filename} ===\n")
    
    # Find Graphic object at 0x001112
    pos = 0x001112
    tag = struct.unpack('<H', data[pos:pos+2])[0]
    length = struct.unpack('<I', data[pos+2:pos+6])[0]
    
    print(f"Graphic Object at [0x{pos:06x}]:")
    print(f"  Tag: 0x{tag:04x}")
    print(f"  Length: {length} bytes")
    print(f"\nHex dump (first 200 bytes):")
    
    obj_start = pos
    obj_data = data[obj_start:obj_start+min(200, 6+length)]
    for i in range(0, len(obj_data), 16):
        hex_str = ' '.join(f'{b:02x}' for b in obj_data[i:i+16])
        ascii_str = ''.join(chr(b) if 32 <= b < 127 else '.' for b in obj_data[i:i+16])
        print(f"  {obj_start+i:06x}: {hex_str:48s} {ascii_str}")
    
    print("\n=== Property Analysis ===")
    
    # Manual parsing
    # After the object header (6 bytes), we have properties
    prop_start = obj_start + 6
    prop_end = obj_start + 6 + length
    
    # Try to parse the first few properties manually
    print(f"\nStarting property parse at 0x{prop_start:06x}:")
    
    # Property 1: 0x0013
    p_tag = struct.unpack('<H', data[prop_start:prop_start+2])[0]
    p_len = struct.unpack('<I', data[prop_start+2:prop_start+6])[0]
    print(f"\n[0x{prop_start:06x}] Property 0x{p_tag:04x}, length={p_len}")
    
    # This might be pointing to nested objects. Check the data
    p_data = data[prop_start+6:prop_start+6+min(100, p_len)]
    print("First 100 bytes of this property:")
    for i in range(0, len(p_data), 16):
        hex_str = ' '.join(f'{b:02x}' for b in p_data[i:i+16])
        ascii_str = ''.join(chr(b) if 32 <= b < 127 else '.' for b in p_data[i:i+16])
        print(f"  {prop_start+6+i:06x}: {hex_str:48s} {ascii_str}")
    
    # Look for nested objects inside this property
    if p_len > 6:
        nested_pos = prop_start + 6
        nested_end = prop_start + 6 + min(p_len, 100)
        
        print("\nLooking for nested objects in property 0x0013:")
        while nested_pos < nested_end - 6:
            try:
                nested_tag = struct.unpack('<H', data[nested_pos:nested_pos+2])[0]
                nested_len = struct.unpack('<I', data[nested_pos+2:nested_pos+6])[0]
                
                if nested_tag >= 0x8000:
                    print(f"  [0x{nested_pos:06x}] NESTED OBJECT 0x{nested_tag:04x}, length={nested_len}")
                elif nested_tag < 0x0100:
                    print(f"  [0x{nested_pos:06x}] Property 0x{nested_tag:04x}, length={nested_len}")
                    if nested_len <= 20:
                        val_hex = ' '.join(f'{b:02x}' for b in data[nested_pos+6:nested_pos+6+nested_len])
                        print(f"         Value: {val_hex}")
                
                nested_pos += 6 + nested_len
            except:
                break

if os.path.exists('sample_cdx/Reaction.cdx'):
    find_all_objects_deep('sample_cdx/Reaction.cdx')
