"""Generate src/cdxml/table.rs from ChemDraw's file-format sources, with the documentation in
doc/md filling in what the sources leave out.

The sources (https://github.com/Glysade/chemdraw, fetched by scripts/fetch_chemdraw_sdk.sh) give:

- CDXConstants.h: every object and property tag (enum CDXDatumID), most with their data type
  in a trailing comment, and the enumerations (CDXNodeType, CDXBondDisplay, ...);
- CDXMLParser.cpp: the element name of each object;
- CDXObject.cpp: the attribute name of each property (sCDXMLAttributeNames);
- the XMLPutEnum tables in the other sources: the CDXML names of enumerated values, and which
  property reads them (`case kCDXProp_X: ... sXMLY.lookup(`).

Tag values are computed from the enums, not taken from the comments: a few comments are wrong
(Transparent and SupersededBy are 0x0012 and 0x0013, not 0x0011 and 0x0012).

    python tools/gen_cdxml_table.py tools/data/chemdraw/chemdraw > src/cdxml/table.rs
    python tools/gen_cdxml_table.py tools/data/chemdraw/chemdraw --check
"""
import os
import re
import sys

DOC = os.path.join(os.path.dirname(__file__), "..", "doc", "md")


def read(path):
    with open(path, encoding="utf-8", errors="replace") as f:
        return f.read()


def strip_comments(text):
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    return re.sub(r"//[^\n]*", "", text)


def enums(header):
    """Every enumerator in the header: name -> (value, type comment or None)."""
    values = {m.group(1): (int(m.group(2), 0), None)
              for m in re.finditer(r"const\s+\w+\s+(kCDX\w+)\s*=\s*(0x[0-9A-Fa-f]+|\d+)\s*;", header)}
    for body in re.findall(r"\benum\s+\w*\s*\{(.*?)\}", header, re.S):
        nxt = 0
        for line in body.split("\n"):
            comment = re.search(r"//(.*)$", line)
            code = strip_comments(line).strip().rstrip(",").strip()
            for item in [c.strip() for c in code.split(",") if c.strip()]:
                m = re.match(r"(\w+)\s*(?:=\s*(.+))?$", item)
                if not m:
                    continue
                name, expr = m.groups()
                if expr is not None:
                    nxt = evaluate(expr, values)
                ty = None
                if comment:
                    t = re.search(r"\(([A-Za-z0-9]+)\)\s*$", comment.group(1).strip())
                    ty = t.group(1) if t else None
                values[name] = (nxt, ty)
                nxt += 1
    return values


def evaluate(expr, values):
    expr = re.sub(r"\((?:INT|UINT)\d+\)", "", expr)
    expr = re.sub(r"\b(\d+)[uUlL]+\b", r"\1", expr)
    names = {k: v[0] for k, v in values.items()}
    return int(eval(expr, {"__builtins__": {}}, names))


def element_names(sdk, constants):
    names = {m.group(1): m.group(2) for m in re.finditer(r'#define\s+(kCDXML_\w+)\s+"([^"]*)"',
                                                         read(os.path.join(sdk, "CDXMLNames.h")))}
    out = {}
    for m in re.finditer(r"value_type\((kCDXML_\w+),\s*(kCDXObj_\w+)\)", read(os.path.join(sdk, "CDXMLParser.cpp"))):
        out[names[m.group(1)]] = constants[m.group(2)][0]
    return out


def attribute_names(sdk):
    text = read(os.path.join(sdk, "CDXObject.cpp"))
    return {m.group(1): m.group(2) for m in re.finditer(r'sCDXMLAttributeNames\[(kCDXProp_\w+)\]\s*=\s*"([^"]*)"', text)}


def xml_enums(sdk, constants):
    """Property constant -> [(value, name)], from the XMLPutEnum tables the properties are read with,
    and the properties whose value is a set of flags (read with lookupTokens: `Circle Filled`)."""
    tables, lookups, flags = {}, {}, set()
    for f in sorted(os.listdir(sdk)):
        if not f.endswith(".cpp"):
            continue
        text = strip_comments(read(os.path.join(sdk, f)))
        for m in re.finditer(r"XMLPutEnum<\s*\w+\s*>::Value\s+(\w+)\s*\[\]\s*=\s*\{(.*?)\};", text, re.S):
            vals = []
            for v in re.finditer(r'\{\s*([^,{}]+?)\s*,\s*"([^"]*)"\s*\}', m.group(2)):
                try:
                    vals.append((evaluate(v.group(1), constants), v.group(2)))
                except Exception:
                    pass
            tables[m.group(1)] = vals
        for m in re.finditer(r"XMLPutEnum<\s*\w+\s*>\s+(\w+)\s*\(\s*(\w+)\s*,", text):
            tables[m.group(1)] = tables.get(m.group(2), [])
        # A run of `case kCDXProp_X:` labels, then the statements up to the next case label.
        for m in re.finditer(r"((?:case\s+kCDXProp_\w+\s*:\s*)+)(.*?)(?=case\s+\w+\s*:|default\s*:|\Z)", text, re.S):
            used = re.search(r"\b(sXML\w+)\.(lookup|lookupTokens)\(\s*\w*[vV]alue", m.group(2))
            if used:
                for prop in re.findall(r"kCDXProp_\w+", m.group(1)):
                    lookups.setdefault(prop, used.group(1))
                    if used.group(2) == "lookupTokens":
                        flags.add(prop)
    return {p: tables[t] for p, t in lookups.items() if tables.get(t)}, flags


