#!/usr/bin/env python3
import struct
import os

def analyze_cdx_structure(filename):
    """Analyze CDX file structure in detail"""
    with open(filename, 'rb') as f:
        data = f.read()
    
    print(f"=== Analyzing {filename} ===")
    print(f"File size: {len(data)} bytes\n")
    
    # Print first 200 bytes as hex
    print("First 200 bytes (hex dump):")
    for i in range(0, min(len(data), 200), 16):
        hex_str = ' '.join(f'{b:02x}' for b in data[i:i+16])
        ascii_str = ''.join(chr(b) if 32 <= b < 127 else '.' for b in data[i:i+16])
        print(f"  {i:06x}: {hex_str:48s} {ascii_str}")
    
    print("\n=== Looking for object tags (0x800X format) ===")
    
    # Search for known object tags
    tags_to_find = {
        0x8000: "Document",
        0x8001: "Page",
        0x8003: "Fragment",
        0x8004: "Node",
        0x8005: "Bond",
        0x8006: "Text",
        0x8007: "Graphic",
        0x8027: "Arrow",
    }
    
    for tag, name in tags_to_find.items():
        tag_bytes = struct.pack('<H', tag)
        pos = 0
        count = 0
        while True:
            idx = data.find(tag_bytes, pos)
            if idx == -1:
                break
            if count < 3:  # Show first 3 occurrences
                # Try to read length
                if idx + 6 <= len(data):
                    try:
                        length = struct.unpack('<I', data[idx+2:idx+6])[0]
                        print(f"  [{idx:06x}] {name:20} (0x{tag:04x}): length={length}")
                    except:
                        print(f"  [{idx:06x}] {name:20} (0x{tag:04x})")
            count += 1
            pos = idx + 2
        
        if count > 0:
            print(f"    Total occurrences: {count}")

# Analyze
if os.path.exists('sample_cdx/Reaction.cdx'):
    analyze_cdx_structure('sample_cdx/Reaction.cdx')
else:
    print("sample_cdx/Reaction.cdx not found")

# Also check Analysis.cdx
print("\n" + "="*60 + "\n")
if os.path.exists('sample_cdx/Analysis.cdx'):
    analyze_cdx_structure('sample_cdx/Analysis.cdx')
