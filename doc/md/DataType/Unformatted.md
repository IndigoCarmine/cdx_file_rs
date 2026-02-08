## CDX Unformatted Data Type

Unformatted data is simply a sequence of bytes. It has no meaning in the context of a `href="../CDXML` file, and can only obtain meaning in some other context. It is used for storing binary blobs of data, for example embedded OLE Objects (in files created on Windows machines) or PICTs (in files created on Macintosh machines).

**In CDXML files**, unformatted data containing bytes outside the normal printable alphanumeric range should be hex-encoded. For example: `MacPrintInfo="00030000025802580000000018F51319F...`

See the complete list of CDX data types

---

CDX Documentation index