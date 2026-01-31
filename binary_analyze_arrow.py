#!/usr/bin/env python3
"""
Binary analysis of Arrow objects - find and dump raw bytes
"""
import struct

def analyze_reaction_cdx():
    with open('sample_cdx/Reaction.cdx', 'rb') as f:
        data = f.read()
    
    print("Searching for Arrow objects (tag 0x8027) in Reaction.cdx\n")
    
    # Search for the tag 0x8027 in the file
    arrow_tag = struct.pack('<H', 0x8027)
    
    positions = []
    offset = 0
    while offset < len(data) - 2:
        if data[offset:offset+2] == arrow_tag:
            positions.append(offset)
        offset += 1
    
    print(f"Found {len(positions)} occurrences of tag 0x8027 at byte offsets:")
    for i, pos in enumerate(positions):
        print(f"  {i+1}. Byte {pos} (0x{pos:04X})")
    
    # Analyze each Arrow object
    for idx, pos in enumerate(positions):
        print(f"\n{'='*70}")
        print(f"Arrow Object #{idx+1} at byte offset {pos}")
        print(f"{'='*70}")
        
        # Tag at pos should be 0x8027
        tag = struct.unpack_from('<H', data, pos)[0]
        print(f"Tag: 0x{tag:04X}")
        
        # Next 4 bytes are ID (u32)
        obj_id = struct.unpack_from('<I', data, pos+2)[0]
        print(f"ID: {obj_id}")
        
        # Show next 100 bytes as hex dump
        start = pos
        end = min(pos + 100, len(data))
        print(f"\nBytes {pos} to {end-1}:")
        
        for i in range(start, end, 16):
            hex_part = ' '.join(f'{data[j]:02X}' for j in range(i, min(i+16, end)))
            ascii_part = ''.join(chr(data[j]) if 32 <= data[j] < 127 else '.' for j in range(i, min(i+16, end)))
            print(f"  {i:04X}: {hex_part:<48} {ascii_part}")
        
        # Try to parse properties
        offset = pos + 6  # Skip tag (2) + id (4)
        prop_count = 0
        
        print(f"\nProperties:")
        while offset < len(data) - 6 and prop_count < 10:
            prop_tag = struct.unpack_from('<H', data, offset)[0]
            
            if prop_tag == 0x0000:  # End marker
                print(f"  End marker (0x0000) at offset {offset}")
                break
            
            if prop_tag & 0x8000:  # Child object marker
                print(f"  [End of properties, child object follows]")
                break
            
            # Read property length
            prop_len = struct.unpack_from('<I', data, offset+2)[0]
            prop_data = data[offset+6:offset+6+prop_len]
            
            print(f"  0x{prop_tag:04X}: len={prop_len}, data={prop_data[:20].hex()}{'...' if prop_len > 20 else ''}")
            
            offset += 6 + prop_len
            prop_count += 1

if __name__ == '__main__':
    analyze_reaction_cdx()
