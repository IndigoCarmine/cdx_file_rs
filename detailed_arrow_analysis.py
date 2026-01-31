#!/usr/bin/env python3
"""
Detailed analysis of Arrow object properties in Reaction.cdx
"""
import struct

def detailed_arrow_analysis():
    with open('sample_cdx/Reaction.cdx', 'rb') as f:
        data = f.read()
    
    # First Arrow at offset 2546
    offset = 2546
    print("Analyzing Arrow at offset 2546\n")
    
    tag = struct.unpack_from('<H', data, offset)[0]
    obj_id = struct.unpack_from('<I', data, offset+2)[0]
    
    print(f"Tag: 0x{tag:04X}, ID: {obj_id}\n")
    
    # Start reading properties
    offset = offset + 6
    prop_num = 0
    
    print("Property stream:")
    for i in range(50):  # Read up to 50 properties
        if offset >= len(data) - 6:
            break
        
        # Check next 10 bytes as hex
        peek = ' '.join(f'{data[j]:02X}' for j in range(offset, min(offset+20, len(data))))
        
        # Try to read as property
        next_tag = struct.unpack_from('<H', data, offset)[0]
        next_len = struct.unpack_from('<I', data, offset+2)[0]
        
        print(f"\nOffset {offset} (0x{offset:04X}):")
        print(f"  Next bytes: {peek}")
        print(f"  Tag interpretation: 0x{next_tag:04X}, Len: {next_len}")
        
        if next_tag == 0x0000:
            print("  >>> END MARKER <<<")
            break
        
        if next_tag & 0x8000:
            print(f"  >>> CHILD OBJECT (tag 0x{next_tag:04X}) <<<")
            break
        
        # Check if this looks like a valid property
        if next_len > 1000000:  # Suspiciously large
            print(f"  WARNING: Length {next_len} seems too large")
            # Try alternative interpretation - maybe tag is different
            alt_tag = struct.unpack_from('<H', data, offset)[0]
            alt_len = struct.unpack_from('<H', data, offset+2)[0]
            alt_len2 = struct.unpack_from('<H', data, offset+4)[0]
            print(f"  Alt interpretation: u16={alt_tag:04X}, u16={alt_len:04X}, u16={alt_len2:04X}")
        
        if next_len > 0 and next_len < 200:
            prop_data = data[offset+6:offset+6+next_len]
            print(f"  Data: {prop_data[:32].hex()}")
            
            # Try to decode as string if looks like ASCII
            try:
                as_str = prop_data.decode('utf-8', errors='replace')
                if all(c.isprintable() or c.isspace() for c in as_str[:50]):
                    print(f"  As string: {repr(as_str[:100])}")
            except:
                pass
        
        # Move to next property
        if next_len < 1000000:
            offset += 6 + next_len
            prop_num += 1
        else:
            break
    
    print(f"\n{'='*80}")
    print(f"Total properties read: {prop_num}\n")

if __name__ == '__main__':
    detailed_arrow_analysis()
