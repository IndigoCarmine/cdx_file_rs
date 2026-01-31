#!/usr/bin/env python3
import struct
import sys

def read_u16_le(data, offset):
    return struct.unpack_from('<H', data, offset)[0]

def read_u32_le(data, offset):
    return struct.unpack_from('<I', data, offset)[0]

def analyze_object(data, offset, depth=0):
    indent = "  " * depth
    
    # Read tag
    tag = read_u16_le(data, offset)
    offset += 2
    
    print(f"{indent}Object Tag: 0x{tag:04x}")
    
    # Read ID
    obj_id = read_u32_le(data, offset)
    offset += 4
    print(f"{indent}Object ID: {obj_id}")
    
    properties = []
    children = []
    
    while True:
        if offset >= len(data):
            break
            
        next_tag = read_u16_le(data, offset)
        offset += 2
        
        # EndObject marker
        if next_tag == 0x0000:
            print(f"{indent}End of object")
            break
        
        if next_tag & 0x8000 == 0:
            # Property
            size_marker = read_u16_le(data, offset)
            offset += 2
            
            if size_marker == 0xFFFF:
                actual_size = read_u32_le(data, offset)
                offset += 4
                prop_data = data[offset:offset + actual_size]
                offset += actual_size
            else:
                prop_data = data[offset:offset + size_marker]
                offset += size_marker
            
            properties.append((next_tag, len(prop_data), prop_data))
            
            # 特定のプロパティを詳しく見る
            if next_tag == 0x0700:  # CDXPROP_TEXT
                print(f"{indent}  Property 0x{next_tag:04x} (TEXT): size={len(prop_data)}")
                try:
                    text = prop_data.decode('utf-8')
                    print(f"{indent}    UTF-8: '{text}'")
                except:
                    print(f"{indent}    NOT UTF-8! Hex: {prop_data[:20].hex()}")
            elif next_tag == 0x0503:  # CDXPROP_MOLE_FORMULA
                print(f"{indent}  Property 0x{next_tag:04x} (FORMULA): size={len(prop_data)}")
                try:
                    text = prop_data.decode('utf-8')
                    print(f"{indent}    UTF-8: '{text}'")
                except:
                    print(f"{indent}    NOT UTF-8! Hex: {prop_data[:20].hex()}")
            else:
                print(f"{indent}  Property 0x{next_tag:04x}: size={len(prop_data)}, hex={prop_data[:8].hex()}")
        else:
            # Child object
            print(f"{indent}Child object:")
            offset = analyze_object(data, offset - 2, depth + 1)
    
    print(f"{indent}Total properties: {len(properties)}, children: {len(children)}")
    return offset

def main():
    files = [
        ('sample_cdx/Reaction.cdx', 'Reaction'),
        ('sample_cdx/Analysis.cdx', 'Analysis'),
    ]
    
    for filepath, name in files:
        print(f"\n{'='*60}")
        print(f"Analyzing: {name}")
        print('='*60)
        
        with open(filepath, 'rb') as f:
            data = f.read()
        
        print(f"File size: {len(data)} bytes\n")
        
        # Skip header (22 bytes)
        print("Header (22 bytes):", data[:22].hex())
        
        # Start analyzing from root object
        print("\nRoot Object Analysis:")
        analyze_object(data, 22)

if __name__ == '__main__':
    main()
