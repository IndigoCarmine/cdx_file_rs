with open('sample_cdx/benzene.cdx', 'rb') as f:
    data = f.read(256)
    print(f"Total bytes read: {len(data)}")
    print(f"\nFirst 50 bytes (hex):")
    print(' '.join(f'{b:02X}' for b in data[:50]))
    
    print(f"\n\nBreakdown:")
    print(f"Bytes 0-7 (magic): {data[0:8].decode('ascii')} = {' '.join(f'{b:02X}' for b in data[0:8])}")
    print(f"Bytes 8-11 (reserved_legacy): {' '.join(f'{b:02X}' for b in data[8:12])}")
    print(f"Bytes 12-27 (reserved_zero): {' '.join(f'{b:02X}' for b in data[12:28])}")
    print(f"Bytes 28-31 (additional reserved): {' '.join(f'{b:02X}' for b in data[28:32])}")
    print(f"Bytes 32-33 (potential root tag): {' '.join(f'{b:02X}' for b in data[32:34])}")
    
    # Check as LE u16
    tag = int.from_bytes(data[32:34], 'little')
    print(f"\nTag at byte 32 (LE): 0x{tag:04X} (is_object={tag & 0x8000 != 0})")