def doc_types():
    """CDXML attribute name -> (data type, enumerated values) from the doc pages."""
    def field(text, name):
        m = re.search(r"^\|\s*" + re.escape(name) + r"\s*\|\s*(.*?)\s*\|", text, re.M)
        return m.group(1).strip() if m else None

    out = {}
    d = os.path.join(DOC, "properties")
    for f in sorted(os.listdir(d)):
        text = read(os.path.join(d, f))
        name = field(text, "CDXML Name:")
        m = re.match(r"\[?([A-Za-z0-9]+)", field(text, "Data Size:") or "")
        if not name:
            continue
        vals = [(int(v.group(1), 0), v.group(2)) for v in re.finditer(
            r"^\|\s*(-?\d+|0x[0-9A-Fa-f]+)\s*\|\s*([A-Za-z][A-Za-z0-9_]*)\s*\|", text, re.M)]
        out.setdefault(name, (m.group(1) if m else None, vals))
    return out


# Types the sources do not state in a comment, from how they read and write the property.
# "RawString" is text without the CDXString style-run count, "Base64" bytes CDXML writes in
# base64 (not hex), and "TagValue"/"SGValue" a value whose type another attribute gives.
SUPPLEMENT_TYPES = {
    "kCDXProp_HighlightColor": "UINT16",
    "kCDXProp_LabelStyleFontName": "CDXString",
    "kCDXProp_CaptionStyleFontName": "CDXString",
    "kCDXProp_Compressed_MacPICT": "Base64",
    "kCDXProp_Compressed_WindowsMetafile": "Base64",
    "kCDXProp_Compressed_EnhancedMetafile": "Base64",
    "kCDXProp_Compressed_OLEObject": "Base64",
    "kCDXProp_GEP_ShowScale": "CDXBooleanImplied",
    "kCDXProp_GEP_ShowValue": "CDXBooleanImplied",
    "kCDXProp_GEP_LaneLabelsAngle": "INT32",
    "kCDXProp_GEP_ScaleUnit": "INT32",
    "kCDXProp_GEP_StartRange": "INT32",
    "kCDXProp_GEP_EndRange": "INT32",
    "kCDXProp_GEP_Value": "INT32",
    "kCDXProp_GEP_AxisWidth": "FLOAT64",
    "kCDXProp_ObjectTag_Value": "TagValue",
    "kCDXProp_SG_DataValue": "SGValue",
    "kCDXProp_ChemicalPropertyIsChemicallySignificant": "CDXBoolean",
    "kCDXProp_ChemicalPropertyExternalBonds": "CDXObjectIDArray",
    "kCDXProp_BioShape_Type": "INT16",
    "kCDXProp_RLogic_Group": "RawString",
    "kCDXProp_RLogic_Occurrence": "RawString",
    "kCDXProp_RLogic_IfThenGroup": "RawString",
    "kCDXProp_RLogic_RestH": "CDXBooleanImplied",
    # Comments that disagree with how the property is read (and with doc/md).
    "kCDXProp_Bracket_RepeatCount": "FLOAT64",
    "kCDXProp_RelationValue": "FLOAT64",
    "kCDXProp_PageDefinition": "UINT8",
}
# Value names the sources write by hand rather than from an XMLPutEnum table.
SUPPLEMENT_VALUES = {
    "kCDXProp_DrawingSpaceType": [(0, "pages"), (1, "poster")],  # CDXPage.cpp
}
# The chemical property strings a document or graphic object keeps (CDXDocument.cpp reads all of
# them as CDXString).
CHEM_PROP_STRINGS = re.compile(r"kCDXProp_ChemicalProperty(?!Type|DisplayID|IsActive|IsChemicallySignificant|ExternalBonds)\w+$")

