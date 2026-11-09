LDVN 0 2            ; 0x200  x position
LDVN 1 2            ; 0x202  y position
LDVN 2 30           ; 0x204  delay in ticks (60 Hz so about 0.5s)
LDVN 4 8            ; 0x206  font digit to draw
LDIF 4              ; 0x208  I = font sprite for V4
DRW 0 1 5           ; 0x20A  toggle the sprite (loop start)
LDDV 2              ; 0x20C  delay timer = V2
LDDT 3              ; 0x20E  V3 = delay timer (wait loop)
SE 3 0              ; 0x210  skip next if the timer reached 0
JP 0x20E            ; 0x212  still counting keep waiting
JP 0x20A            ; 0x214  toggle again