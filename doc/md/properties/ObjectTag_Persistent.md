CDX Format Specification: ObjectTag_Persistent Property
## ObjectTag_Persistent Property

| CDXML Name: | Persistent |
| --- | --- |
| CDX Constant Name: | kCDXProp_ObjectTag_Persistent |
| CDX Constant Value: | 0x0D04 |
| Data Size: | [CDXBoolean](/web/20190326232903/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXBoolean.md) |
| Property of objects: | [kCDXObj_ObjectTag](/web/20190326232903/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ObjectTag.md) |
| First written/read in: | ChemDraw 6.0 |
| Required? | No |

**Description:**  

The tag will be resaved to a CDX file if non-zero.

Developers adding [Object Tags](/web/20190326232903/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ObjectTag.md) to CDX files should keep in mind that ChemDraw will treat them as a black box. If a user opens a file containing Object Tags, makes a change, then resaves the file, ChemDraw cannot tell if the Object Tags are still accurate. As a pathological case, consider a "HasBeenOpenedInChemDraw" tag added by some other program, initially false. If the tag is Persistant, ChemDraw will happily resave the file with the initial value -- false -- preserved, even though it is no longer accurate! On the other hand, if a flag is *not* Persistent, ChemDraw will not resave it, even if it really is still accurate. Obviously, this is not an ideal situation.

**If this property is absent:**  

The tag will be resaved to a CDX file.

---

[CDX Documentation index](/web/20190326232903/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)