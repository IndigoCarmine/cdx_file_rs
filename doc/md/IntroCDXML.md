## The CDXML text-based file format

A CDXML is a CDX file specially formatted so that it conforms to the [XML](https://www.xml.org/) specification. We expect that anyone who manipulates a CDXML file will be familiar with the general XML specifications, so we present only a brief overview here.

A CDXML file has the following general attributes:

- Like a CDX file, it consists of a fixed header followed by a series of tagged items.
- All multi-byte data items are stored as quoted text. That is, the value equal to 28 is stored as the three-byte string "255" rather than as the single byte 0xFF or as any other value. The same value might also be stored as "255.0" or "255.00"; any program reading a CDXML file should be flexible about data types.
- Any amount of whitespace is permitted between objects, and any non-zero amount of whitespace is permitted between properties.

### Header Format

The CDX File Header consists of the following string:

```xml
<?xml version="1.0" encoding="UTF-8" ?>
```

```
<!DOCTYPE CDXML SYSTEM "http://www.camsoft.com/xml/cdxml.dtd">
```

The header is then followed by an object tree of tagged items beginning with a document object.

### End

Since the first object following the header is a document object, the end of the file is signified by the end of the document object:

```
</CDXML>
```

#### [Properties](TableOfProperties.md)

Properties, also called attributes, are self-contained. A property applies to the object which logically contains it. It may also describe other objects contained within the object which logically contains the property. For example, bond order is a property of a bond, and molecular weight is a property of a molecule. Unless otherwise noted, all properties are optional.

All properties have a name. Properties are included in a CDXML file by listing the property name, an equals sign, and then a quoted string representing the property's value.

##### Example

| Order="2" | : | The bond with this property is a double bond |
| --- | --- | --- |

#### [Objects](AllCDXObjects.md)

Like properties, objects are also self-contained. Unlike properties, however, objects can contain properties and other objects.

Each object has a name that identifies the type of object. For example, an object that represents a bond has the name "b". These names, like everything else in XML, are case-sensitive.

The definition of an object starts with a less-than sign, followed by the object's name. Any properties, if present, are listed immediately afterward in pairs formatted as *propertyname*="propertyvalue". A greater-than sign will appear after the last property.

The definition of an object ends with a less-than sign followed by a slash followed by the object name again, followed by a greater-than sign. Alternatively, if there are no subobjects, the end-object marker may be omitted and replaced by a slash immediately before the first closing greater-than sign.

Any subobjects are listed between those object-begin and object-end markers.

##### Example

| <b | : | This is a bond |
| --- | --- | --- |
| id="29" | : | This bond has an ID of 29 |
| B="21" | : | The atom at the first end of this bond has ID 21 |
| E="22" | : | The atom at the first end of this bond has ID 22 |
| Order="2" | : | This bond has a bond order 2. It is a double bond. |
| DoublePosition="Right" | : | This double bond is positioned so that the second line of the double bond is to the right of the first, looking from the first atom to the second atom |
| /> | : | There are no more properties or objects associated with this bond |

[Return to Introduction](General.md)
[Continue to simple example](IntroExampleSimple.md)

---

[CDX Documentation index](index.md)