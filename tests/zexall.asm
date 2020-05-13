; Disassembly of the file "D:\dev\projects\pacman-rs\tests\zexall.com"
; 
; CPU Type: Z80
; 
; Created with dZ80 2.0
; 
; on Wednesday, 06 of May 2020 at 02:43 PM
; 
0040 e20242    jp      po,4202h
0043 03        inc     bc
0044 a2        and     d
0045 03        inc     bc
0046 02        ld      (bc),a
0047 04        inc     b
0048 62        ld      h,d
0049 04        inc     b
004a c20422    jp      nz,2204h
004d 05        dec     b
004e 82        add     a,d
004f 05        dec     b
0050 e20542    jp      po,4205h
0053 06a2      ld      b,0a2h
0055 0602      ld      b,02h
0057 07        rlca    
0058 62        ld      h,d
0059 07        rlca    
005a c20722    jp      nz,2207h
005d 08        ex      af,af'
005e 82        add     a,d
005f 08        ex      af,af'
0060 e20842    jp      po,4208h
0063 09        add     hl,bc
0064 a2        and     d
0065 09        add     hl,bc
0066 02        ld      (bc),a
0067 0a        ld      a,(bc)
0068 62        ld      h,d
0069 0a        ld      a,(bc)
006a c20a22    jp      nz,220ah
006d 0b        dec     bc
006e 82        add     a,d
006f 0b        dec     bc
0070 e20b42    jp      po,420bh
0073 0c        inc     c
0074 a2        and     d
0075 0c        inc     c
0076 02        ld      (bc),a
0077 0d        dec     c
0078 62        ld      h,d
0079 0d        dec     c
007a c20d22    jp      nz,220dh
007d 0e82      ld      c,82h
007f 0ee2      ld      c,0e2h
0081 0e42      ld      c,42h
0083 0f        rrca    
0084 a2        and     d
0085 0f        rrca    
0086 02        ld      (bc),a
0087 1062      djnz    00ebh
0089 10c2      djnz    004dh
008b 1022      djnz    00afh
008d 118211    ld      de,1182h
0090 e21142    jp      po,4211h
0093 12        ld      (de),a
0094 a2        and     d
0095 12        ld      (de),a
0096 02        ld      (bc),a
0097 13        inc     de
0098 62        ld      h,d
0099 13        inc     de
009a c21322    jp      nz,2213h
009d 14        inc     d
009e 82        add     a,d
009f 14        inc     d
00a0 e21442    jp      po,4214h
00a3 15        dec     d
00a4 a2        and     d
00a5 15        dec     d
00a6 02        ld      (bc),a
00a7 1662      ld      d,62h
00a9 16c2      ld      d,0c2h
00ab 1622      ld      d,22h
00ad 17        rla     
00ae 82        add     a,d
00af 17        rla     
00b0 e21742    jp      po,4217h
00b3 18a2      jr      0057h
00b5 1802      jr      00b9h
00b7 19        add     hl,de
00b8 62        ld      h,d
00b9 19        add     hl,de
00ba c21922    jp      nz,2219h
00bd 1a        ld      a,(de)
00be 82        add     a,d
00bf 1a        ld      a,(de)
00c0 00        nop     
00c1 00        nop     
00c2 ff        rst     38h
00c3 ed42      sbc     hl,bc
00c5 00        nop     
00c6 00        nop     
00c7 2c        inc     l
00c8 83        add     a,e
00c9 88        adc     a,b
00ca 4f        ld      c,a
00cb 2b        dec     hl
00cc f239b3    jp      p,0b339h
00cf 1f        rra     
00d0 7e        ld      a,(hl)
00d1 63        ld      h,e
00d2 15        dec     d
00d3 d389      out     (89h),a
00d5 5e        ld      e,(hl)
00d6 46        ld      b,(hl)
00d7 00        nop     
00d8 3800      jr      c,00dah
00da 00        nop     
00db 00        nop     
00dc 00        nop     
00dd 00        nop     
00de 00        nop     
00df 00        nop     
00e0 00        nop     
00e1 21f800    ld      hl,00f8h
00e4 00        nop     
00e5 00        nop     
00e6 00        nop     
00e7 00        nop     
00e8 00        nop     
00e9 00        nop     
00ea 00        nop     
00eb 00        nop     
00ec 00        nop     
00ed 00        nop     
00ee 00        nop     
00ef 00        nop     
00f0 00        nop     
00f1 00        nop     
00f2 00        nop     
00f3 00        nop     
00f4 00        nop     
00f5 ff        rst     38h
00f6 ff        rst     38h
00f7 ff        rst     38h
00f8 ff        rst     38h
00f9 ff        rst     38h
00fa ff        rst     38h
00fb d7        rst     10h
00fc 00        nop     
00fd ff        rst     38h
00fe ff        rst     38h
00ff d48ad5    call    nc,0d58ah
0102 19        add     hl,de
0103 3c        inc     a
0104 61        ld      h,c
0105 64        ld      h,h
0106 63        ld      h,e
0107 2c        inc     l
0108 73        ld      (hl),e
0109 62        ld      h,d
010a 63        ld      h,e
010b 3e20      ld      a,20h
010d 68        ld      l,b
010e 6c        ld      l,h
010f 2c        inc     l
0110 3c        inc     a
0111 62        ld      h,d
0112 63        ld      h,e
0113 2c        inc     l
0114 64        ld      h,h
0115 65        ld      h,l
0116 2c        inc     l
0117 68        ld      l,b
0118 6c        ld      l,h
0119 2c        inc     l
011a 73        ld      (hl),e
011b 70        ld      (hl),b
011c 3e2e      ld      a,2eh
011e 2e2e      ld      l,2eh
0120 2e24      ld      l,24h
0122 ff        rst     38h
0123 09        add     hl,bc
0124 00        nop     
0125 00        nop     
0126 00        nop     
0127 a5        and     l
0128 c4c7c4    call    nz,0c4c7h
012b 26d2      ld      h,0d2h
012d 50        ld      d,b
012e a0        and     b
012f ea5866    jp      pe,6658h
0132 85        add     a,l
0133 c6de      add     a,0deh
0135 c9        ret     

0136 9b        sbc     a,e
0137 3000      jr      nc,0139h
0139 00        nop     
013a 00        nop     
013b 00        nop     
013c 00        nop     
013d 00        nop     
013e 00        nop     
013f 00        nop     
0140 00        nop     
0141 21f800    ld      hl,00f8h
0144 00        nop     
0145 00        nop     
0146 00        nop     
0147 00        nop     
0148 00        nop     
0149 00        nop     
014a 00        nop     
014b 00        nop     
014c 00        nop     
014d 00        nop     
014e 00        nop     
014f 00        nop     
0150 00        nop     
0151 00        nop     
0152 00        nop     
0153 00        nop     
0154 00        nop     
0155 ff        rst     38h
0156 ff        rst     38h
0157 ff        rst     38h
0158 ff        rst     38h
0159 ff        rst     38h
015a ff        rst     38h
015b d7        rst     10h
015c 00        nop     
015d ff        rst     38h
015e ff        rst     38h
015f d9        exx     
0160 a4        and     h
0161 ca0561    jp      z,6105h
0164 64        ld      h,h
0165 64        ld      h,h
0166 2068      jr      nz,01d0h
0168 6c        ld      l,h
0169 2c        inc     l
016a 3c        inc     a
016b 62        ld      h,d
016c 63        ld      h,e
016d 2c        inc     l
016e 64        ld      h,h
016f 65        ld      h,l
0170 2c        inc     l
0171 68        ld      l,b
0172 6c        ld      l,h
0173 2c        inc     l
0174 73        ld      (hl),e
0175 70        ld      (hl),b
0176 3e2e      ld      a,2eh
0178 2e2e      ld      l,2eh
017a 2e2e      ld      l,2eh
017c 2e2e      ld      l,2eh
017e 2e2e      ld      l,2eh
0180 2e24      ld      l,24h
0182 ff        rst     38h
0183 dd09      add     ix,bc
0185 00        nop     
0186 00        nop     
0187 ac        xor     h
0188 dd94      sub     ixh
018a c25b63    jp      nz,635bh
018d d333      out     (33h),a
018f 76        halt    
0190 6a        ld      l,d
0191 20fa      jr      nz,018dh
0193 94        sub     h
0194 68        ld      l,b
0195 f5        push    af
0196 3600      ld      (hl),00h
0198 3000      jr      nc,019ah
019a 00        nop     
019b 00        nop     
019c 00        nop     
019d 00        nop     
019e 00        nop     
019f 21f800    ld      hl,00f8h
01a2 00        nop     
01a3 00        nop     
01a4 00        nop     
01a5 00        nop     
01a6 00        nop     
01a7 00        nop     
01a8 00        nop     
01a9 00        nop     
01aa 00        nop     
01ab 00        nop     
01ac 00        nop     
01ad 00        nop     
01ae 00        nop     
01af 00        nop     
01b0 00        nop     
01b1 00        nop     
01b2 00        nop     
01b3 ff        rst     38h
01b4 ff        rst     38h
01b5 00        nop     
01b6 00        nop     
01b7 ff        rst     38h
01b8 ff        rst     38h
01b9 ff        rst     38h
01ba ff        rst     38h
01bb d7        rst     10h
01bc 00        nop     
01bd ff        rst     38h
01be ff        rst     38h
01bf b1        or      c
01c0 df        rst     18h
01c1 8e        adc     a,(hl)
01c2 c0        ret     nz

01c3 61        ld      h,c
01c4 64        ld      h,h
01c5 64        ld      h,h
01c6 2069      jr      nz,0231h
01c8 78        ld      a,b
01c9 2c        inc     l
01ca 3c        inc     a
01cb 62        ld      h,d
01cc 63        ld      h,e
01cd 2c        inc     l
01ce 64        ld      h,h
01cf 65        ld      h,l
01d0 2c        inc     l
01d1 69        ld      l,c
01d2 78        ld      a,b
01d3 2c        inc     l
01d4 73        ld      (hl),e
01d5 70        ld      (hl),b
01d6 3e2e      ld      a,2eh
01d8 2e2e      ld      l,2eh
01da 2e2e      ld      l,2eh
01dc 2e2e      ld      l,2eh
01de 2e2e      ld      l,2eh
01e0 2e24      ld      l,24h
01e2 ff        rst     38h
01e3 fd09      add     iy,bc
01e5 00        nop     
01e6 00        nop     
01e7 c2c707    jp      nz,07c7h
01ea f4c151    call    p,51c1h
01ed 96        sub     (hl)
01ee 3ef4      ld      a,0f4h
01f0 0b        dec     bc
01f1 0f        rrca    
01f2 51        ld      d,c
01f3 92        sub     d
01f4 1eea      ld      e,0eah
01f6 71        ld      (hl),c
01f7 00        nop     
01f8 3000      jr      nc,01fah
01fa 00        nop     
01fb 00        nop     
01fc 00        nop     
01fd 21f800    ld      hl,00f8h
0200 00        nop     
0201 00        nop     
0202 00        nop     
0203 00        nop     
0204 00        nop     
0205 00        nop     
0206 00        nop     
0207 00        nop     
0208 00        nop     
0209 00        nop     
020a 00        nop     
020b 00        nop     
020c 00        nop     
020d 00        nop     
020e 00        nop     
020f 00        nop     
0210 00        nop     
0211 ff        rst     38h
0212 ff        rst     38h
0213 00        nop     
0214 00        nop     
0215 00        nop     
0216 00        nop     
0217 ff        rst     38h
0218 ff        rst     38h
0219 ff        rst     38h
021a ff        rst     38h
021b d7        rst     10h
021c 00        nop     
021d ff        rst     38h
021e ff        rst     38h
021f 39        add     hl,sp
0220 c8        ret     z

0221 58        ld      e,b
0222 9b        sbc     a,e
0223 61        ld      h,c
0224 64        ld      h,h
0225 64        ld      h,h
0226 2069      jr      nz,0291h
0228 79        ld      a,c
0229 2c        inc     l
022a 3c        inc     a
022b 62        ld      h,d
022c 63        ld      h,e
022d 2c        inc     l
022e 64        ld      h,h
022f 65        ld      h,l
0230 2c        inc     l
0231 69        ld      l,c
0232 79        ld      a,c
0233 2c        inc     l
0234 73        ld      (hl),e
0235 70        ld      (hl),b
0236 3e2e      ld      a,2eh
0238 2e2e      ld      l,2eh
023a 2e2e      ld      l,2eh
023c 2e2e      ld      l,2eh
023e 2e2e      ld      l,2eh
0240 2e24      ld      l,24h
0242 ff        rst     38h
0243 c600      add     a,00h
0245 00        nop     
0246 00        nop     
0247 40        ld      b,b
0248 91        sub     c
0249 3c        inc     a
024a 7e        ld      a,(hl)
024b 67        ld      h,a
024c 7a        ld      a,d
024d 6d        ld      l,l
024e df        rst     18h
024f 61        ld      h,c
0250 5b        ld      e,e
0251 29        add     hl,hl
0252 0b        dec     bc
0253 1066      djnz    02bbh
0255 b2        or      d
0256 85        add     a,l
0257 3800      jr      c,0259h
0259 00        nop     
025a 00        nop     
025b 00        nop     
025c 00        nop     
025d 00        nop     
025e 00        nop     
025f 00        nop     
0260 00        nop     
0261 00        nop     
0262 00        nop     
0263 00        nop     
0264 00        nop     
0265 00        nop     
0266 00        nop     
0267 00        nop     
0268 ff        rst     38h
0269 00        nop     
026a 00        nop     
026b 00        nop     
026c ff        rst     38h
026d 00        nop     
026e 00        nop     
026f 00        nop     
0270 00        nop     
0271 00        nop     
0272 00        nop     
0273 00        nop     
0274 00        nop     
0275 00        nop     
0276 00        nop     
0277 00        nop     
0278 00        nop     
0279 00        nop     
027a 00        nop     
027b d7        rst     10h
027c 00        nop     
027d 00        nop     
027e 00        nop     
027f 51        ld      d,c
0280 c1        pop     bc
0281 9c        sbc     a,h
0282 2e61      ld      l,61h
0284 6c        ld      l,h
0285 75        ld      (hl),l
0286 6f        ld      l,a
0287 70        ld      (hl),b
0288 2061      jr      nz,02ebh
028a 2c        inc     l
028b 6e        ld      l,(hl)
028c 6e        ld      l,(hl)
028d 2e2e      ld      l,2eh
028f 2e2e      ld      l,2eh
0291 2e2e      ld      l,2eh
0293 2e2e      ld      l,2eh
0295 2e2e      ld      l,2eh
0297 2e2e      ld      l,2eh
0299 2e2e      ld      l,2eh
029b 2e2e      ld      l,2eh
029d 2e2e      ld      l,2eh
029f 2e2e      ld      l,2eh
02a1 24        inc     h
02a2 ff        rst     38h
02a3 80        add     a,b
02a4 00        nop     
02a5 00        nop     
02a6 00        nop     
02a7 3ec5      ld      a,0c5h
02a9 3a574d    ld      a,(4d57h)
02ac 4c        ld      c,h
02ad 03        inc     bc
02ae 0109e3    ld      bc,0e309h
02b1 66        ld      h,(hl)
02b2 a6        and     (hl)
02b3 d0        ret     nc

02b4 3b        dec     sp
02b5 bb        cp      e
02b6 ad        xor     l
02b7 3f        ccf     
02b8 00        nop     
02b9 00        nop     
02ba 00        nop     
02bb 00        nop     
02bc 00        nop     
02bd 00        nop     
02be 00        nop     
02bf 00        nop     
02c0 00        nop     
02c1 00        nop     
02c2 00        nop     
02c3 00        nop     
02c4 00        nop     
02c5 00        nop     
02c6 00        nop     
02c7 00        nop     
02c8 ff        rst     38h
02c9 00        nop     
02ca 00        nop     
02cb 00        nop     
02cc 00        nop     
02cd 00        nop     
02ce 00        nop     
02cf ff        rst     38h
02d0 00        nop     
02d1 00        nop     
02d2 00        nop     
02d3 00        nop     
02d4 00        nop     
02d5 00        nop     
02d6 00        nop     
02d7 ff        rst     38h
02d8 ff        rst     38h
02d9 ff        rst     38h
02da ff        rst     38h
02db d7        rst     10h
02dc 00        nop     
02dd 00        nop     
02de 00        nop     
02df 06c7      ld      b,0c7h
02e1 aa        xor     d
02e2 8e        adc     a,(hl)
02e3 61        ld      h,c
02e4 6c        ld      l,h
02e5 75        ld      (hl),l
02e6 6f        ld      l,a
02e7 70        ld      (hl),b
02e8 2061      jr      nz,034bh
02ea 2c        inc     l
02eb 3c        inc     a
02ec 62        ld      h,d
02ed 2c        inc     l
02ee 63        ld      h,e
02ef 2c        inc     l
02f0 64        ld      h,h
02f1 2c        inc     l
02f2 65        ld      h,l
02f3 2c        inc     l
02f4 68        ld      l,b
02f5 2c        inc     l
02f6 6c        ld      l,h
02f7 2c        inc     l
02f8 2868      jr      z,0362h
02fa 6c        ld      l,h
02fb 29        add     hl,hl
02fc 2c        inc     l
02fd 61        ld      h,c
02fe 3e2e      ld      a,2eh
0300 2e24      ld      l,24h
0302 ff        rst     38h
0303 dd84      add     a,ixh
0305 00        nop     
0306 00        nop     
0307 f7        rst     30h
0308 d66e      sub     6eh
030a c7        rst     00h
030b cf        rst     08h
030c ac        xor     h
030d 47        ld      b,a
030e 28dd      jr      z,02edh
0310 2235c0    ld      (0c035h),hl
0313 c5        push    bc
0314 384b      jr      c,0361h
0316 23        inc     hl
0317 2039      jr      nz,0352h
0319 00        nop     
031a 00        nop     
031b 00        nop     
031c 00        nop     
031d 00        nop     
031e 00        nop     
031f 00        nop     
0320 00        nop     
0321 00        nop     
0322 00        nop     
0323 00        nop     
0324 00        nop     
0325 00        nop     
0326 00        nop     
0327 00        nop     
0328 ff        rst     38h
0329 00        nop     
032a 00        nop     
032b 00        nop     
032c 00        nop     
032d 00        nop     
032e 00        nop     
032f ff        rst     38h
0330 00        nop     
0331 00        nop     
0332 00        nop     
0333 00        nop     
0334 00        nop     
0335 00        nop     
0336 00        nop     
0337 ff        rst     38h
0338 ff        rst     38h
0339 ff        rst     38h
033a ff        rst     38h
033b d7        rst     10h
033c 00        nop     
033d 00        nop     
033e 00        nop     
033f a8        xor     b
0340 86        add     a,(hl)
0341 cc4461    call    z,6144h
0344 6c        ld      l,h
0345 75        ld      (hl),l
0346 6f        ld      l,a
0347 70        ld      (hl),b
0348 2061      jr      nz,03abh
034a 2c        inc     l
034b 3c        inc     a
034c 69        ld      l,c
034d 78        ld      a,b
034e 68        ld      l,b
034f 2c        inc     l
0350 69        ld      l,c
0351 78        ld      a,b
0352 6c        ld      l,h
0353 2c        inc     l
0354 69        ld      l,c
0355 79        ld      a,c
0356 68        ld      l,b
0357 2c        inc     l
0358 69        ld      l,c
0359 79        ld      a,c
035a 6c        ld      l,h
035b 3e2e      ld      a,2eh
035d 2e2e      ld      l,2eh
035f 2e2e      ld      l,2eh
0361 24        inc     h
0362 ff        rst     38h
0363 dd8601    add     a,(ix+01h)
0366 00        nop     
0367 b7        or      a
0368 90        sub     b
0369 02        ld      (bc),a
036a 010201    ld      bc,0102h
036d fd326e40  ld      (406eh),a
0371 dcc145    call    c,45c1h
0374 6e        ld      l,(hl)
0375 fae520    jp      m,20e5h
0378 3800      jr      c,037ah
037a 00        nop     
037b 00        nop     
037c 00        nop     
037d 010001    ld      bc,0100h
0380 00        nop     
0381 00        nop     
0382 00        nop     
0383 00        nop     
0384 00        nop     
0385 00        nop     
0386 00        nop     
0387 00        nop     
0388 ff        rst     38h
0389 00        nop     
038a 00        nop     
038b 00        nop     
038c 00        nop     
038d 00        nop     
038e 00        nop     
038f ff        rst     38h
0390 00        nop     
0391 00        nop     
0392 00        nop     
0393 00        nop     
0394 00        nop     
0395 00        nop     
0396 00        nop     
0397 00        nop     
0398 00        nop     
0399 00        nop     
039a 00        nop     
039b d7        rst     10h
039c 00        nop     
039d 00        nop     
039e 00        nop     
039f d3f2      out     (0f2h),a
03a1 d7        rst     10h
03a2 4a        ld      c,d
03a3 61        ld      h,c
03a4 6c        ld      l,h
03a5 75        ld      (hl),l
03a6 6f        ld      l,a
03a7 70        ld      (hl),b
03a8 2061      jr      nz,040bh
03aa 2c        inc     l
03ab 283c      jr      z,03e9h
03ad 69        ld      l,c
03ae 78        ld      a,b
03af 2c        inc     l
03b0 69        ld      l,c
03b1 79        ld      a,c
03b2 3e2b      ld      a,2bh
03b4 31292e    ld      sp,2e29h
03b7 2e2e      ld      l,2eh
03b9 2e2e      ld      l,2eh
03bb 2e2e      ld      l,2eh
03bd 2e2e      ld      l,2eh
03bf 2e2e      ld      l,2eh
03c1 24        inc     h
03c2 ff        rst     38h
03c3 ddcb0146  bit     0,(ix+01h)
03c7 75        ld      (hl),l
03c8 2002      jr      nz,03cch
03ca 010201    ld      bc,0102h
03cd fc3c9a    call    m,9a3ch
03d0 a7        and     a
03d1 74        ld      (hl),h
03d2 3d        dec     a
03d3 51        ld      d,c
03d4 27        daa     
03d5 14        inc     d
03d6 ca2000    jp      z,0020h
03d9 00        nop     
03da 3800      jr      c,03dch
03dc 00        nop     
03dd 00        nop     
03de 00        nop     
03df 00        nop     
03e0 00        nop     
03e1 00        nop     
03e2 00        nop     
03e3 00        nop     
03e4 00        nop     
03e5 00        nop     
03e6 00        nop     
03e7 53        ld      d,e
03e8 00        nop     
03e9 00        nop     
03ea 00        nop     
03eb 00        nop     
03ec 00        nop     
03ed 00        nop     
03ee 00        nop     
03ef ff        rst     38h
03f0 00        nop     
03f1 00        nop     
03f2 00        nop     
03f3 00        nop     
03f4 00        nop     
03f5 00        nop     
03f6 00        nop     
03f7 00        nop     
03f8 00        nop     
03f9 00        nop     
03fa 00        nop     
03fb 00        nop     
03fc 00        nop     
03fd 00        nop     
03fe 00        nop     
03ff 83        add     a,e
0400 53        ld      d,e
0401 4e        ld      c,(hl)
0402 e1        pop     hl
0403 62        ld      h,d
0404 69        ld      l,c
0405 74        ld      (hl),h
0406 206e      jr      nz,0476h
0408 2c        inc     l
0409 283c      jr      z,0447h
040b 69        ld      l,c
040c 78        ld      a,b
040d 2c        inc     l
040e 69        ld      l,c
040f 79        ld      a,c
0410 3e2b      ld      a,2bh
0412 31292e    ld      sp,2e29h
0415 2e2e      ld      l,2eh
0417 2e2e      ld      l,2eh
0419 2e2e      ld      l,2eh
041b 2e2e      ld      l,2eh
041d 2e2e      ld      l,2eh
041f 2e2e      ld      l,2eh
0421 24        inc     h
0422 ff        rst     38h
0423 cb40      bit     0,b
0425 00        nop     
0426 00        nop     
0427 f1        pop     af
0428 3efc      ld      a,0fch
042a 9d        sbc     a,l
042b cc7a03    call    z,037ah
042e 0161be    ld      bc,0be61h
0431 86        add     a,(hl)
0432 7a        ld      a,d
0433 50        ld      d,b
0434 24        inc     h
0435 98        sbc     a,b
0436 19        add     hl,de
0437 00        nop     
0438 3f        ccf     
0439 00        nop     
043a 00        nop     
043b 00        nop     
043c 00        nop     
043d 00        nop     
043e 00        nop     
043f 00        nop     
0440 00        nop     
0441 00        nop     
0442 00        nop     
0443 00        nop     
0444 00        nop     
0445 00        nop     
0446 00        nop     
0447 53        ld      d,e
0448 00        nop     
0449 00        nop     
044a 00        nop     
044b 00        nop     
044c 00        nop     
044d 00        nop     
044e 00        nop     
044f ff        rst     38h
0450 00        nop     
0451 00        nop     
0452 00        nop     
0453 00        nop     
0454 00        nop     
0455 00        nop     
0456 00        nop     
0457 ff        rst     38h
0458 ff        rst     38h
0459 ff        rst     38h
045a ff        rst     38h
045b 00        nop     
045c ff        rst     38h
045d 00        nop     
045e 00        nop     
045f 5e        ld      e,(hl)
0460 02        ld      (bc),a
0461 0e98      ld      c,98h
0463 62        ld      h,d
0464 69        ld      l,c
0465 74        ld      (hl),h
0466 206e      jr      nz,04d6h
0468 2c        inc     l
0469 3c        inc     a
046a 62        ld      h,d
046b 2c        inc     l
046c 63        ld      h,e
046d 2c        inc     l
046e 64        ld      h,h
046f 2c        inc     l
0470 65        ld      h,l
0471 2c        inc     l
0472 68        ld      l,b
0473 2c        inc     l
0474 6c        ld      l,h
0475 2c        inc     l
0476 2868      jr      z,04e0h
0478 6c        ld      l,h
0479 29        add     hl,hl
047a 2c        inc     l
047b 61        ld      h,c
047c 3e2e      ld      a,2eh
047e 2e2e      ld      l,2eh
0480 2e24      ld      l,24h
0482 ff        rst     38h
0483 eda9      cpd     
0485 00        nop     
0486 00        nop     
0487 b6        or      (hl)
0488 c7        rst     00h
0489 b4        or      h
048a 72        ld      (hl),d
048b f618      or      18h
048d 14        inc     d
048e 01bd8d    ld      bc,8dbdh
0491 0100c0    ld      bc,0c000h
0494 30a3      jr      nc,0439h
0496 94        sub     h
0497 00        nop     
0498 1000      djnz    049ah
049a 00        nop     
049b 00        nop     
049c 00        nop     
049d 00        nop     
049e 00        nop     
049f 00        nop     
04a0 00        nop     
04a1 00        nop     
04a2 00        nop     
04a3 00        nop     
04a4 00        nop     
04a5 0a        ld      a,(bc)
04a6 00        nop     
04a7 00        nop     
04a8 ff        rst     38h
04a9 00        nop     
04aa 00        nop     
04ab 00        nop     
04ac 00        nop     
04ad 00        nop     
04ae 00        nop     
04af 00        nop     
04b0 00        nop     
04b1 00        nop     
04b2 00        nop     
04b3 00        nop     
04b4 00        nop     
04b5 00        nop     
04b6 00        nop     
04b7 00        nop     
04b8 00        nop     
04b9 00        nop     
04ba 00        nop     
04bb d7        rst     10h
04bc 00        nop     
04bd 00        nop     
04be 00        nop     
04bf 13        inc     de
04c0 4b        ld      c,e
04c1 62        ld      h,d
04c2 2d        dec     l
04c3 63        ld      h,e
04c4 70        ld      (hl),b
04c5 64        ld      h,h
04c6 3c        inc     a
04c7 72        ld      (hl),d
04c8 3e2e      ld      a,2eh
04ca 2e2e      ld      l,2eh
04cc 2e2e      ld      l,2eh
04ce 2e2e      ld      l,2eh
04d0 2e2e      ld      l,2eh
04d2 2e2e      ld      l,2eh
04d4 2e2e      ld      l,2eh
04d6 2e2e      ld      l,2eh
04d8 2e2e      ld      l,2eh
04da 2e2e      ld      l,2eh
04dc 2e2e      ld      l,2eh
04de 2e2e      ld      l,2eh
04e0 2e24      ld      l,24h
04e2 ff        rst     38h
04e3 eda1      cpi     
04e5 00        nop     
04e6 00        nop     
04e7 48        ld      c,b
04e8 4d        ld      c,l
04e9 4a        ld      c,d
04ea af        xor     a
04eb 6b        ld      l,e
04ec 90        sub     b
04ed 03        inc     bc
04ee 01714e    ld      bc,4e71h
04f1 010093    ld      bc,9300h
04f4 6a        ld      l,d
04f5 7c        ld      a,h
04f6 90        sub     b
04f7 00        nop     
04f8 1000      djnz    04fah
04fa 00        nop     
04fb 00        nop     
04fc 00        nop     
04fd 00        nop     
04fe 00        nop     
04ff 00        nop     
0500 00        nop     
0501 00        nop     
0502 00        nop     
0503 00        nop     
0504 00        nop     
0505 0a        ld      a,(bc)
0506 00        nop     
0507 00        nop     
0508 ff        rst     38h
0509 00        nop     
050a 00        nop     
050b 00        nop     
050c 00        nop     
050d 00        nop     
050e 00        nop     
050f 00        nop     
0510 00        nop     
0511 00        nop     
0512 00        nop     
0513 00        nop     
0514 00        nop     
0515 00        nop     
0516 00        nop     
0517 00        nop     
0518 00        nop     
0519 00        nop     
051a 00        nop     
051b d7        rst     10h
051c 00        nop     
051d 00        nop     
051e 00        nop     
051f 2d        dec     l
0520 a4        and     h
0521 2d        dec     l
0522 19        add     hl,de
0523 63        ld      h,e
0524 70        ld      (hl),b
0525 69        ld      l,c
0526 3c        inc     a
0527 72        ld      (hl),d
0528 3e2e      ld      a,2eh
052a 2e2e      ld      l,2eh
052c 2e2e      ld      l,2eh
052e 2e2e      ld      l,2eh
0530 2e2e      ld      l,2eh
0532 2e2e      ld      l,2eh
0534 2e2e      ld      l,2eh
0536 2e2e      ld      l,2eh
0538 2e2e      ld      l,2eh
053a 2e2e      ld      l,2eh
053c 2e2e      ld      l,2eh
053e 2e2e      ld      l,2eh
0540 2e24      ld      l,24h
0542 ff        rst     38h
0543 27        daa     
0544 00        nop     
0545 00        nop     
0546 00        nop     
0547 41        ld      b,c
0548 21fa09    ld      hl,09fah
054b 60        ld      h,b
054c 1d        dec     e
054d 59        ld      e,c
054e a5        and     l
054f 5b        ld      e,e
0550 8d        adc     a,l
0551 79        ld      a,c
0552 90        sub     b
0553 04        inc     b
0554 8e        adc     a,(hl)
0555 9d        sbc     a,l
0556 29        add     hl,hl
0557 1800      jr      0559h
0559 00        nop     
055a 00        nop     
055b 00        nop     
055c 00        nop     
055d 00        nop     
055e 00        nop     
055f 00        nop     
0560 00        nop     
0561 00        nop     
0562 00        nop     
0563 00        nop     
0564 00        nop     
0565 00        nop     
0566 00        nop     
0567 d7        rst     10h
0568 ff        rst     38h
0569 00        nop     
056a 00        nop     
056b 00        nop     
056c 00        nop     
056d 00        nop     
056e 00        nop     
056f 00        nop     
0570 00        nop     
0571 00        nop     
0572 00        nop     
0573 00        nop     
0574 00        nop     
0575 00        nop     
0576 00        nop     
0577 00        nop     
0578 00        nop     
0579 00        nop     
057a 00        nop     
057b 00        nop     
057c 00        nop     
057d 00        nop     
057e 00        nop     
057f 6d        ld      l,l
0580 2d        dec     l
0581 d2133c    jp      nc,3c13h
0584 64        ld      h,h
0585 61        ld      h,c
0586 61        ld      h,c
0587 2c        inc     l
0588 63        ld      h,e
0589 70        ld      (hl),b
058a 6c        ld      l,h
058b 2c        inc     l
058c 73        ld      (hl),e
058d 63        ld      h,e
058e 66        ld      h,(hl)
058f 2c        inc     l
0590 63        ld      h,e
0591 63        ld      h,e
0592 66        ld      h,(hl)
0593 3e2e      ld      a,2eh
0595 2e2e      ld      l,2eh
0597 2e2e      ld      l,2eh
0599 2e2e      ld      l,2eh
059b 2e2e      ld      l,2eh
059d 2e2e      ld      l,2eh
059f 2e2e      ld      l,2eh
05a1 24        inc     h
05a2 ff        rst     38h
05a3 3c        inc     a
05a4 00        nop     
05a5 00        nop     
05a6 00        nop     
05a7 df        rst     18h
05a8 4a        ld      c,d
05a9 d8        ret     c

