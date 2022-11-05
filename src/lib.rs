#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{BufRead, BufReader, Cursor, Read, Result, Seek},
    path::Path,
    str,
};

/// A file format.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileFormat {
    /// Adaptive Multi-Rate - `amr`
    AdaptiveMultiRate,
    /// Adobe Flash Player Audio - `f4a`
    AdobeFlashPlayerAudio,
    /// Adobe Flash Player Audiobook - `f4b`
    AdobeFlashPlayerAudiobook,
    /// Adobe Flash Player Protected Video - `f4p`
    AdobeFlashPlayerProtectedVideo,
    /// Adobe Flash Player Video - `f4v`
    AdobeFlashPlayerVideo,
    /// Adobe InDesign Document - `indd`
    AdobeInDesignDocument,
    /// Adobe Photoshop Document - `psd`
    AdobePhotoshopDocument,
    /// Advanced Audio Coding - `aac`
    AdvancedAudioCoding,
    /// ALZ - `alz`
    Alz,
    /// Android Binary XML - `xml`
    AndroidBinaryXml,
    /// Android Compiled Resources - `arsc`
    AndroidCompiledResources,
    /// Android Package - `apk`
    #[cfg(feature = "zip")]
    AndroidPackage,
    /// ANI - `ani`
    Ani,
    /// Animated Portable Network Graphics - `apng`
    AnimatedPortableNetworkGraphics,
    /// Apache Arrow Columnar - `arrow`
    ApacheArrowColumnar,
    /// Apple Disk Image - `dmg`
    AppleDiskImage,
    /// Apple Icon Image - `icns`
    AppleIconImage,
    /// Apple iTunes Audio - `m4a`
    AppleItunesAudio,
    /// Apple iTunes Audiobook - `m4b`
    AppleItunesAudiobook,
    /// Apple iTunes Protected Audio - `m4p`
    AppleItunesProtectedAudio,
    /// Apple iTunes Video - `m4v`
    AppleItunesVideo,
    /// Apple QuickTime - `mov`
    AppleQuickTime,
    /// Arbitrary Binary Data - `bin`
    ArbitraryBinaryData,
    /// Archived by Robert Jung - `arj`
    ArchivedByRobertJung,
    /// Au - `au`
    Au,
    /// Audio Codec 3 - `ac3`
    AudioCodec3,
    /// Audio Interchange File Format - `aiff`
    AudioInterchangeFileFormat,
    /// Audio Video Interleave - `avi`
    AudioVideoInterleave,
    /// AV1 Image File Format - `avif`
    Av1ImageFileFormat,
    /// AV1 Image File Format Sequence - `avifs`
    Av1ImageFileFormatSequence,
    /// Better Portable Graphics - `bpg`
    BetterPortableGraphics,
    /// Blender - `blend`
    Blender,
    /// bzip2 - `bz2`
    Bzip2,
    /// Cabinet - `cab`
    Cabinet,
    /// Canon Raw 2 - `cr2`
    CanonRaw2,
    /// Canon Raw 3 - `cr3`
    CanonRaw3,
    /// Cineon - `cin`
    Cineon,
    /// Compound File Binary - `cfb`
    CompoundFileBinary,
    /// cpio - `cpio`
    Cpio,
    /// CUR - `cur`
    Cur,
    /// Dalvik Executable - `dex`
    DalvikExecutable,
    /// Debian Binary Package - `deb`
    DebianBinaryPackage,
    /// Design Web Format XPS - `dwfx`
    #[cfg(feature = "zip")]
    DesignWebFormatXps,
    /// Digital Imaging and Communications in Medicine - `dcm`
    DigitalImagingAndCommunicationsInMedicine,
    /// Digital Picture Exchange - `dpx`
    DigitalPictureExchange,
    /// Electronic Publication - `epub`
    #[cfg(feature = "zip")]
    ElectronicPublication,
    /// Embedded OpenType - `eot`
    EmbeddedOpenType,
    /// Executable and Linkable Format - `elf`
    ExecutableAndLinkableFormat,
    /// Experimental Computing Facility - `xcf`
    ExperimentalComputingFacility,
    /// Extensible Archive - `xar`
    ExtensibleArchive,
    /// FastTracker 2 Extended Module - `xm`
    FastTracker2ExtendedModule,
    /// Flash Video - `flv`
    FlashVideo,
    /// Flexible Image Transport System - `fits`
    FlexibleImageTransportSystem,
    /// Free Lossless Audio Codec - `flac`
    FreeLosslessAudioCodec,
    /// Free Lossless Image Format - `flif`
    FreeLosslessImageFormat,
    /// Fujifilm Raw - `raf`
    FujifilmRaw,
    /// Game Boy Advance ROM - `gba`
    GameBoyAdvanceRom,
    /// Game Boy Color ROM - `gbc`
    GameBoyColorRom,
    /// Game Boy ROM - `gb`
    GameBoyRom,
    /// GL Transmission Format Binary - `glb`
    GlTransmissionFormatBinary,
    /// Google Chrome Extension - `crx`
    GoogleChromeExtension,
    /// Graphics Interchange Format - `gif`
    GraphicsInterchangeFormat,
    /// gzip - `gz`
    Gzip,
    /// High Efficiency Image Coding - `heic`
    HighEfficiencyImageCoding,
    /// High Efficiency Image Coding Sequence - `heics`
    HighEfficiencyImageCodingSequence,
    /// High Efficiency Image File Format - `heif`
    HighEfficiencyImageFileFormat,
    /// High Efficiency Image File Format Sequence - `heifs`
    HighEfficiencyImageFileFormatSequence,
    /// ICO - `ico`
    Ico,
    /// Impulse Tracker Module - `it`
    ImpulseTrackerModule,
    /// ISO 9660 - `iso`
    Iso9660,
    /// Java Archive - `jar`
    #[cfg(feature = "zip")]
    JavaArchive,
    /// Java Class - `class`
    JavaClass,
    /// Java KeyStore - `jks`
    JavaKeyStore,
    /// Joint Photographic Experts Group - `jpg`
    JointPhotographicExpertsGroup,
    /// JPEG 2000 Part 1 - `jp2`
    Jpeg2000Part1,
    /// JPEG 2000 Part 2 - `jpx`
    Jpeg2000Part2,
    /// JPEG 2000 Part 3 - `mj2`
    Jpeg2000Part3,
    /// JPEG 2000 Part 6 - `jpm`
    Jpeg2000Part6,
    /// JPEG Extended Range - `jxr`
    JpegExtendedRange,
    /// JPEG XL - `jxl`
    JpegXl,
    /// Khronos Texture - `ktx`
    KhronosTexture,
    /// Khronos Texture 2 - `ktx2`
    KhronosTexture2,
    /// Lempel–Ziv Finite State Entropy - `lzfse`
    LempelZivFiniteStateEntropy,
    /// LHA - `lzh`
    Lha,
    /// Long Range ZIP - `lrz`
    LongRangeZip,
    /// LZ4 - `lz4`
    Lz4,
    /// lzip - `lz`
    Lzip,
    /// lzop - `lzo`
    Lzop,
    /// macOS Alias - `alias`
    MacOsAlias,
    /// Material Exchange Format - `mxf`
    MaterialExchangeFormat,
    /// Matroska Video - `mkv`
    MatroskaVideo,
    /// Microsoft Compiled HTML Help - `chm`
    MicrosoftCompiledHtmlHelp,
    /// Microsoft DirectDraw Surface - `dds`
    MicrosoftDirectDrawSurface,
    /// Microsoft Virtual Hard Disk - `vhd`
    MicrosoftVirtualHardDisk,
    /// Microsoft Virtual Hard Disk 2 - `vhdx`
    MicrosoftVirtualHardDisk2,
    /// Microsoft Visio Drawing - `vsdx`
    #[cfg(feature = "zip")]
    MicrosoftVisioDrawing,
    /// Microsoft Visual Studio Extension - `vsix`
    #[cfg(feature = "zip")]
    MicrosoftVisualStudioExtension,
    /// Mobipocket - `mobi`
    Mobipocket,
    /// Monkey's Audio - `ape`
    MonkeysAudio,
    /// MPEG-1 Video - `mpg`
    Mpeg1Video,
    /// MPEG-4 Part 14 Video - `mp4`
    Mpeg4Part14Video,
    /// MPEG-1/2 Audio Layer III - `mp3`
    MpegAudioLayer3,
    /// Musepack - `mpc`
    Musepack,
    /// Musical Instrument Digital Interface - `mid`
    MusicalInstrumentDigitalInterface,
    /// Nikon Electronic File - `nef`
    NikonElectronicFile,
    /// Nintendo 64 ROM - `z64`
    Nintendo64Rom,
    /// Nintendo DS ROM - `nds`
    NintendoDsRom,
    /// Nintendo Entertainment System ROM - `nes`
    NintendoEntertainmentSystemRom,
    /// Office Open XML Document - `docx`
    #[cfg(feature = "zip")]
    OfficeOpenXmlDocument,
    /// Office Open XML Presentation - `pptx`
    #[cfg(feature = "zip")]
    OfficeOpenXmlPresentation,
    /// Office Open XML Workbook - `xlsx`
    #[cfg(feature = "zip")]
    OfficeOpenXmlWorkbook,
    /// Ogg FLAC - `oga`
    OggFlac,
    /// Ogg Media - `ogm`
    OggMedia,
    /// Ogg Multiplexed Media - `ogx`
    OggMultiplexedMedia,
    /// Ogg Opus - `opus`
    OggOpus,
    /// Ogg Speex - `spx`
    OggSpeex,
    /// Ogg Theora - `ogv`
    OggTheora,
    /// Ogg Vorbis - `ogg`
    OggVorbis,
    /// Olympus Raw Format - `orf`
    OlympusRawFormat,
    /// OpenDocument Graphics - `odg`
    #[cfg(feature = "zip")]
    OpenDocumentGraphics,
    /// OpenDocument Presentation - `odp`
    #[cfg(feature = "zip")]
    OpenDocumentPresentation,
    /// OpenDocument Spreadsheet - `ods`
    #[cfg(feature = "zip")]
    OpenDocumentSpreadsheet,
    /// OpenDocument Text - `odt`
    #[cfg(feature = "zip")]
    OpenDocumentText,
    /// OpenEXR - `exr`
    OpenExr,
    /// OpenType - `otf`
    OpenType,
    /// Optimized Dalvik Executable - `dey`
    OptimizedDalvikExecutable,
    /// Panasonic Raw - `rw2`
    PanasonicRaw,
    /// PCAP Dump - `pcap`
    PcapDump,
    /// PCAP Next Generation Dump - `pcapng`
    PcapNextGenerationDump,
    /// Portable Document Format - `pdf`
    PortableDocumentFormat,
    /// Portable Network Graphics - `png`
    PortableNetworkGraphics,
    /// Qualcomm PureVoice - `qcp`
    QualcommPureVoice,
    /// Radiance HDR - `hdr`
    RadianceHdr,
    /// Red Hat Package Manager - `rpm`
    RedHatPackageManager,
    /// Roshal Archive - `rar`
    RoshalArchive,
    /// ScreamTracker 3 Module - `s3m`
    ScreamTracker3Module,
    /// SeqBox - `sbx`
    SeqBox,
    /// 7-Zip - `7z`
    SevenZip,
    /// Shapefile - `shp`
    Shapefile,
    /// SketchUp - `skp`
    SketchUp,
    /// Small Web Format - `swf`
    SmallWebFormat,
    /// Snappy - `sz`
    Snappy,
    /// Sony DSD Stream File - `dsf`
    SonyDsdStreamFile,
    /// SQLite 3 - `sqlite`
    Sqlite3,
    /// Tag Image File Format - `tiff`
    TagImageFileFormat,
    /// Tape Archive - `tar`
    TapeArchive,
    /// 3rd Generation Partnership Project - `3gp`
    ThirdGenerationPartnershipProject,
    /// 3rd Generation Partnership Project 2 - `3g2`
    ThirdGenerationPartnershipProject2,
    /// 3D Manufacturing Format - `3mf`
    #[cfg(feature = "zip")]
    ThreeDimensionalManufacturingFormat,
    /// TrueType - `ttf`
    TrueType,
    /// UNIX archiver - `ar`
    UnixArchiver,
    /// UNIX compress - `Z`
    UnixCompress,
    /// VirtualBox Virtual Disk Image - `vdi`
    VirtualBoxVirtualDiskImage,
    /// WavPack - `wv`
    WavPack,
    /// Waveform Audio - `wav`
    WaveformAudio,
    /// WebAssembly Binary - `wasm`
    WebAssemblyBinary,
    /// Web Open Font Format - `woff`
    WebOpenFontFormat,
    /// Web Open Font Format 2 - `woff2`
    WebOpenFontFormat2,
    /// WebP - `webp`
    WebP,
    /// Windows Bitmap - `bmp`
    WindowsBitmap,
    /// Windows Executable - `exe`
    WindowsExecutable,
    /// Windows Media Video - `wmv`
    WindowsMediaVideo,
    /// Windows Metafile - `wmf`
    WindowsMetafile,
    /// Windows Shortcut - `lnk`
    WindowsShortcut,
    /// XAP - `xap`
    #[cfg(feature = "zip")]
    Xap,
    /// XPInstall - `xpi`
    #[cfg(feature = "zip")]
    XpInstall,
    /// XZ - `xz`
    Xz,
    /// ZIP - `zip`
    Zip,
    /// zoo - `zoo`
    Zoo,
    /// Zstandard - `zst`
    Zstandard,
}

