
; Tests sprite RAM access via $2003, $2004, and $4014

      .include "prefix_ppu.a"

sprites = $200

reset:
      lda   #50
      jsr   delay_msec
      
      jsr   wait_vbl
      lda   #0
      sta   $2000
      sta   $2001
      
      lda   #2;) Basic read/write doesn't work
      sta   result
      lda   #0
      sta   $2003
      lda   #$12
      sta   $2004
      lda   #0
      sta   $2003
      lda   $2004
      cmp   #$12
      jsr   error_if_ne
      
      lda   #3;) Address should increment on $2004 write
      sta   result
      lda   #0
      sta   $2003
      lda   #$12
      sta   $2004
      lda   #$34
      sta   $2004
      lda   #1
      sta   $2003
      lda   $2004
      cmp   #$34
      jsr   error_if_ne
      
      lda   #4;) Address should not increment on $2004 read
      sta   result
      lda   #0
      sta   $2003
      lda   #$12
      sta   $2004
      lda   #$34
      sta   $2004
      lda   #0
      sta   $2003
      lda   $2004
      lda   $2004
      cmp   #$34
      jsr   error_if_eq