05aa d5        push    de
05ab 98        sbc     a,b
05ac e5        push    hl
05ad 2b        dec     hl
05ae 8a        adc     a,d
05af b0        or      b
05b0 a7        and     a
05b1 1b        dec     de
05b2 43        ld      b,e
05b3 44        ld      b,h
05b4 5a        ld      e,d
05b5 30d0      jr      nc,0587h
05b7 010000    ld      bc,0000h
05ba 00        nop     
05bb 00        nop     
05bc 00        nop     
05bd 00        nop     
05be 00        nop     
05bf 00        nop     
05c0 00        nop     
05c1 00        nop     
05c2 00        nop     
05c3 00        nop     
05c4 00        nop     
05c5 00        nop     
05c6 00        nop     
05c7 00        nop     
05c8 ff        rst     38h
05c9 00        nop     
05ca 00        nop     
05cb 00        nop     
05cc 00        nop     
05cd 00        nop     
05ce 00        nop     
05cf 00        nop     
05d0 00        nop     
05d1 00        nop     
05d2 00        nop     
05d3 00        nop     
05d4 00        nop     
05d5 00        nop     
05d6 00        nop     
05d7 00        nop     
05d8 00        nop     
05d9 00        nop     
05da 00        nop     
05db d7        rst     10h
05dc 00        nop     
05dd 00        nop     
05de 00        nop     
05df 81        add     a,c
05e0 fa8100    jp      m,0081h
05e3 3c        inc     a
05e4 69        ld      l,c
05e5 6e        ld      l,(hl)
05e6 63        ld      h,e
05e7 2c        inc     l
05e8 64        ld      h,h
05e9 65        ld      h,l
05ea 63        ld      h,e
05eb 3e20      ld      a,20h
05ed 61        ld      h,c
05ee 2e2e      ld      l,2eh
05f0 2e2e      ld      l,2eh
05f2 2e2e      ld      l,2eh
05f4 2e2e      ld      l,2eh
05f6 2e2e      ld      l,2eh
05f8 2e2e      ld      l,2eh
05fa 2e2e      ld      l,2eh
05fc 2e2e      ld      l,2eh
05fe 2e2e      ld      l,2eh
0600 2e24      ld      l,24h
0602 ff        rst     38h
0603 04        inc     b
0604 00        nop     
0605 00        nop     
0606 00        nop     
0607 23        inc     hl
0608 d62d      sub     2dh
060a 43        ld      b,e
060b 61        ld      h,c
060c 7a        ld      a,d
060d 80        add     a,b
060e 81        add     a,c
060f 86        add     a,(hl)
0610 5a        ld      e,d
0611 85        add     a,l
0612 1e86      ld      e,86h
0614 58        ld      e,b
0615 bb        cp      e
0616 9b        sbc     a,e
0617 010000    ld      bc,0000h
061a 00        nop     
061b 00        nop     
061c 00        nop     
061d 00        nop     
061e 00        nop     
061f 00        nop     
0620 00        nop     
0621 00        nop     
0622 00        nop     
0623 00        nop     
0624 00        nop     
0625 00        nop     
0626 ff        rst     38h
0627 00        nop     
0628 00        nop     
0629 00        nop     
062a 00        nop     
062b 00        nop     
062c 00        nop     
062d 00        nop     
062e 00        nop     
062f 00        nop     
0630 00        nop     
0631 00        nop     
0632 00        nop     
0633 00        nop     
0634 00        nop     
0635 00        nop     
0636 00        nop     
0637 00        nop     
0638 00        nop     
0639 00        nop     
063a 00        nop     
063b d7        rst     10h
063c 00        nop     
063d 00        nop     
063e 00        nop     
063f 77        ld      (hl),a
0640 f3        di      
0641 5a        ld      e,d
0642 73        ld      (hl),e
0643 3c        inc     a
0644 69        ld      l,c
0645 6e        ld      l,(hl)
0646 63        ld      h,e
0647 2c        inc     l
0648 64        ld      h,h
0649 65        ld      h,l
064a 63        ld      h,e
064b 3e20      ld      a,20h
064d 62        ld      h,d
064e 2e2e      ld      l,2eh
0650 2e2e      ld      l,2eh
0652 2e2e      ld      l,2eh
0654 2e2e      ld      l,2eh
0656 2e2e      ld      l,2eh
0658 2e2e      ld      l,2eh
065a 2e2e      ld      l,2eh
065c 2e2e      ld      l,2eh
065e 2e2e      ld      l,2eh
0660 2e24      ld      l,24h
0662 ff        rst     38h
0663 03        inc     bc
0664 00        nop     
0665 00        nop     
0666 00        nop     
0667 97        sub     a
0668 cdab44    call    44abh
066b c9        ret     

066c 8d        adc     a,l
066d e3        ex      (sp),hl
066e e3        ex      (sp),hl
066f cc11a4    call    z,0a411h
0672 e8        ret     pe

0673 02        ld      (bc),a
0674 49        ld      c,c
0675 4d        ld      c,l
0676 2a0800    ld      hl,(0008h)
0679 00        nop     
067a 00        nop     
067b 00        nop     
067c 00        nop     
067d 00        nop     
067e 00        nop     
067f 00        nop     
0680 00        nop     
0681 00        nop     
0682 00        nop     
0683 00        nop     
0684 00        nop     
0685 21f800    ld      hl,00f8h
0688 00        nop     
0689 00        nop     
068a 00        nop     
068b 00        nop     
068c 00        nop     
068d 00        nop     
068e 00        nop     
068f 00        nop     
0690 00        nop     
0691 00        nop     
0692 00        nop     
0693 00        nop     
0694 00        nop     
0695 00        nop     
0696 00        nop     
0697 00        nop     
0698 00        nop     
0699 00        nop     
069a 00        nop     
069b d7        rst     10h
069c 00        nop     
069d 00        nop     
069e 00        nop     
069f d2ae3b    jp      nc,3baeh
06a2 ec3c69    call    pe,693ch
06a5 6e        ld      l,(hl)
06a6 63        ld      h,e
06a7 2c        inc     l
06a8 64        ld      h,h
06a9 65        ld      h,l
06aa 63        ld      h,e
06ab 3e20      ld      a,20h
06ad 62        ld      h,d
06ae 63        ld      h,e
06af 2e2e      ld      l,2eh
06b1 2e2e      ld      l,2eh
06b3 2e2e      ld      l,2eh
06b5 2e2e      ld      l,2eh
06b7 2e2e      ld      l,2eh
06b9 2e2e      ld      l,2eh
06bb 2e2e      ld      l,2eh
06bd 2e2e      ld      l,2eh
06bf 2e2e      ld      l,2eh
06c1 24        inc     h
06c2 ff        rst     38h
06c3 0c        inc     c
06c4 00        nop     
06c5 00        nop     
06c6 00        nop     
06c7 89        adc     a,c
06c8 d7        rst     10h
06c9 35        dec     (hl)
06ca 09        add     hl,bc
06cb 5b        ld      e,e
06cc 05        dec     b
06cd 85        add     a,l
06ce 9f        sbc     a,a
06cf 27        daa     
06d0 8b        adc     a,e
06d1 08        ex      af,af'
06d2 d29505    jp      nc,0595h
06d5 60        ld      h,b
06d6 0601      ld      b,01h
06d8 00        nop     
06d9 00        nop     
06da 00        nop     
06db 00        nop     
06dc 00        nop     
06dd 00        nop     
06de 00        nop     
06df 00        nop     
06e0 00        nop     
06e1 00        nop     
06e2 00        nop     
06e3 00        nop     
06e4 00        nop     
06e5 ff        rst     38h
06e6 00        nop     
06e7 00        nop     
06e8 00        nop     
06e9 00        nop     
06ea 00        nop     
06eb 00        nop     
06ec 00        nop     
06ed 00        nop     
06ee 00        nop     
06ef 00        nop     
06f0 00        nop     
06f1 00        nop     
06f2 00        nop     
06f3 00        nop     
06f4 00        nop     
06f5 00        nop     
06f6 00        nop     
06f7 00        nop     
06f8 00        nop     
06f9 00        nop     
06fa 00        nop     
06fb d7        rst     10h
06fc 00        nop     
06fd 00        nop     
06fe 00        nop     
06ff 1a        ld      a,(de)
0700 f612      or      12h
0702 a7        and     a
0703 3c        inc     a
0704 69        ld      l,c
0705 6e        ld      l,(hl)
0706 63        ld      h,e
0707 2c        inc     l
0708 64        ld      h,h
0709 65        ld      h,l
070a 63        ld      h,e
070b 3e20      ld      a,20h
070d 63        ld      h,e
070e 2e2e      ld      l,2eh
0710 2e2e      ld      l,2eh
0712 2e2e      ld      l,2eh
0714 2e2e      ld      l,2eh
0716 2e2e      ld      l,2eh
0718 2e2e      ld      l,2eh
071a 2e2e      ld      l,2eh
071c 2e2e      ld      l,2eh
071e 2e2e      ld      l,2eh
0720 2e24      ld      l,24h
0722 ff        rst     38h
0723 14        inc     d
0724 00        nop     
0725 00        nop     
0726 00        nop     
0727 eaa0ba    jp      pe,0baa0h
072a 5f        ld      e,a
072b fb        ei      
072c 65        ld      h,l
072d 1c        inc     e
072e 98        sbc     a,b
072f cc38bc    call    z,0bc38h
0732 de43      sbc     a,43h
0734 5c        ld      e,h
0735 bd        cp      l
0736 03        inc     bc
0737 010000    ld      bc,0000h
073a 00        nop     
073b 00        nop     
073c 00        nop     
073d 00        nop     
073e 00        nop     
073f 00        nop     
0740 00        nop     
0741 00        nop     
0742 00        nop     
0743 00        nop     
0744 ff        rst     38h
0745 00        nop     
0746 00        nop     
0747 00        nop     
0748 00        nop     
0749 00        nop     
074a 00        nop     
074b 00        nop     
074c 00        nop     
074d 00        nop     
074e 00        nop     
074f 00        nop     
0750 00        nop     
0751 00        nop     
0752 00        nop     
0753 00        nop     
0754 00        nop     
0755 00        nop     
0756 00        nop     
0757 00        nop     
0758 00        nop     
0759 00        nop     
075a 00        nop     
075b d7        rst     10h
075c 00        nop     
075d 00        nop     
075e 00        nop     
075f d1        pop     de
0760 46        ld      b,(hl)
0761 bf        cp      a
0762 51        ld      d,c
0763 3c        inc     a
0764 69        ld      l,c
0765 6e        ld      l,(hl)
0766 63        ld      h,e
0767 2c        inc     l
0768 64        ld      h,h
0769 65        ld      h,l
076a 63        ld      h,e
076b 3e20      ld      a,20h
076d 64        ld      h,h
076e 2e2e      ld      l,2eh
0770 2e2e      ld      l,2eh
0772 2e2e      ld      l,2eh
0774 2e2e      ld      l,2eh
0776 2e2e      ld      l,2eh
0778 2e2e      ld      l,2eh
077a 2e2e      ld      l,2eh
077c 2e2e      ld      l,2eh
077e 2e2e      ld      l,2eh
0780 2e24      ld      l,24h
0782 ff        rst     38h
0783 13        inc     de
0784 00        nop     
0785 00        nop     
0786 00        nop     
0787 2e34      ld      l,34h
0789 1d        dec     e
078a 13        inc     de
078b c9        ret     

078c 28ca      jr      z,0758h
078e 0a        ld      a,(bc)
078f 67        ld      h,a
0790 99        sbc     a,c
0791 2e3a      ld      l,3ah
0793 92        sub     d
0794 f654      or      54h
0796 9d        sbc     a,l
0797 08        ex      af,af'
0798 00        nop     
0799 00        nop     
079a 00        nop     
079b 00        nop     
079c 00        nop     
079d 00        nop     
079e 00        nop     
079f 00        nop     
07a0 00        nop     
07a1 00        nop     
07a2 00        nop     
07a3 21f800    ld      hl,00f8h
07a6 00        nop     
07a7 00        nop     
07a8 00        nop     
07a9 00        nop     
07aa 00        nop     
07ab 00        nop     
07ac 00        nop     
07ad 00        nop     
07ae 00        nop     
07af 00        nop     
07b0 00        nop     
07b1 00        nop     
07b2 00        nop     
07b3 00        nop     
07b4 00        nop     
07b5 00        nop     
07b6 00        nop     
07b7 00        nop     
07b8 00        nop     
07b9 00        nop     
07ba 00        nop     
07bb d7        rst     10h
07bc 00        nop     
07bd 00        nop     
07be 00        nop     
07bf ae        xor     (hl)
07c0 c6d4      add     a,0d4h
07c2 2c        inc     l
07c3 3c        inc     a
07c4 69        ld      l,c
07c5 6e        ld      l,(hl)
07c6 63        ld      h,e
07c7 2c        inc     l
07c8 64        ld      h,h
07c9 65        ld      h,l
07ca 63        ld      h,e
07cb 3e20      ld      a,20h
07cd 64        ld      h,h
07ce 65        ld      h,l
07cf 2e2e      ld      l,2eh
07d1 2e2e      ld      l,2eh
07d3 2e2e      ld      l,2eh
07d5 2e2e      ld      l,2eh
07d7 2e2e      ld      l,2eh
07d9 2e2e      ld      l,2eh
07db 2e2e      ld      l,2eh
07dd 2e2e      ld      l,2eh
07df 2e2e      ld      l,2eh
07e1 24        inc     h
07e2 ff        rst     38h
07e3 1c        inc     e
07e4 00        nop     
07e5 00        nop     
07e6 00        nop     
07e7 2f        cpl     
07e8 60        ld      h,b
07e9 0d        dec     c
07ea 4c        ld      c,h
07eb 02        ld      (bc),a
07ec 24        inc     h
07ed f5        push    af
07ee e2f4a0    jp      po,0a0f4h
07f1 0a        ld      a,(bc)
07f2 a1        and     c
07f3 13        inc     de
07f4 322559    ld      (5925h),a
07f7 010000    ld      bc,0000h
07fa 00        nop     
07fb 00        nop     
07fc 00        nop     
07fd 00        nop     
07fe 00        nop     
07ff 00        nop     
0800 00        nop     
0801 00        nop     
0802 00        nop     
0803 ff        rst     38h
0804 00        nop     
0805 00        nop     
0806 00        nop     
0807 00        nop     
0808 00        nop     
0809 00        nop     
080a 00        nop     
080b 00        nop     
080c 00        nop     
080d 00        nop     
080e 00        nop     
080f 00        nop     
0810 00        nop     
0811 00        nop     
0812 00        nop     
0813 00        nop     
0814 00        nop     
0815 00        nop     
0816 00        nop     
0817 00        nop     
0818 00        nop     
0819 00        nop     
081a 00        nop     
081b d7        rst     10h
081c 00        nop     
081d 00        nop     
081e 00        nop     
081f ca8c6a    jp      z,6a8ch
0822 c23c69    jp      nz,693ch
0825 6e        ld      l,(hl)
0826 63        ld      h,e
0827 2c        inc     l
0828 64        ld      h,h
0829 65        ld      h,l
082a 63        ld      h,e
082b 3e20      ld      a,20h
082d 65        ld      h,l
082e 2e2e      ld      l,2eh
0830 2e2e      ld      l,2eh
0832 2e2e      ld      l,2eh
0834 2e2e      ld      l,2eh
0836 2e2e      ld      l,2eh
0838 2e2e      ld      l,2eh
083a 2e2e      ld      l,2eh
083c 2e2e      ld      l,2eh
083e 2e2e      ld      l,2eh
0840 2e24      ld      l,24h
0842 ff        rst     38h
0843 24        inc     h
0844 00        nop     
0845 00        nop     
0846 00        nop     
0847 0615      ld      b,15h
0849 eb        ex      de,hl
084a f2dde8    jp      p,0e8ddh
084d 2b        dec     hl
084e 26a6      ld      h,0a6h
0850 111abc    ld      de,0bc1ah
0853 17        rla     
0854 0618      ld      b,18h
0856 2801      jr      z,0859h
0858 00        nop     
0859 00        nop     
085a 00        nop     
085b 00        nop     
085c 00        nop     
085d 00        nop     
085e 00        nop     
085f 00        nop     
0860 00        nop     
0861 00        nop     
0862 ff        rst     38h
0863 00        nop     
0864 00        nop     
0865 00        nop     
0866 00        nop     
0867 00        nop     
0868 00        nop     
0869 00        nop     
086a 00        nop     
086b 00        nop     
086c 00        nop     
086d 00        nop     
086e 00        nop     
086f 00        nop     
0870 00        nop     
0871 00        nop     
0872 00        nop     
0873 00        nop     
0874 00        nop     
0875 00        nop     
0876 00        nop     
0877 00        nop     
0878 00        nop     
0879 00        nop     
087a 00        nop     
087b d7        rst     10h
087c 00        nop     
087d 00        nop     
087e 00        nop     
087f 56        ld      d,(hl)
0880 0f        rrca    
0881 95        sub     l
0882 5e        ld      e,(hl)
0883 3c        inc     a
0884 69        ld      l,c
0885 6e        ld      l,(hl)
0886 63        ld      h,e
0887 2c        inc     l
0888 64        ld      h,h
0889 65        ld      h,l
088a 63        ld      h,e
088b 3e20      ld      a,20h
088d 68        ld      l,b
088e 2e2e      ld      l,2eh
0890 2e2e      ld      l,2eh
0892 2e2e      ld      l,2eh
0894 2e2e      ld      l,2eh
0896 2e2e      ld      l,2eh
0898 2e2e      ld      l,2eh
089a 2e2e      ld      l,2eh
089c 2e2e      ld      l,2eh
089e 2e2e      ld      l,2eh
08a0 2e24      ld      l,24h
08a2 ff        rst     38h
08a3 23        inc     hl
08a4 00        nop     
08a5 00        nop     
08a6 00        nop     
08a7 f4c3a5    call    p,0a5c3h
08aa 07        rlca    
08ab 6d        ld      l,l
08ac 1b        dec     de
08ad 04        inc     b
08ae 4f        ld      c,a
08af c2e22a    jp      nz,2ae2h
08b2 82        add     a,d
08b3 57        ld      d,a
08b4 e0        ret     po

