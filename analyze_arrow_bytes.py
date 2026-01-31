#!/usr/bin/env python3
"""
Analyze Arrow (0x8027) object structure in Reaction.cdx
"""
import struct

def read_le_u16(data, offset):
    """Read little-endian u16"""
    return struct.unpack_from('<H', data, offset)[0], offset + 2

def read_le_u32(data, offset):
    """Read little-endian u32"""
    return struct.unpack_from('<I', data, offset)[0], offset + 4

def read_property(data, offset, prop_tag):
    """Read a property value"""
    data_len, offset = read_le_u32(data, offset)
    
    if offset + data_len > len(data):
        return None, offset
    
    prop_data = data[offset:offset+data_len]
    offset += data_len
    
    return (prop_tag, prop_data), offset

def find_arrow_objects(data, offset, obj_tag, depth=0):
    """Recursively find and display Arrow objects"""
    indent = "  " * depth
    
    obj_id, offset = read_le_u32(data, offset)
    
    if obj_tag == 0x8027:  # Arrow
        print(f"\n{'='*70}")
        print(f"{indent}>>> FOUND ARROW OBJECT ID: {obj_id}")
        print(f"{'='*70}")
        
        # Read all properties
        props = []
        while offset < len(data):
            next_tag, new_offset = read_le_u16(data, offset)
            
            if next_tag == 0x0000:  # EndObject
                offset = new_offset
                break
            
            if next_tag & 0x8000:  # Child object
                print(f"{indent}  [Child object: 0x{next_tag:04X}]")
                offset = find_arrow_objects(data, new_offset, next_tag, depth+1)
            else:  # Property
                prop_result, offset = read_property(data, new_offset, next_tag)
                if prop_result:
                    prop_tag, prop_data = prop_result
                    props.append((prop_tag, prop_data))
                    print_property(indent, prop_tag, prop_data)
        
        return offset
    else:
        # Read properties without printing (non-Arrow objects)
        while offset < len(data):
            next_tag, new_offset = read_le_u16(data, offset)
            
            if next_tag == 0x0000:  # EndObject
                offset = new_offset
                break
            
            if next_tag & 0x8000:  # Child object
                offset = find_arrow_objects(data, new_offset, next_tag, depth+1)
            else:  # Property
                prop_result, offset = read_property(data, new_offset, next_tag)
        
        return offset

def print_property(indent, prop_tag, prop_data):
    """Pretty print a property"""
    print(f"{indent}  Prop 0x{prop_tag:04X} (len={len(prop_data)})")
    
    # Show hex dump
    hex_str = ' '.join(f'{b:02X}' for b in prop_data[:32])
    if len(prop_data) > 32:
        hex_str += f' ... ({len(prop_data)} bytes total)'
    print(f"{indent}    Hex: {hex_str}")
    
    # Try to interpret common property types
    if prop_tag == 0x0B61 and len(prop_data) == 32:  # 2D_BOUNDS
        vals = struct.unpack('<dddd', prop_data)
        print(f"{indent}    [2D_BOUNDS] x1={vals[0]:.2f}, y1={vals[1]:.2f}, x2={vals[2]:.2f}, y2={vals[3]:.2f}")
    elif prop_tag == 0x0B73 and len(prop_data) == 24:  # 3D_HEAD
        vals = struct.unpack('<ddd', prop_data)
        print(f"{indent}    [3D_HEAD] x={vals[0]:.2f}, y={vals[1]:.2f}, z={vals[2]:.2f}")
    elif prop_tag == 0x0B74 and len(prop_data) == 24:  # 3D_TAIL
        vals = struct.unpack('<ddd', prop_data)
        print(f"{indent}    [3D_TAIL] x={vals[0]:.2f}, y={vals[1]:.2f}, z={vals[2]:.2f}")
    elif prop_tag == 0x0B75 and len(prop_data) == 24:  # 3D_CENTER
        vals = struct.unpack('<ddd', prop_data)
        print(f"{indent}    [3D_CENTER] x={vals[0]:.2f}, y={vals[1]:.2f}, z={vals[2]:.2f}")
    elif prop_tag == 0x0B76 and len(prop_data) == 24:  # 3D_MAJOR_AXIS_END
        vals = struct.unpack('<ddd', prop_data)
        print(f"{indent}    [3D_MAJOR_AXIS_END] x={vals[0]:.2f}, y={vals[1]:.2f}, z={vals[2]:.2f}")
    elif prop_tag == 0x0B77 and len(prop_data) == 24:  # 3D_MINOR_AXIS_END
        vals = struct.unpack('<ddd', prop_data)
        print(f"{indent}    [3D_MINOR_AXIS_END] x={vals[0]:.2f}, y={vals[1]:.2f}, z={vals[2]:.2f}")
    elif prop_tag == 0x0A00 and len(prop_data) == 4:  # Z_ORDER
        val = struct.unpack('<i', prop_data)[0]
        print(f"{indent}    [Z_ORDER] {val}")
    elif prop_tag == 0x0002 and len(prop_data) == 2:  # COLOR
        val = struct.unpack('<h', prop_data)[0]
        print(f"{indent}    [COLOR] {val}")
    elif prop_tag == 0x0B7E and len(prop_data) == 2:  # LINE_WIDTH
        val = struct.unpack('<h', prop_data)[0]
        print(f"{indent}    [LINE_WIDTH] {val}")
    elif prop_tag == 0x0BB4:  # FILL_TYPE (string)
        try:
            text = prop_data.decode('utf-8', errors='replace').rstrip('\x00')
            print(f"{indent}    [FILL_TYPE] {text}")
        except:
            pass
    elif prop_tag == 0x0BA0:  # ARROWHEAD_HEAD (string)
        try:
            text = prop_data.decode('utf-8', errors='replace').rstrip('\x00')
            print(f"{indent}    [ARROWHEAD_HEAD] {text}")
        except:
            pass
    elif prop_tag == 0x0BA1:  # ARROWHEAD_TYPE (string)
        try:
            text = prop_data.decode('utf-8', errors='replace').rstrip('\x00')
            print(f"{indent}    [ARROWHEAD_TYPE] {text}")
        except:
            pass

def main():
    with open('sample_cdx/Reaction.cdx', 'rb') as f:
        data = f.read()
    
    print(f"Analyzing Reaction.cdx ({len(data)} bytes)")
    print(f"CDX Header: {data[:8]}")
    
    # Skip header (22 bytes)
    offset = 22
    root_tag, offset = read_le_u16(data, offset)
    print(f"Root tag: 0x{root_tag:04X}\n")
    
    # Recursively find Arrow objects
    find_arrow_objects(data, offset, root_tag)

if __name__ == '__main__':
    main()