impl FileFormat {
    /// Maximum number of bytes to read to detect the `FileFormat`.
    const MAX_BYTES: usize = 36870;

    /// Returns the name of the `FileFormat`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_format::FileFormat;
    ///
    /// let format = FileFormat::MpegAudioLayer3;
    /// assert_eq!(format.name(), "MPEG-1/2 Audio Layer III");
    ///```
    pub fn name(&self) -> &str {
        match self {
            Self::AdaptiveMultiRate => "Adaptive Multi-Rate",
            Self::AdobeFlashPlayerAudio => "Adobe Flash Player Audio",
            Self::AdobeFlashPlayerAudiobook => "Adobe Flash Player Audiobook ",
            Self::AdobeFlashPlayerProtectedVideo => "Adobe Flash Player Protected Video",
            Self::AdobeFlashPlayerVideo => "Adobe Flash Player Video",
            Self::AdobeInDesignDocument => "Adobe InDesign Document",
            Self::AdobePhotoshopDocument => "Adobe Photoshop Document",
            Self::AdvancedAudioCoding => "Advanced Audio Coding",
            Self::Alz => "ALZ",
            Self::AndroidBinaryXml => "Android Binary XML",
            Self::AndroidCompiledResources => "Android Compiled Resources",
            #[cfg(feature = "zip")]
            Self::AndroidPackage => "Android Package",
            Self::Ani => "ANI",
            Self::AnimatedPortableNetworkGraphics => "Animated Portable Network Graphics",
            Self::ApacheArrowColumnar => "Apache Arrow Columnar",
            Self::AppleDiskImage => "Apple Disk Image",
            Self::AppleIconImage => "Apple Icon Image",
            Self::AppleItunesAudio => "Apple iTunes Audio",
            Self::AppleItunesAudiobook => "Apple iTunes Audiobook",
            Self::AppleItunesProtectedAudio => "Apple iTunes Protected Audio",
            Self::AppleItunesVideo => "Apple iTunes Video",
            Self::AppleQuickTime => "Apple QuickTime",
            Self::ArbitraryBinaryData => "Arbitrary Binary Data",
            Self::ArchivedByRobertJung => "Archived by Robert Jung",
            Self::Au => "Au",
            Self::AudioCodec3 => "Audio Codec 3",
            Self::AudioInterchangeFileFormat => "Audio Interchange File Format",
            Self::AudioVideoInterleave => "Audio Video Interleave",
            Self::Av1ImageFileFormat => "AV1 Image File Format",
            Self::Av1ImageFileFormatSequence => "AV1 Image File Format Sequence",
            Self::BetterPortableGraphics => "Better Portable Graphics",
            Self::Blender => "Blender",
            Self::Bzip2 => "bzip2",
            Self::Cabinet => "Cabinet",
            Self::CanonRaw2 => "Canon Raw 2",
            Self::CanonRaw3 => "Canon Raw 3",
            Self::Cineon => "Cineon",
            Self::CompoundFileBinary => "Compound File Binary",
            Self::Cpio => "cpio",
            Self::Cur => "CUR",
            Self::DalvikExecutable => "Dalvik Executable",
            Self::DebianBinaryPackage => "Debian Binary Package",
            #[cfg(feature = "zip")]
            Self::DesignWebFormatXps => "Design Web Format XPS",
            Self::DigitalImagingAndCommunicationsInMedicine => {
                "Digital Imaging and Communications in Medicine"
            }
            Self::DigitalPictureExchange => "Digital Picture Exchange",
            #[cfg(feature = "zip")]
            Self::ElectronicPublication => "Electronic Publication",
            Self::EmbeddedOpenType => "Embedded OpenType",
            Self::ExecutableAndLinkableFormat => "Executable and Linkable Format",
            Self::ExperimentalComputingFacility => "Experimental Computing Facility",
            Self::ExtensibleArchive => "Extensible Archive",
            Self::FastTracker2ExtendedModule => "FastTracker 2 Extended Module",
            Self::FlashVideo => "Flash Video",
            Self::FlexibleImageTransportSystem => "Flexible Image Transport System",
            Self::FreeLosslessAudioCodec => "Free Lossless Audio Codec",
            Self::FreeLosslessImageFormat => "Free Lossless Image Format",
            Self::FujifilmRaw => "Fujifilm Raw",
            Self::GameBoyAdvanceRom => "Game Boy Advance ROM",
            Self::GameBoyColorRom => "Game Boy Color ROM",
            Self::GameBoyRom => "Game Boy ROM",
            Self::GlTransmissionFormatBinary => "GL Transmission Format Binary",
            Self::GoogleChromeExtension => "Google Chrome Extension",
            Self::GraphicsInterchangeFormat => "Graphics Interchange Format",
            Self::Gzip => "gzip",
            Self::HighEfficiencyImageCoding => "High Efficiency Image Coding",
            Self::HighEfficiencyImageCodingSequence => "High Efficiency Image Coding Sequence",
            Self::HighEfficiencyImageFileFormat => "High Efficiency Image File Format",
            Self::HighEfficiencyImageFileFormatSequence => {
                "High Efficiency Image File Format Sequence"
            }
            Self::Ico => "ICO",
            Self::ImpulseTrackerModule => "Impulse Tracker Module",
            Self::Iso9660 => "ISO 9660",
            #[cfg(feature = "zip")]
            Self::JavaArchive => "Java Archive",
            Self::JavaClass => "Java Class",
            Self::JavaKeyStore => "Java KeyStore",
            Self::JointPhotographicExpertsGroup => "Joint Photographic Experts Group",
            Self::Jpeg2000Part1 => "JPEG 2000 Part 1",
            Self::Jpeg2000Part2 => "JPEG 2000 Part 2",
            Self::Jpeg2000Part3 => "JPEG 2000 Part 3",
            Self::Jpeg2000Part6 => "JPEG 2000 Part 6",
            Self::JpegExtendedRange => "PEG Extended Range",
            Self::JpegXl => "JPEG XL",
            Self::KhronosTexture => "Khronos Texture",
            Self::KhronosTexture2 => "Khronos Texture 2",
            Self::LempelZivFiniteStateEntropy => "Lempel–Ziv Finite State Entropy",
            Self::Lha => "LHA",
            Self::LongRangeZip => "Long Range ZIP",
            Self::Lz4 => "LZ4",
            Self::Lzip => "lzip",
            Self::Lzop => "lzop",
            Self::MacOsAlias => "macOS Alias",
            Self::MaterialExchangeFormat => "Material Exchange Format",
            Self::MatroskaVideo => "Matroska Video",
            Self::MicrosoftCompiledHtmlHelp => "Microsoft Compiled HTML Help",
            Self::MicrosoftDirectDrawSurface => "Microsoft DirectDraw Surface",
            Self::MicrosoftVirtualHardDisk => "Microsoft Virtual Hard Disk",
            Self::MicrosoftVirtualHardDisk2 => "Microsoft Virtual Hard Disk 2",
            #[cfg(feature = "zip")]
            Self::MicrosoftVisioDrawing => "Microsoft Visio Drawing",
            #[cfg(feature = "zip")]
            Self::MicrosoftVisualStudioExtension => "Microsoft Visual Studio Extension",
            Self::Mobipocket => "Mobipocket",
            Self::MonkeysAudio => "Monkey's Audio",
            Self::Mpeg1Video => "MPEG-1 Video",
            Self::Mpeg4Part14Video => "MPEG-4 Part 14 Video",
            Self::MpegAudioLayer3 => "MPEG-1/2 Audio Layer III",
            Self::Musepack => "Musepack",
            Self::MusicalInstrumentDigitalInterface => "Musical Instrument Digital Interface",
            Self::NikonElectronicFile => "Nikon Electronic File",
            Self::Nintendo64Rom => "Nintendo 64 ROM",
            Self::NintendoDsRom => "Nintendo DS ROM",
            Self::NintendoEntertainmentSystemRom => "Nintendo Entertainment System ROM",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlDocument => "Office Open XML Document",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlPresentation => "Office Open XML Presentation",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlWorkbook => "Office Open XML Workbook",
            Self::OggFlac => "Ogg FLAC",
            Self::OggMedia => "Ogg Media",
            Self::OggMultiplexedMedia => "Ogg Multiplexed Media",
            Self::OggOpus => "Ogg Opus",
            Self::OggSpeex => "Ogg Speex",
            Self::OggTheora => "Ogg Theora",
            Self::OggVorbis => "Ogg Vorbis",
            Self::OlympusRawFormat => "Olympus Raw Format",
            #[cfg(feature = "zip")]
            Self::OpenDocumentGraphics => "OpenDocument Graphics",
            #[cfg(feature = "zip")]
            Self::OpenDocumentPresentation => "OpenDocument Presentation",
            #[cfg(feature = "zip")]
            Self::OpenDocumentSpreadsheet => "OpenDocument Spreadsheet",
            #[cfg(feature = "zip")]
            Self::OpenDocumentText => "OpenDocument Text",
            Self::OpenExr => "OpenEXR",
            Self::OpenType => "OpenType",
            Self::OptimizedDalvikExecutable => "Optimized Dalvik Executable",
            Self::PanasonicRaw => "Panasonic Raw",
            Self::PcapDump => "PCAP Dump",
            Self::PcapNextGenerationDump => "PCAP Next Generation Dump",
            Self::PortableDocumentFormat => "Portable Document Format",
            Self::PortableNetworkGraphics => "Portable Network Graphics",
            Self::QualcommPureVoice => "Qualcomm PureVoice",
            Self::RadianceHdr => "Radiance HDR",
            Self::RedHatPackageManager => "Red Hat Package Manager",
            Self::RoshalArchive => "Roshal Archive",
            Self::ScreamTracker3Module => "ScreamTracker 3 Module",
            Self::SeqBox => "SeqBox",
            Self::SevenZip => "7-Zip",
            Self::Shapefile => "Shapefile",
            Self::SketchUp => "SketchUp",
            Self::SmallWebFormat => "Small Web Format",
            Self::Snappy => "Snappy",
            Self::SonyDsdStreamFile => "Sony DSD Stream File",
            Self::Sqlite3 => "SQLite 3",
            Self::TagImageFileFormat => "Tag Image File Format",
            Self::TapeArchive => "Tape Archive",
            Self::ThirdGenerationPartnershipProject => "3rd Generation Partnership Project",
            Self::ThirdGenerationPartnershipProject2 => "3rd Generation Partnership Project 2",
            #[cfg(feature = "zip")]
            Self::ThreeDimensionalManufacturingFormat => "3D Manufacturing Format",
            Self::TrueType => "TrueType",
            Self::UnixArchiver => "UNIX archiver",
            Self::UnixCompress => "UNIX compress",
            Self::VirtualBoxVirtualDiskImage => "VirtualBox Virtual Disk Image",
            Self::WavPack => "WavPack",
            Self::WaveformAudio => "Waveform Audio",
            Self::WebAssemblyBinary => "WebAssembly Binary",
            Self::WebOpenFontFormat => "Web Open Font Format",
            Self::WebOpenFontFormat2 => "Web Open Font Format 2",
            Self::WebP => "WebP",
            Self::WindowsBitmap => "Windows Bitmap",
            Self::WindowsExecutable => "Windows Executable",
            Self::WindowsMediaVideo => "Windows Media Video",
            Self::WindowsMetafile => "Windows Metafile",
            Self::WindowsShortcut => "Windows Shortcut",
            #[cfg(feature = "zip")]
            Self::Xap => "XAP",
            #[cfg(feature = "zip")]
            Self::XpInstall => "XPInstall",
            Self::Xz => "XZ",
            Self::Zip => "ZIP",
            Self::Zoo => "zoo",
            Self::Zstandard => "Zstandard",
        }
    }

