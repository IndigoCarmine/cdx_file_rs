CDX Format Specification: Spectrum_XType Property
## Spectrum_XType Property

| CDXML Name: | XType |
| --- | --- |
| CDX Constant Name: | kCDXProp_Spectrum_XType |
| CDX Constant Value: | 0x0A82 |
| Data Size: | [INT16](/web/20160912170256/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Spectrum](/web/20160912170256/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Spectrum.md) |
| First written/read in: | ChemDraw 5.0 |
| Required? | Until ChemDraw 6.0 |

**Description:**  

The type of units the X-axis represents.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Unknown | The axis type is unknown. Not recommended |
| 1 | Wavenumbers | Axis is in wavenumbers. Only for IR spectra |
| 2 | Microns | Axis is in microns. Only for IR spectra |
| 3 | Hertz | Axis is in Hertz |
| 4 | MassUnits | Axis is in m/e. Only for MS spectra |
| 5 | PartsPerMillion | Axis is in parts per million. Only for NMR spectra |
| 6 | Other | Axis is some other type |

**If this property is absent:**  

The X-axis is treated as having Unknown units. This is not recommended.

---

[CDX Documentation index](/web/20160912170256/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)