08b5 e1        pop     hl
08b6 c30800    jp      0008h
08b9 00        nop     
08ba 00        nop     
08bb 00        nop     
08bc 00        nop     
08bd 00        nop     
08be 00        nop     
08bf 00        nop     
08c0 00        nop     
08c1 21f800    ld      hl,00f8h
08c4 00        nop     
08c5 00        nop     
08c6 00        nop     
08c7 00        nop     
08c8 00        nop     
08c9 00        nop     
08ca 00        nop     
08cb 00        nop     
08cc 00        nop     
08cd 00        nop     
08ce 00        nop     
08cf 00        nop     
08d0 00        nop     
08d1 00        nop     
08d2 00        nop     
08d3 00        nop     
08d4 00        nop     
08d5 00        nop     
08d6 00        nop     
08d7 00        nop     
08d8 00        nop     
08d9 00        nop     
08da 00        nop     
08db d7        rst     10h
08dc 00        nop     
08dd 00        nop     
08de 00        nop     
08df fc0d6d    call    m,6d0dh
08e2 4a        ld      c,d
08e3 3c        inc     a
08e4 69        ld      l,c
08e5 6e        ld      l,(hl)
08e6 63        ld      h,e
08e7 2c        inc     l
08e8 64        ld      h,h
08e9 65        ld      h,l
08ea 63        ld      h,e
08eb 3e20      ld      a,20h
08ed 68        ld      l,b
08ee 6c        ld      l,h
08ef 2e2e      ld      l,2eh
08f1 2e2e      ld      l,2eh
08f3 2e2e      ld      l,2eh
08f5 2e2e      ld      l,2eh
08f7 2e2e      ld      l,2eh
08f9 2e2e      ld      l,2eh
08fb 2e2e      ld      l,2eh
08fd 2e2e      ld      l,2eh
08ff 2e2e      ld      l,2eh
0901 24        inc     h
0902 ff        rst     38h
0903 dd23      inc     ix
0905 00        nop     
0906 00        nop     
0907 3c        inc     a
0908 bc        cp      h
0909 9b        sbc     a,e
090a 0d        dec     c
090b 81        add     a,c
090c e0        ret     po

090d fdad      xor     iyl
090f 7f        ld      a,a
0910 9a        sbc     a,d
0911 e5        push    hl
0912 96        sub     (hl)
0913 13        inc     de
0914 85        add     a,l
0915 e20b00    jp      po,000bh
0918 08        ex      af,af'
0919 00        nop     
091a 00        nop     
091b 00        nop     
091c 00        nop     
091d 00        nop     
091e 00        nop     
091f 21f800    ld      hl,00f8h
0922 00        nop     
0923 00        nop     
0924 00        nop     
0925 00        nop     
0926 00        nop     
0927 00        nop     
0928 00        nop     
0929 00        nop     
092a 00        nop     
092b 00        nop     
092c 00        nop     
092d 00        nop     
092e 00        nop     
092f 00        nop     
0930 00        nop     
0931 00        nop     
0932 00        nop     
0933 00        nop     
0934 00        nop     
0935 00        nop     
0936 00        nop     
0937 00        nop     
0938 00        nop     
0939 00        nop     
093a 00        nop     
093b d7        rst     10h
093c 00        nop     
093d 00        nop     
093e 00        nop     
093f a5        and     l
0940 4d        ld      c,l
0941 be        cp      (hl)
0942 313c69    ld      sp,693ch
0945 6e        ld      l,(hl)
0946 63        ld      h,e
0947 2c        inc     l
0948 64        ld      h,h
0949 65        ld      h,l
094a 63        ld      h,e
094b 3e20      ld      a,20h
094d 69        ld      l,c
094e 78        ld      a,b
094f 2e2e      ld      l,2eh
0951 2e2e      ld      l,2eh
0953 2e2e      ld      l,2eh
0955 2e2e      ld      l,2eh
0957 2e2e      ld      l,2eh
0959 2e2e      ld      l,2eh
095b 2e2e      ld      l,2eh
095d 2e2e      ld      l,2eh
095f 2e2e      ld      l,2eh
0961 24        inc     h
0962 ff        rst     38h
0963 fd23      inc     iy
0965 00        nop     
0966 00        nop     
0967 02        ld      (bc),a
0968 94        sub     h
0969 7a        ld      a,d
096a 63        ld      h,e
096b 82        add     a,d
096c 315ac6    ld      sp,0c65ah
096f e9        jp      (hl)
0970 b2        or      d
0971 b4        or      h
0972 ab        xor     e
0973 16f2      ld      d,0f2h
0975 05        dec     b
0976 6d        ld      l,l
0977 00        nop     
0978 08        ex      af,af'
0979 00        nop     
097a 00        nop     
097b 00        nop     
097c 00        nop     
097d 21f800    ld      hl,00f8h
0980 00        nop     
0981 00        nop     
0982 00        nop     
0983 00        nop     
0984 00        nop     
0985 00        nop     
0986 00        nop     
0987 00        nop     
0988 00        nop     
0989 00        nop     
098a 00        nop     
098b 00        nop     
098c 00        nop     
098d 00        nop     
098e 00        nop     
098f 00        nop     
0990 00        nop     
0991 00        nop     
0992 00        nop     
0993 00        nop     
0994 00        nop     
0995 00        nop     
0996 00        nop     
0997 00        nop     
0998 00        nop     
0999 00        nop     
099a 00        nop     
099b d7        rst     10h
099c 00        nop     
099d 00        nop     
099e 00        nop     
099f 50        ld      d,b
09a0 5d        ld      e,l
09a1 51        ld      d,c
09a2 a3        and     e
09a3 3c        inc     a
09a4 69        ld      l,c
09a5 6e        ld      l,(hl)
09a6 63        ld      h,e
09a7 2c        inc     l
09a8 64        ld      h,h
09a9 65        ld      h,l
09aa 63        ld      h,e
09ab 3e20      ld      a,20h
09ad 69        ld      l,c
09ae 79        ld      a,c
09af 2e2e      ld      l,2eh
09b1 2e2e      ld      l,2eh
09b3 2e2e      ld      l,2eh
09b5 2e2e      ld      l,2eh
09b7 2e2e      ld      l,2eh
09b9 2e2e      ld      l,2eh
09bb 2e2e      ld      l,2eh
09bd 2e2e      ld      l,2eh
09bf 2e2e      ld      l,2eh
09c1 24        inc     h
09c2 ff        rst     38h
09c3 2c        inc     l
09c4 00        nop     
09c5 00        nop     
09c6 00        nop     
09c7 318020    ld      sp,2080h
09ca a5        and     l
09cb 56        ld      d,(hl)
09cc 43        ld      b,e
09cd 09        add     hl,bc
09ce b4        or      h
09cf c1        pop     bc
09d0 f4a2df    call    p,0dfa2h
09d3 d1        pop     de
09d4 3c        inc     a
09d5 a2        and     d
09d6 3e01      ld      a,01h
09d8 00        nop     
09d9 00        nop     
09da 00        nop     
09db 00        nop     
09dc 00        nop     
09dd 00        nop     
09de 00        nop     
09df 00        nop     
09e0 00        nop     
09e1 ff        rst     38h
09e2 00        nop     
09e3 00        nop     
09e4 00        nop     
09e5 00        nop     
09e6 00        nop     
09e7 00        nop     
09e8 00        nop     
09e9 00        nop     
09ea 00        nop     
09eb 00        nop     
09ec 00        nop     
09ed 00        nop     
09ee 00        nop     
09ef 00        nop     
09f0 00        nop     
09f1 00        nop     
09f2 00        nop     
09f3 00        nop     
09f4 00        nop     
09f5 00        nop     
09f6 00        nop     
09f7 00        nop     
09f8 00        nop     
09f9 00        nop     
09fa 00        nop     
09fb d7        rst     10h
09fc 00        nop     
09fd 00        nop     
09fe 00        nop     
09ff a0        and     b
0a00 a1        and     c
0a01 b4        or      h
0a02 9f        sbc     a,a
0a03 3c        inc     a
0a04 69        ld      l,c
0a05 6e        ld      l,(hl)
0a06 63        ld      h,e
0a07 2c        inc     l
0a08 64        ld      h,h
0a09 65        ld      h,l
0a0a 63        ld      h,e
0a0b 3e20      ld      a,20h
0a0d 6c        ld      l,h
0a0e 2e2e      ld      l,2eh
0a10 2e2e      ld      l,2eh
0a12 2e2e      ld      l,2eh
0a14 2e2e      ld      l,2eh
0a16 2e2e      ld      l,2eh
0a18 2e2e      ld      l,2eh
0a1a 2e2e      ld      l,2eh
0a1c 2e2e      ld      l,2eh
0a1e 2e2e      ld      l,2eh
0a20 2e24      ld      l,24h
0a22 ff        rst     38h
0a23 34        inc     (hl)
0a24 00        nop     
0a25 00        nop     
0a26 00        nop     
0a27 56        ld      d,(hl)
0a28 b8        cp      b
0a29 7c        ld      a,h
0a2a 0c        inc     c
0a2b 3ee5      ld      a,0e5h
0a2d 03        inc     bc
0a2e 017e87    ld      bc,877eh
0a31 58        ld      e,b
0a32 da155c    jp      c,5c15h
0a35 37        scf     
0a36 1f        rra     
0a37 010000    ld      bc,0000h
0a3a 00        nop     
0a3b ff        rst     38h
0a3c 00        nop     
0a3d 00        nop     
0a3e 00        nop     
0a3f 00        nop     
0a40 00        nop     
0a41 00        nop     
0a42 00        nop     
0a43 00        nop     
0a44 00        nop     
0a45 00        nop     
0a46 00        nop     
0a47 00        nop     
0a48 00        nop     
0a49 00        nop     
0a4a 00        nop     
0a4b 00        nop     
0a4c 00        nop     
0a4d 00        nop     
0a4e 00        nop     
0a4f 00        nop     
0a50 00        nop     
0a51 00        nop     
0a52 00        nop     
0a53 00        nop     
0a54 00        nop     
0a55 00        nop     
0a56 00        nop     
0a57 00        nop     
0a58 00        nop     
0a59 00        nop     
0a5a 00        nop     
0a5b d7        rst     10h
0a5c 00        nop     
0a5d 00        nop     
0a5e 00        nop     
0a5f 2829      jr      z,0a8ah
0a61 5e        ld      e,(hl)
0a62 ce3c      adc     a,3ch
0a64 69        ld      l,c
0a65 6e        ld      l,(hl)
0a66 63        ld      h,e
0a67 2c        inc     l
0a68 64        ld      h,h
0a69 65        ld      h,l
0a6a 63        ld      h,e
0a6b 3e20      ld      a,20h
0a6d 2868      jr      z,0ad7h
0a6f 6c        ld      l,h
0a70 29        add     hl,hl
0a71 2e2e      ld      l,2eh
0a73 2e2e      ld      l,2eh
0a75 2e2e      ld      l,2eh
0a77 2e2e      ld      l,2eh
0a79 2e2e      ld      l,2eh
0a7b 2e2e      ld      l,2eh
0a7d 2e2e      ld      l,2eh
0a7f 2e2e      ld      l,2eh
0a81 24        inc     h
0a82 ff        rst     38h
0a83 33        inc     sp
0a84 00        nop     
0a85 00        nop     
0a86 00        nop     
0a87 6f        ld      l,a
0a88 34        inc     (hl)
0a89 82        add     a,d
0a8a d469d1    call    nc,0d169h
0a8d b6        or      (hl)
0a8e de94      sbc     a,94h
0a90 a4        and     h
0a91 76        halt    
0a92 f45302    call    p,0253h
0a95 5b        ld      e,e
0a96 85        add     a,l
0a97 08        ex      af,af'
0a98 00        nop     
0a99 00        nop     
0a9a 00        nop     
0a9b 00        nop     
0a9c 00        nop     
0a9d 00        nop     
0a9e 00        nop     
0a9f 00        nop     
0aa0 00        nop     
0aa1 00        nop     
0aa2 00        nop     
0aa3 00        nop     
0aa4 00        nop     
0aa5 00        nop     
0aa6 00        nop     
0aa7 00        nop     
0aa8 00        nop     
0aa9 21f800    ld      hl,00f8h
0aac 00        nop     
0aad 00        nop     
0aae 00        nop     
0aaf 00        nop     
0ab0 00        nop     
0ab1 00        nop     
0ab2 00        nop     
0ab3 00        nop     
0ab4 00        nop     
0ab5 00        nop     
0ab6 00        nop     
0ab7 00        nop     
0ab8 00        nop     
0ab9 00        nop     
0aba 00        nop     
0abb d7        rst     10h
0abc 00        nop     
0abd 00        nop     
0abe 00        nop     
0abf 5d        ld      e,l
0ac0 ac        xor     h
0ac1 d5        push    de
0ac2 27        daa     
0ac3 3c        inc     a
0ac4 69        ld      l,c
0ac5 6e        ld      l,(hl)
0ac6 63        ld      h,e
0ac7 2c        inc     l
0ac8 64        ld      h,h
0ac9 65        ld      h,l
0aca 63        ld      h,e
0acb 3e20      ld      a,20h
0acd 73        ld      (hl),e
0ace 70        ld      (hl),b
0acf 2e2e      ld      l,2eh
0ad1 2e2e      ld      l,2eh
0ad3 2e2e      ld      l,2eh
0ad5 2e2e      ld      l,2eh
0ad7 2e2e      ld      l,2eh
0ad9 2e2e      ld      l,2eh
0adb 2e2e      ld      l,2eh
0add 2e2e      ld      l,2eh
0adf 2e2e      ld      l,2eh
0ae1 24        inc     h
0ae2 ff        rst     38h
0ae3 dd3401    inc     (ix+01h)
0ae6 00        nop     
0ae7 6e        ld      l,(hl)
0ae8 fa0201    jp      m,0102h
0aeb 02        ld      (bc),a
0aec 01282c    ld      bc,2c28h
0aef 94        sub     h
0af0 88        adc     a,b
0af1 57        ld      d,a
0af2 50        ld      d,b
0af3 1633      ld      d,33h
0af5 6f        ld      l,a
0af6 2820      jr      z,0b18h
0af8 010000    ld      bc,0000h
0afb ff        rst     38h
0afc 00        nop     
0afd 00        nop     
0afe 00        nop     
0aff 00        nop     
0b00 00        nop     
0b01 00        nop     
0b02 00        nop     
0b03 00        nop     
0b04 00        nop     
0b05 00        nop     
0b06 00        nop     
0b07 00        nop     
0b08 00        nop     
0b09 00        nop     
0b0a 00        nop     
0b0b 00        nop     
0b0c 00        nop     
0b0d 00        nop     
0b0e 00        nop     
0b0f 00        nop     
0b10 00        nop     
0b11 00        nop     
0b12 00        nop     
0b13 00        nop     
0b14 00        nop     
0b15 00        nop     
0b16 00        nop     
0b17 00        nop     
0b18 00        nop     
0b19 00        nop     
0b1a 00        nop     
0b1b d7        rst     10h
0b1c 00        nop     
0b1d 00        nop     
0b1e 00        nop     
0b1f 0b        dec     bc
0b20 95        sub     l
0b21 a8        xor     b
0b22 ea3c69    jp      pe,693ch
0b25 6e        ld      l,(hl)
0b26 63        ld      h,e
0b27 2c        inc     l
0b28 64        ld      h,h
0b29 65        ld      h,l
0b2a 63        ld      h,e
0b2b 3e20      ld      a,20h
0b2d 283c      jr      z,0b6bh
0b2f 69        ld      l,c
0b30 78        ld      a,b
0b31 2c        inc     l
0b32 69        ld      l,c
0b33 79        ld      a,c
0b34 3e2b      ld      a,2bh
0b36 31292e    ld      sp,2e29h
0b39 2e2e      ld      l,2eh
0b3b 2e2e      ld      l,2eh
0b3d 2e2e      ld      l,2eh
0b3f 2e2e      ld      l,2eh
0b41 24        inc     h
0b42 ff        rst     38h
0b43 dd24      inc     ixh
0b45 00        nop     
0b46 00        nop     
0b47 38b8      jr      c,0b01h
0b49 6c        ld      l,h
0b4a 31d4c6    ld      sp,0c6d4h
0b4d 013e58    ld      bc,583eh
0b50 83        add     a,e
0b51 b4        or      h
0b52 15        dec     d
0b53 81        add     a,c
0b54 de59      sbc     a,59h
0b56 42        ld      b,d
0b57 00        nop     
0b58 010000    ld      bc,0000h
0b5b 00        nop     
0b5c 00        nop     
0b5d 00        nop     
0b5e ff        rst     38h
0b5f 00        nop     
0b60 00        nop     
0b61 00        nop     
0b62 00        nop     
0b63 00        nop     
0b64 00        nop     
0b65 00        nop     
0b66 00        nop     
0b67 00        nop     
0b68 00        nop     
0b69 00        nop     
0b6a 00        nop     
0b6b 00        nop     
0b6c 00        nop     
0b6d 00        nop     
0b6e 00        nop     
0b6f 00        nop     
0b70 00        nop     
0b71 00        nop     
0b72 00        nop     
0b73 00        nop     
0b74 00        nop     
0b75 00        nop     
0b76 00        nop     
0b77 00        nop     
0b78 00        nop     
0b79 00        nop     
0b7a 00        nop     
0b7b d7        rst     10h
0b7c 00        nop     
0b7d 00        nop     
0b7e 00        nop     
0b7f 6f        ld      l,a
0b80 46        ld      b,(hl)
0b81 3662      ld      (hl),62h
0b83 3c        inc     a
0b84 69        ld      l,c
0b85 6e        ld      l,(hl)
0b86 63        ld      h,e
0b87 2c        inc     l
0b88 64        ld      h,h
0b89 65        ld      h,l
0b8a 63        ld      h,e
0b8b 3e20      ld      a,20h
0b8d 69        ld      l,c
0b8e 78        ld      a,b
0b8f 68        ld      l,b
0b90 2e2e      ld      l,2eh
0b92 2e2e      ld      l,2eh
0b94 2e2e      ld      l,2eh
0b96 2e2e      ld      l,2eh
0b98 2e2e      ld      l,2eh
0b9a 2e2e      ld      l,2eh
0b9c 2e2e      ld      l,2eh
0b9e 2e2e      ld      l,2eh
0ba0 2e24      ld      l,24h
0ba2 ff        rst     38h
0ba3 dd2c      inc     ixl
0ba5 00        nop     
0ba6 00        nop     
0ba7 14        inc     d
0ba8 4d        ld      c,l
0ba9 60        ld      h,b
0baa 74        ld      (hl),h
0bab d476e7    call    nc,0e776h
0bae 06a2      ld      b,0a2h
0bb0 323c21    ld      (213ch),a
0bb3 d6d7      sub     0d7h
0bb5 a5        and     l
0bb6 99        sbc     a,c
0bb7 00        nop     
0bb8 010000    ld      bc,0000h
0bbb 00        nop     
0bbc 00        nop     
0bbd ff        rst     38h
0bbe 00        nop     
0bbf 00        nop     
0bc0 00        nop     
0bc1 00        nop     
0bc2 00        nop     
0bc3 00        nop     
0bc4 00        nop     
0bc5 00        nop     
0bc6 00        nop     
0bc7 00        nop     
0bc8 00        nop     
0bc9 00        nop     
0bca 00        nop     
0bcb 00        nop     
0bcc 00        nop     
0bcd 00        nop     
0bce 00        nop     
0bcf 00        nop     
0bd0 00        nop     
0bd1 00        nop     
0bd2 00        nop     
0bd3 00        nop     
0bd4 00        nop     
0bd5 00        nop     
0bd6 00        nop     
0bd7 00        nop     
0bd8 00        nop     
0bd9 00        nop     
0bda 00        nop     
0bdb d7        rst     10h
0bdc 00        nop     
0bdd 00        nop     
0bde 00        nop     
0bdf 02        ld      (bc),a
0be0 7b        ld      a,e
0be1 ef        rst     28h
0be2 2c        inc     l
0be3 3c        inc     a
0be4 69        ld      l,c
0be5 6e        ld      l,(hl)
0be6 63        ld      h,e
0be7 2c        inc     l
0be8 64        ld      h,h
0be9 65        ld      h,l
0bea 63        ld      h,e
0beb 3e20      ld      a,20h
0bed 69        ld      l,c
0bee 78        ld      a,b
0bef 6c        ld      l,h
0bf0 2e2e      ld      l,2eh
0bf2 2e2e      ld      l,2eh
0bf4 2e2e      ld      l,2eh
0bf6 2e2e      ld      l,2eh
0bf8 2e2e      ld      l,2eh
0bfa 2e2e      ld      l,2eh
0bfc 2e2e      ld      l,2eh
0bfe 2e2e      ld      l,2eh
0c00 2e24      ld      l,24h
0c02 ff        rst     38h
0c03 dd24      inc     ixh
0c05 00        nop     
0c06 00        nop     
0c07 3628      ld      (hl),28h
0c09 6f        ld      l,a
0c0a 9f        sbc     a,a
0c0b 1691      ld      d,91h
0c0d b9        cp      c
0c0e 61        ld      h,c
0c0f cb82      res     0,d
0c11 19        add     hl,de
0c12 e29273    jp      po,7392h
0c15 8c        adc     a,h
0c16 a9        xor     c
0c17 00        nop     
0c18 010000    ld      bc,0000h
0c1b 00        nop     
0c1c ff        rst     38h
0c1d 00        nop     
0c1e 00        nop     
0c1f 00        nop     
0c20 00        nop     
0c21 00        nop     
0c22 00        nop     
0c23 00        nop     
0c24 00        nop     
0c25 00        nop     
0c26 00        nop     
0c27 00        nop     
0c28 00        nop     
0c29 00        nop     
0c2a 00        nop     
0c2b 00        nop     
0c2c 00        nop     
0c2d 00        nop     
0c2e 00        nop     
0c2f 00        nop     
0c30 00        nop     
0c31 00        nop     
0c32 00        nop     
0c33 00        nop     
0c34 00        nop     
0c35 00        nop     
0c36 00        nop     
0c37 00        nop     
0c38 00        nop     
0c39 00        nop     
0c3a 00        nop     
0c3b d7        rst     10h
0c3c 00        nop     
0c3d 00        nop     
0c3e 00        nop     
0c3f 2d        dec     l
0c40 96        sub     (hl)
0c41 6c        ld      l,h
0c42 f3        di      
0c43 3c        inc     a
0c44 69        ld      l,c
0c45 6e        ld      l,(hl)
0c46 63        ld      h,e
0c47 2c        inc     l
0c48 64        ld      h,h
0c49 65        ld      h,l
0c4a 63        ld      h,e
0c4b 3e20      ld      a,20h
0c4d 69        ld      l,c
0c4e 79        ld      a,c
0c4f 68        ld      l,b
0c50 2e2e      ld      l,2eh
0c52 2e2e      ld      l,2eh
0c54 2e2e      ld      l,2eh
0c56 2e2e      ld      l,2eh
0c58 2e2e      ld      l,2eh
0c5a 2e2e      ld      l,2eh
0c5c 2e2e      ld      l,2eh
0c5e 2e2e      ld      l,2eh
0c60 2e24      ld      l,24h
0c62 ff        rst     38h
0c63 dd2c      inc     ixl
0c65 00        nop     
0c66 00        nop     
0c67 c6d7      add     a,0d7h
0c69 d5        push    de
0c6a 62        ld      h,d
0c6b 9e        sbc     a,(hl)
0c6c a0        and     b
0c6d 39        add     hl,sp
0c6e 70        ld      (hl),b
0c6f 7e        ld      a,(hl)
0c70 3e12      ld      a,12h
0c72 9f        sbc     a,a
0c73 90        sub     b
0c74 d9        exx     
0c75 0f        rrca    
0c76 220001    ld      (0100h),hl
0c79 00        nop     
0c7a 00        nop     
0c7b ff        rst     38h
0c7c 00        nop     
0c7d 00        nop     
0c7e 00        nop     
0c7f 00        nop     
0c80 00        nop     
0c81 00        nop     
0c82 00        nop     
0c83 00        nop     
0c84 00        nop     
0c85 00        nop     
0c86 00        nop     
0c87 00        nop     
0c88 00        nop     
0c89 00        nop     
0c8a 00        nop     
0c8b 00        nop     
0c8c 00        nop     
0c8d 00        nop     
0c8e 00        nop     
0c8f 00        nop     
0c90 00        nop     
0c91 00        nop     
0c92 00        nop     
0c93 00        nop     
0c94 00        nop     
0c95 00        nop     
0c96 00        nop     
0c97 00        nop     
0c98 00        nop     
0c99 00        nop     
0c9a 00        nop     
0c9b d7        rst     10h
0c9c 00        nop     
0c9d 00        nop     
0c9e 00        nop     
0c9f 36c1      ld      (hl),0c1h
0ca1 1e75      ld      e,75h
0ca3 3c        inc     a
0ca4 69        ld      l,c
0ca5 6e        ld      l,(hl)
0ca6 63        ld      h,e
0ca7 2c        inc     l
0ca8 64        ld      h,h
0ca9 65        ld      h,l
0caa 63        ld      h,e
0cab 3e20      ld      a,20h
0cad 69        ld      l,c
0cae 79        ld      a,c
0caf 6c        ld      l,h
0cb0 2e2e      ld      l,2eh
0cb2 2e2e      ld      l,2eh
0cb4 2e2e      ld      l,2eh
0cb6 2e2e      ld      l,2eh
0cb8 2e2e      ld      l,2eh
0cba 2e2e      ld      l,2eh
0cbc 2e2e      ld      l,2eh
0cbe 2e2e      ld      l,2eh
0cc0 2e24      ld      l,24h
0cc2 ff        rst     38h
0cc3 ed4b0301  ld      bc,(0103h)
0cc7 a8        xor     b
0cc8 f9        ld      sp,hl
0cc9 59        ld      e,c
0cca f5        push    af
0ccb a4        and     h
0ccc 93        sub     e
0ccd edf5      db      0edh, 0f5h       ; Undocumented 8 T-State NOP
0ccf 96        sub     (hl)
0cd0 6f        ld      l,a
0cd1 68        ld      l,b
0cd2 d9        exx     
0cd3 86        add     a,(hl)
0cd4 e6d8      and     0d8h
0cd6 4b        ld      c,e
0cd7 00        nop     
0cd8 1000      djnz    0cdah
0cda 00        nop     
0cdb 00        nop     
0cdc 00        nop     
0cdd 00        nop     
0cde 00        nop     
0cdf 00        nop     
0ce0 00        nop     
0ce1 00        nop     
0ce2 00        nop     
0ce3 00        nop     
0ce4 00        nop     
0ce5 00        nop     
0ce6 00        nop     
0ce7 00        nop     
0ce8 00        nop     
0ce9 00        nop     
0cea 00        nop     
0ceb 00        nop     
0cec 00        nop     
0ced 00        nop     
0cee 00        nop     
0cef ff        rst     38h
0cf0 ff        rst     38h
0cf1 00        nop     
0cf2 00        nop     
0cf3 00        nop     
0cf4 00        nop     
0cf5 00        nop     
0cf6 00        nop     
0cf7 00        nop     
0cf8 00        nop     
0cf9 00        nop     
0cfa 00        nop     
0cfb 00        nop     
0cfc 00        nop     
0cfd 00        nop     
0cfe 00        nop     
0cff 4d        ld      c,l
0d00 45        ld      b,l
0d01 a9        xor     c
0d02 ac        xor     h
0d03 6c        ld      l,h
0d04 64        ld      h,h
0d05 203c      jr      nz,0d43h
0d07 62        ld      h,d
0d08 63        ld      h,e
0d09 2c        inc     l
0d0a 64        ld      h,h
0d0b 65        ld      h,l
0d0c 3e2c      ld      a,2ch
0d0e 286e      jr      z,0d7eh
0d10 6e        ld      l,(hl)
0d11 6e        ld      l,(hl)
0d12 6e        ld      l,(hl)
0d13 29        add     hl,hl
0d14 2e2e      ld      l,2eh
0d16 2e2e      ld      l,2eh
0d18 2e2e      ld      l,2eh
0d1a 2e2e      ld      l,2eh
0d1c 2e2e      ld      l,2eh
0d1e 2e2e      ld      l,2eh
0d20 2e24      ld      l,24h
0d22 ff        rst     38h
0d23 2a0301    ld      hl,(0103h)
0d26 00        nop     
0d27 63        ld      h,e
0d28 98        sbc     a,b
0d29 3078      jr      nc,0da3h
0d2b 77        ld      (hl),a
0d2c 20fe      jr      nz,0d2ch
0d2e b1        or      c
0d2f fab9b8    jp      m,0b8b9h
0d32 ab        xor     e
0d33 04        inc     b
0d34 0615      ld      b,15h
0d36 60        ld      h,b
0d37 00        nop     
0d38 00        nop     
0d39 00        nop     
0d3a 00        nop     
0d3b 00        nop     
0d3c 00        nop     
0d3d 00        nop     
0d3e 00        nop     
0d3f 00        nop     
0d40 00        nop     
0d41 00        nop     
0d42 00        nop     
0d43 00        nop     
0d44 00        nop     
0d45 00        nop     
0d46 00        nop     
0d47 00        nop     
0d48 00        nop     
0d49 00        nop     
0d4a 00        nop     
0d4b 00        nop     
0d4c 00        nop     
0d4d 00        nop     
0d4e 00        nop     
0d4f ff        rst     38h
0d50 ff        rst     38h
0d51 00        nop     
0d52 00        nop     
0d53 00        nop     
0d54 00        nop     
0d55 00        nop     
0d56 00        nop     
0d57 00        nop     
0d58 00        nop     
0d59 00        nop     
0d5a 00        nop     
0d5b 00        nop     
0d5c 00        nop     
0d5d 00        nop     
0d5e 00        nop     
0d5f 5f        ld      e,a
0d60 97        sub     a
0d61 24        inc     h
0d62 87        add     a,a
0d63 6c        ld      l,h
0d64 64        ld      h,h
0d65 2068      jr      nz,0dcfh
0d67 6c        ld      l,h
0d68 2c        inc     l
0d69 286e      jr      z,0dd9h
0d6b 6e        ld      l,(hl)
0d6c 6e        ld      l,(hl)
0d6d 6e        ld      l,(hl)
0d6e 29        add     hl,hl
0d6f 2e2e      ld      l,2eh
0d71 2e2e      ld      l,2eh
0d73 2e2e      ld      l,2eh
0d75 2e2e      ld      l,2eh
0d77 2e2e      ld      l,2eh
0d79 2e2e      ld      l,2eh
0d7b 2e2e      ld      l,2eh
0d7d 2e2e      ld      l,2eh
0d7f 2e2e      ld      l,2eh
0d81 24        inc     h
0d82 ff        rst     38h
0d83 ed7b0301  ld      sp,(0103h)
0d87 fc8dd7    call    m,0d78dh
0d8a 57        ld      d,a
0d8b 61        ld      h,c
0d8c 2118ca    ld      hl,0ca18h
0d8f 85        add     a,l
0d90 c1        pop     bc
0d91 da2783    jp      c,8327h
0d94 1e60      ld      e,60h
0d96 f40000    call    p,0000h
0d99 00        nop     
0d9a 00        nop     
0d9b 00        nop     
0d9c 00        nop     
0d9d 00        nop     
0d9e 00        nop     
0d9f 00        nop     
0da0 00        nop     
0da1 00        nop     
0da2 00        nop     
0da3 00        nop     
0da4 00        nop     
0da5 00        nop     
0da6 00        nop     
0da7 00        nop     
0da8 00        nop     
0da9 00        nop     
0daa 00        nop     
0dab 00        nop     
0dac 00        nop     
0dad 00        nop     
0dae 00        nop     
0daf ff        rst     38h
0db0 ff        rst     38h
0db1 00        nop     
0db2 00        nop     
0db3 00        nop     
0db4 00        nop     
0db5 00        nop     
0db6 00        nop     
0db7 00        nop     
0db8 00        nop     
0db9 00        nop     
0dba 00        nop     
0dbb 00        nop     
0dbc 00        nop     
0dbd 00        nop     
0dbe 00        nop     
0dbf 7a        ld      a,d
0dc0 cea1      adc     a,0a1h
0dc2 1b        dec     de
0dc3 6c        ld      l,h
0dc4 64        ld      h,h
0dc5 2073      jr      nz,0e3ah
0dc7 70        ld      (hl),b
0dc8 2c        inc     l
0dc9 286e      jr      z,0e39h
0dcb 6e        ld      l,(hl)
0dcc 6e        ld      l,(hl)
0dcd 6e        ld      l,(hl)
0dce 29        add     hl,hl
0dcf 2e2e      ld      l,2eh
0dd1 2e2e      ld      l,2eh
0dd3 2e2e      ld      l,2eh
0dd5 2e2e      ld      l,2eh
0dd7 2e2e      ld      l,2eh
0dd9 2e2e      ld      l,2eh
0ddb 2e2e      ld      l,2eh
0ddd 2e2e      ld      l,2eh
0ddf 2e2e      ld      l,2eh
0de1 24        inc     h
0de2 ff        rst     38h
0de3 dd2a0301  ld      ix,(0103h)
0de7 d7        rst     10h
0de8 defa      sbc     a,0fah
0dea a6        and     (hl)
0deb 80        add     a,b
0dec f7        rst     30h
0ded 4c        ld      c,h
0dee 24        inc     h
0def de87      sbc     a,87h
0df1 c2bc16    jp      nz,16bch
0df4 63        ld      h,e
0df5 96        sub     (hl)
0df6 4c        ld      c,h
0df7 2000      jr      nz,0df9h
0df9 00        nop     
0dfa 00        nop     
0dfb 00        nop     
0dfc 00        nop     
0dfd 00        nop     
0dfe 00        nop     
0dff 00        nop     
0e00 00        nop     
0e01 00        nop     
0e02 00        nop     
0e03 00        nop     
0e04 00        nop     
0e05 00        nop     
0e06 00        nop     
0e07 00        nop     
0e08 00        nop     
0e09 00        nop     
0e0a 00        nop     
0e0b 00        nop     
0e0c 00        nop     
0e0d 00        nop     
0e0e 00        nop     
0e0f ff        rst     38h
0e10 ff        rst     38h
0e11 00        nop     
0e12 00        nop     
0e13 00        nop     
0e14 00        nop     
0e15 00        nop     
0e16 00        nop     
0e17 00        nop     
0e18 00        nop     
0e19 00        nop     
0e1a 00        nop     
0e1b 00        nop     
0e1c 00        nop     
0e1d 00        nop     
0e1e 00        nop     
0e1f 85        add     a,l
0e20 8b        adc     a,e
0e21 f1        pop     af
0e22 6d        ld      l,l
0e23 6c        ld      l,h
0e24 64        ld      h,h
0e25 203c      jr      nz,0e63h
0e27 69        ld      l,c
0e28 78        ld      a,b
0e29 2c        inc     l
0e2a 69        ld      l,c
0e2b 79        ld      a,c
0e2c 3e2c      ld      a,2ch
0e2e 286e      jr      z,0e9eh
0e30 6e        ld      l,(hl)
0e31 6e        ld      l,(hl)
0e32 6e        ld      l,(hl)
0e33 29        add     hl,hl
0e34 2e2e      ld      l,2eh
0e36 2e2e      ld      l,2eh
0e38 2e2e      ld      l,2eh
0e3a 2e2e      ld      l,2eh
0e3c 2e2e      ld      l,2eh
0e3e 2e2e      ld      l,2eh
0e40 2e24      ld      l,24h
0e42 ff        rst     38h
0e43 ed430301  ld      (0103h),bc
0e47 98        sbc     a,b
0e48 1f        rra     
0e49 4d        ld      c,l
0e4a 84        add     a,h
0e4b ac        xor     h
0e4c e8        ret     pe

