structures-generator is a tool to automatically generate Rust code to operate C-style data from APIs of Windows.

It collects URLs from json config file, then parse code from the designated web page.
Eventually it can generate Rust code that interop wich C APIs.
It can also used to analysis structures of a set of C APIs.

The project is still under heavy development.

# TODO
- ~~support unnamed union fields inside struct~~
- ~~support unnamed struct fields inside union~~
- support macro
- support array


# Sample Output
```
   Compiling structures-generator v0.0.1 (file:///D:/msys64rust/home/bombless/structures-generator)
     Running `target\structures-generator`
[https://msdn.microsoft.com/en-us/library/windows/desktop/ms680301]
1 code block(s):
typedef struct {
	00 - 04                          NumberOfSymbols;
	04 - 08                          LvaToFirstSymbol;
	08 - 0C                          NumberOfLinenumbers;
	0C - 10                          LvaToFirstLinenumber;
	10 - 14                          RvaToFirstByteOfCode;
	14 - 18                          RvaToLastByteOfCode;
	18 - 1C                          RvaToFirstByteOfData;
	1C - 20                          RvaToLastByteOfData;
} IMAGE_COFF_SYMBOLS_HEADER;
typedef struct {
	00 - 04                          NumberOfSymbols;
	04 - 08                          LvaToFirstSymbol;
	08 - 0C                          NumberOfLinenumbers;
	0C - 10                          LvaToFirstLinenumber;
	10 - 14                          RvaToFirstByteOfCode;
	14 - 18                          RvaToLastByteOfCode;
	18 - 1C                          RvaToFirstByteOfData;
	1C - 20                          RvaToLastByteOfData;
}* PIMAGE_COFF_SYMBOLS_HEADER;
struct _IMAGE_COFF_SYMBOLS_HEADER {
	00 - 04                          NumberOfSymbols;
	04 - 08                          LvaToFirstSymbol;
	08 - 0C                          NumberOfLinenumbers;
	0C - 10                          LvaToFirstLinenumber;
	10 - 14                          RvaToFirstByteOfCode;
	14 - 18                          RvaToLastByteOfCode;
	18 - 1C                          RvaToFirstByteOfData;
	1C - 20                          RvaToLastByteOfData;
};

[https://msdn.microsoft.com/en-us/library/windows/desktop/ms680305]
1 code block(s):
typedef struct {
	00 - 04                          VirtualAddress;
	04 - 08                          Size;
} IMAGE_DATA_DIRECTORY;
typedef struct {
	00 - 04                          VirtualAddress;
	04 - 08                          Size;
}* PIMAGE_DATA_DIRECTORY;
struct _IMAGE_DATA_DIRECTORY {
	00 - 04                          VirtualAddress;
	04 - 08                          Size;
};

[https://msdn.microsoft.com/en-us/library/windows/desktop/ms680307]
1 code block(s):
struct _IMAGE_DEBUG_DIRECTORY {
	00 - 04                          Characteristics;
	04 - 08                          TimeDateStamp;
	08 - 0A                          MajorVersion;
	0A - 0C                          MinorVersion;
	0C - 10                          Type;
	10 - 14                          SizeOfData;
	14 - 18                          AddressOfRawData;
	18 - 1C                          PointerToRawData;
};
typedef struct {
	00 - 04                          Characteristics;
	04 - 08                          TimeDateStamp;
	08 - 0A                          MajorVersion;
	0A - 0C                          MinorVersion;
	0C - 10                          Type;
	10 - 14                          SizeOfData;
	14 - 18                          AddressOfRawData;
	18 - 1C                          PointerToRawData;
} IMAGE_DEBUG_DIRECTORY;
typedef struct {
	00 - 04                          Characteristics;
	04 - 08                          TimeDateStamp;
	08 - 0A                          MajorVersion;
	0A - 0C                          MinorVersion;
	0C - 10                          Type;
	10 - 14                          SizeOfData;
	14 - 18                          AddressOfRawData;
	18 - 1C                          PointerToRawData;
}* PIMAGE_DEBUG_DIRECTORY;

[https://msdn.microsoft.com/en-us/library/windows/desktop/ms680313]
1 code block(s):
struct _IMAGE_FILE_HEADER {
	00 - 02                          Machine;
	02 - 04                          NumberOfSections;
	04 - 08                          TimeDateStamp;
	08 - 0C                          PointerToSymbolTable;
	0C - 10                          NumberOfSymbols;
	10 - 12                          SizeOfOptionalHeader;
	12 - 14                          Characteristics;
};
typedef struct {
	00 - 02                          Machine;
	02 - 04                          NumberOfSections;
	04 - 08                          TimeDateStamp;
	08 - 0C                          PointerToSymbolTable;
	0C - 10                          NumberOfSymbols;
	10 - 12                          SizeOfOptionalHeader;
	12 - 14                          Characteristics;
} IMAGE_FILE_HEADER;
typedef struct {
	00 - 02                          Machine;
	02 - 04                          NumberOfSections;
	04 - 08                          TimeDateStamp;
	08 - 0C                          PointerToSymbolTable;
	0C - 10                          NumberOfSymbols;
	10 - 12                          SizeOfOptionalHeader;
	12 - 14                          Characteristics;
}* PIMAGE_FILE_HEADER;

[https://msdn.microsoft.com/en-us/library/windows/desktop/ms680316]
2 code block(s):
typedef struct {
	00 - 04                          StartingAddress;
	04 - 08                          EndingAddress;
	08 - 0C                          EndOfPrologue;
}* PIMAGE_FUNCTION_ENTRY;
struct _IMAGE_FUNCTION_ENTRY {
	00 - 04                          StartingAddress;
	04 - 08                          EndingAddress;
	08 - 0C                          EndOfPrologue;
};
typedef struct {
	00 - 04                          StartingAddress;
	04 - 08                          EndingAddress;
	08 - 0C                          EndOfPrologue;
} IMAGE_FUNCTION_ENTRY;

error: unexpected token SemiColon
[https://msdn.microsoft.com/en-us/library/windows/desktop/ms680328]
2 code block(s):
typedef struct {
	00 - 04                          Size;
	04 - 08                          TimeDateStamp;
	08 - 0A                          MajorVersion;
	0A - 0C                          MinorVersion;
	0C - 10                          GlobalFlagsClear;
	10 - 14                          GlobalFlagsSet;
	14 - 18                          CriticalSectionDefaultTimeout;
	18 (offset only, size unknown)   DeCommitFreeBlockThreshold;
	18 (offset only, size unknown)   DeCommitTotalFreeThreshold;
	18 (offset only, size unknown)   LockPrefixTable;
	18 (offset only, size unknown)   MaximumAllocationSize;
	18 (offset only, size unknown)   ProcessAffinityMask;
	18 (offset only, size unknown)   VirtualMemoryThreshold;
	18 - 1C                          ProcessHeapFlags;
	1C - 1E                          CSDVersion;
	1E - 20                          Reserved1;
	20 (offset only, size unknown)   EditList;
	20 (offset only, size unknown)   SEHandlerCount;
	20 (offset only, size unknown)   SEHandlerTable;
	20 (offset only, size unknown)   SecurityCookie;
} IMAGE_LOAD_CONFIG_DIRECTORY64;
typedef struct {
	00 - 04                          Size;
	04 - 08                          TimeDateStamp;
	08 - 0A                          MajorVersion;
	0A - 0C                          MinorVersion;
	0C - 10                          GlobalFlagsClear;
	10 - 14                          GlobalFlagsSet;
	14 - 18                          CriticalSectionDefaultTimeout;
	18 (offset only, size unknown)   DeCommitFreeBlockThreshold;
	18 (offset only, size unknown)   DeCommitTotalFreeThreshold;
	18 (offset only, size unknown)   LockPrefixTable;
	18 (offset only, size unknown)   MaximumAllocationSize;
	18 (offset only, size unknown)   ProcessAffinityMask;
	18 (offset only, size unknown)   VirtualMemoryThreshold;
	18 - 1C                          ProcessHeapFlags;
	1C - 1E                          CSDVersion;
	1E - 20                          Reserved1;
	20 (offset only, size unknown)   EditList;
	20 (offset only, size unknown)   SEHandlerCount;
	20 (offset only, size unknown)   SEHandlerTable;
	20 (offset only, size unknown)   SecurityCookie;
}* PIMAGE_LOAD_CONFIG_DIRECTORY64;

typedef struct {
	00 - 04                          Size;
	04 - 08                          TimeDateStamp;
	08 - 0A                          MajorVersion;
	0A - 0C                          MinorVersion;
	0C - 10                          GlobalFlagsClear;
	10 - 14                          GlobalFlagsSet;
	14 - 18                          CriticalSectionDefaultTimeout;
	18 - 1C                          DeCommitFreeBlockThreshold;
	1C - 20                          DeCommitTotalFreeThreshold;
	20 - 24                          LockPrefixTable;
	24 - 28                          MaximumAllocationSize;
	28 - 2C                          VirtualMemoryThreshold;
	2C - 30                          ProcessHeapFlags;
	30 - 34                          ProcessAffinityMask;
	34 - 36                          CSDVersion;
	36 - 38                          Reserved1;
	38 - 3C                          EditList;
	3C - 40                          SecurityCookie;
	40 - 44                          SEHandlerTable;
	44 - 48                          SEHandlerCount;
}* PIMAGE_LOAD_CONFIG_DIRECTORY32;
typedef struct {
	00 - 04                          Size;
	04 - 08                          TimeDateStamp;
	08 - 0A                          MajorVersion;
	0A - 0C                          MinorVersion;
	0C - 10                          GlobalFlagsClear;
	10 - 14                          GlobalFlagsSet;
	14 - 18                          CriticalSectionDefaultTimeout;
	18 - 1C                          DeCommitFreeBlockThreshold;
	1C - 20                          DeCommitTotalFreeThreshold;
	20 - 24                          LockPrefixTable;
	24 - 28                          MaximumAllocationSize;
	28 - 2C                          VirtualMemoryThreshold;
	2C - 30                          ProcessHeapFlags;
	30 - 34                          ProcessAffinityMask;
	34 - 36                          CSDVersion;
	36 - 38                          Reserved1;
	38 - 3C                          EditList;
	3C - 40                          SecurityCookie;
	40 - 44                          SEHandlerTable;
	44 - 48                          SEHandlerCount;
} IMAGE_LOAD_CONFIG_DIRECTORY32;

[https://msdn.microsoft.com/en-us/library/windows/desktop/ms680336]
2 code block(s):
struct _IMAGE_NT_HEADERS {
	00 - 04                          Signature;
	04 (offset only, size unknown)   FileHeader;
	04 (offset only, size unknown)   OptionalHeader;
};
typedef struct {
	00 - 04                          Signature;
	04 (offset only, size unknown)   FileHeader;
	04 (offset only, size unknown)   OptionalHeader;
}* PIMAGE_NT_HEADERS;
typedef struct {
	00 - 04                          Signature;
	04 (offset only, size unknown)   FileHeader;
	04 (offset only, size unknown)   OptionalHeader;
} IMAGE_NT_HEADERS;

struct _IMAGE_NT_HEADERS64 {
	00 - 04                          Signature;
	04 (offset only, size unknown)   FileHeader;
	04 (offset only, size unknown)   OptionalHeader;
};
typedef struct {
	00 - 04                          Signature;
	04 (offset only, size unknown)   FileHeader;
	04 (offset only, size unknown)   OptionalHeader;
} IMAGE_NT_HEADERS64;
typedef struct {
	00 - 04                          Signature;
	04 (offset only, size unknown)   FileHeader;
	04 (offset only, size unknown)   OptionalHeader;
}* PIMAGE_NT_HEADERS64;

[https://msdn.microsoft.com/en-us/library/windows/desktop/ms680339]
2 code block(s):
error: unexpected character '['
error: unexpected character '['
[https://msdn.microsoft.com/en-us/library/windows/desktop/ms680341]
1 code block(s):
error: unexpected character '['
```
