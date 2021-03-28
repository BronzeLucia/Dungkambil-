; Tests sprite 0 hit timing for which pixel it first reports hit on
; when some pixels are under clip, or at or beyond right edge.

      .include "prefix_sprite_hit.a"

test_name:
      .db   "SPRITE HIT EDGE TIMING",0
      .code

reset:
      jsr   begin_sprite_hit_tests
      
      lda   #solid_tile
      jsr   fill_nametable

      lda   #120
      sta   sprite_y
      lda   #two_corners_tile
      sta   sprite_tile
      lda   #0
      sta   sprite_attr
      
      lda   #7
      sta   sprite_x
      lda   #2;) Hit time shouldn't be based on pixels under left 