0e4d edc9      db      0edh, 0c9h       ; Undocumented 8 T-State NOP
0e4f 5d        ld      e,l
0e50 c9        ret     

0e51 61        ld      h,c
0e52 8f        adc     a,a
0e53 80        add     a,b
0e54 3f        ccf     
0e55 bf        cp      a
0e56 c7        rst     00h
0e57 00        nop     
0e58 1000      djnz    0e5ah
0e5a 00        nop     
0e5b 00        nop     
0e5c 00        nop     
0e5d 00        nop     
0e5e 00        nop     
0e5f 00        nop     
0e60 00        nop     
0e61 00        nop     
0e62 00        nop     
0e63 00        nop     
0e64 00        nop     
0e65 00        nop     
0e66 00        nop     
0e67 00        nop     
0e68 00        nop     
0e69 00        nop     
0e6a 00        nop     
0e6b 00        nop     
0e6c 00        nop     
0e6d 00        nop     
0e6e 00        nop     
0e6f 00        nop     
0e70 00        nop     
0e71 00        nop     
0e72 00        nop     
0e73 00        nop     
0e74 00        nop     
0e75 00        nop     
0e76 00        nop     
0e77 ff        rst     38h
0e78 ff        rst     38h
0e79 ff        rst     38h
0e7a ff        rst     38h
0e7b 00        nop     
0e7c 00        nop     
0e7d 00        nop     
0e7e 00        nop     
0e7f 64        ld      h,h
0e80 1e87      ld      e,87h
0e82 15        dec     d
0e83 6c        ld      l,h
0e84 64        ld      h,h
0e85 2028      jr      nz,0eafh
0e87 6e        ld      l,(hl)
0e88 6e        ld      l,(hl)
0e89 6e        ld      l,(hl)
0e8a 6e        ld      l,(hl)
0e8b 29        add     hl,hl
0e8c 2c        inc     l
0e8d 3c        inc     a
0e8e 62        ld      h,d
0e8f 63        ld      h,e
0e90 2c        inc     l
0e91 64        ld      h,h
0e92 65        ld      h,l
0e93 3e2e      ld      a,2eh
0e95 2e2e      ld      l,2eh
0e97 2e2e      ld      l,2eh
0e99 2e2e      ld      l,2eh
0e9b 2e2e      ld      l,2eh
0e9d 2e2e      ld      l,2eh
0e9f 2e2e      ld      l,2eh
0ea1 24        inc     h
0ea2 ff        rst     38h
0ea3 220301    ld      (0103h),hl
0ea6 00        nop     
0ea7 03        inc     bc
0ea8 d0        ret     nc

0ea9 72        ld      (hl),d
0eaa 77        ld      (hl),a
0eab 53        ld      d,e
0eac 7f        ld      a,a
0ead 72        ld      (hl),d
0eae 3f        ccf     
0eaf ea6480    jp      pe,8064h
0eb2 e1        pop     hl
0eb3 102d      djnz    0ee2h
0eb5 e9        jp      (hl)
0eb6 35        dec     (hl)
0eb7 00        nop     
0eb8 00        nop     
0eb9 00        nop     
0eba 00        nop     
0ebb 00        nop     
0ebc 00        nop     
0ebd 00        nop     
0ebe 00        nop     
0ebf 00        nop     
0ec0 00        nop     
0ec1 00        nop     
0ec2 00        nop     
0ec3 00        nop     
0ec4 00        nop     
0ec5 00        nop     
0ec6 00        nop     
0ec7 00        nop     
0ec8 00        nop     
0ec9 00        nop     
0eca 00        nop     
0ecb 00        nop     
0ecc 00        nop     
0ecd 00        nop     
0ece 00        nop     
0ecf 00        nop     
0ed0 00        nop     
0ed1 00        nop     
0ed2 00        nop     
0ed3 00        nop     
0ed4 00        nop     
0ed5 ff        rst     38h
0ed6 ff        rst     38h
0ed7 00        nop     
0ed8 00        nop     
0ed9 00        nop     
0eda 00        nop     
0edb 00        nop     
0edc 00        nop     
0edd 00        nop     
0ede 00        nop     
0edf a3        and     e
0ee0 60        ld      h,b
0ee1 8b        adc     a,e
0ee2 47        ld      b,a
0ee3 6c        ld      l,h
0ee4 64        ld      h,h
0ee5 2028      jr      nz,0f0fh
0ee7 6e        ld      l,(hl)
0ee8 6e        ld      l,(hl)
0ee9 6e        ld      l,(hl)
0eea 6e        ld      l,(hl)
0eeb 29        add     hl,hl
0eec 2c        inc     l
0eed 68        ld      l,b
0eee 6c        ld      l,h
0eef 2e2e      ld      l,2eh
0ef1 2e2e      ld      l,2eh
0ef3 2e2e      ld      l,2eh
0ef5 2e2e      ld      l,2eh
0ef7 2e2e      ld      l,2eh
0ef9 2e2e      ld      l,2eh
0efb 2e2e      ld      l,2eh
0efd 2e2e      ld      l,2eh
0eff 2e2e      ld      l,2eh
0f01 24        inc     h
0f02 ff        rst     38h
0f03 ed730301  ld      (0103h),sp
0f07 dcc0d6    call    c,0d6c0h
0f0a d1        pop     de
0f0b 5a        ld      e,d
0f0c ed56      im      1
0f0e f3        di      
0f0f daafa7    jp      c,0a7afh
0f12 6c        ld      l,h
0f13 44        ld      b,h
0f14 9f        sbc     a,a
0f15 0a        ld      a,(bc)
0f16 3f        ccf     
0f17 00        nop     
0f18 00        nop     
0f19 00        nop     
0f1a 00        nop     
0f1b 00        nop     
0f1c 00        nop     
0f1d 00        nop     
0f1e 00        nop     
0f1f 00        nop     
0f20 00        nop     
0f21 00        nop     
0f22 00        nop     
0f23 00        nop     
0f24 00        nop     
0f25 00        nop     
0f26 00        nop     
0f27 00        nop     
0f28 00        nop     
0f29 00        nop     
0f2a 00        nop     
0f2b 00        nop     
0f2c 00        nop     
0f2d 00        nop     
0f2e 00        nop     
0f2f 00        nop     
0f30 00        nop     
0f31 00        nop     
0f32 00        nop     
0f33 00        nop     
0f34 00        nop     
0f35 00        nop     
0f36 00        nop     
0f37 00        nop     
0f38 00        nop     
0f39 00        nop     
0f3a 00        nop     
0f3b 00        nop     
0f3c 00        nop     
0f3d ff        rst     38h
0f3e ff        rst     38h
0f3f 1658      ld      d,58h
0f41 5f        ld      e,a
0f42 d7        rst     10h
0f43 6c        ld      l,h
0f44 64        ld      h,h
0f45 2028      jr      nz,0f6fh
0f47 6e        ld      l,(hl)
0f48 6e        ld      l,(hl)
0f49 6e        ld      l,(hl)
0f4a 6e        ld      l,(hl)
0f4b 29        add     hl,hl
0f4c 2c        inc     l
0f4d 73        ld      (hl),e
0f4e 70        ld      (hl),b
0f4f 2e2e      ld      l,2eh
0f51 2e2e      ld      l,2eh
0f53 2e2e      ld      l,2eh
0f55 2e2e      ld      l,2eh
0f57 2e2e      ld      l,2eh
0f59 2e2e      ld      l,2eh
0f5b 2e2e      ld      l,2eh
0f5d 2e2e      ld      l,2eh
0f5f 2e2e      ld      l,2eh
0f61 24        inc     h
0f62 ff        rst     38h
0f63 dd220301  ld      (0103h),ix
0f67 c36c91    jp      916ch
0f6a 0d        dec     c
0f6b 00        nop     
0f6c 69        ld      l,c
0f6d f8        ret     m

0f6e 8e        adc     a,(hl)
0f6f d6e3      sub     0e3h
0f71 f7        rst     30h
0f72 c3c6d9    jp      0d9c6h
0f75 df        rst     18h
0f76 c22000    jp      nz,0020h
0f79 00        nop     
0f7a 00        nop     
0f7b 00        nop     
0f7c 00        nop     
0f7d 00        nop     
0f7e 00        nop     
0f7f 00        nop     
0f80 00        nop     
0f81 00        nop     
0f82 00        nop     
0f83 00        nop     
0f84 00        nop     
0f85 00        nop     
0f86 00        nop     
0f87 00        nop     
0f88 00        nop     
0f89 00        nop     
0f8a 00        nop     
0f8b 00        nop     
0f8c 00        nop     
0f8d 00        nop     
0f8e 00        nop     
0f8f 00        nop     
0f90 00        nop     
0f91 ff        rst     38h
0f92 ff        rst     38h
0f93 ff        rst     38h
0f94 ff        rst     38h
0f95 00        nop     
0f96 00        nop     
0f97 00        nop     
0f98 00        nop     
0f99 00        nop     
0f9a 00        nop     
0f9b 00        nop     
0f9c 00        nop     
0f9d 00        nop     
0f9e 00        nop     
0f9f ba        cp      d
0fa0 102a      djnz    0fcch
0fa2 6b        ld      l,e
0fa3 6c        ld      l,h
0fa4 64        ld      h,h
0fa5 2028      jr      nz,0fcfh
0fa7 6e        ld      l,(hl)
0fa8 6e        ld      l,(hl)
0fa9 6e        ld      l,(hl)
0faa 6e        ld      l,(hl)
0fab 29        add     hl,hl
0fac 2c        inc     l
0fad 3c        inc     a
0fae 69        ld      l,c
0faf 78        ld      a,b
0fb0 2c        inc     l
0fb1 69        ld      l,c
0fb2 79        ld      a,c
0fb3 3e2e      ld      a,2eh
0fb5 2e2e      ld      l,2eh
0fb7 2e2e      ld      l,2eh
0fb9 2e2e      ld      l,2eh
0fbb 2e2e      ld      l,2eh
0fbd 2e2e      ld      l,2eh
0fbf 2e2e      ld      l,2eh
0fc1 24        inc     h
0fc2 ff        rst     38h
0fc3 010000    ld      bc,0000h
0fc6 00        nop     
0fc7 1c        inc     e
0fc8 5c        ld      e,h
0fc9 46        ld      b,(hl)
0fca 2d        dec     l
0fcb b9        cp      c
0fcc 8e        adc     a,(hl)
0fcd 78        ld      a,b
0fce 60        ld      h,b
0fcf b1        or      c
0fd0 74        ld      (hl),h
0fd1 0eb3      ld      c,0b3h
0fd3 46        ld      b,(hl)
0fd4 d1        pop     de
0fd5 cc3030    call    z,3030h
0fd8 00        nop     
0fd9 00        nop     
0fda 00        nop     
0fdb 00        nop     
0fdc 00        nop     
0fdd 00        nop     
0fde 00        nop     
0fdf 00        nop     
0fe0 00        nop     
0fe1 00        nop     
0fe2 00        nop     
0fe3 00        nop     
0fe4 00        nop     
0fe5 00        nop     
0fe6 00        nop     
0fe7 00        nop     
0fe8 00        nop     
0fe9 00        nop     
0fea 00        nop     
0feb 00        nop     
0fec ff        rst     38h
0fed ff        rst     38h
0fee 00        nop     
0fef 00        nop     
0ff0 00        nop     
0ff1 00        nop     
0ff2 00        nop     
0ff3 00        nop     
0ff4 00        nop     
0ff5 00        nop     
0ff6 00        nop     
0ff7 00        nop     
0ff8 00        nop     
0ff9 00        nop     
0ffa 00        nop     
0ffb 00        nop     
0ffc 00        nop     
0ffd 00        nop     
0ffe 00        nop     
0fff de39      sbc     a,39h
1001 19        add     hl,de
1002 69        ld      l,c
1003 6c        ld      l,h
1004 64        ld      h,h
1005 203c      jr      nz,1043h
1007 62        ld      h,d
1008 63        ld      h,e
1009 2c        inc     l
100a 64        ld      h,h
100b 65        ld      h,l
100c 2c        inc     l
100d 68        ld      l,b
100e 6c        ld      l,h
100f 2c        inc     l
1010 73        ld      (hl),e
1011 70        ld      (hl),b
1012 3e2c      ld      a,2ch
1014 6e        ld      l,(hl)
1015 6e        ld      l,(hl)
1016 6e        ld      l,(hl)
1017 6e        ld      l,(hl)
1018 2e2e      ld      l,2eh
101a 2e2e      ld      l,2eh
101c 2e2e      ld      l,2eh
101e 2e2e      ld      l,2eh
1020 2e24      ld      l,24h
1022 ff        rst     38h
1023 dd210000  ld      ix,0000h
1027 e8        ret     pe

1028 87        add     a,a
1029 0620      ld      b,20h
102b 12        ld      (de),a
102c bd        cp      l
102d 9b        sbc     a,e
102e b6        or      (hl)
102f 53        ld      d,e
1030 72        ld      (hl),d
1031 e5        push    hl
1032 a1        and     c
1033 51        ld      d,c
1034 13        inc     de
1035 bd        cp      l
1036 f1        pop     af
1037 2000      jr      nz,1039h
1039 00        nop     
103a 00        nop     
103b 00        nop     
103c 00        nop     
103d 00        nop     
103e 00        nop     
103f 00        nop     
1040 00        nop     
1041 00        nop     
1042 00        nop     
1043 00        nop     
1044 00        nop     
1045 00        nop     
1046 00        nop     
1047 00        nop     
1048 00        nop     
1049 00        nop     
104a 00        nop     
104b 00        nop     
104c 00        nop     
104d ff        rst     38h
104e ff        rst     38h
104f 00        nop     
1050 00        nop     
1051 00        nop     
1052 00        nop     
1053 00        nop     
1054 00        nop     
1055 00        nop     
1056 00        nop     
1057 00        nop     
1058 00        nop     
1059 00        nop     
105a 00        nop     
105b 00        nop     
105c 00        nop     
105d 00        nop     
105e 00        nop     
105f 227dd5    ld      (0d57dh),hl
1062 25        dec     h
1063 6c        ld      l,h
1064 64        ld      h,h
1065 203c      jr      nz,10a3h
1067 69        ld      l,c
1068 78        ld      a,b
1069 2c        inc     l
106a 69        ld      l,c
106b 79        ld      a,c
106c 3e2c      ld      a,2ch
106e 6e        ld      l,(hl)
106f 6e        ld      l,(hl)
1070 6e        ld      l,(hl)
1071 6e        ld      l,(hl)
1072 2e2e      ld      l,2eh
1074 2e2e      ld      l,2eh
1076 2e2e      ld      l,2eh
1078 2e2e      ld      l,2eh
107a 2e2e      ld      l,2eh
107c 2e2e      ld      l,2eh
107e 2e2e      ld      l,2eh
1080 2e24      ld      l,24h
1082 ff        rst     38h
1083 0a        ld      a,(bc)
1084 00        nop     
1085 00        nop     
1086 00        nop     
1087 a8        xor     b
1088 b3        or      e
1089 2a1d8e    ld      hl,(8e1dh)
108c 7f        ld      a,a
108d ac        xor     h
108e 42        ld      b,d
108f 03        inc     bc
1090 010301    ld      bc,0103h
1093 c6b1      add     a,0b1h
1095 8e        adc     a,(hl)
1096 ef        rst     28h
1097 1000      djnz    1099h
1099 00        nop     
109a 00        nop     
109b 00        nop     
109c 00        nop     
109d 00        nop     
109e 00        nop     
109f 00        nop     
10a0 00        nop     
10a1 00        nop     
10a2 00        nop     
10a3 00        nop     
10a4 00        nop     
10a5 00        nop     
10a6 00        nop     
10a7 00        nop     
10a8 00        nop     
10a9 00        nop     
10aa 00        nop     
10ab 00        nop     
10ac 00        nop     
10ad 00        nop     
10ae 00        nop     
10af ff        rst     38h
10b0 00        nop     
10b1 00        nop     
10b2 00        nop     
10b3 00        nop     
10b4 00        nop     
10b5 00        nop     
10b6 00        nop     
10b7 00        nop     
10b8 00        nop     
10b9 00        nop     
10ba 00        nop     
10bb d7        rst     10h
10bc ff        rst     38h
10bd 00        nop     
10be 00        nop     
10bf b0        or      b
10c0 81        add     a,c
10c1 89        adc     a,c
10c2 35        dec     (hl)
10c3 6c        ld      l,h
10c4 64        ld      h,h
10c5 2061      jr      nz,1128h
10c7 2c        inc     l
10c8 3c        inc     a
10c9 2862      jr      z,112dh
10cb 63        ld      h,e
10cc 29        add     hl,hl
10cd 2c        inc     l
10ce 2864      jr      z,1134h
10d0 65        ld      h,l
10d1 29        add     hl,hl
10d2 3e2e      ld      a,2eh
10d4 2e2e      ld      l,2eh
10d6 2e2e      ld      l,2eh
10d8 2e2e      ld      l,2eh
10da 2e2e      ld      l,2eh
10dc 2e2e      ld      l,2eh
10de 2e2e      ld      l,2eh
10e0 2e24      ld      l,24h
10e2 ff        rst     38h
10e3 0600      ld      b,00h
10e5 00        nop     
10e6 00        nop     
10e7 07        rlca    
10e8 c49df4    call    nz,0f49dh
10eb 3d        dec     a
10ec d1        pop     de
10ed 39        add     hl,sp
10ee 03        inc     bc
10ef 89        adc     a,c
10f0 de55      sbc     a,55h
10f2 74        ld      (hl),h
10f3 53        ld      d,e
10f4 c0        ret     nz

