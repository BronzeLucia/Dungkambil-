
; Allows precise PPU synchronization in NMI handler, without
; having to cycle-time code outside NMI handler.

.zeropage
nmi_sync_count: .res 1

.code
.align 256 ; branches must not cross page

; Initializes synchronization and enables NMI
; Preserved: X, Y
; Time: 15 frames average, 28 frames max
init_nmi_sync:
	; Disable interrupts and rendering
	sei
	lda #0
	sta $2000
	sta $2001
	
	; Coarse synchronize
	bit $2002
init_nmi_sync_1:
	bit $2002
	bpl init_nmi_sync_1
	
	; Synchronize to odd CPU cycle
	sta $4014

	; Fine synchronize