# CDX data type -> the Rust `Kind` variant that encodes it.
KINDS = {
    "INT8": "I8", "UINT8": "U8", "INT16": "I16", "UINT16": "U16", "INT32": "I32", "UINT32": "U32",
    "FLOAT64": "F64", "CDXBoolean": "Bool", "CDXBooleanImplied": "BoolImplied",
    "CDXCoordinate": "Coordinate", "CDXPoint2D": "Point2D", "CDXPoint3D": "Point3D",
    "CDXRectangle": "Rectangle", "CDXObjectID": "ObjectId", "CDXObjectIDArray": "ObjectIdArray",
    "CDXObjectIDArrayWithCounts": "ObjectIdArrayWithCounts", "CDXString": "String",
    "CDXCurvePoints": "CurvePoints", "CDXCurvePoints3D": "CurvePoints3D",
    "CDXElementList": "ElementList", "CDXGenericList": "GenericList",
    "INT16ListWithCounts": "Int16ListWithCounts", "Unformatted": "Unformatted", "Base64": "Base64",
    "RawString": "RawString", "TagValue": "TagValue", "SGValue": "SgValue",
}
# Written by the codec itself, not as attributes.
SPECIAL = {"kCDXProp_FontTable", "kCDXProp_ColorTable", "kCDXProp_Text", "kCDXProp_RepresentsProperty",
           "kCDXProp_LabelStyle", "kCDXProp_CaptionStyle"}


def merge_values(sdk_vals, doc_vals):
    """The sources' value names, then the names only the documentation has (older files use them)."""
    names = {n for _, n in sdk_vals}
    return list(sdk_vals) + [(v, n) for v, n in doc_vals if n not in names]


def build(sdk):
    constants = enums(read(os.path.join(sdk, "CDXConstants.h")))
    objects = element_names(sdk, constants)
    attrs = attribute_names(sdk)
    enum_values, flag_props = xml_enums(sdk, constants)
    enum_values.update(SUPPLEMENT_VALUES)
    docs = doc_types()
    props, untyped = [], []
    for const, name in attrs.items():
        if const in SPECIAL or const not in constants:
            continue
        tag, ty = constants[const]
        ty = SUPPLEMENT_TYPES.get(const) or ty
        if CHEM_PROP_STRINGS.match(const):
            ty = "CDXString"
        doc_ty, doc_vals = docs.get(name, (None, []))
        if ty is None or ty not in KINDS:
            ty = doc_ty if doc_ty in KINDS else ty
        if ty not in KINDS:
            untyped.append((const, name, ty))
            continue
        props.append((name, tag, KINDS[ty], merge_values(enum_values.get(const, []), doc_vals)))
    flags = sorted(constants[c][0] for c in flag_props if c in attrs)
    return objects, sorted(props, key=lambda p: (p[1], p[0])), untyped, flags


def emit(objects, props, flags):
    print("// @generated by tools/gen_cdxml_table.py from ChemDraw's sources -- do not edit by hand.")
    print("use super::Kind;")
    print()
    print("/// CDXML element name and CDX object tag.")
    print("pub(super) const OBJECTS: &[(&str, u16)] = &[")
    for n, t in sorted(objects.items(), key=lambda x: (x[1], x[0])):
        print(f'    ("{n}", 0x{t:04X}),')
    print("];")
    print()
    print("/// CDXML attribute name, CDX property tag, value encoding and enumerated value names.")
    print("pub(super) const PROPERTIES: &[(&str, u16, Kind, &[(i32, &str)])] = &[")
    for n, t, kind, vals in props:
        en = ", ".join(f'({v}, "{nm}")' for v, nm in vals)
        print(f'    ("{n}", 0x{t:04X}, Kind::{kind}, &[{en}]),')
    print("];")
    print()
    print("/// Properties whose value is a set of flags, written as their names separated by spaces.")
    print("pub(super) const FLAGS: &[u16] = &[" + ", ".join(f"0x{t:04X}" for t in flags) + "];")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    objects, props, untyped, flags = build(sys.argv[1])
    if "--check" in sys.argv:
        sdk = sys.argv[1]
        constants, docs = enums(read(os.path.join(sdk, "CDXConstants.h"))), doc_types()
        for const, name in sorted(attribute_names(sdk).items()):
            ty, doc_ty = constants.get(const, (0, None))[1], docs.get(name, (None,))[0]
            if ty and doc_ty and KINDS.get(ty) != KINDS.get(doc_ty) and const not in SUPPLEMENT_TYPES:
                print(f"type differs from doc/md: {name} {const} {ty} (doc: {doc_ty})")
        for const, name, ty in untyped:
            print(f"left out (no usable type): {name} {const} {ty}")
        print(f"{len(objects)} objects, {len(props)} properties, {len(untyped)} left out")
        sys.exit()
    emit(objects, props, flags)
