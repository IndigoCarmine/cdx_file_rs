## The CDX binary file format

A CDX file has the following general attributes:

- It consists of a fixed header, followed by a series of tagged items.
- All multi-byte data items are stored in little-endian byte order, that is, least significant byte first. (Intel uses little-endian; Macintosh uses big-endian. Accordingly, CDX files must be byte-swapped appropriately if they are to be read on Macintosh computers.)

### Header Format

The CDX File Header consists of:

- 8 bytes with the value "VjCD0100" (56 6A 43 44 30 31 30 30)
- 4 bytes reserved (04 03 02 01)
- 16 bytes reserved, set to zero (00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00).

The header is then followed by an object tree of tagged items beginning with a document object.

### End

The end of the CDX file is marked with two bytes of 0 (00 00).

### [Properties](TableOfProperties.md)

Properties, also called attributes, are self-contained. A property applies to the object which logically contains it. It may also describe other objects contained within the object which logically contains the property. For example, bond order is a property of a bond, and molecular weight is a property of a molecule. Unless otherwise noted, all properties are optional.

Properties have three parts, as follows:

#### Tag identifier

The tag identifier defines what this property represents -- for instance, the order of a bond. The 2-byte tag identifier identifies the property. Having the most significant bit (bit 15) clear indicates that this is a property rather than an object. Bit 14 is clear for predefined properties, set for user-defined properties. Therefore, there are 16383 tags reserved for predefined Properties and 16384 for user-defined Properties.

#### Length

The 2-byte length item specifies the number of (8-bit) bytes that comprise the data the property. The length is zero if the mere presence of the property is sufficient.

A length of 0xFFFF is a special value that indicates the object is greater than 65534 bytes in size. In this case it is followed immediately by an additional 4-byte Length item to specify the actual length.

#### Data

The *n*-byte data item may be an integer, a floating point number, or some other type determined by the property tag. For instance, a RotationAngle property's value consists of a CDX angle.

#### Example

| 06 00 | : | The tag identifier 0x0600 indicates a bond order |
| --- | --- | --- |
| 02 00 | : | Bond orders are stored as an INT16 value, consisting of two (0x0002) bytes |
| 02 00 | : | This bond has a bond order of 0x0002. It is a double bond. |

### [Objects](AllCDXObjects.md)

Like properties, objects are also self-contained. Unlike properties, however, objects can contain properties and other objects.

Objects have four parts, as follows:

##### Tag Identifier

An object's tag is a two-byte value, which will always have the most significant bit (bit 15) set. Bit 14 is zero for predefined objects, or one for user-defined. There are therefore 16383 tags reserved for predefined objects, and 16384 for user-defined objects.

#### Object Identifier

A four-byte object ID immediately follows the tag. It should be a unique value within the container. A value of zero indicates that no ID is assigned, and may only be used if the object is not referenced by any other object. Object IDs need only be unique within the containing object; however, we recommend that they be unique within the entire document.

(In principle, a complete reference to any object in a CDX file may require a series of object identifiers, each indicating the branch to take at one level of the containment hierarchy. It is therefore sufficient that identifiers be unique with respect to the other objects housed in the same container. However, there is currently no mechanism to specify series of object identifiers, and it is therefore recommended that all object identifiers be "globally" unique for the time being, when convenient.)

#### Object contents

An object may contain any number of properties or other objects.

#### EndObject

Every object ends with a pair of zero bytes (00 00).

#### Example

| 05 80 | : | The tag identifier 0x8005 indicates a bond |
| --- | --- | --- |
| 1D 00 00 00 | : | This bond has an ID of 29 (0x0000001D) |
| 04 06 04 00 15 00 00 00 | : | The atom at the first end of this bond (property 0x0604) has ID 21 (0x00000015) |
| 05 06 04 00 16 00 00 00 | : | The atom at the second end of this bond (property 0x0605) has ID 22 (0x00000016) |
| 00 06 02 00 02 00 | : | This bond has a bond order (property 0x0600) of 0x0002. It is a double bond. |
| 03 06 02 00 01 00 | : | This double bond is positioned (property 0x0603) so that the second line of the double bond is to the right of the first, looking from the first atom to the second atom |
| 00 00 | : | There are no more properties or objects associated with this bond |

### Sample header file

As a convenience, we have provided a C++ header file with human-readable enumerations of all of the CDX object and property values. The enum names used in this file (kCDXProp_*, etc) are the same as used throughout this documentation. This header file is provided simply to save you the problem of typing the same values in again yourself.

[Return to Introduction](General.md)
[Continue to simple example](IntroExampleSimple.md)

---

[CDX Documentation index](index.md)