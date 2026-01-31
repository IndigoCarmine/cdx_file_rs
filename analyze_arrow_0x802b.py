#!/usr/bin/env python3
"""
Binary analysis of Arrow (0x802B) objects
"""
import struct

def analyze_arrow_bytes():
    with open('sample_cdx/Reaction.cdx', 'rb') as f:
        data = f.read()
    
    print("Analyzing Arrow objects (tag 0x802B) in Reaction.cdx\n")
    
    # Find 0x802B tags
    arrow_tag = struct.pack('<H', 0x802B)
    
    positions = []
    offset = 0
    while offset < len(data) - 2:
        if data[offset:offset+2] == arrow_tag:
            positions.append(offset)
        offset += 1
    
    print(f"Found {len(positions)} Arrow objects at offsets: {positions}\n")
    
    for idx, pos in enumerate(positions):
        print(f"{'='*80}")
        print(f"Arrow Object #{idx+1} at byte offset {pos} (0x{pos:04X})")
        print(f"{'='*80}")
        
        # Parse object
        tag = struct.unpack_from('<H', data, pos)[0]
        obj_id = struct.unpack_from('<I', data, pos+2)[0]
        
        print(f"Tag: 0x{tag:04X}")
        print(f"ID: {obj_id}\n")
        
        # Dump hex with ASCII
        print("Raw bytes (hex dump):")
        start = pos
        end = min(pos + 160, len(data))
        
        for i in range(start, end, 16):
            hex_part = ' '.join(f'{data[j]:02X}' for j in range(i, min(i+16, end)))
            ascii_part = ''.join(chr(data[j]) if 32 <= data[j] < 127 else '.' for j in range(i, min(i+16, end)))
            offset_str = f"{i:04X}:"
            print(f"  {offset_str:<6} {hex_part:<48} {ascii_part}")
        
        # Parse properties
        print("\nProperties:")
        offset = pos + 6
        prop_num = 0
        
        while offset < len(data) - 6:
            next_tag = struct.unpack_from('<H', data, offset)[0]
            
            if next_tag == 0x0000:  # End marker
                print(f"  [End of properties at offset {offset}]")
                break
            
            if next_tag & 0x8000:  # Could be child object
                print(f"  [Child object at offset {offset}: tag=0x{next_tag:04X}]")
                break
            
            prop_len = struct.unpack_from('<I', data, offset+2)[0]
            prop_data = data[offset+6:offset+6+prop_len]
            
            prop_num += 1
            print(f"\n  Property #{prop_num}:")
            print(f"    Tag: 0x{next_tag:04X}")
            print(f"    Length: {prop_len}")
            print(f"    Data (hex): {' '.join(f'{b:02X}' for b in prop_data[:48])}")
            if prop_len > 48:
                print(f"              ... ({prop_len-48} more bytes)")
            
            # Try to interpret
            interpret_property(next_tag, prop_data)
            
            offset += 6 + prop_len
        
        print()

def interpret_property(tag, data):
    """Try to interpret common property types"""
    if tag == 0x0B61 and len(data) == 32:  # 2D_BOUNDS
        vals = struct.unpack('<dddd', data)
        print(f"    [2D_BOUNDS] ({vals[0]:.4f}, {vals[1]:.4f}) to ({vals[2]:.4f}, {vals[3]:.4f})")
    elif tag == 0x0B73 and len(data) == 24:  # 3D_HEAD
        vals = struct.unpack('<ddd', data)
        print(f"    [3D_HEAD] ({vals[0]:.4f}, {vals[1]:.4f}, {vals[2]:.4f})")
    elif tag == 0x0B74 and len(data) == 24:  # 3D_TAIL
        vals = struct.unpack('<ddd', data)
        print(f"    [3D_TAIL] ({vals[0]:.4f}, {vals[1]:.4f}, {vals[2]:.4f})")
    elif tag == 0x0B75 and len(data) == 24:  # 3D_CENTER
        vals = struct.unpack('<ddd', data)
        print(f"    [3D_CENTER] ({vals[0]:.4f}, {vals[1]:.4f}, {vals[2]:.4f})")
    elif tag == 0x0B76 and len(data) == 24:  # 3D_MAJOR_AXIS_END
        vals = struct.unpack('<ddd', data)
        print(f"    [3D_MAJOR_AXIS_END] ({vals[0]:.4f}, {vals[1]:.4f}, {vals[2]:.4f})")
    elif tag == 0x0B77 and len(data) == 24:  # 3D_MINOR_AXIS_END
        vals = struct.unpack('<ddd', data)
        print(f"    [3D_MINOR_AXIS_END] ({vals[0]:.4f}, {vals[1]:.4f}, {vals[2]:.4f})")
    elif tag == 0x0A00 and len(data) == 4:  # Z_ORDER
        val = struct.unpack('<i', data)[0]
        print(f"    [Z_ORDER] {val}")
    elif tag == 0x0002 and len(data) == 2:  # COLOR
        val = struct.unpack('<h', data)[0]
        print(f"    [COLOR] {val}")
    elif tag == 0x0B7E and len(data) == 2:  # LINE_WIDTH
        val = struct.unpack('<h', data)[0]
        print(f"    [LINE_WIDTH] {val}")
    elif tag == 0x0BB4:  # FILL_TYPE (string)
        try:
            text = data.decode('utf-8', errors='replace').rstrip('\x00')
            if text:
                print(f"    [FILL_TYPE] \"{text}\"")
        except:
            pass
    elif tag == 0x0BA0:  # ARROWHEAD_HEAD (string)
        try:
            text = data.decode('utf-8', errors='replace').rstrip('\x00')
            if text:
                print(f"    [ARROWHEAD_HEAD] \"{text}\"")
        except:
            pass
    elif tag == 0x0BA1:  # ARROWHEAD_TYPE (string)
        try:
            text = data.decode('utf-8', errors='replace').rstrip('\x00')
            if text:
                print(f"    [ARROWHEAD_TYPE] \"{text}\"")
        except:
            pass

if __name__ == '__main__':
    analyze_arrow_bytes()
