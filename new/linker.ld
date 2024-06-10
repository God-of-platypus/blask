MEMORY
{
    ROM (rx)  : ORIGIN = 0x00000000, LENGTH = 256K
    RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 256K
}
ENTRY(_start)

/* Sections /
SECTIONS
{
    .text :
    {
        _text_start = .;
        KEEP((.init))

        *(.text)


        _text_end = .;
    } > ROM


    .rodata :
    {
        _rodata_start = .;
        *(.rodata)
        _rodata_end = .;
    } > ROM


    .data : AT (ADDR(.text) + SIZEOF(.text) + SIZEOF(.rodata))
    {
        _data_start = .;
        *(.data)
        _data_end = .;
    } > RAM


    .bss :
    {
        _bss_start = .;
        *(.bss)
        (COMMON)
        _bss_end = .;
    } > RAM


    .heap (NOLOAD) :
    {
        _heap_start = .;
        _heap_end = ORIGIN(RAM) + LENGTH(RAM);
    } > RAM


    .stack (NOLOAD) :
    {
        _stack_start = _heap_end;
        _stack_end = _stack_start + 0x1000;
    } > RAM
    .debug_info     0 (NOLOAD) : {(.debug_info) }
    .debug_abbrev   0 (NOLOAD) : { (.debug_abbrev) }
    .debug_loc      0 (NOLOAD) : {(.debug_loc) }
    .debug_aranges  0 (NOLOAD) : { (.debug_aranges) }
    .debug_line     0 (NOLOAD) : {(.debug_line) }
    .debug_str      0 (NOLOAD) : { (.debug_str) }
    .debug_ranges   0 (NOLOAD) : {(.debug_ranges) }


    PROVIDE(_end = .);


    _end_of_memory = ORIGIN(RAM) + LENGTH(RAM);
}

PROVIDE(_etext = ADDR(.text) + SIZEOF(.text));
PROVIDE(_edata = ADDR(.data) + SIZEOF(.data));
PROVIDE(_end = ADDR(.bss) + SIZEOF(.bss));