    /// Returns the media type (formerly known as MIME type) of the `FileFormat`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_format::FileFormat;
    ///
    /// let format = FileFormat::Zstandard;
    /// assert_eq!(format.media_type(), "application/zstd");
    ///```
    pub fn media_type(&self) -> &str {
        match self {
            Self::AdaptiveMultiRate => "audio/amr",
            Self::AdobeFlashPlayerAudio => "audio/mp4",
            Self::AdobeFlashPlayerAudiobook => "audio/mp4",
            Self::AdobeFlashPlayerProtectedVideo => "video/mp4",
            Self::AdobeFlashPlayerVideo => "video/mp4",
            Self::AdobeInDesignDocument => "application/x-indesign",
            Self::AdobePhotoshopDocument => "image/vnd.adobe.photoshop",
            Self::AdvancedAudioCoding => "audio/aac",
            Self::Alz => "application/x-alz-compressed",
            Self::AndroidBinaryXml => "application/vnd.android.axml",
            Self::AndroidCompiledResources => "application/vnd.android.arsc",
            #[cfg(feature = "zip")]
            Self::AndroidPackage => "application/vnd.android.package-archive",
            Self::Ani => "application/x-navi-animation",
            Self::AnimatedPortableNetworkGraphics => "image/apng",
            Self::ApacheArrowColumnar => "application/x-apache-arrow",
            Self::AppleDiskImage => "application/x-apple-diskimage",
            Self::AppleIconImage => "image/x-icns",
            Self::AppleItunesAudio => "audio/x-m4a",
            Self::AppleItunesAudiobook => "audio/mp4",
            Self::AppleItunesProtectedAudio => "audio/mp4",
            Self::AppleItunesVideo => "video/x-m4v",
            Self::AppleQuickTime => "video/quicktime",
            Self::ArbitraryBinaryData => "application/octet-stream",
            Self::ArchivedByRobertJung => "application/x-arj",
            Self::Au => "audio/basic",
            Self::AudioCodec3 => "audio/vnd.dolby.dd-raw",
            Self::AudioInterchangeFileFormat => "audio/aiff",
            Self::AudioVideoInterleave => "video/avi",
            Self::Av1ImageFileFormat => "image/avif",
            Self::Av1ImageFileFormatSequence => "image/avif-sequence",
            Self::BetterPortableGraphics => "image/bpg",
            Self::Blender => "application/x-blender",
            Self::Bzip2 => "application/x-bzip2",
            Self::Cabinet => "application/vnd.ms-cab-compressed",
            Self::CanonRaw2 => "image/x-canon-cr2",
            Self::CanonRaw3 => "image/x-canon-cr3",
            Self::Cineon => "image/cineon",
            Self::CompoundFileBinary => "application/x-cfb",
            Self::Cpio => "application/x-cpio",
            Self::Cur => "image/x-icon",
            Self::DalvikExecutable => "application/vnd.android.dex",
            Self::DebianBinaryPackage => "application/vnd.debian.binary-package",
            #[cfg(feature = "zip")]
            Self::DesignWebFormatXps => "model/vnd.dwfx+xps",
            Self::DigitalImagingAndCommunicationsInMedicine => "application/dicom",
            Self::DigitalPictureExchange => "image/x-dpx",
            #[cfg(feature = "zip")]
            Self::ElectronicPublication => "application/epub+zip",
            Self::EmbeddedOpenType => "application/vnd.ms-fontobject",
            Self::ExecutableAndLinkableFormat => "application/x-executable",
            Self::ExperimentalComputingFacility => "image/x-xcf",
            Self::ExtensibleArchive => "application/x-xar",
            Self::FastTracker2ExtendedModule => "audio/x-xm",
            Self::FlashVideo => "video/x-flv",
            Self::FlexibleImageTransportSystem => "image/fits",
            Self::FreeLosslessAudioCodec => "audio/x-flac",
            Self::FreeLosslessImageFormat => "image/flif",
            Self::FujifilmRaw => "image/x-fuji-raf",
            Self::GameBoyAdvanceRom => "application/x-gba-rom",
            Self::GameBoyColorRom => "application/x-gameboy-color-rom",
            Self::GameBoyRom => "application/x-gameboy-rom",
            Self::GlTransmissionFormatBinary => "model/gltf-binary",
            Self::GoogleChromeExtension => "application/x-google-chrome-extension",
            Self::GraphicsInterchangeFormat => "image/gif",
            Self::Gzip => "application/gzip",
            Self::HighEfficiencyImageCoding => "image/heic",
            Self::HighEfficiencyImageCodingSequence => "image/heic-sequence",
            Self::HighEfficiencyImageFileFormat => "image/heif",
            Self::HighEfficiencyImageFileFormatSequence => "image/heif-sequence",
            Self::Ico => "image/x-icon",
            Self::ImpulseTrackerModule => "audio/x-it",
            Self::Iso9660 => "application/x-iso9660-image",
            #[cfg(feature = "zip")]
            Self::JavaArchive => "application/java-archive",
            Self::JavaClass => "application/java-vm",
            Self::JavaKeyStore => "application/x-java-keystore",
            Self::JointPhotographicExpertsGroup => "image/jpeg",
            Self::Jpeg2000Part1 => "image/jp2",
            Self::Jpeg2000Part2 => "image/jpx",
            Self::Jpeg2000Part3 => "image/mj2",
            Self::Jpeg2000Part6 => "image/jpm",
            Self::JpegExtendedRange => "image/jxr",
            Self::JpegXl => "image/jxl",
            Self::KhronosTexture => "image/ktx",
            Self::KhronosTexture2 => "image/ktx2",
            Self::LempelZivFiniteStateEntropy => "application/x-lzfse",
            Self::Lha => "application/x-lzh-compressed",
            Self::LongRangeZip => "application/x-lrzip",
            Self::Lz4 => "application/x-lz4",
            Self::Lzip => "application/x-lzip",
            Self::Lzop => "application/x-lzop",
            Self::MacOsAlias => "application/x-apple-alias",
            Self::MaterialExchangeFormat => "application/mxf",
            Self::MatroskaVideo => "video/x-matroska",
            Self::MicrosoftCompiledHtmlHelp => "application/vnd.ms-htmlhelp",
            Self::MicrosoftDirectDrawSurface => "image/vnd-ms.dds",
            Self::MicrosoftVirtualHardDisk => "application/x-vhd",
            Self::MicrosoftVirtualHardDisk2 => "application/x-vhdx",
            #[cfg(feature = "zip")]
            Self::MicrosoftVisioDrawing => "application/vnd.ms-visio.drawing.main+xml",
            #[cfg(feature = "zip")]
            Self::MicrosoftVisualStudioExtension => "application/vsix",
            Self::Mobipocket => "application/x-mobipocket-ebook",
            Self::MonkeysAudio => "audio/x-ape",
            Self::Mpeg1Video => "video/mpeg",
            Self::Mpeg4Part14Video => "video/mp4",
            Self::MpegAudioLayer3 => "audio/mpeg",
            Self::Musepack => "audio/x-musepack",
            Self::MusicalInstrumentDigitalInterface => "audio/midi",
            Self::NikonElectronicFile => "image/x-nikon-nef",
            Self::Nintendo64Rom => "application/x-n64-rom",
            Self::NintendoDsRom => "application/x-nintendo-ds-rom",
            Self::NintendoEntertainmentSystemRom => "application/x-nintendo-nes-rom",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlDocument => {
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            }
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlPresentation => {
                "application/vnd.openxmlformats-officedocument.presentationml.presentation"
            }
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlWorkbook => {
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            }
            Self::OggFlac => "audio/ogg",
            Self::OggMedia => "video/ogg",
            Self::OggMultiplexedMedia => "application/ogg",
            Self::OggOpus => "audio/opus",
            Self::OggSpeex => "audio/ogg",
            Self::OggTheora => "video/ogg",
            Self::OggVorbis => "audio/ogg",
            Self::OlympusRawFormat => "image/x-olympus-orf",
            #[cfg(feature = "zip")]
            Self::OpenDocumentGraphics => "application/vnd.oasis.opendocument.graphics",
            #[cfg(feature = "zip")]
            Self::OpenDocumentPresentation => "application/vnd.oasis.opendocument.presentation",
            #[cfg(feature = "zip")]
            Self::OpenDocumentSpreadsheet => "application/vnd.oasis.opendocument.spreadsheet",
            #[cfg(feature = "zip")]
            Self::OpenDocumentText => "application/vnd.oasis.opendocument.text",
            Self::OpenExr => "image/x-exr",
            Self::OpenType => "font/otf",
            Self::OptimizedDalvikExecutable => "application/vnd.android.dey",
            Self::PanasonicRaw => "image/x-panasonic-rw2",
            Self::PcapDump => "application/vnd.tcpdump.pcap",
            Self::PcapNextGenerationDump => "application/x-pcapng",
            Self::PortableDocumentFormat => "application/pdf",
            Self::PortableNetworkGraphics => "image/png",
            Self::QualcommPureVoice => "audio/qcelp",
            Self::RadianceHdr => "image/vnd.radiance",
            Self::RedHatPackageManager => "application/x-rpm",
            Self::RoshalArchive => "application/vnd.rar",
            Self::ScreamTracker3Module => "audio/x-s3m",
            Self::SeqBox => "application/x-sbx",
            Self::SevenZip => "application/x-7z-compressed",
            Self::Shapefile => "application/x-esri-shape",
            Self::SketchUp => "application/vnd.sketchup.skp",
            Self::SmallWebFormat => "application/x-shockwave-flash",
            Self::Snappy => "application/x-snappy-framed",
            Self::SonyDsdStreamFile => "audio/x-dsf",
            Self::Sqlite3 => "application/vnd.sqlite3",
            Self::TagImageFileFormat => "image/tiff",
            Self::TapeArchive => "application/x-tar",
            Self::ThirdGenerationPartnershipProject => "video/3gpp",
            Self::ThirdGenerationPartnershipProject2 => "video/3gpp2",
            #[cfg(feature = "zip")]
            Self::ThreeDimensionalManufacturingFormat => {
                "application/vnd.ms-package.3dmanufacturing-3dmodel+xml"
            }
            Self::TrueType => "font/ttf",
            Self::UnixArchiver => "application/x-archive",
            Self::UnixCompress => "application/x-compress",
            Self::VirtualBoxVirtualDiskImage => "application/x-virtualbox-vdi",
            Self::WavPack => "audio/wavpack",
            Self::WaveformAudio => "audio/vnd.wave",
            Self::WebAssemblyBinary => "application/wasm",
            Self::WebOpenFontFormat => "font/woff",
            Self::WebOpenFontFormat2 => "font/woff2",
            Self::WebP => "image/webp",
            Self::WindowsBitmap => "image/bmp",
            Self::WindowsExecutable => "application/x-msdownload",
            Self::WindowsMediaVideo => "video/x-ms-asf",
            Self::WindowsMetafile => "image/wmf",
            Self::WindowsShortcut => "application/x-ms-shortcut",
            #[cfg(feature = "zip")]
            Self::Xap => "application/x-silverlight-app",
            #[cfg(feature = "zip")]
            Self::XpInstall => "application/x-xpinstall",
            Self::Xz => "application/x-xz",
            Self::Zip => "application/zip",
            Self::Zoo => "application/x-zoo",
            Self::Zstandard => "application/zstd",
        }
    }