10f5 09        add     hl,bc
10f6 55        ld      d,l
10f7 3800      jr      c,10f9h
10f9 00        nop     
10fa 00        nop     
10fb 00        nop     
10fc 00        nop     
10fd 00        nop     
10fe 00        nop     
10ff 00        nop     
1100 00        nop     
1101 00        nop     
1102 00        nop     
1103 00        nop     
1104 00        nop     
1105 00        nop     
1106 00        nop     
1107 00        nop     
1108 00        nop     
1109 00        nop     
110a 00        nop     
110b 00        nop     
110c 00        nop     
110d 00        nop     
110e 00        nop     
110f 00        nop     
1110 00        nop     
1111 00        nop     
1112 00        nop     
1113 00        nop     
1114 00        nop     
1115 00        nop     
1116 00        nop     
1117 00        nop     
1118 00        nop     
1119 00        nop     
111a 00        nop     
111b 00        nop     
111c ff        rst     38h
111d 00        nop     
111e 00        nop     
111f f1        pop     af
1120 dab556    jp      c,56b5h
1123 6c        ld      l,h
1124 64        ld      h,h
1125 203c      jr      nz,1163h
1127 62        ld      h,d
1128 2c        inc     l
1129 63        ld      h,e
112a 2c        inc     l
112b 64        ld      h,h
112c 2c        inc     l
112d 65        ld      h,l
112e 2c        inc     l
112f 68        ld      l,b
1130 2c        inc     l
1131 6c        ld      l,h
1132 2c        inc     l
1133 2868      jr      z,119dh
1135 6c        ld      l,h
1136 29        add     hl,hl
1137 2c        inc     l
1138 61        ld      h,c
1139 3e2c      ld      a,2ch
113b 6e        ld      l,(hl)
113c 6e        ld      l,(hl)
113d 2e2e      ld      l,2eh
113f 2e2e      ld      l,2eh
1141 24        inc     h
1142 ff        rst     38h
1143 dd360100  ld      (ix+01h),00h
1147 45        ld      b,l
1148 1b        dec     de
1149 02        ld      (bc),a
114a 010201    ld      bc,0102h
114d c1        pop     bc
114e d5        push    de
114f c7        rst     00h
1150 61        ld      h,c
1151 c4bdc0    call    nz,0c0bdh
1154 85        add     a,l
1155 16cd      ld      d,0cdh
1157 2000      jr      nz,1159h
1159 00        nop     
115a 00        nop     
115b 00        nop     
115c 00        nop     
115d 00        nop     
115e 00        nop     
115f 00        nop     
1160 00        nop     
1161 00        nop     
1162 00        nop     
1163 00        nop     
1164 00        nop     
1165 00        nop     
1166 00        nop     
1167 00        nop     
1168 00        nop     
1169 00        nop     
116a 00        nop     
116b 00        nop     
116c 00        nop     
116d 00        nop     
116e ff        rst     38h
116f 00        nop     
1170 00        nop     
1171 00        nop     
1172 00        nop     
1173 00        nop     
1174 00        nop     
1175 00        nop     
1176 00        nop     
1177 00        nop     
1178 00        nop     
1179 00        nop     
117a 00        nop     
117b 00        nop     
117c ff        rst     38h
117d 00        nop     
117e 00        nop     
117f 26db      ld      h,0dbh
1181 47        ld      b,a
1182 7e        ld      a,(hl)
1183 6c        ld      l,h
1184 64        ld      h,h
1185 2028      jr      nz,11afh
1187 3c        inc     a
1188 69        ld      l,c
1189 78        ld      a,b
118a 2c        inc     l
118b 69        ld      l,c
118c 79        ld      a,c
118d 3e2b      ld      a,2bh
118f 31292c    ld      sp,2c29h
1192 6e        ld      l,(hl)
1193 6e        ld      l,(hl)
1194 2e2e      ld      l,2eh
1196 2e2e      ld      l,2eh
1198 2e2e      ld      l,2eh
119a 2e2e      ld      l,2eh
119c 2e2e      ld      l,2eh
119e 2e2e      ld      l,2eh
11a0 2e24      ld      l,24h
11a2 ff        rst     38h
11a3 dd4601    ld      b,(ix+01h)
11a6 00        nop     
11a7 16d0      ld      d,0d0h
11a9 02        ld      (bc),a
11aa 010201    ld      bc,0102h
11ad 60        ld      h,b
11ae 42        ld      b,d
11af 39        add     hl,sp
11b0 7f        ld      a,a
11b1 04        inc     b
11b2 04        inc     b
11b3 97        sub     a
11b4 4a        ld      c,d
11b5 85        add     a,l
11b6 d0        ret     nc

11b7 2018      jr      nz,11d1h
11b9 00        nop     
11ba 00        nop     
11bb 00        nop     
11bc 00        nop     
11bd 010001    ld      bc,0100h
11c0 00        nop     
11c1 00        nop     
11c2 00        nop     
11c3 00        nop     
11c4 00        nop     
11c5 00        nop     
11c6 00        nop     
11c7 00        nop     
11c8 00        nop     
11c9 00        nop     
11ca 00        nop     
11cb 00        nop     
11cc 00        nop     
11cd 00        nop     
11ce 00        nop     
11cf ff        rst     38h
11d0 ff        rst     38h
11d1 00        nop     
11d2 00        nop     
11d3 00        nop     
11d4 00        nop     
11d5 00        nop     
11d6 00        nop     
11d7 00        nop     
11d8 00        nop     
11d9 00        nop     
11da 00        nop     
11db 00        nop     
11dc 00        nop     
11dd 00        nop     
11de 00        nop     
11df cc1106    call    z,0611h
11e2 a8        xor     b
11e3 6c        ld      l,h
11e4 64        ld      h,h
11e5 203c      jr      nz,1223h
11e7 62        ld      h,d
11e8 2c        inc     l
11e9 63        ld      h,e
11ea 2c        inc     l
11eb 64        ld      h,h
11ec 2c        inc     l
11ed 65        ld      h,l
11ee 3e2c      ld      a,2ch
11f0 283c      jr      z,122eh
11f2 69        ld      l,c
11f3 78        ld      a,b
11f4 2c        inc     l
11f5 69        ld      l,c
11f6 79        ld      a,c
11f7 3e2b      ld      a,2bh
11f9 31292e    ld      sp,2e29h
11fc 2e2e      ld      l,2eh
11fe 2e2e      ld      l,2eh
1200 2e24      ld      l,24h
1202 ff        rst     38h
1203 dd6601    ld      h,(ix+01h)
1206 00        nop     
1207 e0        ret     po

1208 84        add     a,h
1209 02        ld      (bc),a
120a 010201    ld      bc,0102h
120d 52        ld      d,d
120e 9c        sbc     a,h
120f 99        sbc     a,c
1210 a7        and     a
1211 b6        or      (hl)
1212 49        ld      c,c
1213 93        sub     e
1214 00        nop     
1215 ad        xor     l
1216 ee20      xor     20h
1218 08        ex      af,af'
1219 00        nop     
121a 00        nop     
121b 00        nop     
121c 00        nop     
121d 010001    ld      bc,0100h
1220 00        nop     
1221 00        nop     
1222 00        nop     
1223 00        nop     
1224 00        nop     
1225 00        nop     
1226 00        nop     
1227 00        nop     
1228 00        nop     
1229 00        nop     
122a 00        nop     
122b 00        nop     
122c 00        nop     
122d 00        nop     
122e 00        nop     
122f ff        rst     38h
1230 ff        rst     38h
1231 00        nop     
1232 00        nop     
1233 00        nop     
1234 00        nop     
1235 00        nop     
1236 00        nop     
1237 00        nop     
1238 00        nop     
1239 00        nop     
123a 00        nop     
123b 00        nop     
123c 00        nop     
123d 00        nop     
123e 00        nop     
123f fa2a4d    jp      m,4d2ah
1242 03        inc     bc
1243 6c        ld      l,h
1244 64        ld      h,h
1245 203c      jr      nz,1283h
1247 68        ld      l,b
1248 2c        inc     l
1249 6c        ld      l,h
124a 3e2c      ld      a,2ch
124c 283c      jr      z,128ah
124e 69        ld      l,c
124f 78        ld      a,b
1250 2c        inc     l
1251 69        ld      l,c
1252 79        ld      a,c
1253 3e2b      ld      a,2bh
1255 31292e    ld      sp,2e29h
1258 2e2e      ld      l,2eh
125a 2e2e      ld      l,2eh
125c 2e2e      ld      l,2eh
125e 2e2e      ld      l,2eh
1260 2e24      ld      l,24h
1262 ff        rst     38h
1263 dd7e01    ld      a,(ix+01h)
1266 00        nop     
1267 b6        or      (hl)
1268 d8        ret     c

1269 02        ld      (bc),a
126a 010201    ld      bc,0102h
126d 12        ld      (de),a
126e c607      add     a,07h
1270 df        rst     18h
1271 d0        ret     nc

1272 9c        sbc     a,h
1273 43        ld      b,e
1274 a6        and     (hl)
1275 e5        push    hl
1276 a0        and     b
1277 2000      jr      nz,1279h
1279 00        nop     
127a 00        nop     
127b 00        nop     
127c 00        nop     
127d 010001    ld      bc,0100h
1280 00        nop     
1281 00        nop     
1282 00        nop     
1283 00        nop     
1284 00        nop     
1285 00        nop     
1286 00        nop     
1287 00        nop     
1288 00        nop     
1289 00        nop     
128a 00        nop     
128b 00        nop     
128c 00        nop     
128d 00        nop     
128e 00        nop     
128f ff        rst     38h
1290 ff        rst     38h
1291 00        nop     
1292 00        nop     
1293 00        nop     
1294 00        nop     
1295 00        nop     
1296 00        nop     
1297 00        nop     
1298 00        nop     
1299 00        nop     
129a 00        nop     
129b 00        nop     
129c 00        nop     
129d 00        nop     
129e 00        nop     
129f a5        and     l
12a0 e9        jp      (hl)
12a1 ac        xor     h
12a2 64        ld      h,h
12a3 6c        ld      l,h
12a4 64        ld      h,h
12a5 2061      jr      nz,1308h
12a7 2c        inc     l
12a8 283c      jr      z,12e6h
12aa 69        ld      l,c
12ab 78        ld      a,b
12ac 2c        inc     l
12ad 69        ld      l,c
12ae 79        ld      a,c
12af 3e2b      ld      a,2bh
12b1 31292e    ld      sp,2e29h
12b4 2e2e      ld      l,2eh
12b6 2e2e      ld      l,2eh
12b8 2e2e      ld      l,2eh
12ba 2e2e      ld      l,2eh
12bc 2e2e      ld      l,2eh
12be 2e2e      ld      l,2eh
12c0 2e24      ld      l,24h
12c2 ff        rst     38h
12c3 dd2600    ld      ixh,00h
12c6 00        nop     
12c7 53        ld      d,e
12c8 3c        inc     a
12c9 40        ld      b,b
12ca 46        ld      b,(hl)
12cb 79        ld      a,c
12cc e1        pop     hl
12cd 117707    ld      de,0777h
12d0 c1        pop     bc
12d1 fa1a81    jp      m,811ah
12d4 ad        xor     l
12d5 9b        sbc     a,e
12d6 5d        ld      e,l
12d7 2008      jr      nz,12e1h
12d9 00        nop     
12da 00        nop     
12db 00        nop     
12dc 00        nop     
12dd 00        nop     
12de 00        nop     
12df 00        nop     
12e0 00        nop     
12e1 00        nop     
12e2 00        nop     
12e3 00        nop     
12e4 00        nop     
12e5 00        nop     
12e6 00        nop     
12e7 00        nop     
12e8 00        nop     
12e9 00        nop     
12ea 00        nop     
12eb 00        nop     
12ec 00        nop     
12ed 00        nop     
12ee 00        nop     
12ef 00        nop     
12f0 00        nop     
12f1 00        nop     
12f2 00        nop     
12f3 00        nop     
12f4 00        nop     
12f5 00        nop     
12f6 00        nop     
12f7 00        nop     
12f8 00        nop     
12f9 00        nop     
12fa 00        nop     
12fb 00        nop     
12fc ff        rst     38h
12fd 00        nop     
12fe 00        nop     
12ff 24        inc     h
1300 e8        ret     pe

1301 82        add     a,d
1302 8b        adc     a,e
1303 6c        ld      l,h
1304 64        ld      h,h
1305 203c      jr      nz,1343h
1307 69        ld      l,c
1308 78        ld      a,b
1309 68        ld      l,b
130a 2c        inc     l
130b 69        ld      l,c
130c 78        ld      a,b
130d 6c        ld      l,h
130e 2c        inc     l
130f 69        ld      l,c
1310 79        ld      a,c
1311 68        ld      l,b
1312 2c        inc     l
1313 69        ld      l,c
1314 79        ld      a,c
1315 6c        ld      l,h
1316 3e2c      ld      a,2ch
1318 6e        ld      l,(hl)
1319 6e        ld      l,(hl)
131a 2e2e      ld      l,2eh
131c 2e2e      ld      l,2eh
131e 2e2e      ld      l,2eh
1320 2e24      ld      l,24h
1322 ff        rst     38h
1323 40        ld      b,b
1324 00        nop     
1325 00        nop     
1326 00        nop     
1327 a4        and     h
1328 72        ld      (hl),d
1329 24        inc     h
132a a0        and     b
132b ac        xor     h
132c 61        ld      h,c
132d 03        inc     bc
132e 01c782    ld      bc,82c7h
1331 8f        adc     a,a
1332 71        ld      (hl),c
1333 97        sub     a
1334 8f        adc     a,a
1335 8e        adc     a,(hl)
1336 ef        rst     28h
1337 3f        ccf     
1338 00        nop     
1339 00        nop     
133a 00        nop     
133b 00        nop     
133c 00        nop     
133d 00        nop     
133e 00        nop     
133f 00        nop     
1340 00        nop     
1341 00        nop     
1342 00        nop     
1343 00        nop     
1344 00        nop     
1345 00        nop     
1346 00        nop     
1347 00        nop     
1348 00        nop     
1349 00        nop     
134a 00        nop     
134b 00        nop     
134c 00        nop     
134d 00        nop     
134e 00        nop     
134f ff        rst     38h
1350 00        nop     
1351 00        nop     
1352 00        nop     
1353 00        nop     
1354 00        nop     
1355 00        nop     
1356 00        nop     
1357 ff        rst     38h
1358 ff        rst     38h
1359 ff        rst     38h
135a ff        rst     38h
135b d7        rst     10h
135c ff        rst     38h
135d 00        nop     
135e 00        nop     
135f 74        ld      (hl),h
1360 4b        ld      c,e
1361 01186c    ld      bc,6c18h
1364 64        ld      h,h
1365 203c      jr      nz,13a3h
1367 62        ld      h,d
1368 63        ld      h,e
1369 64        ld      h,h
136a 65        ld      h,l
136b 68        ld      l,b
136c 6c        ld      l,h
136d 61        ld      h,c
136e 3e2c      ld      a,2ch
1370 3c        inc     a
1371 62        ld      h,d
1372 63        ld      h,e
1373 64        ld      h,h
1374 65        ld      h,l
1375 68        ld      l,b
1376 6c        ld      l,h
1377 61        ld      h,c
1378 3e2e      ld      a,2eh
137a 2e2e      ld      l,2eh
137c 2e2e      ld      l,2eh
137e 2e2e      ld      l,2eh
1380 2e24      ld      l,24h
1382 ff        rst     38h
1383 dd40      ld      b,b
1385 00        nop     
1386 00        nop     
1387 c5        push    bc
1388 bc        cp      h
1389 03        inc     bc
138a 010301    ld      bc,0103h
138d 03        inc     bc
138e 01c22f    ld      bc,2fc2h
1391 c0        ret     nz

1392 98        sbc     a,b
1393 83        add     a,e
1394 1f        rra     
1395 cd3b20    call    203bh
1398 3f        ccf     
1399 00        nop     
139a 00        nop     
139b 00        nop     
139c 00        nop     
139d 00        nop     
139e 00        nop     
139f 00        nop     
13a0 00        nop     
13a1 00        nop     
13a2 00        nop     
13a3 00        nop     
13a4 00        nop     
13a5 00        nop     
13a6 00        nop     
13a7 00        nop     
13a8 00        nop     
13a9 00        nop     
13aa 00        nop     
13ab 00        nop     
13ac 00        nop     
13ad 00        nop     
13ae 00        nop     
13af ff        rst     38h
13b0 00        nop     
13b1 00        nop     
13b2 00        nop     
13b3 00        nop     
13b4 00        nop     
13b5 00        nop     
13b6 00        nop     
13b7 ff        rst     38h
13b8 ff        rst     38h
13b9 ff        rst     38h
13ba ff        rst     38h
13bb d7        rst     10h
13bc ff        rst     38h
13bd 00        nop     
13be 00        nop     
13bf 47        ld      b,a
13c0 8b        adc     a,e
13c1 a3        and     e
13c2 6b        ld      l,e
13c3 6c        ld      l,h
13c4 64        ld      h,h
13c5 203c      jr      nz,1403h
13c7 62        ld      h,d
13c8 63        ld      h,e
13c9 64        ld      h,h
13ca 65        ld      h,l
13cb 78        ld      a,b
13cc 79        ld      a,c
13cd 61        ld      h,c
13ce 3e2c      ld      a,2ch
13d0 3c        inc     a
13d1 62        ld      h,d
13d2 63        ld      h,e
13d3 64        ld      h,h
13d4 65        ld      h,l
13d5 78        ld      a,b
13d6 79        ld      a,c
13d7 61        ld      h,c
13d8 3e2e      ld      a,2eh
13da 2e2e      ld      l,2eh
13dc 2e2e      ld      l,2eh
13de 2e2e      ld      l,2eh
13e0 2e24      ld      l,24h
13e2 ff        rst     38h
13e3 320301    ld      (0103h),a
13e6 00        nop     
13e7 68        ld      l,b
13e8 fdecf4a0  call    pe,0a0f4h
13ec 44        ld      b,h
13ed 43        ld      b,e
13ee b5        or      l
13ef 53        ld      d,e
13f0 06ba      ld      b,0bah
13f2 cdd24f    call    4fd2h
13f5 d8        ret     c

13f6 1f        rra     
13f7 08        ex      af,af'
13f8 00        nop     
13f9 00        nop     
13fa 00        nop     
13fb 00        nop     
13fc 00        nop     
13fd 00        nop     
13fe 00        nop     
13ff 00        nop     
1400 00        nop     
1401 00        nop     
1402 00        nop     
1403 00        nop     
1404 00        nop     
1405 00        nop     
1406 00        nop     
1407 00        nop     
1408 00        nop     
1409 00        nop     
140a 00        nop     
140b 00        nop     
140c 00        nop     
140d 00        nop     
140e 00        nop     
140f ff        rst     38h
1410 00        nop     
1411 00        nop     
1412 00        nop     
1413 00        nop     
1414 00        nop     
1415 00        nop     
1416 00        nop     
1417 00        nop     
1418 00        nop     
1419 00        nop     
141a 00        nop     
141b d7        rst     10h
141c ff        rst     38h
141d 00        nop     
141e 00        nop     
141f c9        ret     

