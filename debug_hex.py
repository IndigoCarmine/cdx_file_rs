with open('sample_cdx/benzene.cdx', 'rb') as f:
    data = f.read(64)
    print(' '.join(f'{b:02X}' for b in data))
    print(f"\nHeader (first 28 bytes): {' '.join(f'{b:02X}' for b in data[:28])}")
    print(f"After header (next 20 bytes): {' '.join(f'{b:02X}' for b in data[28:48])}")
    # Check for the root tag
    print(f"\nByte 28-29 (potential tag): 0x{data[28]:02X}{data[29]:02X} (LE) = 0x{int.from_bytes(data[28:30], 'little'):04X}")
