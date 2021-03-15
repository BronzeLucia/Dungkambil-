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
	lda #$10
	sta $2001
	
	; Our instructions up to this point MUST total
	; 6900 cycles, so we'll burn the rest in a loop.
	
	; delay 6900 - 30
	lda #9
	sec
nmi_1:  pha
	lda #150
nmi_2:  sbc #1
	bne nmi_2
	pla
	sbc #1
	bne nmi_1
	
	jsr end_nmi_sync
	
	; We're now synchronized exactly to 7471 cycles
	; after beginning of frame.
	
	; delay 20486 - 7471 - 5
	nop
	lda #85
	sec
nmi_3:  pha
	lda #28
nmi_4:  sbc #1
	