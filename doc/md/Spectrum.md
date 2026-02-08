## Spectrum Object

| CDXML Name: | spectrum |
| --- | --- |
| CDX Constant Name: | kCDXObj_Spectrum |
| CDX Constant Value: | 0x8010 |
| Contained by objects: | [kCDXObj_Page](Page.md),[kCDXObj_Group](Group.md) |
| First written/read in: | ChemDraw 4.5.1 |

**Description:**

A Spectrum Object stores an NMR, MS, IR, or other type of spectrum. The spectrum is defined on a regular grid of x-axis values running from XLow to XHigh in steps of XSpacing. The y-values may be specified similarly, or may be arbitrary floating-point values. This is a graphical object and doesn't store spectral information beyond that needed to display the spectrum.

The bounding box, XLow, and XSpacing properties are required, but a Spectrum has no required subobjects.

**Subobjects:**

| Value | Name | CDXML Name |
| --- | --- | --- |
| 0x8011 | [kCDXObj_ObjectTag](ObjectTag.md) | objecttag |
|  | Arbitrarily named property, one or more of which can be attached to any ChemDraw object. |  |

**Properties:**

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x000A | [kCDXProp_ZOrder](properties/ZOrder.md) | Z | [INT16](DataType/CDXNumeric.md) |
|  | Back-to-front ordering index in 2D drawing. |  |  |
| 0x000F | [kCDXProp_IgnoreWarnings](properties/IgnoreWarnings.md) | IgnoreWarnings | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies whether chemical warnings should be suppressed on this object. |  |  |
| 0x0010 | [kCDXProp_ChemicalWarning](properties/ChemicalWarning.md) | Warning | [CDXString](DataType/CDXString.md) |
|  | A warning concerning possible chemical problems with this object. |  |  |
| 0x0011 | [kCDXProp_Visible](properties/Visible.md) | Visible | [CDXBoolean](DataType/CDXBoolean.md) |
|  | The object is visible if non-zero. |  |  |
| 0x0204 | [kCDXProp_BoundingBox](properties/BoundingBox.md) | BoundingBox | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | Required for pictures and spectra. Required for graphics and text until 6.0. The smallest rectangle that encloses the graphical representation of the object. |  |  |
| 0x0301 | [kCDXProp_ForegroundColor](properties/ForegroundColor.md) | color | [UINT16](DataType/CDXNumeric.md) |
|  | The foreground color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0302 | [kCDXProp_BackgroundColor](properties/BackgroundColor.md) | bgcolor | [INT16](DataType/CDXNumeric.md) |
|  | The background color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0806 | [kCDXProp_BoldWidth](properties/BoldWidth.md) | BoldWidth | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default bold bond width. |  |  |
| 0x0807 | [kCDXProp_LineWidth](properties/LineWidth.md) | LineWidth | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default line width. |  |  |
| 0x080A | [kCDXProp_LabelStyle](properties/LabelStyle.md) | (not used) | [CDXFontStyle](DataType/CDXString.md) |
|  | The default style for atom labels. |  |  |
| 0x081A | [kCDXProp_LabelStyleFont](properties/LabelStyleFont.md) | LabelFont | [INT16](DataType/CDXNumeric.md) |
|  | The default font family for atom labels. |  |  |
| 0x081C | [kCDXProp_LabelStyleSize](properties/LabelStyleSize.md) | LabelSize | [INT16](DataType/CDXNumeric.md) |
|  | The default font size for atom labels. |  |  |
| 0x081E | [kCDXProp_LabelStyleFace](properties/LabelStyleFace.md) | LabelFace | [INT16](DataType/CDXNumeric.md) |
|  | The default font style for atom labels. |  |  |
| 0x0A80 | [kCDXProp_Spectrum_XSpacing](properties/Spectrum_XSpacing.md) | XSpacing | [FLOAT64](DataType/CDXNumeric.md) |
|  | Required for spectra. The spacing in logical units (ppm, Hz, wavenumbers) between points along the X-axis of an evenly-spaced grid. |  |  |
| 0x0A81 | [kCDXProp_Spectrum_XLow](properties/Spectrum_XLow.md) | XLow | [FLOAT64](DataType/CDXNumeric.md) |
|  | Required for spectra. The first data point for the X-axis of an evenly-spaced grid. |  |  |
| 0x0A82 | [kCDXProp_Spectrum_XType](properties/Spectrum_XType.md) | XType | [INT16](DataType/CDXNumeric.md) |
|  | The type of units the X-axis represents. This is an enumerated property. |  |  |
| 0x0A83 | [kCDXProp_Spectrum_YType](properties/Spectrum_YType.md) | YType | [INT16](DataType/CDXNumeric.md) |
|  | The type of units the Y-axis represents. This is an enumerated property. |  |  |
| 0x0A84 | [kCDXProp_Spectrum_XAxisLabel](properties/Spectrum_XAxisLabel.md) | XAxisLabel | [CDXString](DataType/CDXString.md) |
|  | A label for the X-axis. |  |  |
| 0x0A85 | [kCDXProp_Spectrum_YAxisLabel](properties/Spectrum_YAxisLabel.md) | YAxisLabel | [CDXString](DataType/CDXString.md) |
|  | A label for the Y-axis. |  |  |
| 0x0A86 | [kCDXProp_Spectrum_DataPoint](properties/Spectrum_DataPoint.md) | (not used) | [FLOAT64](DataType/CDXNumeric.md) |
|  | Required for spectra. The Y-axis values for the spectrum. It is an array of double values corresponding to X-axis values. |  |  |
| 0x0A87 | [kCDXProp_Spectrum_Class](properties/Spectrum_Class.md) | Class | [INT16](DataType/CDXNumeric.md) |
|  | The type of spectrum represented. This is an enumerated property. |  |  |
| 0x0A88 | [kCDXProp_Spectrum_YLow](properties/Spectrum_YLow.md) | YLow | [FLOAT64](DataType/CDXNumeric.md) |
|  | Y value to be used to offset data when storing XML. |  |  |
| 0x0A89 | [kCDXProp_Spectrum_YScale](properties/Spectrum_YScale.md) | YScale | [FLOAT64](DataType/CDXNumeric.md) |
|  | Y scaling used to scale data when storing XML. |  |  |

---

[CDX Documentation index](index.md)