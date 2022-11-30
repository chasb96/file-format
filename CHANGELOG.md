# Version 0.9.0 (2022-11-30)

## API

- Add `FileFormat::kind`
- Rename `AdobeInDesignDocument` to `AdobeIndesignDocument`
- Rename `Ani` to `WindowsAnimatedCursor`
- Rename `AppleQuickTime` to `AppleQuicktime`
- Rename `Cur` to `WindowsCursor`
- Rename `EmbeddedOpenType` to `EmbeddedOpentype`
- Rename `Ico` to `WindowsIcon`
- Rename `JavaKeyStore` to `JavaKeystore`
- Rename `MacOsAlias` to `MacosAlias`
- Rename `MicrosoftVisioDrawing` to `OfficeOpenXmlDrawing`
- Rename `MpegAudioLayer3` to `Mpeg12AudioLayer3`
- Rename `OfficeOpenXmlWorkbook` to `OfficeOpenXmlSpreadsheet`
- Rename `OpenExr` to `Openexr`
- Rename `OpenType` to `Opentype`
- Rename `ScreamTracker3Module` to `Screamtracker3Module`
- Rename `SeqBox` to `Seqbox`
- Rename `SketchUp` to `Sketchup`
- Rename `TrueType` to `Truetype`
- Rename `VirtualBoxVirtualDiskImage` to `VirtualboxVirtualDiskImage`
- Rename `WavPack` to `Wavpack`
- Rename `WebAssemblyBinary` to `WebassemblyBinary`
- Rename `WebP` to `Webp`
- Rename `WindowsExecutable` to `MsDosExecutable`
- Rename `XpInstall` to `Xpinstall`

## Fixes

- Add `MPEG-1/2 Audio Layer 3` signature
- Add `Matroska Video` signature
- Change UNIX archiver extension from `ar` to `a` (preferred)
- Fix `Apple QuickTime` signature
- Fix `Joint Photographic Experts Group` signature

## Internal changes

- Add `formats` macro
- Split items into modules

## New formats support

- Adobe Illustrator Artwork - `ai`
- Circuit Diagram Document - `cddx`
- Creative Voice - `voc`
- DER Certificate - `der`
- DjVu - `djvu`
- Dynamic Link Library - `dll`
- Encapsulated PostScript - `eps`
- Enterprise Application Archive - `ear`
- Keyhole Markup Language Zipped - `kmz`
- LLVM Bitcode - `bc`
- Lua Bytecode - `luac`
- MPEG-1 Audio Layer 1 - `mp1`
- MPEG-1 Audio Layer 2 - `mp2`
- MPEG-2 Transport Stream - `mts`
- Meta Information Encapsulation - `mie`
- Microsoft Access 2007 Database - `accdb`
- Microsoft Access Database - `mdb`
- Microsoft Excel Spreadsheet - `xls`
- Microsoft PowerPoint Presentation - `ppt`
- Microsoft Project Plan - `mpp`
- Microsoft Publisher Document - `pub`
- Microsoft Software Installer - `msi`
- Microsoft Visio Drawing - `vsd`
- Microsoft Word Document - `doc`
- PEM Certificate - `crt`
- PEM Certificate Signing Request - `csr`
- PEM Private Key - `key`
- PGP Message - `asc`
- PGP Private Key Block - `asc`
- PGP Public Key Block - `asc`
- PGP Signature - `asc`
- PgpSignedMessage - `asc`
- Portable Executable - `exe`
- PostScript - `ps`
- Rich Text Format - `rtf`
- Sony Movie - `mqv`
- TASTy - `tasty`
- Web Application Archive - `war`
- WebM - `webm`
- Windows App Package - `appx`
- iOS App Store Package - `ipa`

# Version 0.8.0 (2022-11-06)

## API

- Add `FileFormat::from_reader`
- Add `impl From<&[u8]> for FileFormat`

## Docs

- Add `lefi` example
- Add `CHANGELOG.md`

## Internal changes

- Make signature offset optional
- Remove FileFormat enum generation with macro
- Simplify `signatures` macro

## New formats support

- 3D Manufacturing Format - `3mf`
- Android Package - `apk`
- Design Web Format XPS - `dwfx`
- Electronic Publication - `epub`
- Java Archive - `jar`
- Microsoft Visio Drawing - `vsdx`
- Microsoft Visual Studio Extension - `vsix`
- Office Open XML Document - `docx`
- Office Open XML Presentation - `pptx`
- Office Open XML Workbook - `xlsx`
- OpenDocument Graphics - `odg`
- OpenDocument Presentation - `odp`
- OpenDocument Spreadsheet - `ods`
- OpenDocument Text - `odt`
- XAP - `xap`
- XPInstall - `xpi`