1420 262d      ld      h,2dh
1422 e5        push    hl
1423 6c        ld      l,h
1424 64        ld      h,h
1425 2061      jr      nz,1488h
1427 2c        inc     l
1428 286e      jr      z,1498h
142a 6e        ld      l,(hl)
142b 6e        ld      l,(hl)
142c 6e        ld      l,(hl)
142d 29        add     hl,hl
142e 202f      jr      nz,145fh
1430 206c      jr      nz,149eh
1432 64        ld      h,h
1433 2028      jr      nz,145dh
1435 6e        ld      l,(hl)
1436 6e        ld      l,(hl)
1437 6e        ld      l,(hl)
1438 6e        ld      l,(hl)
1439 29        add     hl,hl
143a 2c        inc     l
143b 61        ld      h,c
143c 2e2e      ld      l,2eh
143e 2e2e      ld      l,2eh
1440 2e24      ld      l,24h
1442 ff        rst     38h
1443 eda8      ldd     
1445 00        nop     
1446 00        nop     
1447 52        ld      d,d
1448 98        sbc     a,b
1449 fa68a1    jp      m,0a168h
144c 66        ld      h,(hl)
144d 0601      ld      b,01h
144f 04        inc     b
1450 010100    ld      bc,0001h
1453 c1        pop     bc
1454 68        ld      l,b
1455 b7        or      a
1456 2000      jr      nz,1458h
1458 1000      djnz    145ah
145a 00        nop     
145b 00        nop     
145c 00        nop     
145d 00        nop     
145e 00        nop     
145f 00        nop     
1460 00        nop     
1461 00        nop     
1462 00        nop     
1463 00        nop     
1464 00        nop     
1465 00        nop     
1466 00        nop     
1467 00        nop     
1468 00        nop     
1469 00        nop     
146a 00        nop     
146b 00        nop     
146c 00        nop     
146d 00        nop     
146e 00        nop     
146f ff        rst     38h
1470 ff        rst     38h
1471 00        nop     
1472 00        nop     
1473 00        nop     
1474 00        nop     
1475 00        nop     
1476 00        nop     
1477 00        nop     
1478 00        nop     
1479 00        nop     
147a 00        nop     
147b d7        rst     10h
147c 00        nop     
147d 00        nop     
147e 00        nop     
147f 94        sub     h
1480 f42769    call    p,6927h
1483 6c        ld      l,h
1484 64        ld      h,h
1485 64        ld      h,h
1486 3c        inc     a
1487 72        ld      (hl),d
1488 3e20      ld      a,20h
148a 2831      jr      z,14bdh
148c 29        add     hl,hl
148d 2e2e      ld      l,2eh
148f 2e2e      ld      l,2eh
1491 2e2e      ld      l,2eh
1493 2e2e      ld      l,2eh
1495 2e2e      ld      l,2eh
1497 2e2e      ld      l,2eh
1499 2e2e      ld      l,2eh
149b 2e2e      ld      l,2eh
149d 2e2e      ld      l,2eh
149f 2e2e      ld      l,2eh
14a1 24        inc     h
14a2 ff        rst     38h
14a3 eda8      ldd     
14a5 00        nop     
14a6 00        nop     
14a7 2ef1      ld      l,0f1h
14a9 2aebba    ld      hl,(0baebh)
14ac d5        push    de
14ad 0601      ld      b,01h
14af 04        inc     b
14b0 010200    ld      bc,0002h
14b3 47        ld      b,a
14b4 ff        rst     38h
14b5 e4fb00    call    po,00fbh
14b8 1000      djnz    14bah
14ba 00        nop     
14bb 00        nop     
14bc 00        nop     
14bd 00        nop     
14be 00        nop     
14bf 00        nop     
14c0 00        nop     
14c1 00        nop     
14c2 00        nop     
14c3 00        nop     
14c4 00        nop     
14c5 00        nop     
14c6 00        nop     
14c7 00        nop     
14c8 00        nop     
14c9 00        nop     
14ca 00        nop     
14cb 00        nop     
14cc 00        nop     
14cd 00        nop     
14ce 00        nop     
14cf ff        rst     38h
14d0 ff        rst     38h
14d1 00        nop     
14d2 00        nop     
14d3 00        nop     
14d4 00        nop     
14d5 00        nop     
14d6 00        nop     
14d7 00        nop     
14d8 00        nop     
14d9 00        nop     
14da 00        nop     
14db d7        rst     10h
14dc 00        nop     
14dd 00        nop     
14de 00        nop     
14df 39        add     hl,sp
14e0 dd3d      dec     a
14e2 e1        pop     hl
14e3 6c        ld      l,h
14e4 64        ld      h,h
14e5 64        ld      h,h
14e6 3c        inc     a
14e7 72        ld      (hl),d
14e8 3e20      ld      a,20h
14ea 2832      jr      z,151eh
14ec 29        add     hl,hl
14ed 2e2e      ld      l,2eh
14ef 2e2e      ld      l,2eh
14f1 2e2e      ld      l,2eh
14f3 2e2e      ld      l,2eh
14f5 2e2e      ld      l,2eh
14f7 2e2e      ld      l,2eh
14f9 2e2e      ld      l,2eh
14fb 2e2e      ld      l,2eh
14fd 2e2e      ld      l,2eh
14ff 2e2e      ld      l,2eh
1501 24        inc     h
1502 ff        rst     38h
1503 eda0      ldi     
1505 00        nop     
1506 00        nop     
1507 30fe      jr      nc,1507h
1509 cd0358    call    5803h
150c 60        ld      h,b
150d 05        dec     b
150e 010301    ld      bc,0103h
1511 010004    ld      bc,0400h
1514 60        ld      h,b
1515 88        adc     a,b
1516 2600      ld      h,00h
1518 1000      djnz    151ah
151a 00        nop     
151b 00        nop     
151c 00        nop     
151d 00        nop     
151e 00        nop     
151f 00        nop     
1520 00        nop     
1521 00        nop     
1522 00        nop     
1523 00        nop     
1524 00        nop     
1525 00        nop     
1526 00        nop     
1527 00        nop     
1528 00        nop     
1529 00        nop     
152a 00        nop     
152b 00        nop     
152c 00        nop     
152d 00        nop     
152e 00        nop     
152f ff        rst     38h
1530 ff        rst     38h
1531 00        nop     
1532 00        nop     
1533 00        nop     
1534 00        nop     
1535 00        nop     
1536 00        nop     
1537 00        nop     
1538 00        nop     
1539 00        nop     
153a 00        nop     
153b d7        rst     10h
153c 00        nop     
153d 00        nop     
153e 00        nop     
153f f7        rst     30h
1540 82        add     a,d
1541 b0        or      b
1542 d1        pop     de
1543 6c        ld      l,h
1544 64        ld      h,h
1545 69        ld      l,c
1546 3c        inc     a
1547 72        ld      (hl),d
1548 3e20      ld      a,20h
154a 2831      jr      z,157dh
154c 29        add     hl,hl
154d 2e2e      ld      l,2eh
154f 2e2e      ld      l,2eh
1551 2e2e      ld      l,2eh
1553 2e2e      ld      l,2eh
1555 2e2e      ld      l,2eh
1557 2e2e      ld      l,2eh
1559 2e2e      ld      l,2eh
155b 2e2e      ld      l,2eh
155d 2e2e      ld      l,2eh
155f 2e2e      ld      l,2eh
1561 24        inc     h
1562 ff        rst     38h
1563 eda0      ldi     
1565 00        nop     
1566 00        nop     
1567 ce4a      adc     a,4ah
1569 6e        ld      l,(hl)
156a c288b1    jp      nz,0b188h
156d 05        dec     b
156e 010301    ld      bc,0103h
1571 02        ld      (bc),a
1572 00        nop     
1573 14        inc     d
1574 2d        dec     l
1575 9f        sbc     a,a
1576 a3        and     e
1577 00        nop     
1578 1000      djnz    157ah
157a 00        nop     
157b 00        nop     
157c 00        nop     
157d 00        nop     
157e 00        nop     
157f 00        nop     
1580 00        nop     
1581 00        nop     
1582 00        nop     
1583 00        nop     
1584 00        nop     
1585 00        nop     
1586 00        nop     
1587 00        nop     
1588 00        nop     
1589 00        nop     
158a 00        nop     
158b 00        nop     
158c 00        nop     
158d 00        nop     
158e 00        nop     
158f ff        rst     38h
1590 ff        rst     38h
1591 00        nop     
1592 00        nop     
1593 00        nop     
1594 00        nop     
1595 00        nop     
1596 00        nop     
1597 00        nop     
1598 00        nop     
1599 00        nop     
159a 00        nop     
159b d7        rst     10h
159c 00        nop     
159d 00        nop     
159e 00        nop     
159f e9        jp      (hl)
15a0 ead0ae    jp      pe,0aed0h
15a3 6c        ld      l,h
15a4 64        ld      h,h
15a5 69        ld      l,c
15a6 3c        inc     a
15a7 72        ld      (hl),d
15a8 3e20      ld      a,20h
15aa 2832      jr      z,15deh
15ac 29        add     hl,hl
15ad 2e2e      ld      l,2eh
15af 2e2e      ld      l,2eh
15b1 2e2e      ld      l,2eh
15b3 2e2e      ld      l,2eh
15b5 2e2e      ld      l,2eh
15b7 2e2e      ld      l,2eh
15b9 2e2e      ld      l,2eh
15bb 2e2e      ld      l,2eh
15bd 2e2e      ld      l,2eh
15bf 2e2e      ld      l,2eh
15c1 24        inc     h
15c2 ff        rst     38h
15c3 ed44      neg     
15c5 00        nop     
15c6 00        nop     
15c7 a2        and     d
15c8 386b      jr      c,1635h
15ca 5f        ld      e,a
15cb 34        inc     (hl)
15cc d9        exx     
15cd e457d6    call    po,0d657h
15d0 d24246    jp      nc,4642h
15d3 43        ld      b,e
15d4 5a        ld      e,d
15d5 cc0900    call    z,0009h
15d8 00        nop     
15d9 00        nop     
15da 00        nop     
15db 00        nop     
15dc 00        nop     
15dd 00        nop     
15de 00        nop     
15df 00        nop     
15e0 00        nop     
15e1 00        nop     
15e2 00        nop     
15e3 00        nop     
15e4 00        nop     
15e5 00        nop     
15e6 00        nop     
15e7 d7        rst     10h
15e8 ff        rst     38h
15e9 00        nop     
15ea 00        nop     
15eb 00        nop     
15ec 00        nop     
15ed 00        nop     
15ee 00        nop     
15ef 00        nop     
15f0 00        nop     
15f1 00        nop     
15f2 00        nop     
15f3 00        nop     
15f4 00        nop     
15f5 00        nop     
15f6 00        nop     
15f7 00        nop     
15f8 00        nop     
15f9 00        nop     
15fa 00        nop     
15fb 00        nop     
15fc 00        nop     
15fd 00        nop     
15fe 00        nop     
15ff d638      sub     38h
1601 dd6a      ld      ixl,d
1603 6e        ld      l,(hl)
1604 65        ld      h,l
1605 67        ld      h,a
1606 2e2e      ld      l,2eh
1608 2e2e      ld      l,2eh
160a 2e2e      ld      l,2eh
160c 2e2e      ld      l,2eh
160e 2e2e      ld      l,2eh
1610 2e2e      ld      l,2eh
1612 2e2e      ld      l,2eh
1614 2e2e      ld      l,2eh
1616 2e2e      ld      l,2eh
1618 2e2e      ld      l,2eh
161a 2e2e      ld      l,2eh
161c 2e2e      ld      l,2eh
161e 2e2e      ld      l,2eh
1620 2e24      ld      l,24h
1622 ff        rst     38h
1623 ed67      rrd     
1625 00        nop     
1626 00        nop     
1627 cb91      res     2,c
1629 8b        adc     a,e
162a c462fa    call    nz,0fa62h
162d 03        inc     bc
162e 0120e7    ld      bc,0e720h
1631 79        ld      a,c
1632 b4        or      h
1633 40        ld      b,b
1634 06e2      ld      b,0e2h
1636 8a        adc     a,d
1637 00        nop     
1638 08        ex      af,af'
1639 00        nop     
163a 00        nop     
163b ff        rst     38h
163c 00        nop     
163d 00        nop     
163e 00        nop     
163f 00        nop     
1640 00        nop     
1641 00        nop     
1642 00        nop     
1643 00        nop     
1644 00        nop     
1645 00        nop     
1646 00        nop     
1647 00        nop     
1648 00        nop     
1649 00        nop     
164a 00        nop     
164b 00        nop     
164c 00        nop     
164d 00        nop     
164e 00        nop     
164f 00        nop     
1650 00        nop     
1651 00        nop     
1652 00        nop     
1653 00        nop     
1654 00        nop     
1655 00        nop     
1656 00        nop     
1657 00        nop     
1658 00        nop     
1659 00        nop     
165a 00        nop     
165b d7        rst     10h
165c ff        rst     38h
165d 00        nop     
165e 00        nop     
165f ff        rst     38h
1660 82        add     a,d
1661 3e77      ld      a,77h
1663 3c        inc     a
1664 72        ld      (hl),d
1665 72        ld      (hl),d
1666 64        ld      h,h
1667 2c        inc     l
1668 72        ld      (hl),d
1669 6c        ld      l,h
166a 64        ld      h,h
166b 3e2e      ld      a,2eh
166d 2e2e      ld      l,2eh
166f 2e2e      ld      l,2eh
1671 2e2e      ld      l,2eh
1673 2e2e      ld      l,2eh
1675 2e2e      ld      l,2eh
1677 2e2e      ld      l,2eh
1679 2e2e      ld      l,2eh
167b 2e2e      ld      l,2eh
167d 2e2e      ld      l,2eh
167f 2e2e      ld      l,2eh
1681 24        inc     h
1682 ff        rst     38h
1683 07        rlca    
1684 00        nop     
1685 00        nop     
1686 00        nop     
1687 92        sub     d
1688 cb43      bit     0,e
168a 6d        ld      l,l
168b 90        sub     b
168c 0a        ld      a,(bc)
168d 84        add     a,h
168e c2530c    jp      nz,0c53h
1691 0ef5      ld      c,0f5h
1693 91        sub     c
1694 eb        ex      de,hl
1695 fc4018    call    m,1840h
1698 00        nop     
1699 00        nop     
169a 00        nop     
169b 00        nop     
169c 00        nop     
169d 00        nop     
169e 00        nop     
169f 00        nop     
16a0 00        nop     
16a1 00        nop     
16a2 00        nop     
16a3 00        nop     
16a4 00        nop     
16a5 00        nop     
16a6 00        nop     
16a7 00        nop     
16a8 ff        rst     38h
16a9 00        nop     
16aa 00        nop     
16ab 00        nop     
16ac 00        nop     
16ad 00        nop     
16ae 00        nop     
16af 00        nop     
16b0 00        nop     
16b1 00        nop     
16b2 00        nop     
16b3 00        nop     
16b4 00        nop     
16b5 00        nop     
16b6 00        nop     
16b7 00        nop     
16b8 00        nop     
16b9 00        nop     
16ba 00        nop     
16bb d7        rst     10h
16bc 00        nop     
16bd 00        nop     
16be 00        nop     
16bf 9b        sbc     a,e
16c0 a3        and     e
16c1 80        add     a,b
16c2 7c        ld      a,h
16c3 3c        inc     a
16c4 72        ld      (hl),d
16c5 6c        ld      l,h
16c6 63        ld      h,e
16c7 61        ld      h,c
16c8 2c        inc     l
16c9 72        ld      (hl),d
16ca 72        ld      (hl),d
16cb 63        ld      h,e
16cc 61        ld      h,c
16cd 2c        inc     l
16ce 72        ld      (hl),d
16cf 6c        ld      l,h
16d0 61        ld      h,c
16d1 2c        inc     l
16d2 72        ld      (hl),d
16d3 72        ld      (hl),d
16d4 61        ld      h,c
16d5 3e2e      ld      a,2eh
16d7 2e2e      ld      l,2eh
16d9 2e2e      ld      l,2eh
16db 2e2e      ld      l,2eh
16dd 2e2e      ld      l,2eh
16df 2e2e      ld      l,2eh
16e1 24        inc     h
16e2 ff        rst     38h
16e3 ddcb0106  rlc     (ix+01h)
16e7 af        xor     a
16e8 dd02      ld      (bc),a
16ea 010201    ld      bc,0102h
16ed 3c        inc     a
16ee ff        rst     38h
16ef f6db      or      0dbh
16f1 f49482    call    p,8294h
16f4 80        add     a,b
16f5 d9        exx     
16f6 61        ld      h,c
16f7 2000      jr      nz,16f9h
16f9 00        nop     
16fa 3800      jr      c,16fch
16fc 00        nop     
16fd 00        nop     
16fe 00        nop     
16ff 00        nop     
1700 00        nop     
1701 00        nop     
1702 00        nop     
1703 00        nop     
1704 00        nop     
1705 00        nop     
1706 00        nop     
1707 80        add     a,b
1708 00        nop     
1709 00        nop     
170a 00        nop     
170b 00        nop     
170c 00        nop     
170d 00        nop     
170e 00        nop     
170f ff        rst     38h
1710 00        nop     
1711 00        nop     
1712 00        nop     
1713 00        nop     
1714 00        nop     
1715 00        nop     
1716 00        nop     
1717 00        nop     
1718 00        nop     
1719 00        nop     
171a 00        nop     
171b 57        ld      d,a
171c 00        nop     
171d 00        nop     
171e 00        nop     
171f 71        ld      (hl),c
1720 00        nop     
1721 34        inc     (hl)
1722 cb73      bit     6,e
1724 68        ld      l,b
1725 66        ld      h,(hl)
1726 2f        cpl     
1727 72        ld      (hl),d
1728 6f        ld      l,a
1729 74        ld      (hl),h
172a 2028      jr      nz,1754h
172c 3c        inc     a
172d 69        ld      l,c
172e 78        ld      a,b
172f 2c        inc     l
1730 69        ld      l,c
1731 79        ld      a,c
1732 3e2b      ld      a,2bh
1734 31292e    ld      sp,2e29h
1737 2e2e      ld      l,2eh
1739 2e2e      ld      l,2eh
173b 2e2e      ld      l,2eh
173d 2e2e      ld      l,2eh
173f 2e2e      ld      l,2eh
1741 24        inc     h
1742 ff        rst     38h
1743 cb00      rlc     b
1745 00        nop     
1746 00        nop     
1747 eb        ex      de,hl
1748 cc4a5d    call    z,5d4ah
174b 07        rlca    
174c e0        ret     po

174d 03        inc     bc
174e 019513    ld      bc,1395h
1751 ee30      xor     30h
1753 43        ld      b,e
1754 78        ld      a,b
1755 ad        xor     l
1756 3d        dec     a
1757 00        nop     
1758 3f        ccf     
1759 00        nop     
175a 00        nop     
175b 00        nop     
175c 00        nop     
175d 00        nop     
175e 00        nop     
175f 00        nop     
1760 00        nop     
1761 00        nop     
1762 00        nop     
1763 00        nop     
1764 00        nop     
1765 00        nop     
1766 00        nop     
1767 80        add     a,b
1768 00        nop     
1769 00        nop     
176a 00        nop     
176b 00        nop     
176c 00        nop     
176d 00        nop     
176e 00        nop     
176f ff        rst     38h
1770 00        nop     
1771 00        nop     
1772 00        nop     
1773 00        nop     
1774 00        nop     
1775 00        nop     
1776 00        nop     
1777 ff        rst     38h
1778 ff        rst     38h
1779 ff        rst     38h
177a ff        rst     38h
177b 57        ld      d,a
177c ff        rst     38h
177d 00        nop     
177e 00        nop     
177f a4        and     h
1780 25        dec     h
1781 58        ld      e,b
1782 33        inc     sp
1783 73        ld      (hl),e
1784 68        ld      l,b
1785 66        ld      h,(hl)
1786 2f        cpl     
1787 72        ld      (hl),d
1788 6f        ld      l,a
1789 74        ld      (hl),h
178a 203c      jr      nz,17c8h
178c 62        ld      h,d
178d 2c        inc     l
178e 63        ld      h,e
178f 2c        inc     l
1790 64        ld      h,h
1791 2c        inc     l
1792 65        ld      h,l
1793 2c        inc     l
1794 68        ld      l,b
1795 2c        inc     l
1796 6c        ld      l,h
1797 2c        inc     l
1798 2868      jr      z,1802h
179a 6c        ld      l,h
179b 29        add     hl,hl
179c 2c        inc     l
179d 61        ld      h,c
179e 3e2e      ld      a,2eh
17a0 2e24      ld      l,24h
17a2 ff        rst     38h
17a3 cb80      res     0,b
17a5 00        nop     
17a6 00        nop     
17a7 d5        push    de
17a8 2c        inc     l
17a9 ab        xor     e
17aa 97        sub     a
17ab ff        rst     38h
17ac 39        add     hl,sp
17ad 03        inc     bc
17ae 014bd1    ld      bc,0d14bh
17b1 b2        or      d
17b2 6a        ld      l,d
17b3 53        ld      d,e
17b4 27        daa     
17b5 38b5      jr      c,176ch
17b7 00        nop     
17b8 7f        ld      a,a
17b9 00        nop     
17ba 00        nop     
17bb 00        nop     
17bc 00        nop     
17bd 00        nop     
17be 00        nop     
17bf 00        nop     
17c0 00        nop     
17c1 00        nop     
17c2 00        nop     
17c3 00        nop     
17c4 00        nop     
17c5 00        nop     
17c6 00        nop     
17c7 00        nop     
17c8 00        nop     
17c9 00        nop     
17ca 00        nop     
17cb 00        nop     
17cc 00        nop     
17cd 00        nop     
17ce 00        nop     
17cf ff        rst     38h
17d0 00        nop     
17d1 00        nop     
17d2 00        nop     
17d3 00        nop     
17d4 00        nop     
17d5 00        nop     
17d6 00        nop     
17d7 ff        rst     38h
17d8 ff        rst     38h
17d9 ff        rst     38h
17da ff        rst     38h
17db d7        rst     10h
17dc ff        rst     38h
17dd 00        nop     
17de 00        nop     
17df 8b        adc     a,e
17e0 57        ld      d,a
17e1 f0        ret     p

17e2 08        ex      af,af'
17e3 3c        inc     a
17e4 73        ld      (hl),e
17e5 65        ld      h,l
17e6 74        ld      (hl),h
17e7 2c        inc     l
17e8 72        ld      (hl),d
17e9 65        ld      h,l
17ea 73        ld      (hl),e
17eb 3e20      ld      a,20h
17ed 6e        ld      l,(hl)
17ee 2c        inc     l
17ef 3c        inc     a
17f0 62        ld      h,d
17f1 63        ld      h,e
17f2 64        ld      h,h
17f3 65        ld      h,l
17f4 68        ld      l,b
17f5 6c        ld      l,h
17f6 2868      jr      z,1860h
17f8 6c        ld      l,h
17f9 29        add     hl,hl
17fa 61        ld      h,c
17fb 3e2e      ld      a,2eh
17fd 2e2e      ld      l,2eh
17ff 2e2e      ld      l,2eh
1801 24        inc     h
1802 ff        rst     38h
1803 ddcb0186  res     0,(ix+01h)
1807 44        ld      b,h
1808 fb        ei      
1809 02        ld      (bc),a
180a 010201    ld      bc,0102h
180d 09        add     hl,bc
180e ba        cp      d
180f be        cp      (hl)
1810 68        ld      l,b
1811 d8        ret     c

1812 32105e    ld      (5e10h),a
1815 67        ld      h,a
1816 a8        xor     b
1817 2000      jr      nz,1819h
1819 00        nop     
181a 78        ld      a,b
181b 00        nop     
181c 00        nop     
181d 00        nop     
181e 00        nop     
181f 00        nop     
1820 00        nop     
1821 00        nop     
1822 00        nop     
1823 00        nop     
1824 00        nop     
1825 00        nop     
1826 00        nop     
1827 00        nop     
1828 00        nop     
1829 00        nop     
182a 00        nop     
182b 00        nop     
182c 00        nop     
182d 00        nop     
182e 00        nop     
182f ff        rst     38h
1830 00        nop     
1831 00        nop     
1832 00        nop     
1833 00        nop     
1834 00        nop     
1835 00        nop     
1836 00        nop     
1837 00        nop     
1838 00        nop     
1839 00        nop     
183a 00        nop     
183b d7        rst     10h
183c 00        nop     
183d 00        nop     
183e 00        nop     
183f cc63f9    call    z,0f963h
1842 8a        adc     a,d
1843 3c        inc     a
1844 73        ld      (hl),e
1845 65        ld      h,l
1846 74        ld      (hl),h
1847 2c        inc     l
1848 72        ld      (hl),d
1849 65        ld      h,l
184a 73        ld      (hl),e
184b 3e20      ld      a,20h
184d 6e        ld      l,(hl)
184e 2c        inc     l
184f 283c      jr      z,188dh
1851 69        ld      l,c
1852 78        ld      a,b
1853 2c        inc     l
1854 69        ld      l,c
1855 79        ld      a,c
1856 3e2b      ld      a,2bh
1858 31292e    ld      sp,2e29h
185b 2e2e      ld      l,2eh
185d 2e2e      ld      l,2eh
185f 2e2e      ld      l,2eh
1861 24        inc     h
1862 ff        rst     38h
1863 dd7001    ld      (ix+01h),b
1866 00        nop     
1867 0d        dec     c
1868 27        daa     
1869 02        ld      (bc),a
186a 010201    ld      bc,0102h
186d 3ab77b    ld      a,(7bb7h)
1870 88        adc     a,b
1871 ee99      xor     99h
1873 86        add     a,(hl)
1874 70        ld      (hl),b
1875 07        rlca    
1876 ca2003    jp      z,0320h
1879 00        nop     
187a 00        nop     
187b 00        nop     
187c 00        nop     
187d 010001    ld      bc,0100h
1880 00        nop     
1881 00        nop     
1882 00        nop     
1883 00        nop     
1884 00        nop     
1885 00        nop     
1886 00        nop     
1887 00        nop     
1888 00        nop     
1889 00        nop     
188a 00        nop     
188b 00        nop     
188c 00        nop     
188d 00        nop     
188e 00        nop     
188f 00        nop     
1890 00        nop     
1891 00        nop     
1892 00        nop     
1893 00        nop     
1894 00        nop     
1895 00        nop     
1896 00        nop     
1897 ff        rst     38h
1898 ff        rst     38h
1899 ff        rst     38h
189a ff        rst     38h
189b 00        nop     
189c 00        nop     
189d 00        nop     
189e 00        nop     
189f 04        inc     b
18a0 62        ld      h,d
18a1 6a        ld      l,d
18a2 bf        cp      a
18a3 6c        ld      l,h
18a4 64        ld      h,h
18a5 2028      jr      nz,18cfh
18a7 3c        inc     a
18a8 69        ld      l,c
18a9 78        ld      a,b
18aa 2c        inc     l
18ab 69        ld      l,c
18ac 79        ld      a,c
18ad 3e2b      ld      a,2bh
18af 31292c    ld      sp,2c29h
18b2 3c        inc     a
18b3 62        ld      h,d
18b4 2c        inc     l
18b5 63        ld      h,e
18b6 2c        inc     l
18b7 64        ld      h,h
18b8 2c        inc     l
18b9 65        ld      h,l
18ba 3e2e      ld      a,2eh
18bc 2e2e      ld      l,2eh
18be 2e2e      ld      l,2eh
18c0 2e24      ld      l,24h
18c2 ff        rst     38h
18c3 dd7401    ld      (ix+01h),h
18c6 00        nop     
18c7 64        ld      h,h
18c8 b6        or      (hl)
18c9 02        ld      (bc),a
18ca 010201    ld      bc,0102h
18cd ac        xor     h
18ce e8        ret     pe

18cf f5        push    af
18d0 b5        or      l
18d1 feaa      cp      0aah
18d3 12        ld      (de),a
18d4 1066      djnz    193ch
18d6 95        sub     l
18d7 2001      jr      nz,18dah
18d9 00        nop     
18da 00        nop     
18db 00        nop     
18dc 00        nop     
18dd 010001    ld      bc,0100h
18e0 00        nop     
18e1 00        nop     
18e2 00        nop     
18e3 00        nop     
18e4 00        nop     
18e5 00        nop     
18e6 00        nop     
18e7 00        nop     
18e8 00        nop     
18e9 00        nop     
18ea 00        nop     
18eb 00        nop     
18ec 00        nop     
18ed 00        nop     
18ee 00        nop     
18ef 00        nop     
18f0 00        nop     
18f1 00        nop     
18f2 00        nop     
18f3 00        nop     
18f4 00        nop     
18f5 ff        rst     38h
18f6 ff        rst     38h
18f7 00        nop     
18f8 00        nop     
18f9 00        nop     
18fa 00        nop     
18fb 00        nop     
18fc 00        nop     
18fd 00        nop     
18fe 00        nop     
18ff 6a        ld      l,d
1900 1a        ld      a,(de)
1901 88        adc     a,b
1902 316c64    ld      sp,646ch
1905 2028      jr      nz,192fh
1907 3c        inc     a
1908 69        ld      l,c
1909 78        ld      a,b
190a 2c        inc     l
190b 69        ld      l,c
190c 79        ld      a,c
190d 3e2b      ld      a,2bh
190f 31292c    ld      sp,2c29h
1912 3c        inc     a
1913 68        ld      l,b
1914 2c        inc     l
1915 6c        ld      l,h
1916 3e2e      ld      a,2eh
1918 2e2e      ld      l,2eh
191a 2e2e      ld      l,2eh
191c 2e2e      ld      l,2eh
191e 2e2e      ld      l,2eh
1920 2e24      ld      l,24h
1922 ff        rst     38h
1923 dd7701    ld      (ix+01h),a
1926 00        nop     
1927 af        xor     a
1928 67        ld      h,a
1929 02        ld      (bc),a
192a 010201    ld      bc,0102h
192d 13        inc     de
192e 4f        ld      c,a
192f 44        ld      b,h
1930 06d7      ld      b,0d7h
1932 bc        cp      h
1933 50        ld      d,b
1934 ac        xor     h
1935 af        xor     a
1936 5f        ld      e,a
1937 2000      jr      nz,1939h
1939 00        nop     
193a 00        nop     
193b 00        nop     
193c 00        nop     
193d 010001    ld      bc,0100h
1940 00        nop     
1941 00        nop     
1942 00        nop     
1943 00        nop     
1944 00        nop     
1945 00        nop     
1946 00        nop     
1947 00        nop     
1948 00        nop     
1949 00        nop     
194a 00        nop     
194b 00        nop     
194c 00        nop     
194d 00        nop     
194e 00        nop     
194f 00        nop     
1950 00        nop     
1951 00        nop     
1952 00        nop     
1953 00        nop     
1954 00        nop     
1955 00        nop     
1956 00        nop     
1957 00        nop     
1958 00        nop     
1959 00        nop     
195a 00        nop     
195b 00        nop     
195c ff        rst     38h
195d 00        nop     
195e 00        nop     
195f ccbe5a    call    z,5abeh
1962 96        sub     (hl)
1963 6c        ld      l,h
1964 64        ld      h,h
1965 2028      jr      nz,198fh
1967 3c        inc     a
1968 69        ld      l,c
1969 78        ld      a,b
196a 2c        inc     l
196b 69        ld      l,c
196c 79        ld      a,c
196d 3e2b      ld      a,2bh
196f 31292c    ld      sp,2c29h
1972 61        ld      h,c
1973 2e2e      ld      l,2eh
1975 2e2e      ld      l,2eh
1977 2e2e      ld      l,2eh
1979 2e2e      ld      l,2eh
197b 2e2e      ld      l,2eh
197d 2e2e      ld      l,2eh
197f 2e2e      ld      l,2eh
1981 24        inc     h
1982 ff        rst     38h
1983 02        ld      (bc),a
1984 00        nop     
1985 00        nop     
1986 00        nop     
1987 3b        dec     sp
1988 0c        inc     c
1989 92        sub     d
198a b5        or      l
198b ff        rst     38h
198c 6c        ld      l,h
198d 9e        sbc     a,(hl)
198e 95        sub     l
198f 03        inc     bc
1990 010401    ld      bc,0104h
1993 c1        pop     bc
1994 21e7bd    ld      hl,0bde7h
1997 1800      jr      1999h
1999 00        nop     
199a 00        nop     
199b 00        nop     
199c 00        nop     
199d 00        nop     
199e 00        nop     
199f 00        nop     
19a0 00        nop     
19a1 00        nop     
19a2 00        nop     
19a3 00        nop     
19a4 00        nop     
19a5 00        nop     
19a6 00        nop     
19a7 00        nop     
19a8 00        nop     
19a9 00        nop     
19aa 00        nop     
19ab 00        nop     
19ac 00        nop     
19ad 00        nop     
19ae 00        nop     
19af ff        rst     38h
19b0 ff        rst     38h
19b1 00        nop     
19b2 00        nop     
19b3 00        nop     
19b4 00        nop     
19b5 00        nop     
19b6 00        nop     
19b7 00        nop     
19b8 00        nop     
19b9 00        nop     
19ba 00        nop     
19bb 00        nop     
19bc ff        rst     38h
19bd 00        nop     
19be 00        nop     
19bf 7a        ld      a,d
19c0 4c        ld      c,h
19c1 114f6c    ld      de,6c4fh
19c4 64        ld      h,h
19c5 2028      jr      nz,19efh
19c7 3c        inc     a
19c8 62        ld      h,d
19c9 63        ld      h,e
19ca 2c        inc     l
19cb 64        ld      h,h
19cc 65        ld      h,l
19cd 3e29      ld      a,29h
19cf 2c        inc     l
19d0 61        ld      h,c
19d1 2e2e      ld      l,2eh
19d3 2e2e      ld      l,2eh
19d5 2e2e      ld      l,2eh
19d7 2e2e      ld      l,2eh
19d9 2e2e      ld      l,2eh
19db 2e2e      ld      l,2eh
19dd 2e2e      ld      l,2eh
19df 2e2e      ld      l,2eh
19e1 24        inc     h
19e2 e5        push    hl
19e3 7e        ld      a,(hl)
19e4 23        inc     hl
19e5 66        ld      h,(hl)
19e6 6f        ld      l,a
19e7 7e        ld      a,(hl)
19e8 32651d    ld      (1d65h),a
19eb 23        inc     hl
19ec e5        push    hl
19ed 111400    ld      de,0014h
19f0 19        add     hl,de
19f1 11da1c    ld      de,1cdah
19f4 cd491c    call    1c49h
19f7 e1        pop     hl
19f8 e5        push    hl
19f9 112800    ld      de,0028h
19fc 19        add     hl,de
19fd 11021d    ld      de,1d02h
1a00 cd491c    call    1c49h
1a03 21021d    ld      hl,1d02h
1a06 3601      ld      (hl),01h
1a08 e1        pop     hl
1a09 e5        push    hl
1a0a 11421d    ld      de,1d42h
1a0d 010400    ld      bc,0004h
1a10 edb0      ldir    
1a12 110301    ld      de,0103h
1a15 011000    ld      bc,0010h
1a18 edb0      ldir    
1a1a 112c00    ld      de,002ch
1a1d 19        add     hl,de
1a1e eb        ex      de,hl
1a1f 0e09      ld      c,09h
1a21 cdce1d    call    1dceh
1a24 cd711e    call    1e71h
1a27 3a421d    ld      a,(1d42h)
1a2a fe76      cp      76h
1a2c ca3e1b    jp      z,1b3eh
1a2f e6df      and     0dfh
1a31 fedd      cp      0ddh
1a33 c23b1b    jp      nz,1b3bh
1a36 3a431d    ld      a,(1d43h)
1a39 fe76      cp      76h
1a3b c42a1d    call    nz,1d2ah
1a3e cd891c    call    1c89h
1a41 c4ad1c    call    nz,1cadh
1a44 e1        pop     hl
1a45 ca7a1b    jp      z,1b7ah
1a48 113c00    ld      de,003ch
1a4b 19        add     hl,de
1a4c cd321e    call    1e32h
1a4f 11051e    ld      de,1e05h
1a52 ca711b    jp      z,1b71h
1a55 110c1e    ld      de,1e0ch
1a58 0e09      ld      c,09h
1a5a cdce1d    call    1dceh
1a5d cd991d    call    1d99h
1a60 11271e    ld      de,1e27h
1a63 0e09      ld      c,09h
1a65 cdce1d    call    1dceh
1a68 21851e    ld      hl,1e85h
1a6b cd991d    call    1d99h
1a6e 112f1e    ld      de,1e2fh
1a71 0e09      ld      c,09h
1a73 cdce1d    call    1dceh
1a76 e1        pop     hl
1a77 23        inc     hl
1a78 23        inc     hl
1a79 c9        ret     

