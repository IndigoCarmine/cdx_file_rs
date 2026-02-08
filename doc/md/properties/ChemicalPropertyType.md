## ChemicalPropertyType Property

| CDXML Name: | ChemicalPropertyType |
| --- | --- |
| CDX Constant Name: | kCDXProp_ChemicalPropertyType |
| CDX Constant Value: | 0x0BB0 |
| Data Size: | [UINT32](/web/20160913174226/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_ChemicalProperty](/web/20160913174226/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ChemicalProperty.md) |
| First written/read in: | ChemDraw 9.0 |
| Required? | No |

**Description:**

The type of property (name, formula, molecular weight, etc.).

The list of enumerated values shown reflects the current implementation in ChemDraw. User-defined enums are acceptable, but should be created with CDX values greater than 0x8000, and with CDXML names that are likely not to be duplicated by others.

This is an enumerated property. Acceptable values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Unspecified | Unspecified |
| 1 | ChemicalName | A (probably systematic) chemical name |

**If this property is absent:**

The property is treated as Undefined. A supporting program will not know how to treat the property, and the display object will not be able to be updated even if the [kCDXProp_ChemicalPropertyIsActive](ChemicalPropertyIsActive.md) value is set to 'true'.

---

[CDX Documentation index](/web/20160913174226/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)