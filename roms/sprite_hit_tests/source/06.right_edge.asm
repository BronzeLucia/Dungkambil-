; Tests sprite 0 hit with regard to column 255 (ignored) and off
; right edge of screen.

      .include "prefix_sprite_hit.a"

test_name:
      .db   "SPRITE HIT RIGHT EDGE",0
      .code

reset:
      jsr   begin_sprite_hit_tests
      lda   #solid_tile
      jsr   fill_nametable
      lda   #0
      sta   sprite_attr
      lda   #120
      sta   sprite_y
      
      ; Basic
      
      lda   #solid_tile
      sta   sprite_tile
      lda   #255
      sta   sprite_x
      lda   #2;) Should always miss when X = 255
      ldx   #$1e
      jsr   sprite_should_miss