1a7a e5        push    hl
1a7b 3e01      ld      a,01h
1a7d 32f01b    ld      (1bf0h),a
1a80 32141c    ld      (1c14h),a
1a83 21da1c    ld      hl,1cdah
1a86 22f11b    ld      (1bf1h),hl
1a89 21021d    ld      hl,1d02h
1a8c 22151c    ld      (1c15h),hl
1a8f 0604      ld      b,04h
1a91 e1        pop     hl
1a92 e5        push    hl
1a93 11421d    ld      de,1d42h
1a96 cda41b    call    1ba4h
1a99 0610      ld      b,10h
1a9b 110301    ld      de,0103h
1a9e cda41b    call    1ba4h
1aa1 c3271b    jp      1b27h
1aa4 cdad1b    call    1badh
1aa7 23        inc     hl
1aa8 05        dec     b
1aa9 c2a41b    jp      nz,1ba4h
1aac c9        ret     

1aad c5        push    bc
1aae d5        push    de
1aaf e5        push    hl
1ab0 4e        ld      c,(hl)
1ab1 111400    ld      de,0014h
1ab4 19        add     hl,de
1ab5 7e        ld      a,(hl)
1ab6 fe00      cp      00h
1ab8 cace1b    jp      z,1bceh
1abb 0608      ld      b,08h
1abd 0f        rrca    
1abe f5        push    af
1abf 3e00      ld      a,00h
1ac1 dcf31b    call    c,1bf3h
1ac4 a9        xor     c
1ac5 0f        rrca    
1ac6 4f        ld      c,a
1ac7 f1        pop     af
1ac8 05        dec     b
1ac9 c2bd1b    jp      nz,1bbdh
1acc 0608      ld      b,08h
1ace 111400    ld      de,0014h
1ad1 19        add     hl,de
1ad2 7e        ld      a,(hl)
1ad3 fe00      cp      00h
1ad5 cae91b    jp      z,1be9h
1ad8 0608      ld      b,08h
1ada 0f        rrca    
1adb f5        push    af
1adc 3e00      ld      a,00h
1ade dc171c    call    c,1c17h
1ae1 a9        xor     c
1ae2 0f        rrca    
1ae3 4f        ld      c,a
1ae4 f1        pop     af
1ae5 05        dec     b
1ae6 c2da1b    jp      nz,1bdah
1ae9 e1        pop     hl
1aea d1        pop     de
1aeb 79        ld      a,c
1aec 12        ld      (de),a
1aed 13        inc     de
1aee c1        pop     bc
1aef c9        ret     

1af0 00        nop     
1af1 00        nop     
1af2 00        nop     
1af3 c5        push    bc
1af4 e5        push    hl
1af5 2af11b    ld      hl,(1bf1h)
1af8 46        ld      b,(hl)
1af9 21f01b    ld      hl,1bf0h
1afc 7e        ld      a,(hl)
1afd 4f        ld      c,a
1afe 07        rlca    
1aff 77        ld      (hl),a
1b00 fe01      cp      01h
1b02 c20c1c    jp      nz,1c0ch
1b05 2af11b    ld      hl,(1bf1h)
1b08 23        inc     hl
1b09 22f11b    ld      (1bf1h),hl
1b0c 78        ld      a,b
1b0d a1        and     c
1b0e e1        pop     hl
1b0f c1        pop     bc
1b10 c8        ret     z

1b11 3e01      ld      a,01h
1b13 c9        ret     

1b14 00        nop     
1b15 00        nop     
1b16 00        nop     
1b17 c5        push    bc
1b18 e5        push    hl
1b19 2a151c    ld      hl,(1c15h)
1b1c 46        ld      b,(hl)
1b1d 21141c    ld      hl,1c14h
1b20 7e        ld      a,(hl)
1b21 4f        ld      c,a
1b22 07        rlca    
1b23 77        ld      (hl),a
1b24 fe01      cp      01h
1b26 c2301c    jp      nz,1c30h
1b29 2a151c    ld      hl,(1c15h)
1b2c 23        inc     hl
1b2d 22151c    ld      (1c15h),hl
1b30 78        ld      a,b
1b31 a1        and     c
1b32 e1        pop     hl
1b33 c1        pop     bc
1b34 c8        ret     z

1b35 3e01      ld      a,01h
1b37 c9        ret     

1b38 f5        push    af
1b39 c5        push    bc
1b3a d5        push    de
1b3b e5        push    hl
1b3c 3600      ld      (hl),00h
1b3e 54        ld      d,h
1b3f 5d        ld      e,l
1b40 13        inc     de
1b41 0b        dec     bc
1b42 edb0      ldir    
1b44 e1        pop     hl
1b45 d1        pop     de
1b46 c1        pop     bc
1b47 f1        pop     af
1b48 c9        ret     

1b49 d5        push    de
1b4a eb        ex      de,hl
1b4b 012800    ld      bc,0028h
1b4e cd381c    call    1c38h
1b51 eb        ex      de,hl
1b52 0614      ld      b,14h
1b54 0e01      ld      c,01h
1b56 1600      ld      d,00h
1b58 5e        ld      e,(hl)
1b59 7b        ld      a,e
1b5a a1        and     c
1b5b ca5f1c    jp      z,1c5fh
1b5e 14        inc     d
1b5f 79        ld      a,c
1b60 07        rlca    
1b61 4f        ld      c,a
1b62 fe01      cp      01h
1b64 c2591c    jp      nz,1c59h
1b67 23        inc     hl
1b68 05        dec     b
1b69 c2581c    jp      nz,1c58h
1b6c 7a        ld      a,d
1b6d e6f8      and     0f8h
1b6f 0f        rrca    
1b70 0f        rrca    
1b71 0f        rrca    
1b72 6f        ld      l,a
1b73 2600      ld      h,00h
1b75 7a        ld      a,d
1b76 e607      and     07h
1b78 3c        inc     a
1b79 47        ld      b,a
1b7a 3e80      ld      a,80h
1b7c 07        rlca    
1b7d 05        dec     b
1b7e c27c1c    jp      nz,1c7ch
1b81 d1        pop     de
1b82 19        add     hl,de
1b83 111400    ld      de,0014h
1b86 19        add     hl,de
1b87 77        ld      (hl),a
1b88 c9        ret     

1b89 c5        push    bc
1b8a d5        push    de
1b8b e5        push    hl
1b8c 21da1c    ld      hl,1cdah
1b8f 111400    ld      de,0014h
1b92 eb        ex      de,hl
1b93 19        add     hl,de
1b94 eb        ex      de,hl
1b95 34        inc     (hl)
1b96 7e        ld      a,(hl)
1b97 fe00      cp      00h
1b99 caa81c    jp      z,1ca8h
1b9c 47        ld      b,a
1b9d 1a        ld      a,(de)
1b9e a0        and     b
1b9f caa41c    jp      z,1ca4h
1ba2 3600      ld      (hl),00h
1ba4 c1        pop     bc
1ba5 d1        pop     de
1ba6 e1        pop     hl
1ba7 c9        ret     

1ba8 23        inc     hl
1ba9 13        inc     de
1baa c3951c    jp      1c95h
1bad c5        push    bc
1bae d5        push    de
1baf e5        push    hl
1bb0 21021d    ld      hl,1d02h
1bb3 111400    ld      de,0014h
1bb6 eb        ex      de,hl
1bb7 19        add     hl,de
1bb8 eb        ex      de,hl
1bb9 7e        ld      a,(hl)
1bba b7        or      a
1bbb cad51c    jp      z,1cd5h
1bbe 47        ld      b,a
1bbf 1a        ld      a,(de)
1bc0 a0        and     b
1bc1 c2d11c    jp      nz,1cd1h
1bc4 78        ld      a,b
1bc5 07        rlca    
1bc6 fe01      cp      01h
1bc8 c2cf1c    jp      nz,1ccfh
1bcb 3600      ld      (hl),00h
1bcd 23        inc     hl
1bce 13        inc     de
1bcf 77        ld      (hl),a
1bd0 af        xor     a
1bd1 e1        pop     hl
1bd2 d1        pop     de
1bd3 c1        pop     bc
1bd4 c9        ret     

1bd5 23        inc     hl
1bd6 13        inc     de
1bd7 c3b91c    jp      1cb9h
1bda 00        nop     
1bdb 00        nop     
1bdc 00        nop     
1bdd 00        nop     
1bde 00        nop     
1bdf 00        nop     
1be0 00        nop     
1be1 00        nop     
1be2 00        nop     
1be3 00        nop     
1be4 00        nop     
1be5 00        nop     
1be6 00        nop     
1be7 00        nop     
1be8 00        nop     
1be9 00        nop     
1bea 00        nop     
1beb 00        nop     
1bec 00        nop     
1bed 00        nop     
1bee 00        nop     
1bef 00        nop     
1bf0 00        nop     
1bf1 00        nop     
1bf2 00        nop     
1bf3 00        nop     
1bf4 00        nop     
1bf5 00        nop     
1bf6 00        nop     
1bf7 00        nop     
1bf8 00        nop     
1bf9 00        nop     
1bfa 00        nop     
1bfb 00        nop     
1bfc 00        nop     
1bfd 00        nop     
1bfe 00        nop     
1bff 00        nop     
1c00 00        nop     
1c01 00        nop     
1c02 00        nop     
1c03 00        nop     
1c04 00        nop     
1c05 00        nop     
1c06 00        nop     
1c07 00        nop     
1c08 00        nop     
1c09 00        nop     
1c0a 00        nop     
1c0b 00        nop     
1c0c 00        nop     
1c0d 00        nop     
1c0e 00        nop     
1c0f 00        nop     
1c10 00        nop     
1c11 00        nop     
1c12 00        nop     
1c13 00        nop     
1c14 00        nop     
1c15 00        nop     
1c16 00        nop     
1c17 00        nop     
1c18 00        nop     
1c19 00        nop     
1c1a 00        nop     
1c1b 00        nop     
1c1c 00        nop     
1c1d 00        nop     
1c1e 00        nop     
1c1f 00        nop     
1c20 00        nop     
1c21 00        nop     
1c22 00        nop     
1c23 00        nop     
1c24 00        nop     
1c25 00        nop     
1c26 00        nop     
1c27 00        nop     
1c28 00        nop     
1c29 00        nop     
1c2a f5        push    af
1c2b c5        push    bc
1c2c d5        push    de
1c2d e5        push    hl
1c2e f3        di      
1c2f ed738d1d  ld      (1d8dh),sp
1c33 310501    ld      sp,0105h
1c36 fde1      pop     iy
1c38 dde1      pop     ix
1c3a e1        pop     hl
1c3b d1        pop     de
1c3c c1        pop     bc
1c3d f1        pop     af
1c3e ed7b1101  ld      sp,(0111h)
1c42 00        nop     
1c43 00        nop     
1c44 00        nop     
1c45 00        nop     
1c46 ed738b1d  ld      (1d8bh),sp
1c4a 318b1d    ld      sp,1d8bh
1c4d f5        push    af
1c4e c5        push    bc
1c4f d5        push    de
1c50 e5        push    hl
1c51 dde5      push    ix
1c53 fde5      push    iy
1c55 ed7b8d1d  ld      sp,(1d8dh)
1c59 fb        ei      
1c5a 2a0301    ld      hl,(0103h)
1c5d 227d1d    ld      (1d7dh),hl
1c60 21891d    ld      hl,1d89h
1c63 7e        ld      a,(hl)
1c64 e6d7      and     0d7h
1c66 77        ld      (hl),a
1c67 0610      ld      b,10h
1c69 117d1d    ld      de,1d7dh
1c6c 21851e    ld      hl,1e85h
1c6f 1a        ld      a,(de)
1c70 13        inc     de
1c71 cd491e    call    1e49h
1c74 05        dec     b
1c75 c26f1d    jp      nz,1d6fh
1c78 e1        pop     hl
1c79 d1        pop     de
1c7a c1        pop     bc
1c7b f1        pop     af
1c7c c9        ret     

1c7d 00        nop     
1c7e 00        nop     
1c7f 00        nop     
1c80 00        nop     
1c81 00        nop     
1c82 00        nop     
1c83 00        nop     
1c84 00        nop     
1c85 00        nop     
1c86 00        nop     
1c87 00        nop     
1c88 00        nop     
1c89 00        nop     
1c8a 00        nop     
1c8b 00        nop     
1c8c 00        nop     
1c8d 00        nop     
1c8e 00        nop     
1c8f 7e        ld      a,(hl)
1c90 cdab1d    call    1dabh
1c93 23        inc     hl
1c94 05        dec     b
1c95 c28f1d    jp      nz,1d8fh
1c98 c9        ret     

1c99 f5        push    af
1c9a c5        push    bc
1c9b e5        push    hl
1c9c 0604      ld      b,04h
1c9e 7e        ld      a,(hl)
1c9f cdab1d    call    1dabh
1ca2 23        inc     hl
1ca3 05        dec     b
1ca4 c29e1d    jp      nz,1d9eh
1ca7 e1        pop     hl
1ca8 c1        pop     bc
1ca9 f1        pop     af
1caa c9        ret     

1cab f5        push    af
1cac 0f        rrca    
1cad 0f        rrca    
1cae 0f        rrca    
1caf 0f        rrca    
1cb0 cdb41d    call    1db4h
1cb3 f1        pop     af
1cb4 f5        push    af
1cb5 c5        push    bc
1cb6 d5        push    de
1cb7 e5        push    hl
1cb8 e60f      and     0fh
1cba fe0a      cp      0ah
1cbc dac11d    jp      c,1dc1h
1cbf c627      add     a,27h
1cc1 c630      add     a,30h
1cc3 5f        ld      e,a
1cc4 0e02      ld      c,02h
1cc6 cdce1d    call    1dceh
1cc9 e1        pop     hl
1cca d1        pop     de
1ccb c1        pop     bc
1ccc f1        pop     af
1ccd c9        ret     

1cce f5        push    af
1ccf c5        push    bc
1cd0 d5        push    de
1cd1 e5        push    hl
1cd2 cd0500    call    0005h
1cd5 e1        pop     hl
1cd6 d1        pop     de
1cd7 c1        pop     bc
1cd8 f1        pop     af
1cd9 c9        ret     

1cda 5a        ld      e,d
1cdb 3830      jr      c,1d0dh
1cdd 2069      jr      nz,1d48h
1cdf 6e        ld      l,(hl)
1ce0 73        ld      (hl),e
1ce1 74        ld      (hl),h
1ce2 72        ld      (hl),d
1ce3 75        ld      (hl),l
1ce4 63        ld      h,e
1ce5 74        ld      (hl),h
1ce6 69        ld      l,c
1ce7 6f        ld      l,a
1ce8 6e        ld      l,(hl)
1ce9 2065      jr      nz,1d50h
1ceb 78        ld      a,b
1cec 65        ld      h,l
1ced 72        ld      (hl),d
1cee 63        ld      h,e
1cef 69        ld      l,c
1cf0 73        ld      (hl),e
1cf1 65        ld      h,l
1cf2 72        ld      (hl),d
1cf3 0a        ld      a,(bc)
1cf4 0d        dec     c
1cf5 24        inc     h
1cf6 54        ld      d,h
1cf7 65        ld      h,l
1cf8 73        ld      (hl),e
1cf9 74        ld      (hl),h
1cfa 73        ld      (hl),e
1cfb 2063      jr      nz,1d60h
1cfd 6f        ld      l,a
1cfe 6d        ld      l,l
1cff 70        ld      (hl),b
1d00 6c        ld      l,h
1d01 65        ld      h,l
1d02 74        ld      (hl),h
1d03 65        ld      h,l
1d04 24        inc     h
1d05 2020      jr      nz,1d27h
1d07 4f        ld      c,a
1d08 4b        ld      c,e
1d09 0a        ld      a,(bc)
1d0a 0d        dec     c
1d0b 24        inc     h
1d0c 2020      jr      nz,1d2eh
1d0e 45        ld      b,l
1d0f 52        ld      d,d
1d10 52        ld      d,d
1d11 4f        ld      c,a
1d12 52        ld      d,d
1d13 202a      jr      nz,1d3fh
1d15 2a2a2a    ld      hl,(2a2ah)
1d18 2063      jr      nz,1d7dh
1d1a 72        ld      (hl),d
1d1b 63        ld      h,e
1d1c 2065      jr      nz,1d83h
1d1e 78        ld      a,b
1d1f 70        ld      (hl),b
1d20 65        ld      h,l
1d21 63        ld      h,e
1d22 74        ld      (hl),h
1d23 65        ld      h,l
1d24 64        ld      h,h
1d25 3a2420    ld      a,(2024h)
1d28 66        ld      h,(hl)
1d29 6f        ld      l,a
1d2a 75        ld      (hl),l
1d2b 6e        ld      l,(hl)
1d2c 64        ld      h,h
1d2d 3a240a    ld      a,(0a24h)
1d30 0d        dec     c
1d31 24        inc     h
1d32 c5        push    bc
1d33 d5        push    de
1d34 e5        push    hl
1d35 11851e    ld      de,1e85h
1d38 0604      ld      b,04h
1d3a 1a        ld      a,(de)
1d3b be        cp      (hl)
1d3c c2451e    jp      nz,1e45h
1d3f 23        inc     hl
1d40 13        inc     de
1d41 05        dec     b
1d42 c23a1e    jp      nz,1e3ah
1d45 e1        pop     hl
1d46 d1        pop     de
1d47 c1        pop     bc
1d48 c9        ret     

1d49 f5        push    af
1d4a c5        push    bc
1d4b d5        push    de
1d4c e5        push    hl
1d4d e5        push    hl
1d4e 110300    ld      de,0003h
1d51 19        add     hl,de
1d52 ae        xor     (hl)
1d53 6f        ld      l,a
1d54 2600      ld      h,00h
1d56 29        add     hl,hl
1d57 29        add     hl,hl
1d58 eb        ex      de,hl
1d59 21891e    ld      hl,1e89h
1d5c 19        add     hl,de
1d5d eb        ex      de,hl
1d5e e1        pop     hl
1d5f 010400    ld      bc,0004h
1d62 1a        ld      a,(de)
1d63 a8        xor     b
1d64 46        ld      b,(hl)
1d65 77        ld      (hl),a
1d66 13        inc     de
1d67 23        inc     hl
1d68 0d        dec     c
1d69 c2621e    jp      nz,1e62h
1d6c e1        pop     hl
1d6d d1        pop     de
1d6e c1        pop     bc
1d6f f1        pop     af
1d70 c9        ret     

1d71 f5        push    af
1d72 c5        push    bc
1d73 e5        push    hl
1d74 21851e    ld      hl,1e85h
1d77 3eff      ld      a,0ffh
1d79 0604      ld      b,04h
1d7b 77        ld      (hl),a
1d7c 23        inc     hl
1d7d 05        dec     b
1d7e c27b1e    jp      nz,1e7bh
1d81 e1        pop     hl
1d82 c1        pop     bc
1d83 f1        pop     af
1d84 c9        ret     

1d85 00        nop     
1d86 00        nop     
1d87 00        nop     
1d88 00        nop     
1d89 00        nop     
1d8a 00        nop     
1d8b 00        nop     
1d8c 00        nop     
1d8d 77        ld      (hl),a
1d8e 07        rlca    
1d8f 3096      jr      nc,1d27h
1d91 ee0e      xor     0eh
1d93 61        ld      h,c
1d94 2c        inc     l
1d95 99        sbc     a,c
1d96 09        add     hl,bc
1d97 51        ld      d,c
1d98 ba        cp      d
1d99 07        rlca    
1d9a 6d        ld      l,l
1d9b c41970    call    nz,7019h
1d9e 6a        ld      l,d
1d9f f48fe9    call    p,0e98fh
1da2 63        ld      h,e
1da3 a5        and     l
1da4 35        dec     (hl)
1da5 9e        sbc     a,(hl)
1da6 64        ld      h,h
1da7 95        sub     l
1da8 a3        and     e
1da9 0edb      ld      c,0dbh
1dab 88        adc     a,b
1dac 3279dc    ld      (0dc79h),a
1daf b8        cp      b
1db0 a4        and     h
1db1 e0        ret     po

1db2 d5        push    de
1db3 e9        jp      (hl)
1db4 1e97      ld      e,97h
1db6 d2d988    jp      nc,88d9h
1db9 09        add     hl,bc
1dba b6        or      (hl)
1dbb 4c        ld      c,h
1dbc 2b        dec     hl
1dbd 7e        ld      a,(hl)
1dbe b1        or      c
1dbf 7c        ld      a,h
1dc0 bd        cp      l
1dc1 e7        rst     20h
1dc2 b8        cp      b
1dc3 2d        dec     l
1dc4 07        rlca    
1dc5 90        sub     b
1dc6 bf        cp      a
1dc7 1d        dec     e
1dc8 91        sub     c
1dc9 1d        dec     e
1dca b7        or      a
1dcb 1064      djnz    1e31h
1dcd 6a        ld      l,d
1dce b0        or      b
1dcf 20f2      jr      nz,1dc3h
1dd1 f3        di      
1dd2 b9        cp      c
1dd3 71        ld      (hl),c
1dd4 48        ld      c,b
1dd5 84        add     a,h
1dd6 be        cp      (hl)
1dd7 41        ld      b,c
1dd8 de1a      sbc     a,1ah
1dda dad47d    jp      c,7dd4h
1ddd 6d        ld      l,l
1dde dde4ebf4  call    po,0f4ebh
1de2 d4b551    call    nc,51b5h
1de5 83        add     a,e
1de6 d385      out     (85h),a
1de8 c7        rst     00h
1de9 13        inc     de
1dea 6c        ld      l,h
1deb 98        sbc     a,b
1dec 56        ld      d,(hl)
1ded 64        ld      h,h
1dee 6b        ld      l,e
1def a8        xor     b
1df0 c0        ret     nz

