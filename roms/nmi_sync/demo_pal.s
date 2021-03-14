; Uses nmi_sync to manually display line on screen
; using timed write. See readme.txt.
;
; PAL NES only. Tested on hardware.
;
; ca65 -o rom.o demo_pal.s
; ld65 -C unrom.cfg rom.o -o demo_pal.nes
;
; Shay Green <gblargg@gmail.com>

.include "nmi_sync.s"

reset:
	; Initialize PPU and palette
	jsr init_graphics
	
	; Synchronize with PPU and enable NMI
	jsr init_nmi_sync_pal
	
	; Loop endlessly
loop:   jsr wait_nmi
	
	; You could run normal code between NMIs here,
	; as long as it completes BEFORE NMI. If it
	; takes too long, synchronization may be off
	; by a few cycles for that frame.
	; ...
	
	jmp loop