    /// Returns the extension of the `FileFormat`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_format::FileFormat;
    ///
    /// let format = FileFormat::WindowsMediaVideo;
    /// assert_eq!(format.extension(), "wmv");
    ///```
    pub fn extension(&self) -> &str {
        match self {
            Self::AdaptiveMultiRate => "amr",
            Self::AdobeFlashPlayerAudio => "f4a",
            Self::AdobeFlashPlayerAudiobook => "f4b",
            Self::AdobeFlashPlayerProtectedVideo => "f4p",
            Self::AdobeFlashPlayerVideo => "f4v",
            Self::AdobeInDesignDocument => "indd",
            Self::AdobePhotoshopDocument => "psd",
            Self::AdvancedAudioCoding => "aac",
            Self::Alz => "alz",
            Self::AndroidBinaryXml => "xml",
            Self::AndroidCompiledResources => "arsc",
            #[cfg(feature = "zip")]
            Self::AndroidPackage => "apk",
            Self::Ani => "ani",
            Self::AnimatedPortableNetworkGraphics => "apng",
            Self::ApacheArrowColumnar => "arrow",
            Self::AppleDiskImage => "dmg",
            Self::AppleIconImage => "icns",
            Self::AppleItunesAudio => "m4a",
            Self::AppleItunesAudiobook => "m4b",
            Self::AppleItunesProtectedAudio => "m4p",
            Self::AppleItunesVideo => "m4v",
            Self::AppleQuickTime => "mov",
            Self::ArbitraryBinaryData => "bin",
            Self::ArchivedByRobertJung => "arj",
            Self::Au => "au",
            Self::AudioCodec3 => "ac3",
            Self::AudioInterchangeFileFormat => "aiff",
            Self::AudioVideoInterleave => "avi",
            Self::Av1ImageFileFormat => "avif",
            Self::Av1ImageFileFormatSequence => "avifs",
            Self::BetterPortableGraphics => "bpg",
            Self::Blender => "blend",
            Self::Bzip2 => "bz2",
            Self::Cabinet => "cab",
            Self::CanonRaw2 => "cr2",
            Self::CanonRaw3 => "cr3",
            Self::Cineon => "cin",
            Self::CompoundFileBinary => "cfb",
            Self::Cpio => "cpio",
            Self::Cur => "cur",
            Self::DalvikExecutable => "dex",
            Self::DebianBinaryPackage => "deb",
            #[cfg(feature = "zip")]
            Self::DesignWebFormatXps => "dwfx",
            Self::DigitalImagingAndCommunicationsInMedicine => "dcm",
            Self::DigitalPictureExchange => "dpx",
            #[cfg(feature = "zip")]
            Self::ElectronicPublication => "epub",
            Self::EmbeddedOpenType => "eot",
            Self::ExecutableAndLinkableFormat => "elf",
            Self::ExperimentalComputingFacility => "xcf",
            Self::ExtensibleArchive => "xar",
            Self::FastTracker2ExtendedModule => "xm",
            Self::FlashVideo => "flv",
            Self::FlexibleImageTransportSystem => "fits",
            Self::FreeLosslessAudioCodec => "flac",
            Self::FreeLosslessImageFormat => "flif",
            Self::FujifilmRaw => "raf",
            Self::GameBoyAdvanceRom => "gba",
            Self::GameBoyColorRom => "gbc",
            Self::GameBoyRom => "gb",
            Self::GlTransmissionFormatBinary => "glb",
            Self::GoogleChromeExtension => "crx",
            Self::GraphicsInterchangeFormat => "gif",
            Self::Gzip => "gz",
            Self::HighEfficiencyImageCoding => "heic",
            Self::HighEfficiencyImageCodingSequence => "heics",
            Self::HighEfficiencyImageFileFormat => "heif",
            Self::HighEfficiencyImageFileFormatSequence => "heifs",
            Self::Ico => "ico",
            Self::ImpulseTrackerModule => "it",
            Self::Iso9660 => "iso",
            #[cfg(feature = "zip")]
            Self::JavaArchive => "jar",
            Self::JavaClass => "class",
            Self::JavaKeyStore => "jks",
            Self::JointPhotographicExpertsGroup => "jpg",
            Self::Jpeg2000Part1 => "jp2",
            Self::Jpeg2000Part2 => "jpx",
            Self::Jpeg2000Part3 => "mj2",
            Self::Jpeg2000Part6 => "jpm",
            Self::JpegExtendedRange => "jxr",
            Self::JpegXl => "jxl",
            Self::KhronosTexture => "ktx",
            Self::KhronosTexture2 => "ktx2",
            Self::LempelZivFiniteStateEntropy => "lzfse",
            Self::Lha => "lzh",
            Self::LongRangeZip => "lrz",
            Self::Lz4 => "lz4",
            Self::Lzip => "lz",
            Self::Lzop => "lzo",
            Self::MacOsAlias => "alias",
            Self::MaterialExchangeFormat => "mxf",
            Self::MatroskaVideo => "mkv",
            Self::MicrosoftCompiledHtmlHelp => "chm",
            Self::MicrosoftDirectDrawSurface => "dds",
            Self::MicrosoftVirtualHardDisk => "vhd",
            Self::MicrosoftVirtualHardDisk2 => "vhdx",
            #[cfg(feature = "zip")]
            Self::MicrosoftVisioDrawing => "vsdx",
            #[cfg(feature = "zip")]
            Self::MicrosoftVisualStudioExtension => "vsix",
            Self::Mobipocket => "mobi",
            Self::MonkeysAudio => "ape",
            Self::Mpeg1Video => "mpg",
            Self::Mpeg4Part14Video => "mp4",
            Self::MpegAudioLayer3 => "mp3",
            Self::Musepack => "mpc",
            Self::MusicalInstrumentDigitalInterface => "mid",
            Self::NikonElectronicFile => "nef",
            Self::Nintendo64Rom => "z64",
            Self::NintendoDsRom => "nds",
            Self::NintendoEntertainmentSystemRom => "nes",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlDocument => "docx",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlPresentation => "pptx",
            #[cfg(feature = "zip")]
            Self::OfficeOpenXmlWorkbook => "xlsx",
            Self::OggFlac => "oga",
            Self::OggMedia => "ogm",
            Self::OggMultiplexedMedia => "ogx",
            Self::OggOpus => "opus",
            Self::OggSpeex => "spx",
            Self::OggTheora => "ogv",
            Self::OggVorbis => "ogg",
            Self::OlympusRawFormat => "orf",
            #[cfg(feature = "zip")]
            Self::OpenDocumentGraphics => "odg",
            #[cfg(feature = "zip")]
            Self::OpenDocumentPresentation => "odp",
            #[cfg(feature = "zip")]
            Self::OpenDocumentSpreadsheet => "ods",
            #[cfg(feature = "zip")]
            Self::OpenDocumentText => "odt",
            Self::OpenExr => "exr",
            Self::OpenType => "otf",
            Self::OptimizedDalvikExecutable => "dey",
            Self::PanasonicRaw => "rw2",
            Self::PcapDump => "pcap",
            Self::PcapNextGenerationDump => "pcapng",
            Self::PortableDocumentFormat => "pdf",
            Self::PortableNetworkGraphics => "png",
            Self::QualcommPureVoice => "qcp",
            Self::RadianceHdr => "hdr",
            Self::RedHatPackageManager => "rpm",
            Self::RoshalArchive => "rar",
            Self::ScreamTracker3Module => "s3m",
            Self::SeqBox => "sbx",
            Self::SevenZip => "7z",
            Self::Shapefile => "shp",
            Self::SketchUp => "skp",
            Self::SmallWebFormat => "swf",
            Self::Snappy => "sz",
            Self::SonyDsdStreamFile => "dsf",
            Self::Sqlite3 => "sqlite",
            Self::TagImageFileFormat => "tiff",
            Self::TapeArchive => "tar",
            Self::ThirdGenerationPartnershipProject => "3gp",
            Self::ThirdGenerationPartnershipProject2 => "3g2",
            #[cfg(feature = "zip")]
            Self::ThreeDimensionalManufacturingFormat => "3mf",
            Self::TrueType => "ttf",
            Self::UnixArchiver => "ar",
            Self::UnixCompress => "Z",
            Self::VirtualBoxVirtualDiskImage => "vdi",
            Self::WavPack => "wv",
            Self::WaveformAudio => "wav",
            Self::WebAssemblyBinary => "wasm",
            Self::WebOpenFontFormat => "woff",
            Self::WebOpenFontFormat2 => "woff2",
            Self::WebP => "webp",
            Self::WindowsBitmap => "bmp",
            Self::WindowsExecutable => "exe",
            Self::WindowsMediaVideo => "wmv",
            Self::WindowsMetafile => "wmf",
            Self::WindowsShortcut => "lnk",
            #[cfg(feature = "zip")]
            Self::Xap => "xap",
            #[cfg(feature = "zip")]
            Self::XpInstall => "xpi",
            Self::Xz => "xz",
            Self::Zip => "zip",
            Self::Zoo => "zoo",
            Self::Zstandard => "zst",
        }
    }

