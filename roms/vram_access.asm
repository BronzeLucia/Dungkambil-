; Tests PPU VRAM read/write and internal read buffer operation

      .include "prefix_ppu.a"

; Set VRAM addr to $2f00 + A
; Preserved: A, X, Y
set_vram_pos:
      pha
      lda   #$2f
      sta   $2006
      pla
      sta   $2006
      rts

reset:
      lda   #50
      jsr   delay_msec
      
      jsr   wait_vbl