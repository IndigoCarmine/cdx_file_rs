## CDX Coordinates Data Types

### CDXCoordinate:

**In CDX files**, a CDXCoordinate is an [INT32](CDXNumeric.md). 1 unit represents 1/65536 points, or 1/4718592 inches, or 1/1857710 cm. This permits a drawing space of about 23.1 meters. In contexts where appropriate, 1 unit represents 10-15 meters, permitting a coordinate space of approx. ±2.1x10-6 meters (±21,474 Angstroms).

**In CDXML files**, a CDXCoordinate is scaled differently, so that 1 unit represents 1 point. CDXCoordinates in CDXML files may be represented as decimal values.

In 2D coordinate spaces, the origin is at the top left corner, and the coordinates increase down and to the right.
**Example:** 1 inch (72 points):

| CDX: | 00 00 48 00 |
| --- | --- |
| CDXML: | "72" |

### CDXPoint2D:

**In CDX files**, a CDXPoint2D is an x- and a y-CDXCoordinate stored as a pair of [INT32](CDXNumeric.md)s, y coordinate followed by x coordinate.

**In CDXML files**, a CDXPoint2D is stored as a pair of numeric values, x coordinate followed by y coordinate. Note that this ordering is different than in CDX files!
**Example:** 1 inch (72 points) to the right, and 2 inches down:

| CDX: | 00 00 90 00 00 00 48 00 |
| --- | --- |
| CDXML: | "72 144" |

### CDXPoint3D:

**In CDX files**, a CDXPoint3D is an x- and a y-CDXCoordinate stored as a pair of [INT32](CDXNumeric.md)s, z coordinate followed by y coordinate followed by x coordinate.

**In CDXML files**, a CDXPoint3D is stored as a pair of numeric values, x coordinate followed by y coordinate followed by z coordinate. Note that this ordering is different than in CDX files!
**Example:** 1 inch (72 points) to the right, 2 inches down, and 3 inches deep:

| CDX: | 00 00 d8 00 00 00 90 00 00 00 48 00 |
| --- | --- |
| CDXML: | "72 144 216" |

### CDXRectangle:

**In CDX files**, rectangles are stored as four CDXCoordinate values, representing, in order: top, left, bottom, and right edges of the rectangle.

**In CDXML files**, rectangles are stored as four CDXCoordinate values, representing, in order: left, top, right, and bottom edges of the rectangle. Note that this ordering is different than in CDX files!
**Example:** top: 1 inch, left: 2 inches, bottom: 3 inches, right: 4 inches:

| CDX: | 00 00 48 00 00 00 90 00 00 00 D8 00 00 00 20 01 |
| --- | --- |
| CDXML: | "144 72 288 216" |

See the [complete list of CDX data types](../DataTypes.md)

---

[CDX Documentation index](../index.md)