; Tests basic sprite 0 hit double-height operation.

      .include "prefix_sprite_hit.a"

test_name:
      .db   "SPRITE HIT DOUBLE HEIGHT",0
      .code

reset:
      jsr   begin_s