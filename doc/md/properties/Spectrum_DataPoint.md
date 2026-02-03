CDX Format Specification: Spectrum_DataPoint Property
## Spectrum_DataPoint Property

| CDXML Name: | temp_SpectrumDataPoint |
| --- | --- |
| CDX Constant Name: | kCDXProp_Spectrum_DataPoint |
| CDX Constant Value: | 0x0A86 |
| Data Size: | [FLOAT64](/web/20160913174112/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Spectrum](/web/20160913174112/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Spectrum.md) |
| First written/read in: | ChemDraw 5.0 |
| Required? | Required for spectra |

**Description:**  

The Y-axis values for the spectrum. It is an array of double values corresponding to X-axis values.

ChemDraw supports only those spectrum types with evenly-spaced X values. Accordingly, this property stores only the Y-coordinate values. Corresponding X-coordinate values can be calculated via the [kCDXProp_Spectrum_XLow](Spectrum_XLow.md) and [kCDXProp_Spectrum_XSpacing](Spectrum_XSpacing.md) properties.

This property is used explicitly only for CDX files. In CDXML files, the corresponding data is stored within the [Spectrum](/web/20160913174112/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Spectrum.md) object directly, as #PCDATA

---

[CDX Documentation index](/web/20160913174112/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)