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
      lda   #0
      sta   $2000
      sta   $2001
      
      lda   #2;) VRAM reads should be delayed in a buffer
      sta   result
      lda   #$00
      jsr   set_vram_pos
      lda   #$12
      sta   $2007
      lda   #$34
      sta   $2007
      lda   #$00
      jsr   set_vram_pos
      lda   $2007
      lda   $2007
      cmp   #$34
      jsr   error_if_eq
      
      lda   #3;) Basic Write/read doesn't work
      sta   result
      lda   #$00
      jsr   set_vram_pos
      lda   #$56
      sta   $2007
      lda   #$00
      jsr   set_vram_pos
      lda   $2007
      lda   $2007
      cmp   #$56
      jsr   error_if_ne
      
      lda   #4;) Read buffer shouldn't be affected by VRAM write
      sta   result
      lda   #$00
      jsr   set_vram_pos
      lda   #$78
      sta   $2007
      lda   #$00
      jsr   set_vram_pos
      lda   #$00
      lda   $2007       ; buffer now contains $78
      lda   #$12
      sta   $2007       ; shouldn't affect buffer
      lda   $2007
      cmp   #$78
      jsr   error_if_ne
      
      lda   #5;) Read buffer shouldn't be affected by palette write
      sta   result
      lda   #$00
      jsr   set_vram_pos
      lda   #$9a
      sta   $2007
      lda   #$00
      jsr   set_vram_pos
      lda   $2007       ; buffer now contains $9a
      lda   #$3f
      sta   $2006
      lda   #$00
      sta   $2006
      lda   #$34
      sta   $2007       ; shouldn't affect buffer
      lda   #$01        ; change back to non-palette addr to enable buffer
      jsr   set_vram_pos
      lda   $2007
      cmp   #$9a
      jsr   error_if