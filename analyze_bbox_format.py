#!/usr/bin/env python3
"""
Analyze the 16-byte bounding box data from Reaction.cdx
"""
import struct

with open('sample_cdx/Reaction.cdx', 'rb') as f:
    data = f.read()

# The bbox data from our earlier analysis
bbox_hex = bytes.fromhex('1e052f0066 26a1001e052f00 33f3 82 00')
print("=== BoundingBox Data (16 bytes) ===")
print(f"Hex: {bbox_hex.hex()}")

# Try different interpretations
print("\n1. As 4 x INT32 (fixed point):")
for i in range(4):
    val = struct.unpack('<i', bbox_hex[i*4:(i+1)*4])[0]
    print(f"   [{i}] = {val} (0x{val:08x})")

print("\n2. As 2 x Point2d (2 x i32, i32):")
p1_x = struct.unpack('<i', bbox_hex[0:4])[0]
p1_y = struct.unpack('<i', bbox_hex[4:8])[0]
p2_x = struct.unpack('<i', bbox_hex[8:12])[0]
p2_y = struct.unpack('<i', bbox_hex[12:16])[0]
print(f"   Point2d 1: x={p1_x}, y={p1_y}")
print(f"   Point2d 2: x={p2_x}, y={p2_y}")

# Convert with fixed point (divide by 65536)
print("\n3. As 2 x Point2d with fixed point (divided by 65536):")
p1_x_fp = p1_x / 65536.0
p1_y_fp = p1_y / 65536.0
p2_x_fp = p2_x / 65536.0
p2_y_fp = p2_y / 65536.0
print(f"   Point2d 1: x={p1_x_fp:.2f}, y={p1_y_fp:.2f}")
print(f"   Point2d 2: x={p2_x_fp:.2f}, y={p2_y_fp:.2f}")

# Compare to what we got from Point2d::decode
print("\n4. Compare to our parsed values from the test:")
print(f"   head_2d: x=161.15, y=47.02")
print(f"   tail_2d: x=130.95, y=47.02")

print("\n5. As Rectangle (left, top, right, bottom) in different byte orders:")
# Little-endian i32
left = struct.unpack('<i', bbox_hex[0:4])[0] / 65536.0
top = struct.unpack('<i', bbox_hex[4:8])[0] / 65536.0
right = struct.unpack('<i', bbox_hex[8:12])[0] / 65536.0
bottom = struct.unpack('<i', bbox_hex[12:16])[0] / 65536.0
print(f"   left={left:.2f}, top={top:.2f}, right={right:.2f}, bottom={bottom:.2f}")
print(f"   => Rectangle from ({left:.2f}, {top:.2f}) to ({right:.2f}, {bottom:.2f})")
