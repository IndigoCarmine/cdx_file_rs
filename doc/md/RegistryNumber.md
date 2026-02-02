CDX Format Specification: Registry Number Object
## Registry Number Object

| CDXML Name: | regnum |
| --- | --- |
| CDX Constant Name: | kCDXObj_RegistryNumber |
| CDX Constant Value: | 0x800C |
| Contained by objects: |  |
| First written/read in: | (not written/read) |

**Description:**  

A Registry Number represents a registry or catalog number, along with the name of the authority which issued the number.

The Number and Authority properties are required, but there are no required objects.

**Subobjects:**  
*(none)*

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| 0x000B | [kCDXProp_RegistryNumber](properties/RegistryNumber.md) | RegistryNumber | [CDXString](DataType/CDXString.md) |
|  | A registry or catalog number of a molecule object. |  |  |
| 0x000C | [kCDXProp_RegistryAuthority](properties/RegistryAuthority.md) | RegistryAuthority | [CDXString](DataType/CDXString.md) |
|  | A string that specifies the authority which issued a registry or catalog number. Some examples of registry authorities are CAS, Beilstein, Aldrich, and Merck. |  |  |

---

[CDX Documentation index](index.md)