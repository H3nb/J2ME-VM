# SPDX-License-Identifier: GPL-3.0-only
"""Rebuild the original package-container fixture using Python's standard library."""
from io import BytesIO, UnsupportedOperation
from pathlib import Path
from zipfile import ZIP_DEFLATED, ZIP_STORED, ZipFile, ZipInfo

root = Path(__file__).resolve().parent


class StreamingBuffer(BytesIO):
    def seek(self, *args):
        raise UnsupportedOperation("non-seekable output emits data descriptors")


stream = StreamingBuffer()
with ZipFile(stream, "w", allowZip64=False) as archive:
    for name, data, method in (
        ("data/", b"", ZIP_STORED),
        ("MinimalClass.class", (root / "compiled/MinimalClass.class").read_bytes(), ZIP_STORED),
        ("data/readme.txt", b"Original package resource.\n" * 12, ZIP_DEFLATED),
        ("data/second.txt", b"Second resource.", ZIP_STORED),
    ):
        entry = ZipInfo(name)
        entry.create_system = 0
        if name == "data/":
            entry.extra = b"\x0f\xf0\x02\x00ab"
        archive.writestr(entry, data, compress_type=method, compresslevel=6)
(root / "package-archive.zip").write_bytes(stream.getvalue())
