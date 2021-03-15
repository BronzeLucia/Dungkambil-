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


.align 256 ; branches must not cross page
nmi:
	pha
	
	; Do this sometime before you DMA sprites
	jsr begin_nmi_sync
	
	; DMA then enable sprites. Instructions before
	; STA $4014 (excluding begin_nmi_sync) must take
	; an even number of cycles. The only required
	; instruction here is STA $4014.
	bit <0          ; to make cycle count even
	lda #0
	sta $2003
	lda #>sprites
	sta $4014
	ld