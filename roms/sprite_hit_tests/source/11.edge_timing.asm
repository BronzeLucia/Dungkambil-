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
      sta   sprite_