# Version 0.7.0 (2022-08-22)

## New formats support

- Android Binary XML - `xml`
- Android Compiled Resources - `arsc`
- Optimized Dalvik Executable - `dey`

# Version 0.6.0 (2021-12-18)

## API

- Add `FileFormat::from_bytes`

# Version 0.5.0 (2021-12-12)

## API

- Switch back FileFormat type from a structure to an enum

## Discontinued formats

- 3D Manufacturing Format - `3mf`
- BDAV MPEG-2 Transport Stream - `m2ts`
- Java Archive - `jar`
- Microsoft Visio Drawing - `vsdx`
- Office Open XML Document - `docx`
- Office Open XML Presentation - `pptx`
- Office Open XML Workbook - `xlsx`
- OpenDocument Graphics - `odg`
- OpenDocument Presentation - `odp`
- OpenDocument Spreadsheet - `ods`
- OpenDocument Text - `odt`
- Web Application Resource - `war`
- XAP - `xap`
- XPInstall - `xpi`

## Improvements

- Add precision to the `Dalvik Executable` signature
- Switch to Rust 2021

## New formats support

- Java KeyStore - `jks`

# Version 0.4.0 (2021-10-22)

## Docs

- Reorganize supported file formats table

## Improvements

- Add tests for all `High Efficiency Image Coding Sequence` format
- Add tests for all `High Efficiency Image Coding` format

## New formats support

- 3D Manufacturing Format - `3mf`
- Java Archive - `jar`
- Microsoft DirectDraw Surface - `dds`
- Microsoft Visio Drawing - `vsdx`
- Office Open XML Document - `docx`
- Office Open XML Presentation - `pptx`
- Office Open XML Workbook - `xlsx`
- Radiance HDR - `hdr`
- Web Application Resource - `war`
- XAP - `xap`
- XPInstall - `xpi`

# Version 0.3.0 (2021-10-18)

## API

- Switch FileFormat type from an enum to a structure

## Discontinued formats

- HyperText Markup Language - `html`

## Fixes

- Use of the correct `Tag Image File Format` extension

## Improvements

- Add new `Apple QuickTime` signatures
- Add new `Audio Interchange File Format` signature
- Add precision to the `Debian Binary Package` signature
- Add precision to the `Flexible Image Transport System` signature
- Add precision to the `Windows Media Video` signature
- Add precision to the `Windows Shortcut` signature
- Improve support of some formats
- Replace `Windows Installer` by `Compound File Binary`

## New formats support

- ALZ - `alz`
- ANI - `ani`
- Adobe Flash Player Audio - `f4a`
- Adobe Flash Player Audiobook - `f4b`
- Apache Arrow Columnar - `arrow`
- Apple iTunes Audiobook - `m4b`
- CUR - `cur`
- Canon Raw 2 - `cr2`
- Canon Raw 3 - `cr3`
- FastTracker 2 Extended Module - `xm`
- Fujifilm Raw - `raf`
- Impulse Tracker Module - `it`
- LHA - `lzh`
- Lempel–Ziv Finite State Entropy - `lzfse`
- Microsoft Compiled HTML Help - `chm`
- Microsoft Virtual Hard Disk - `vhd`
- Microsoft Virtual Hard Disk 2 - `vhdx`
- Nikon Electronic File - `nef`
- Panasonic Raw - `rw2`
- Qualcomm PureVoice - `qcp`
- ScreamTracker 3 Module - `s3m`
- SeqBox - `sbx`
- Snappy - `sz`
- Sony DSD Stream File - `dsf`
- cpio - `cpio`
- macOS Alias - `alias`
- zoo - `zoo`

# Version 0.2.1 (2021-10-14)

## Fixes

- Fix `Tag Image File Format` signature

# Version 0.2.0 (2021-10-07)

## New formats support

- Animated Portable Network Graphics - `apng`
- BDAV MPEG-2 Transport Stream - `m2ts`
- Electronic Publication - `epub`
- Game Boy Color ROM - `gbc`
- HyperText Markup Language - `html`
- Khronos Texture - `ktx`
- Khronos Texture 2 - `ktx2`
- Material Exchange Format - `mxf`
- Mobipocket - `mobi`
- Olympus Raw Format - `orf`
- OpenDocument Graphics - `odg`
- OpenDocument Presentation - `odp`
- OpenDocument Spreadsheet - `ods`
- OpenDocument Text - `odt`
- Rich Text Format - `rtf`
- Shapefile - `shp`
- SketchUp - `skp`
- UNIX archiver - `ar`

# Version 0.1.0 (2021-10-03)

First version.