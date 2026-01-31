#!/usr/bin/env python3
import struct

def parse_properties_carefully(filename, obj_offset):
    """Parse properties of the Graphic object carefully"""
    with open(filename, 'rb') as f:
        data = f.read()
    
    # Graphic object at 0x001112
    pos = obj_offset
    tag = struct.unpack('<H', data[pos:pos+2])[0]
    length = struct.unpack('<I', data[pos+2:pos+6])[0]
    
    print(f"=== Object at 0x{pos:06x} ===")
    print(f"Tag: 0x{tag:04x}, Length: {length}\n")
    
    # Now parse properties that follow
    prop_pos = pos + 6
    prop_end = pos + 6 + length
    
    print(f"Properties from 0x{prop_pos:06x} to 0x{prop_end:06x}:\n")
    
    prop_num = 0
    while prop_pos < prop_end - 6:
        try:
            prop_tag = struct.unpack('<H', data[prop_pos:prop_pos+2])[0]
            prop_len = struct.unpack('<I', data[prop_pos+2:prop_pos+6])[0]
            
            prop_num += 1
            print(f"Property {prop_num}: Tag=0x{prop_tag:04x}, Length={prop_len} bytes")
            
            if prop_tag >= 0x8000:
                print(f"  -> This is a NESTED OBJECT")
            else:
                # Show the value
                if prop_len <= 50:
                    val_hex = ' '.join(f'{b:02x}' for b in data[prop_pos+6:prop_pos+6+prop_len])
                    print(f"  Data: {val_hex}")
                    
                    # Try to interpret based on tag
                    if prop_tag == 0x0204:  # Bounding box
                        if prop_len >= 4:
                            left = struct.unpack('<d', data[prop_pos+6:prop_pos+14])[0]
                            print(f"  -> Bbox left={left}")
                    elif prop_tag == 0x0207:  # Head 3D
                        if prop_len >= 24:
                            x = struct.unpack('<d', data[prop_pos+6:prop_pos+14])[0]
                            y = struct.unpack('<d', data[prop_pos+14:prop_pos+22])[0]
                            z = struct.unpack('<d', data[prop_pos+22:prop_pos+30])[0]
                            print(f"  -> Head3D: x={x}, y={y}, z={z}")
            
            prop_pos += 6 + prop_len
            
            if prop_pos > prop_end:
                print(f"\nWARNING: Overread properties!")
                break
        except struct.error as e:
            print(f"Error at 0x{prop_pos:06x}: {e}")
            break

parse_properties_carefully('sample_cdx/Reaction.cdx', 0x001112)