    /// Determines `FileFormat` from bytes.
    ///
    /// If the `FileFormat` is not recognized, the [default value] will be returned.
    ///
    /// # Examples
    ///
    /// Detects from the first bytes of a PNG file:
    ///
    /// ```rust
    /// use file_format::FileFormat;
    ///
    /// let format = FileFormat::from_bytes(b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A");
    /// assert_eq!(format, FileFormat::PortableNetworkGraphics);
    ///```
    ///
    /// Detects from a zeroed buffer:
    ///
    /// ```rust
    /// use file_format::FileFormat;
    ///
    /// let format = FileFormat::from_bytes(&[0; 1000]);
    /// assert_eq!(format, FileFormat::ArbitraryBinaryData);
    ///```
    ///
    /// [default value]: FileFormat::default
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> FileFormat {
        FileFormat::from_reader(Cursor::new(bytes)).unwrap_or_default()
    }

    /// Determines `FileFormat` from a file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_format::FileFormat;
    ///
    /// let format = FileFormat::from_file("fixtures/video/sample.mkv")?;
    /// assert_eq!(format, FileFormat::MatroskaVideo);
    /// # Ok::<(), std::io::Error>(())
    ///```
    #[inline]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<FileFormat> {
        FileFormat::from_reader(File::open(path)?)
    }

    /// Determines `FileFormat` from a reader.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_format::FileFormat;
    ///
    /// let format = FileFormat::from_reader(std::io::empty())?;
    /// assert_eq!(format, FileFormat::default());
    /// # Ok::<(), std::io::Error>(())
    ///```
    #[cfg(not(feature = "zip"))]
    pub fn from_reader<R: Read + Seek>(reader: R) -> Result<FileFormat> {
        let mut reader = BufReader::with_capacity(FileFormat::MAX_BYTES, reader);
        Ok(FileFormat::from_signature(reader.fill_buf()?).unwrap_or_default())
    }

    /// Determines `FileFormat` from a reader.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_format::FileFormat;
    ///
    /// let format = FileFormat::from_reader(std::io::empty())?;
    /// assert_eq!(format, FileFormat::default());
    /// # Ok::<(), std::io::Error>(())
    ///```
    #[cfg(feature = "zip")]
    pub fn from_reader<R: Read + Seek>(reader: R) -> Result<FileFormat> {
        let mut reader = BufReader::with_capacity(FileFormat::MAX_BYTES, reader);
        Ok(match FileFormat::from_signature(reader.fill_buf()?) {
            Some(FileFormat::Zip) => FileFormat::from_zip(reader).unwrap_or_default(),
            Some(format) => format,
            _ => FileFormat::default(),
        })
    }

    #[cfg(feature = "zip")]
    fn from_zip<R: Read + Seek>(reader: R) -> Result<FileFormat> {
        let mut archive = zip::ZipArchive::new(reader)?;
        for index in 0..archive.len() {
            let mut file = archive.by_index(index)?;
            let filename = file.name();
            if filename == "AndroidManifest.xml" {
                return Ok(FileFormat::AndroidPackage);
            } else if filename == "AppManifest.xaml" {
                return Ok(FileFormat::Xap);
            } else if filename == "extension.vsixmanifest" {
                return Ok(FileFormat::MicrosoftVisualStudioExtension);
            } else if filename == "META-INF/mozilla.rsa" {
                return Ok(FileFormat::XpInstall);
            } else if filename == "META-INF/MANIFEST.MF" {
                return Ok(FileFormat::JavaArchive);
            } else if filename == "mimetype" {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                match content.as_str() {
                    "application/epub+zip" => {
                        return Ok(FileFormat::ElectronicPublication);
                    }
                    "application/vnd.oasis.opendocument.text" => {
                        return Ok(FileFormat::OpenDocumentText);
                    }
                    "application/vnd.oasis.opendocument.spreadsheet" => {
                        return Ok(FileFormat::OpenDocumentSpreadsheet);
                    }
                    "application/vnd.oasis.opendocument.presentation" => {
                        return Ok(FileFormat::OpenDocumentPresentation);
                    }
                    "application/vnd.oasis.opendocument.graphics" => {
                        return Ok(FileFormat::OpenDocumentGraphics);
                    }
                    _ => {}
                }
            } else if filename.starts_with("dwf/") {
                return Ok(FileFormat::DesignWebFormatXps);
            } else if filename.starts_with("word/") {
                return Ok(FileFormat::OfficeOpenXmlDocument);
            } else if filename.starts_with("ppt/") {
                return Ok(FileFormat::OfficeOpenXmlPresentation);
            } else if filename.starts_with("xl/") {
                return Ok(FileFormat::OfficeOpenXmlWorkbook);
            } else if filename.starts_with("visio/") {
                return Ok(FileFormat::MicrosoftVisioDrawing);
            } else if filename.starts_with("3D/") && filename.ends_with(".model") {
                return Ok(FileFormat::ThreeDimensionalManufacturingFormat);
            }
        }
        Ok(FileFormat::Zip)
    }
}

