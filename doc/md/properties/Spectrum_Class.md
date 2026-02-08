## Spectrum_Class Property

| CDXML Name: | Class |
| --- | --- |
| CDX Constant Name: | kCDXProp_Spectrum_Class |
| CDX Constant Value: | 0x0A87 |
| Data Size: | [INT16](/web/20160912170231/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Spectrum](/web/20160912170231/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Spectrum.md) |
| First written/read in: | ChemDraw 5.0 |
| Required? | No |

**Description:**  

The type of spectrum represented.

This is an enumerated property. Acceptable values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Unknown | Unknown spectral type. Not recommended |
| 1 | Chromatogram | GC (not supported in ChemDraw) |
| 2 | Infrared | Infrared |
| 3 | UVVis | UVVis (not supported in ChemDraw prior to ChemDraw 10.0) |
| 4 | XRayDiffraction | X-Ray Diffraction (not supported in ChemDraw) |
| 5 | MassSpectrum | Mass Spectrum |
| 6 | NMR | NMR |
| 7 | Raman | Raman (not supported in ChemDraw prior to ChemDraw 10.0) |
| 8 | Fluorescence | Fluorescence (not supported in ChemDraw) |
| 9 | Atomic | Atomic Absorption (not supported in ChemDraw) |

**If this property is absent:**  

The spectrum is treated as having Unknown type. This is not recommended.

---

[CDX Documentation index](/web/20160912170231/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)