# Wallpaper Engine `.pkg` File Format Documentation

This document describes the binary structure of the `.pkg` files used by Wallpaper Engine to package scene assets (models, textures, scripts, etc.).

## Overview

The `.pkg` format is a simple, index-based archive. It consists of a **Header**, a **File Table** (Directory), and a **Data Blob**.

*   **Endianness:** Little Endian
*   **String Encoding:** UTF-8
*   **Integer Sizes:** 32-bit (`u32`)
*   **Note:** Because offsets are `u32`, the theoretical maximum size of a `.pkg` file is 4 GB.

---

## File Structure

The file is laid out sequentially in three distinct sections.

```text
+------------------+ 
|  HEADER          |  <-- Starts at Byte 0
+------------------+
|  FILE TABLE      |  <-- Starts after Header
|  (Entry 1)       |
|  (Entry 2)       |
|  ...             |
+------------------+
|  DATA BLOB       |  <-- Starts at arbitrary offsets
|  [File Data]     |
|  [File Data]     |
+------------------+
```

---

## 1. Header

The header contains metadata required to parse the file table.

| Offset | Type | Size | Description |
|--------|------|------|-------------|
| `0x00` | `u32` | 4 | **Version String Length**. The number of bytes to read for the version string. |
| `0x04` | `char[]` | Variable | **Version String**. Typically `PKGV0022` (or similar). |
| `0x04+len` | `u32` | 4 | **File Count**. The total number of file entries in the table. |

**Example:**
If the version string is `PKGV0022` (8 bytes):
1. Read `u32` -> `8`
2. Read 8 bytes -> `PKGV0022`
3. Read `u32` -> Total file count (e.g., `167`)

---

## 2. File Table

The File Table is a flat array of entries. It contains no directory hierarchy; directories are implied by the file paths (e.g., `models/box.json`).

The table repeats the following structure for every file defined in **File Count**:

### Entry Structure

| Field | Type | Size | Description |
|-------|------|------|-------------|
| **Path Length** | `u32` | 4 | Length of the file path string. |
| **Path** | `char[]` | Variable | The relative path of the file within the package (e.g., `scene.json`, `materials/texture.tex`). |
| **Offset** | `u32` | 4 | The absolute byte offset in the file where the file's data begins. |
| **Size** | `u32` | 4 | The size of the file data in bytes. |

---

## 3. Data Blob

The data section contains the raw content of the files. There are no separators or padding bytes between files.

To read a file:
1. Locate its entry in the **File Table**.
2. Seek to the **Offset**.
3. Read **Size** bytes.
