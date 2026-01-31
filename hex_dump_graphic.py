#!/usr/bin/env python3
"""
Detailed hex dump and analysis of Graphic object
"""
import struct

with open('sample_cdx/Reaction.cdx', 'rb') as f:
    data = f.read()

# Graphic at 0x001112
print("=== RAW HEX DUMP (0x001112 to 0x001180) ===\n")
pos = 0x001112
for i in range(0, 110, 16):
    offset = pos + i
    chunk = data[offset:offset+16]
    hex_str = ' '.join(f'{b:02x}' for b in chunk)
    ascii_str = ''.join(chr(b) if 32 <= b < 127 else '.' for b in chunk)
    print(f"{offset:06x}: {hex_str:48s} {ascii_str}")

# Now let's manually parse
print("\n=== MANUAL PARSE ===")
print(f"\nObject Header (0x{pos:06x}):")
tag = struct.unpack('<H', data[pos:pos+2])[0]
length = struct.unpack('<I', data[pos+2:pos+6])[0]
print(f"  Tag: 0x{tag:04x} (Object Tag = 0x8007 for Graphic)")
print(f"  Length: {length} bytes (little-endian: bytes {data[pos+2:pos+6].hex()})")

# First property
prop_offset = pos + 6
print(f"\nProperty at 0x{prop_offset:06x}:")
prop_tag = struct.unpack('<H', data[prop_offset:prop_offset+2])[0]
prop_len_bytes = data[prop_offset+2:prop_offset+6]
prop_len = struct.unpack('<I', prop_len_bytes)[0]
print(f"  Tag: 0x{prop_tag:04x}")
print(f"  Length bytes: {prop_len_bytes.hex()} -> {prop_len}")

# Check what tag 0x0013 means
print(f"\nTag 0x{prop_tag:04x} interpretation:")
print(f"  {prop_tag} in decimal = {prop_tag}")
print(f"  This doesn't match expected property tags (0x02xx, 0x03xx, 0x0Axx, etc.)")
print(f"  Is this data misaligned?")

# Let's check if we're reading the object ID instead
print(f"\n=== Alternative parse (object ID + properties) ===")
id_val = struct.unpack('<I', data[pos+6:pos+10])[0]
print(f"First 4 bytes after header (0x{pos+6:06x}): {id_val} (possible ID)")

# The next 2 bytes
next_tag = struct.unpack('<H', data[pos+10:pos+12])[0]
print(f"Next tag at 0x{pos+10:06x}: 0x{next_tag:04x}")

# And its length
next_len = struct.unpack('<I', data[pos+12:pos+16])[0]
print(f"Next length: {next_len}")