impl Default for FileFormat {
    /// Returns the default `FileFormat` which corresponds to [`FileFormat::ArbitraryBinaryData`].
    #[inline]
    fn default() -> FileFormat {
        FileFormat::ArbitraryBinaryData
    }
}

impl Display for FileFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Generates [`FileFormat::from_signature`] function.
macro_rules! signatures {
    {
        $(
            -   file_format: $file_format:ident
                signatures:
                $(
                    -   parts:
                    $(
                        - value: $signature:literal
                          $(offset: $offset:literal)?
                    )+
                )+
        )*
    } => {
        impl FileFormat {
            /// Determines `FileFormat` by checking its signature.
            fn from_signature(bytes: &[u8]) -> Option<FileFormat> {
                $(
                    if
                        $(
                            $(
                                bytes.len() >= $($offset +)? $signature.len()
                                    && &bytes[$($offset)?..$($offset +)? $signature.len()] == $signature
                            )&&*
                        )||*
                    { return Some(FileFormat::$file_format); }
                )*
                None
            }
        }
    };
}

signatures! {
  - file_format: VirtualBoxVirtualDiskImage
    signatures:
      - parts:
        - value: b"<<< Oracle VM VirtualBox Disk Image >>>"

  // 32-byte signatures
  - file_format: SketchUp
    signatures:
      - parts:
        - value: b"\xFF\xFE\xFF\x0E\x53\x00\x6B\x00\x65\x00\x74\x00\x63\x00\x68\x00"
        - value: b"\x55\x00\x70\x00\x20\x00\x4D\x00\x6F\x00\x64\x00\x65\x00\x6C\x00"
          offset: 16

  // 29-byte signatures
  - file_format: FlexibleImageTransportSystem
    signatures:
      - parts:
        - value: b"\x49\x4D\x50\x4C\x45\x20\x20\x3D\x20\x20\x20\x20\x20\x20\x20"
        - value: b"\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x54"
          offset: 15

  // 21-byte signatures
  - file_format: DebianBinaryPackage
    signatures:
      - parts:
        - value: b"!<arch>\ndebian-binary"

  // 20-byte signatures
  - file_format: WindowsShortcut
    signatures:
      - parts:
        - value: b"\x4C\x00\x00\x00\x01\x14\x02\x00\x00\x00\x00\x00\xC0\x00\x00\x00\x00\x00\x00\x46"

  // 16-byte signatures
  - file_format: AdobeInDesignDocument
    signatures:
      - parts:
        - value: b"\x06\x06\xED\xF5\xD8\x1D\x46\xE5\xBD\x31\xEF\xE7\xFE\x74\xB7\x1D"

  - file_format: FastTracker2ExtendedModule
    signatures:
      - parts:
        - value: b"Extended Module:"

  - file_format: MacOsAlias
    signatures:
      - parts:
        - value: b"\x62\x6F\x6F\x6B\x00\x00\x00\x00\x6D\x61\x72\x6B\x00\x00\x00\x00"

  - file_format: Sqlite3
    signatures:
      - parts:
        - value: b"\x53\x51\x4C\x69\x74\x65\x20\x66\x6F\x72\x6D\x61\x74\x20\x33\x00"

  - file_format: WindowsMediaVideo
    signatures:
      - parts:
        - value: b"\x30\x26\xB2\x75\x8E\x66\xCF\x11\xA6\xD9\x00\xAA\x00\x62\xCE\x6C"

  // 15-byte signatures
  - file_format: FujifilmRaw
    signatures:
      - parts:
        - value: b"FUJIFILMCCD-RAW"

  // 14-byte signatures
  - file_format: MaterialExchangeFormat
    signatures:
      - parts:
        - value: b"\x06\x0E\x2B\x34\x02\x05\x01\x01\x0D\x01\x02\x01\x01\x02"

  // 12-byte signatures
  - file_format: AnimatedPortableNetworkGraphics
    signatures:
      - parts:
        - value: b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A"
        - value: b"acTL"
          offset: 0x25

  - file_format: JointPhotographicExpertsGroup
    signatures:
      - parts:
        - value: b"\xFF\xD8\xFF\xE0\x00\x10\x4A\x46\x49\x46\x00\x01"
      - parts:
        - value: b"\xFF\xD8\xFF\xE1"
        - value: b"\x45\x78\x69\x66\x00\x00"
          offset: 6
      - parts:
        - value: b"\xFF\xD8\xFF\xDB"
      - parts:
        - value: b"\xFF\xD8\xFF\xEE"

  - file_format: JpegXl
    signatures:
      - parts:
        - value: b"\x00\x00\x00\x0C\x4A\x58\x4C\x20\x0D\x0A\x87\x0A"
      - parts:
        - value: b"\xFF\x0A"

  - file_format: KhronosTexture
    signatures:
      - parts:
        - value: b"\xAB\x4B\x54\x58\x20\x31\x31\xBB\x0D\x0A\x1A\x0A"

  - file_format: KhronosTexture2
    signatures:
      - parts:
        - value: b"\xAB\x4B\x54\x58\x20\x32\x30\xBB\x0D\x0A\x1A\x0A"

  - file_format: MatroskaVideo
    signatures:
      - parts:
        - value: b"\x1A\x45\xDF\xA3"
        - value: b"matroska"
          offset: 24

  - file_format: OggOpus
    signatures:
      - parts:
        - value: b"OggS"
        - value: b"OpusHead"
          offset: 28

  - file_format: PanasonicRaw
    signatures:
      - parts:
        - value: b"\x49\x49\x55\x00\x18\x00\x00\x00\x88\xE7\x74\xD8"

  // 11-byte signatures
  - file_format: OggSpeex
    signatures:
      - parts:
        - value: b"OggS"
        - value: b"Speex  "
          offset: 28

  - file_format: OggTheora
    signatures:
      - parts:
        - value: b"OggS"
        - value: b"\x80\x74\x68\x65\x6F\x72\x61"
          offset: 28

  - file_format: OggVorbis
    signatures:
      - parts:
        - value: b"OggS"
        - value: b"\x01\x76\x6F\x72\x62\x69\x73"
          offset: 28

  - file_format: RadianceHdr
    signatures:
      - parts:
        - value: b"\x23\x3F\x52\x41\x44\x49\x41\x4E\x43\x45\x0A"

  // 10-byte signatures
  - file_format: AppleQuickTime
    signatures:
      - parts:
        - value: b"\x00\x00\x00\x14"
        - value: b"ftypqt"
          offset: 4
      - parts:
        - value: b"\x66\x72\x65\x65"
          offset: 4
      - parts:
        - value: b"\x6D\x64\x61\x74"
          offset: 4
      - parts:
        - value: b"\x6D\x6F\x6F\x76"
          offset: 4
      - parts:
        - value: b"\x77\x69\x64\x65"
          offset: 4

  - file_format: OggMedia
    signatures:
      - parts:
        - value: b"OggS"
        - value: b"\x01\x76\x69\x64\x65\x6F"
          offset: 28

  - file_format: Snappy
    signatures:
      - parts:
        - value: b"\xFF\x06\x00\x00\x73\x4E\x61\x50\x70\x59"

  // 9-byte signatures
  - file_format: GameBoyColorRom
    signatures:
      - parts:
        - value: b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B"
          offset: 0x104
        - value: b"\x80"
          offset: 0x143
      - parts:
        - value: b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B"
          offset: 0x104
        - value: b"\xC0"
          offset: 0x143

  - file_format: Lzop
    signatures:
      - parts:
        - value: b"\x89\x4C\x5A\x4F\x00\x0D\x0A\x1A\x0A"

  - file_format: MicrosoftVirtualHardDisk
    signatures:
      - parts:
        - value: b"connectix"

  - file_format: OggFlac
    signatures:
      - parts:
        - value: b"OggS"
        - value: b"\x7F\x46\x4C\x41\x43"
          offset: 28

  - file_format: OlympusRawFormat
    signatures:
      - parts:
        - value: b"\x49\x49\x52\x4F\x08\x00\x00\x00\x18"

  // 8-byte signatures
  - file_format: Ani
    signatures:
      - parts:
        - value: b"RIFF"
        - value: b"ACON"
          offset: 8

  - file_format: AudioInterchangeFileFormat
    signatures:
      - parts:
        - value: b"FORM"
        - value: b"AIFF"
          offset: 8
      - parts:
        - value: b"FORM"
        - value: b"AIFC"
          offset: 8

  - file_format: AudioVideoInterleave
    signatures:
      - parts:
        - value: b"RIFF"
        - value: b"\x41\x56\x49\x20"
          offset: 8

  - file_format: Av1ImageFileFormat
    signatures:
      - parts:
        - value: b"ftypavif"
          offset: 4

  - file_format: Av1ImageFileFormatSequence
    signatures:
      - parts:
        - value: b"ftypavis"
          offset: 4

  - file_format: CompoundFileBinary
    signatures:
      - parts:
        - value: b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1"

  - file_format: DalvikExecutable
    signatures:
      - parts:
        - value: b"\x64\x65\x78\x0A\x30\x33\x35\x00"

  - file_format: ExperimentalComputingFacility
    signatures:
      - parts:
        - value: b"gimp xcf"

  - file_format: GameBoyAdvanceRom
    signatures:
      - parts:
        - value: b"\x24\xFF\xAE\x51\x69\x9A\xA2\x21"
          offset: 4

  - file_format: GameBoyRom
    signatures:
      - parts:
        - value: b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B"
          offset: 0x104

  - file_format: HighEfficiencyImageCoding
    signatures:
      - parts:
        - value: b"ftypheic"
          offset: 4
      - parts:
        - value: b"ftypheix"
          offset: 4

  - file_format: HighEfficiencyImageCodingSequence
    signatures:
      - parts:
        - value: b"ftyphevc"
          offset: 4
      - parts:
        - value: b"ftyphevx"
          offset: 4

  - file_format: HighEfficiencyImageFileFormat
    signatures:
      - parts:
        - value: b"ftypmif1"
          offset: 4

  - file_format: HighEfficiencyImageFileFormatSequence
    signatures:
      - parts:
        - value: b"ftypmsf1"
          offset: 4

  - file_format: Jpeg2000Part1
    signatures:
      - parts:
        - value: b"ftypjp2 "
          offset: 16

  - file_format: Jpeg2000Part2
    signatures:
      - parts:
        - value: b"ftypjpx "
          offset: 16

  - file_format: Jpeg2000Part3
    signatures:
      - parts:
        - value: b"ftypmjp2"
          offset: 16

  - file_format: Jpeg2000Part6
    signatures:
      - parts:
        - value: b"ftypjpm "
          offset: 16

  - file_format: MicrosoftVirtualHardDisk2
    signatures:
      - parts:
        - value: b"vhdxfile"

  - file_format: Mobipocket
    signatures:
      - parts:
        - value: b"BOOKMOBI"
          offset: 60

  - file_format: Mpeg4Part14Video
    signatures:
      - parts:
        - value: b"ftypavc1"
          offset: 4
      - parts:
        - value: b"ftypdash"
          offset: 4
      - parts:
        - value: b"ftypiso2"
          offset: 4
      - parts:
        - value: b"ftypiso3"
          offset: 4
      - parts:
        - value: b"ftypiso4"
          offset: 4
      - parts:
        - value: b"ftypiso5"
          offset: 4
      - parts:
        - value: b"ftypiso6"
          offset: 4
      - parts:
        - value: b"ftypisom"
          offset: 4
      - parts:
        - value: b"ftypmmp4"
          offset: 4
      - parts:
        - value: b"ftypmp41"
          offset: 4
      - parts:
        - value: b"ftypmp42"
          offset: 4
      - parts:
        - value: b"ftypmp4v"
          offset: 4
      - parts:
        - value: b"ftypmp71"
          offset: 4
      - parts:
        - value: b"ftypMSNV"
          offset: 4
      - parts:
        - value: b"ftypNDAS"
          offset: 4
      - parts:
        - value: b"ftypNDSC"
          offset: 4
      - parts:
        - value: b"ftypNDSH"
          offset: 4
      - parts:
        - value: b"ftypNDSM"
          offset: 4
      - parts:
        - value: b"ftypNDSP"
          offset: 4
      - parts:
        - value: b"ftypNDSS"
          offset: 4
      - parts:
        - value: b"ftypNDXC"
          offset: 4
      - parts:
        - value: b"ftypNDXH"
          offset: 4
      - parts:
        - value: b"ftypNDXM"
          offset: 4
      - parts:
        - value: b"ftypNDXP"
          offset: 4

  - file_format: NikonElectronicFile
    signatures:
      - parts:
        - value: b"\x4D\x4D\x00\x2A"
        - value: b"\x1C\x00\xFE\x00"
          offset: 8
      - parts:
        - value: b"\x4D\x4D\x00\x2A"
        - value: b"\x1F\x00\x0B\x00"
          offset: 8
      - parts:
        - value: b"\x49\x49\x2A\x00"
        - value: b"\x1C\x00\xFE\x00"
          offset: 8
      - parts:
        - value: b"\x49\x49\x2A\x00"
        - value: b"\x1F\x00\x0B\x00"
          offset: 8

  - file_format: Nintendo64Rom
    signatures:
      - parts:
        - value: b"\x80\x37\x12\x40\x00\x00\x00\x0F"
      - parts:
        - value: b"\x37\x80\x40\x12\x00\x00\x0F\x00"
      - parts:
        - value: b"\x12\x40\x80\x37\x00\x0F\x00\x00"
      - parts:
        - value: b"\x40\x12\x37\x80\x0F\x00\x00\x00"

  - file_format: NintendoDsRom
    signatures:
      - parts:
        - value: b"\x24\xFF\xAE\x51\x69\x9A\xA2\x21"
          offset: 0xC0
      - parts:
        - value: b"\xC8\x60\x4F\xE2\x01\x70\x8F\xE2"
          offset: 0xC0

  - file_format: PortableNetworkGraphics
    signatures:
      - parts:
        - value: b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A"

  - file_format: QualcommPureVoice
    signatures:
      - parts:
        - value: b"RIFF"
        - value: b"QLCM"
          offset: 8

  - file_format: RoshalArchive
    signatures:
      - parts:
        - value: b"\x52\x61\x72\x21\x1A\x07\x01\x00"
      - parts:
        - value: b"\x52\x61\x72\x21\x1A\x07\x00"

  - file_format: TapeArchive
    signatures:
      - parts:
        - value: b"\x75\x73\x74\x61\x72\x00\x30\x30"
          offset: 257
      - parts:
        - value: b"\x75\x73\x74\x61\x72\x20\x20\x00"
          offset: 257

  - file_format: WaveformAudio
    signatures:
      - parts:
        - value: b"RIFF"
        - value: b"WAVE"
          offset: 8

  - file_format: WebP
    signatures:
      - parts:
        - value: b"RIFF"
        - value: b"WEBP"
          offset: 8

  // 7-byte signatures
  - file_format: AdobeFlashPlayerAudio
    signatures:
      - parts:
        - value: b"ftypF4A"
          offset: 4

  - file_format: AdobeFlashPlayerAudiobook
    signatures:
      - parts:
        - value: b"ftypF4B"
          offset: 4

  - file_format: AdobeFlashPlayerProtectedVideo
    signatures:
      - parts:
        - value: b"ftypF4P"
          offset: 4

  - file_format: AdobeFlashPlayerVideo
    signatures:
      - parts:
        - value: b"ftypF4V"
          offset: 4

  - file_format: AppleItunesAudio
    signatures:
      - parts:
        - value: b"ftypM4A"
          offset: 4

  - file_format: AppleItunesAudiobook
    signatures:
      - parts:
        - value: b"ftypM4B"
          offset: 4

  - file_format: AppleItunesProtectedAudio
    signatures:
      - parts:
        - value: b"ftypM4P"
          offset: 4

  - file_format: AppleItunesVideo
    signatures:
      - parts:
        - value: b"ftypM4V"
          offset: 4

  - file_format: Blender
    signatures:
      - parts:
        - value: b"BLENDER"

  - file_format: CanonRaw3
    signatures:
      - parts:
        - value: b"ftypcrx"
          offset: 4

  - file_format: ThirdGenerationPartnershipProject
    signatures:
      - parts:
        - value: b"ftyp3gp"
          offset: 4

  - file_format: ThirdGenerationPartnershipProject2
    signatures:
      - parts:
        - value: b"ftyp3g2"
          offset: 4

  - file_format: UnixArchiver
    signatures:
      - parts:
        - value: b"!<arch>"

  // 6-byte signatures
  - file_format: ApacheArrowColumnar
    signatures:
      - parts:
        - value: b"ARROW1"

  - file_format: CanonRaw2
    signatures:
      - parts:
        - value: b"\x4D\x4D\x00\x2A"
        - value: b"CR"
          offset: 8
      - parts:
        - value: b"\x49\x49\x2A\x00"
        - value: b"CR"
          offset: 8

  - file_format: GraphicsInterchangeFormat
    signatures:
      - parts:
        - value: b"GIF87a"
      - parts:
        - value: b"GIF89a"

  - file_format: SevenZip
    signatures:
      - parts:
        - value: b"\x37\x7A\xBC\xAF\x27\x1C"

  - file_format: Xz
    signatures:
      - parts:
        - value: b"\xFD\x37\x7A\x58\x5A\x00"

  // 5-byte signatures
  - file_format: AdaptiveMultiRate
    signatures:
      - parts:
        - value: b"#!AMR"

  - file_format: EmbeddedOpenType
    signatures:
      - parts:
        - value: b"\x00\x00\x01"
          offset: 8
        - value: b"\x4C\x50"
          offset: 34
      - parts:
        - value: b"\x01\x00\x02"
          offset: 8
        - value: b"\x4C\x50"
          offset: 34
      - parts:
        - value: b"\x02\x00\x02"
          offset: 8
        - value: b"\x4C\x50"
          offset: 34

  - file_format: Iso9660
    signatures:
      - parts:
        - value: b"CD001"
          offset: 0x8001
      - parts:
        - value: b"CD001"
          offset: 0x8801
      - parts:
        - value: b"CD001"
          offset: 0x9001

  - file_format: Lha
    signatures:
      - parts:
        - value: b"-lh0-"
          offset: 2
      - parts:
        - value: b"-lh1-"
          offset: 2
      - parts:
        - value: b"-lh2-"
          offset: 2
      - parts:
        - value: b"-lh3-"
          offset: 2
      - parts:
        - value: b"-lh4-"
          offset: 2
      - parts:
        - value: b"-lh5-"
          offset: 2
      - parts:
        - value: b"-lh6-"
          offset: 2
      - parts:
        - value: b"-lh7-"
          offset: 2
      - parts:
        - value: b"-lzs-"
          offset: 2
      - parts:
        - value: b"-lz4-"
          offset: 2
      - parts:
        - value: b"-lz5-"
          offset: 2
      - parts:
        - value: b"-lhd-"
          offset: 2

  - file_format: OpenType
    signatures:
      - parts:
        - value: b"\x4F\x54\x54\x4F\x00"

  - file_format: PortableDocumentFormat
    signatures:
      - parts:
        - value: b"%PDF-"

  - file_format: TrueType
    signatures:
      - parts:
        - value: b"\x00\x01\x00\x00\x00"

  // 4-byte signatures
  - file_format: AdobePhotoshopDocument
    signatures:
      - parts:
        - value: b"8BPS"

  - file_format: Alz
    signatures:
      - parts:
        - value: b"\x41\x4C\x5A\x01"

  - file_format: AndroidBinaryXml
    signatures:
      - parts:
        - value: b"\x03\x00\x08\x00"

  - file_format: AndroidCompiledResources
    signatures:
      - parts:
        - value: b"\x02\x00\x0C\x00"

  - file_format: AppleIconImage
    signatures:
      - parts:
        - value: b"icns"

  - file_format: Au
    signatures:
      - parts:
        - value: b".snd"

  - file_format: BetterPortableGraphics
    signatures:
      - parts:
        - value: b"\x42\x50\x47\xFB"

  - file_format: Cabinet
    signatures:
      - parts:
        - value: b"MSCF"
      - parts:
        - value: b"ISc("

  - file_format: Cineon
    signatures:
      - parts:
        - value: b"\x80\x2A\x5F\xD7"

  - file_format: Cur
    signatures:
      - parts:
        - value: b"\x00\x00\x02\x00"

  - file_format: DigitalImagingAndCommunicationsInMedicine
    signatures:
      - parts:
        - value: b"\x44\x49\x43\x4D"
          offset: 128

  - file_format: DigitalPictureExchange
    signatures:
      - parts:
        - value: b"SDPX"
      - parts:
        - value: b"XPDS"

  - file_format: ExecutableAndLinkableFormat
    signatures:
      - parts:
        - value: b"\x7F\x45\x4C\x46"

  - file_format: ExtensibleArchive
    signatures:
      - parts:
        - value: b"xar!"

  - file_format: FlashVideo
    signatures:
      - parts:
        - value: b"\x46\x4C\x56\x01"

  - file_format: FreeLosslessAudioCodec
    signatures:
      - parts:
        - value: b"fLaC"

  - file_format: FreeLosslessImageFormat
    signatures:
      - parts:
        - value: b"FLIF"

  - file_format: GlTransmissionFormatBinary
    signatures:
      - parts:
        - value: b"glTF"

  - file_format: GoogleChromeExtension
    signatures:
      - parts:
        - value: b"Cr24"

  - file_format: Ico
    signatures:
      - parts:
        - value: b"\x00\x00\x01\x00"

  - file_format: ImpulseTrackerModule
    signatures:
      - parts:
        - value: b"IMPM"

  - file_format: JavaClass
    signatures:
      - parts:
        - value: b"\xCA\xFE\xBA\xBE"

  - file_format: JavaKeyStore
    signatures:
      - parts:
        - value: b"\xFE\xED\xFE\xED"

  - file_format: LempelZivFiniteStateEntropy
    signatures:
      - parts:
        - value: b"bvx-"
      - parts:
        - value: b"bvx1"
      - parts:
        - value: b"bvx2"
      - parts:
        - value: b"bvxn"

  - file_format: LongRangeZip
    signatures:
      - parts:
        - value: b"LRZI"

  - file_format: Lz4
    signatures:
      - parts:
        - value: b"\x04\x22\x4D\x18"

  - file_format: Lzip
    signatures:
      - parts:
        - value: b"LZIP"

  - file_format: MicrosoftCompiledHtmlHelp
    signatures:
      - parts:
        - value: b"ITSF"

  - file_format: MicrosoftDirectDrawSurface
    signatures:
      - parts:
        - value: b"DDS "

  - file_format: MonkeysAudio
    signatures:
      - parts:
        - value: b"MAC "

  - file_format: Mpeg1Video
    signatures:
      - parts:
        - value: b"\x00\x00\x01\xBA"
      - parts:
        - value: b"\x00\x00\x01\xB3"

  - file_format: Musepack
    signatures:
      - parts:
        - value: b"MPCK"
      - parts:
        - value: b"MP+"

  - file_format: MusicalInstrumentDigitalInterface
    signatures:
      - parts:
        - value: b"MThd"

  - file_format: NintendoEntertainmentSystemRom
    signatures:
      - parts:
        - value: b"\x4E\x45\x53\x1A"

  - file_format: OggMultiplexedMedia
    signatures:
      - parts:
        - value: b"OggS"

  - file_format: OpenExr
    signatures:
      - parts:
        - value: b"\x76\x2F\x31\x01"

  - file_format: OptimizedDalvikExecutable
    signatures:
      - parts:
        - value: b"dey\n"

  - file_format: PcapDump
    signatures:
      - parts:
        - value: b"\xA1\xB2\xC3\xD4"
      - parts:
        - value: b"\xD4\xC3\xB2\xA1"

  - file_format: PcapNextGenerationDump
    signatures:
      - parts:
        - value: b"\x0A\x0D\x0D\x0A"

  - file_format: RedHatPackageManager
    signatures:
      - parts:
        - value: b"\xED\xAB\xEE\xDB"

  - file_format: ScreamTracker3Module
    signatures:
      - parts:
        - value: b"SCRM"
          offset: 44

  - file_format: Shapefile
    signatures:
      - parts:
        - value: b"\x00\x00\x27\x0A"

  - file_format: SonyDsdStreamFile
    signatures:
      - parts:
        - value: b"DSD "

  - file_format: TagImageFileFormat
    signatures:
      - parts:
        - value: b"\x4D\x4D\x00\x2A"
      - parts:
        - value: b"\x49\x49\x2A\x00"
      - parts:
        - value: b"\x4D\x4D\x00\x2B"
      - parts:
        - value: b"\x49\x49\x2B\x00"

  - file_format: WavPack
    signatures:
      - parts:
        - value: b"wvpk"

  - file_format: WebAssemblyBinary
    signatures:
      - parts:
        - value: b"\x00\x61\x73\x6D"

  - file_format: WebOpenFontFormat
    signatures:
      - parts:
        - value: b"wOFF"

  - file_format: WebOpenFontFormat2
    signatures:
      - parts:
        - value: b"wOF2"

  - file_format: WindowsMetafile
    signatures:
      - parts:
        - value: b"\xD7\xCD\xC6\x9A"
      - parts:
        - value: b"\x02\x00\x09\x00"
      - parts:
        - value: b"\x01\x00\x09\x00"

  - file_format: Zip
    signatures:
      - parts:
        - value: b"\x50\x4B\x03\x04"
      - parts:
        - value: b"\x50\x4B\x05\x06"
      - parts:
        - value: b"\x50\x4B\x07\x08"

  - file_format: Zstandard
    signatures:
      - parts:
        - value: b"\x28\xB5\x2F\xFD"

  // 3-byte signatures
  - file_format: Bzip2
    signatures:
      - parts:
        - value: b"BZh"

  - file_format: JpegExtendedRange
    signatures:
      - parts:
        - value: b"\x49\x49\xBC"

  - file_format: MpegAudioLayer3
    signatures:
      - parts:
        - value: b"ID3"

  - file_format: SeqBox
    signatures:
      - parts:
        - value: b"SBx"

  - file_format: SmallWebFormat
    signatures:
      - parts:
        - value: b"\x43\x57\x53"
      - parts:
        - value: b"\x46\x57\x53"

  - file_format: Zoo
    signatures:
      - parts:
        - value: b"ZOO"

  // 2-byte signatures
  - file_format: AdvancedAudioCoding
    signatures:
      - parts:
        - value: b"\xFF\xF1"
      - parts:
        - value: b"\xFF\xF9"

  - file_format: AppleDiskImage
    signatures:
      - parts:
        - value: b"\x78\x01"

  - file_format: ArchivedByRobertJung
    signatures:
      - parts:
        - value: b"\x60\xEA"

  - file_format: AudioCodec3
    signatures:
      - parts:
        - value: b"\x0B\x77"

  - file_format: Cpio
    signatures:
      - parts:
        - value: b"\xC7\x71"
      - parts:
        - value: b"\x71\xC7"

  - file_format: Gzip
    signatures:
      - parts:
        - value: b"\x1F\x8B"

  - file_format: UnixCompress
    signatures:
      - parts:
        - value: b"\x1F\xA0"
      - parts:
        - value: b"\x1F\x9D"

  - file_format: WindowsBitmap
    signatures:
      - parts:
        - value: b"BM"

  - file_format: WindowsExecutable
    signatures:
      - parts:
        - value: b"MZ"
}
