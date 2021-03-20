; Tests sprite 0 hit using a sprite with a single pixel set,
; for each of the four corners.

      .include "prefix_sprite_hit.a"

test_name:
      .db   "SPRITE HIT CORNERS",0
      .code

set_params:
      sta   sprite_tile
      eor   #$03
      pha
      jsr   set_sprite_xy
      lda   #$21
      ldx   #$f0
      jsr   set_vaddr
      pla
      sta   $2007
      rts
      .code

reset:
    