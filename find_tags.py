#!/usr/bin/env python3
"""
Find all object tags in the Reaction.cdx file
"""
import struct

def find_all_tags():
    with open('sample_cdx/Reaction.cdx', 'rb') as f:
        data = f.read()
    
    print("Scanning for object tags (tags with high bit set: >= 0x8000)\n")
    
    # Skip header
    offset = 22
    
    # Read root tag
    root_tag = struct.unpack_from('<H', data, offset)[0]
    print(f"Root tag: 0x{root_tag:04X}\n")
    
    found_tags = {}
    
    # Simple scan for u16 values >= 0x8000
    for i in range(22, len(data) - 2, 2):
        val = struct.unpack_from('<H', data, i)[0]
        if val >= 0x8000:  # Could be a tag
            if val not in found_tags:
                found_tags[val] = []
            found_tags[val].append(i)
    
    print("All tags found (>= 0x8000):")
    for tag in sorted(found_tags.keys()):
        count = len(found_tags[tag])
        decimal = tag
        print(f"  0x{tag:04X} ({decimal:5d}): {count} times")
    
    # Check specific tags we expect
    print("\nExpected tags:")
    expected = [0x8000, 0x8001, 0x8003, 0x8004, 0x8005, 0x8006, 0x8007, 0x8027, 0x802B]
    for tag in expected:
        if tag in found_tags:
            print(f"  ✓ 0x{tag:04X} ({tag:5d}): {len(found_tags[tag])} times")
        else:
            print(f"  ✗ 0x{tag:04X} ({tag:5d}): not found")

if __name__ == '__main__':
    find_all_tags()
