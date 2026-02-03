CDX Format Specification: Spectrum_YType Property
## Spectrum_YType Property

| CDXML Name: | YType |
| --- | --- |
| CDX Constant Name: | kCDXProp_Spectrum_YType |
| CDX Constant Value: | 0x0A83 |
| Data Size: | [INT16](/web/20160912170130/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Spectrum](/web/20160912170130/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Spectrum.md) |
| First written/read in: | ChemDraw 5.0 |
| Required? | Until ChemDraw 6.0 |

**Description:**  

The type of units the Y-axis represents.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Unknown | The axis type is unknown. Not recommended |
| 1 | Absorbance | Axis is in absorbance units, and consequently has a baseline of 0.0 with peaks pointing up. Only for IR spectra |
| 2 | Transmittance | Axis is in transmittance units. The baseline is at 1.0 and peaks points down to a value of 0.0 being no transmission. Only for IR spectra |
| 3 | PercentTransmittance | Axis is in transmittance units*100. The baseline is at 100% and peaks points down to a value of 0.0 being no transmission. Only for IR spectra< |
| 4 | Other | Axis is some other type |
| 5 | ArbitraryUnits | Axis is unscaled -- essentially, the absolute values have no meaning and only relative values matter |

**If this property is absent:**  

The Y-axis is treated as having Unknown units. This is not recommended.

---

[CDX Documentation index](/web/20160912170130/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)