1df1 fd62      ld      iyh,d
1df3 f9        ld      sp,hl
1df4 7a        ld      a,d
1df5 8a        adc     a,d
1df6 65        ld      h,l
1df7 c9        ret     

1df8 ec1401    call    pe,0114h
1dfb 5c        ld      e,h
1dfc 4f        ld      c,a
1dfd 63        ld      h,e
1dfe 066c      ld      b,6ch
1e00 d9        exx     
1e01 fa0f3d    jp      m,3d0fh
1e04 63        ld      h,e
1e05 8d        adc     a,l
1e06 08        ex      af,af'
1e07 0d        dec     c
1e08 f5        push    af
1e09 3b        dec     sp
1e0a 6e        ld      l,(hl)
1e0b 20c8      jr      nz,1dd5h
1e0d 4c        ld      c,h
1e0e 69        ld      l,c
1e0f 105e      djnz    1e6fh
1e11 d5        push    de
1e12 60        ld      h,b
1e13 41        ld      b,c
1e14 e4a267    call    po,67a2h
1e17 71        ld      (hl),c
1e18 72        ld      (hl),d
1e19 3c        inc     a
1e1a 03        inc     bc
1e1b e4d14b    call    po,4bd1h
1e1e 04        inc     b
1e1f d447d2    call    nc,0d247h
1e22 0d        dec     c
1e23 85        add     a,l
1e24 fda5      and     iyl
1e26 0a        ld      a,(bc)
1e27 b5        or      l
1e28 6b        ld      l,e
1e29 35        dec     (hl)
1e2a b5        or      l
1e2b a8        xor     b
1e2c fa42b2    jp      m,0b242h
1e2f 98        sbc     a,b
1e30 6c        ld      l,h
1e31 dbbb      in      a,(0bbh)
1e33 c9        ret     

1e34 d6ac      sub     0ach
1e36 bc        cp      h
1e37 f9        ld      sp,hl
1e38 40        ld      b,b
1e39 32d86c    ld      (6cd8h),a
1e3c e3        ex      (sp),hl
1e3d 45        ld      b,l
1e3e df        rst     18h
1e3f 5c        ld      e,h
1e40 75        ld      (hl),l
1e41 dcd60d    call    c,0dd6h
1e44 cf        rst     08h
1e45 ab        xor     e
1e46 d1        pop     de
1e47 3d        dec     a
1e48 59        ld      e,c
1e49 26d9      ld      h,0d9h
1e4b 30ac      jr      nc,1df9h
1e4d 51        ld      d,c
1e4e de00      sbc     a,00h
1e50 3ac8d7    ld      a,(0d7c8h)
1e53 51        ld      d,c
1e54 80        add     a,b
1e55 bf        cp      a
1e56 d0        ret     nc

1e57 61        ld      h,c
1e58 1621      ld      d,21h
1e5a b4        or      h
1e5b f4b556    call    p,56b5h
1e5e b3        or      e
1e5f c423cf    call    nz,0cf23h
1e62 ba        cp      d
1e63 95        sub     l
1e64 99        sbc     a,c
1e65 b8        cp      b
1e66 bd        cp      l
1e67 a5        and     l
1e68 0f        rrca    
1e69 2802      jr      z,1e6dh
1e6b b8        cp      b
1e6c 9e        sbc     a,(hl)
1e6d 5f        ld      e,a
1e6e 05        dec     b
1e6f 88        adc     a,b
1e70 08        ex      af,af'
1e71 c60c      add     a,0ch
1e73 d9        exx     
1e74 b2        or      d
1e75 b1        or      c
1e76 0b        dec     bc
1e77 e9        jp      (hl)
1e78 24        inc     h
1e79 2f        cpl     
1e7a 6f        ld      l,a
1e7b 7c        ld      a,h
1e7c 87        add     a,a
1e7d 58        ld      e,b
1e7e 68        ld      l,b
1e7f 4c        ld      c,h
1e80 11c161    ld      de,61c1h
1e83 1d        dec     e
1e84 ab        xor     e
1e85 b6        or      (hl)
1e86 66        ld      h,(hl)
1e87 2d        dec     l
1e88 3d        dec     a
1e89 76        halt    
1e8a dc4190    call    c,9041h
1e8d 01db71    ld      bc,71dbh
1e90 0698      ld      b,98h
1e92 d220bc    jp      nc,0bc20h
1e95 ef        rst     28h
1e96 d5        push    de
1e97 102a      djnz    1ec3h
1e99 71        ld      (hl),c
1e9a b1        or      c
1e9b 85        add     a,l
1e9c 89        adc     a,c
1e9d 06b6      ld      b,0b6h
1e9f b5        or      l
1ea0 1f        rra     
1ea1 9f        sbc     a,a
1ea2 bf        cp      a
1ea3 e4a5e8    call    po,0e8a5h
1ea6 b8        cp      b
1ea7 d43378    call    nc,7833h
1eaa 07        rlca    
1eab c9        ret     

1eac a2        and     d
1ead 0f        rrca    
1eae 00        nop     
1eaf f9        ld      sp,hl
1eb0 34        inc     (hl)
1eb1 96        sub     (hl)
1eb2 09        add     hl,bc
1eb3 a8        xor     b
1eb4 8e        adc     a,(hl)
1eb5 e1        pop     hl
1eb6 0e98      ld      c,98h
1eb8 187f      jr      1f39h
1eba 6a        ld      l,d
1ebb 0d        dec     c
1ebc bb        cp      e
1ebd 08        ex      af,af'
1ebe 6d        ld      l,l
1ebf 3d        dec     a
1ec0 2d        dec     l
1ec1 91        sub     c
1ec2 64        ld      h,h
1ec3 6c        ld      l,h
1ec4 97        sub     a
1ec5 e663      and     63h
1ec7 5c        ld      e,h
1ec8 016b6b    ld      bc,6b6bh
1ecb 51        ld      d,c
1ecc f41c6c    call    p,6c1ch
1ecf 61        ld      h,c
1ed0 62        ld      h,d
1ed1 85        add     a,l
1ed2 65        ld      h,l
1ed3 30d8      jr      nc,1eadh
1ed5 f26200    jp      p,0062h
1ed8 4e        ld      c,(hl)
1ed9 6c        ld      l,h
1eda 0695      ld      b,95h
1edc ed1b      db      0edh, 1bh        ; Undocumented 8 T-State NOP
1ede 01a57b    ld      bc,7ba5h
1ee1 82        add     a,d
1ee2 08        ex      af,af'
1ee3 f4c1f5    call    p,0f5c1h
1ee6 0f        rrca    
1ee7 c45765    call    nz,6557h
1eea b0        or      b
1eeb d9        exx     
1eec c612      add     a,12h
1eee b7        or      a
1eef e9        jp      (hl)
1ef0 50        ld      d,b
1ef1 8b        adc     a,e
1ef2 be        cp      (hl)
1ef3 b8        cp      b
1ef4 eafcb9    jp      pe,0b9fch
1ef7 88        adc     a,b
1ef8 7c        ld      a,h
1ef9 62        ld      h,d
1efa dd1d      dec     e
1efc df        rst     18h
1efd 15        dec     d
1efe da2d49    jp      c,492dh
1f01 8c        adc     a,h
1f02 d37c      out     (7ch),a
1f04 f3        di      
1f05 fb        ei      
1f06 d44c65    call    nc,654ch
1f09 4d        ld      c,l
1f0a b2        or      d
1f0b 61        ld      h,c
1f0c 58        ld      e,b
1f0d 3ab551    ld      a,(51b5h)
1f10 cea3      adc     a,0a3h
1f12 bc        cp      h
1f13 00        nop     
1f14 74        ld      (hl),h
1f15 d4bb30    call    nc,30bbh
1f18 e24adf    jp      po,0df4ah
1f1b a5        and     l
1f1c 41        ld      b,c
1f1d 3d        dec     a
1f1e d8        ret     c

1f1f 95        sub     l
1f20 d7        rst     10h
1f21 a4        and     h
1f22 d1        pop     de
1f23 c46dd3    call    nz,0d36dh
1f26 d6f4      sub     0f4h
1f28 fb        ei      
1f29 43        ld      b,e
1f2a 69        ld      l,c
1f2b e9        jp      (hl)
1f2c 6a        ld      l,d
1f2d 34        inc     (hl)
1f2e 6e        ld      l,(hl)
1f2f d9        exx     
1f30 fcad67    call    m,67adh
1f33 88        adc     a,b
1f34 46        ld      b,(hl)
1f35 da60b8    jp      c,0b860h
1f38 d0        ret     nc

1f39 44        ld      b,h
1f3a 04        inc     b
1f3b 2d        dec     l
1f3c 73        ld      (hl),e
1f3d 33        inc     sp
1f3e 03        inc     bc
1f3f 1d        dec     e
1f40 e5        push    hl
1f41 aa        xor     d
1f42 0a        ld      a,(bc)
1f43 4c        ld      c,h
1f44 5f        ld      e,a
1f45 dd0d      dec     c
1f47 7c        ld      a,h
1f48 c9        ret     

1f49 50        ld      d,b
1f4a 05        dec     b
1f4b 71        ld      (hl),c
1f4c 3c        inc     a
1f4d 27        daa     
1f4e 02        ld      (bc),a
1f4f 41        ld      b,c
1f50 aa        xor     d
1f51 be        cp      (hl)
1f52 0b        dec     bc
1f53 1010      djnz    1f65h
1f55 c9        ret     

1f56 0c        inc     c
1f57 2086      jr      nz,1edfh
1f59 57        ld      d,a
1f5a 68        ld      l,b
1f5b b5        or      l
1f5c 25        dec     h
1f5d 206f      jr      nz,1fceh
1f5f 85        add     a,l
1f60 b3        or      e
1f61 b9        cp      c
1f62 66        ld      h,(hl)
1f63 d409ce    call    nc,0ce09h
1f66 61        ld      h,c
1f67 e49f5e    call    po,5e9fh
1f6a def9      sbc     a,0f9h
1f6c 0e29      ld      c,29h
1f6e d9        exx     
1f6f c9        ret     

1f70 98        sbc     a,b
1f71 b0        or      b
1f72 d0        ret     nc

1f73 98        sbc     a,b
1f74 22c7d7    ld      (0d7c7h),hl
1f77 a8        xor     b
1f78 b4        or      h
1f79 59        ld      e,c
1f7a b3        or      e
1f7b 3d        dec     a
1f7c 17        rla     
1f7d 2eb4      ld      l,0b4h
1f7f 0d        dec     c
1f80 81        add     a,c
1f81 b7        or      a
1f82 bd        cp      l
1f83 5c        ld      e,h
1f84 3b        dec     sp
1f85 c0        ret     nz

1f86 ba        cp      d
1f87 6c        ld      l,h
1f88 ad        xor     l
1f89 edb8      lddr    
1f8b 83        add     a,e
1f8c 209a      jr      nz,1f28h
1f8e bf        cp      a
1f8f b3        or      e
1f90 b6        or      (hl)
1f91 03        inc     bc
1f92 b6        or      (hl)
1f93 e20c74    jp      po,740ch
1f96 b1        or      c
1f97 d29aea    jp      nc,0ea9ah
1f9a d5        push    de
1f9b 47        ld      b,a
1f9c 39        add     hl,sp
1f9d 9d        sbc     a,l
1f9e d277af    jp      nc,0af77h
1fa1 04        inc     b
1fa2 db26      in      a,(26h)
1fa4 15        dec     d
1fa5 73        ld      (hl),e
1fa6 dc1683    call    c,8316h
1fa9 e3        ex      (sp),hl
1faa 63        ld      h,e
1fab 0b        dec     bc
1fac 12        ld      (de),a
1fad 94        sub     h
1fae 64        ld      h,h
1faf 3b        dec     sp
1fb0 84        add     a,h
1fb1 0d        dec     c
1fb2 6d        ld      l,l
1fb3 6a        ld      l,d
1fb4 3e7a      ld      a,7ah
1fb6 6a        ld      l,d
1fb7 5a        ld      e,d
1fb8 a8        xor     b
1fb9 e40ecf    call    po,0cf0eh
1fbc 0b        dec     bc
1fbd 93        sub     e
1fbe 09        add     hl,bc
1fbf ff        rst     38h
1fc0 9d        sbc     a,l
1fc1 0a        ld      a,(bc)
1fc2 00        nop     
1fc3 ae        xor     (hl)
1fc4 27        daa     
1fc5 7d        ld      a,l
1fc6 07        rlca    
1fc7 9e        sbc     a,(hl)
1fc8 b1        or      c
1fc9 f0        ret     p

1fca 0f        rrca    
1fcb 93        sub     e
1fcc 44        ld      b,h
1fcd 87        add     a,a
1fce 08        ex      af,af'
1fcf a3        and     e
1fd0 d21e01    jp      nc,011eh
1fd3 f26869    jp      p,6968h
1fd6 06c2      ld      b,0c2h
1fd8 fef7      cp      0f7h
1fda 62        ld      h,d
1fdb 57        ld      d,a
1fdc 5d        ld      e,l
1fdd 80        add     a,b
1fde 65        ld      h,l
1fdf 67        ld      h,a
1fe0 cb19      rr      c
1fe2 6c        ld      l,h
1fe3 3671      ld      (hl),71h
1fe5 6e        ld      l,(hl)
1fe6 6b        ld      l,e
1fe7 06e7      ld      b,0e7h
1fe9 fed4      cp      0d4h
1feb 1b        dec     de
1fec 76        halt    
1fed 89        adc     a,c
1fee d32b      out     (2bh),a
1ff0 e0        ret     po

1ff1 10da      djnz    1fcdh
1ff3 7a        ld      a,d
1ff4 5a        ld      e,d
1ff5 67        ld      h,a
1ff6 dd4a      ld      c,d
1ff8 ccf9b9    call    z,0b9f9h
1ffb df        rst     18h
1ffc 6f        ld      l,a
1ffd 8e        adc     a,(hl)
1ffe be        cp      (hl)
1fff ef        rst     28h
2000 f9        ld      sp,hl
2001 17        rla     
2002 b7        or      a
2003 be        cp      (hl)
2004 43        ld      b,e
2005 60        ld      h,b
2006 b0        or      b
2007 8e        adc     a,(hl)
2008 d5        push    de
2009 d6d6      sub     0d6h
200b a3        and     e
200c e8        ret     pe

200d a1        and     c
200e d1        pop     de
200f 93        sub     e
2010 7e        ld      a,(hl)
2011 38d8      jr      c,1febh
2013 c2c44f    jp      nz,4fc4h
2016 df        rst     18h
2017 f252d1    jp      p,0d152h
201a bb        cp      e
201b 67        ld      h,a
201c f1        pop     af
201d a6        and     (hl)
201e bc        cp      h
201f 57        ld      d,a
2020 67        ld      h,a
2021 3f        ccf     
2022 b5        or      l
2023 06dd      ld      b,0ddh
2025 48        ld      c,b
2026 b2        or      d
2027 364b      ld      (hl),4bh
2029 d8        ret     c

202a 0d        dec     c
202b 2b        dec     hl
202c daaf0a    jp      c,0aafh
202f 1b        dec     de
2030 4c        ld      c,h
2031 3603      ld      (hl),03h
2033 4a        ld      c,d
2034 f641      or      41h
2036 04        inc     b
2037 7a        ld      a,d
2038 60        ld      h,b
2039 df        rst     18h
203a 60        ld      h,b
203b ef        rst     28h
203c c3a867    jp      67a8h
203f df        rst     18h
2040 55        ld      d,l
2041 316e8e    ld      sp,8e6eh
2044 ef        rst     28h
2045 46        ld      b,(hl)
2046 69        ld      l,c
2047 be        cp      (hl)
2048 79        ld      a,c
2049 cb61      bit     4,c
204b b3        or      e
204c 8c        adc     a,h
204d bc        cp      h
204e 66        ld      h,(hl)
204f 83        add     a,e
2050 1a        ld      a,(de)
2051 25        dec     h
2052 6f        ld      l,a
2053 d2a052    jp      nc,52a0h
2056 68        ld      l,b
2057 e236cc    jp      po,0cc36h
205a 0c        inc     c
205b 77        ld      (hl),a
205c 95        sub     l
205d bb        cp      e
205e 0b        dec     bc
205f 47        ld      b,a
2060 03        inc     bc
2061 220216    ld      (1602h),hl
2064 b9        cp      c
2065 55        ld      d,l
2066 05        dec     b
2067 262f      ld      h,2fh
2069 c5        push    bc
206a ba        cp      d
206b 3b        dec     sp
206c be        cp      (hl)
206d b2        or      d
206e bd        cp      l
206f 0b        dec     bc
2070 282b      jr      z,209dh
2072 b4        or      h
2073 5a        ld      e,d
2074 92        sub     d
2075 5c        ld      e,h
2076 b3        or      e
2077 6a        ld      l,d
2078 04        inc     b
2079 c2d7ff    jp      nz,0ffd7h
207c a7        and     a
207d b5        or      l
207e d0        ret     nc

207f cf        rst     08h
2080 312cd9    ld      sp,0d92ch
2083 9e        sbc     a,(hl)
2084 8b        adc     a,e
2085 5b        ld      e,e
2086 deae      sbc     a,0aeh
2088 1d        dec     e
2089 9b        sbc     a,e
208a 64        ld      h,h
208b c2b0ec    jp      nz,0ecb0h
208e 63        ld      h,e
208f f22675    jp      p,7526h
2092 6a        ld      l,d
2093 a3        and     e
2094 9c        sbc     a,h
2095 02        ld      (bc),a
2096 6d        ld      l,l
2097 93        sub     e
2098 0a        ld      a,(bc)
2099 9c        sbc     a,h
209a 09        add     hl,bc
209b 06a9      ld      b,0a9h
209d eb        ex      de,hl
209e 0e36      ld      c,36h
20a0 3f        ccf     
20a1 72        ld      (hl),d
20a2 07        rlca    
20a3 67        ld      h,a
20a4 85        add     a,l
20a5 05        dec     b
20a6 00        nop     
20a7 57        ld      d,a
20a8 13        inc     de
20a9 95        sub     l
20aa bf        cp      a
20ab 4a        ld      c,d
20ac 82        add     a,d
20ad e2b87a    jp      po,7ab8h
20b0 14        inc     d
20b1 7b        ld      a,e
20b2 b1        or      c
20b3 2b        dec     hl
20b4 ae        xor     (hl)
20b5 0c        inc     c
20b6 b6        or      (hl)
20b7 1b        dec     de
20b8 3892      jr      c,204ch
20ba d28e9b    jp      nc,9b8eh
20bd e5        push    hl
20be d5        push    de
20bf be        cp      (hl)
20c0 0d        dec     c
20c1 7c        ld      a,h
20c2 dcefb7    call    c,0b7efh
20c5 0b        dec     bc
20c6 dbdf      in      a,(0dfh)
20c8 2186d3    ld      hl,0d386h
20cb d2d4f1    jp      nc,0f1d4h
20ce d4e242    call    nc,42e2h
20d1 68        ld      l,b
20d2 ddb3      or      e
20d4 f8        ret     m

20d5 1f        rra     
20d6 da836e    jp      c,6e83h
20d9 81        add     a,c
20da be        cp      (hl)
20db 16cd      ld      d,0cdh
20dd f6b9      or      0b9h
20df 265b      ld      h,5bh
20e1 6f        ld      l,a
20e2 b0        or      b
20e3 77        ld      (hl),a
20e4 e1        pop     hl
20e5 18b7      jr      209eh
20e7 47        ld      b,a
20e8 77        ld      (hl),a
20e9 88        adc     a,b
20ea 08        ex      af,af'
20eb 5a        ld      e,d
20ec e6ff      and     0ffh
20ee 0f        rrca    
20ef 6a        ld      l,d
20f0 70        ld      (hl),b
20f1 66        ld      h,(hl)
20f2 063b      ld      b,3bh
20f4 ca1101    jp      z,0111h
20f7 0b        dec     bc
20f8 5c        ld      e,h
20f9 8f        adc     a,a
20fa 65        ld      h,l
20fb 9e        sbc     a,(hl)
20fc ff        rst     38h
20fd f8        ret     m

20fe 62        ld      h,d
20ff ae        xor     (hl)
2100 69        ld      l,c
2101 61        ld      h,c
2102 6b        ld      l,e
2103 ff        rst     38h
2104 d316      out     (16h),a
2106 6c        ld      l,h
2107 cf        rst     08h
2108 45        ld      b,l
2109 a0        and     b
210a 0a        ld      a,(bc)
210b e278d7    jp      po,0d778h
210e 0d        dec     c
210f d2ee4e    jp      nc,4eeeh
2112 04        inc     b
2113 83        add     a,e
2114 54        ld      d,h
2115 39        add     hl,sp
2116 03        inc     bc
2117 b3        or      e
2118 c2a767    jp      nz,67a7h
211b 2661      ld      h,61h
211d d0        ret     nc

211e 60        ld      h,b
211f 16f7      ld      d,0f7h
2121 49        ld      c,c
2122 69        ld      l,c
2123 47        ld      b,a
2124 4d        ld      c,l
2125 3e6e      ld      a,6eh
2127 77        ld      (hl),a
2128 dbae      in      a,(0aeh)
212a d1        pop     de
212b 6a        ld      l,d
212c 4a        ld      c,d
212d d9        exx     
212e d65a      sub     5ah
2130 dc40df    call    c,0df40h
2133 0b        dec     bc
2134 66        ld      h,(hl)
2135 37        scf     
2136 d8        ret     c

2137 3b        dec     sp
2138 f0        ret     p

2139 a9        xor     c
213a bc        cp      h
213b ae        xor     (hl)
213c 53        ld      d,e
213d debb      sbc     a,0bbh
213f 9e        sbc     a,(hl)
2140 c5        push    bc
2141 47        ld      b,a
2142 b2        or      d
2143 cf        rst     08h
2144 7f        ld      a,a
2145 30b5      jr      nc,20fch
2147 ff        rst     38h
2148 e9        jp      (hl)
2149 bd        cp      l
214a bd        cp      l
214b f21cca    jp      p,0ca1ch
214e ba        cp      d
214f c28a53    jp      nz,538ah
2152 b3        or      e
2153 93        sub     e
2154 3024      jr      nc,217ah
2156 b4        or      h
2157 a3        and     e
2158 a6        and     (hl)
2159 ba        cp      d
215a d0        ret     nc

215b 3605      ld      (hl),05h
215d cdd706    call    06d7h
2160 93        sub     e
2161 54        ld      d,h
2162 de57      sbc     a,57h
2164 29        add     hl,hl
2165 23        inc     hl
2166 d9        exx     
2167 67        ld      h,a
2168 bf        cp      a
2169 b3        or      e
216a 66        ld      h,(hl)
216b 7a        ld      a,d
216c 2ec4      ld      l,0c4h
216e 61        ld      h,c
216f 4a        ld      c,d
2170 b8        cp      b
2171 5d        ld      e,l
2172 68        ld      l,b
2173 1b        dec     de
2174 02        ld      (bc),a
2175 2a6f2b    ld      hl,(2b6fh)
2178 94        sub     h
2179 b4        or      h
217a 0b        dec     bc
217b be        cp      (hl)
217c 37        scf     
217d c30c8e    jp      8e0ch
2180 a1        and     c
2181 5a        ld      e,d
2182 05        dec     b
2183 df        rst     18h
2184 1b        dec     de
2185 2d        dec     l
2186 02        ld      (bc),a
2187 ef        rst     28h
2188 8d        adc     a,l
2189 cd2bc9    call    0c92bh
218c cda83d    call    3da8h
218f c3ef3c    jp      3cefh
2192 ddcb28c6  set     0,(ix+28h)
2196 12        ld      (de),a
2197 13        inc     de
2198 7e        ld      a,(hl)
2199 23        inc     hl
219a 12        ld      (de),a
219b 13        inc     de
219c fe0d      cp      0dh
219e 2810      jr      z,21b0h
21a0 fe27      cp      27h
21a2 20f4      jr      nz,2198h
21a4 eb        ex      de,hl
21a5 cdc039    call    39c0h
21a8 eb        ex      de,hl
21a9 ddcb2846  bit     0,(ix+28h)
21ad c8        ret     z

21ae 18e8      jr      2198h
21b0 2b        dec     hl
21b1 1b        dec     de
21b2 c9        ret     

21b3 e1        pop     hl
21b4 d1        pop     de
21b5 1b        dec     de
21b6 1a        ld      a,(de)
21b7 fe26      cp      26h
21b9 2801      jr      z,21bch
21bb 13        inc     de
21bc d5        push    de
21bd e9        jp      (hl)
21be 0e01      ld      c,01h
21c0 dd4614    ld      b,(ix+14h)
21c3 2a3541    ld      hl,(4135h)
21c6 1a        ld      a,(de)
21c7 fe0d      cp      0dh
21c9 2821      jr      z,21ech
21cb cd3f3c    call    3c3fh
21ce 280f      jr      z,21dfh
21d0 be        cp      (hl)
21d1 200c      jr      nz,21dfh
21d3 2b        dec     hl
21d4 1b        dec     de
21d5 10ef      djnz    21c6h
21d7 1a        ld      a,(de)
21d8 cd3f3c    call    3c3fh
21db 2002      jr      nz,21dfh
21dd b7        or      a
21de c9        ret     

21df 1a        ld      a,(de)
21e0 1b        dec     de
21e1 fe0d      cp      0dh
21e3 2807      jr      z,21ech
21e5 fe2c      cp      2ch
21e7 20f6      jr      nz,21dfh
21e9 0c        inc     c
21ea 18d4      jr      21c0h
21ec 37        scf     
21ed c9        ret     

21ee 3a3f41    ld      a,(413fh)
21f1 b7        or      a
21f2 2829      jr      z,221dh
21f4 47        ld      b,a
21f5 ed5b3b41  ld      de,(413bh)
21f9 214e42    ld      hl,424eh
21fc 3aad15    ld      a,(15adh)
21ff b7        or      a
