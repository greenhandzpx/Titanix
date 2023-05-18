
shell:     file format elf64-littleriscv


Disassembly of section .text:

0000000000010000 <_start>:
   10000:	7119                	add	sp,sp,-128
   10002:	fc86                	sd	ra,120(sp)
   10004:	f8a2                	sd	s0,112(sp)
   10006:	f4a6                	sd	s1,104(sp)
   10008:	f0ca                	sd	s2,96(sp)
   1000a:	ecce                	sd	s3,88(sp)
   1000c:	e8d2                	sd	s4,80(sp)
   1000e:	e4d6                	sd	s5,72(sp)
   10010:	e0da                	sd	s6,64(sp)
   10012:	0100                	add	s0,sp,128
   10014:	89ae                	mv	s3,a1
   10016:	892a                	mv	s2,a0

0000000000010018 <.LBB2_13>:
   10018:	00038517          	auipc	a0,0x38
   1001c:	35850513          	add	a0,a0,856 # 48370 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E>
   10020:	00001097          	auipc	ra,0x1
   10024:	010080e7          	jalr	16(ra) # 11030 <_ZN78_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..ops..deref..Deref$GT$5deref17h07e5175ddf4fbb7aE>
   10028:	8a2a                	mv	s4,a0
   1002a:	4505                	li	a0,1
   1002c:	00aa34af          	amoadd.d	s1,a0,(s4)
   10030:	008a3503          	ld	a0,8(s4)
   10034:	0230000f          	fence	r,rw
   10038:	00950a63          	beq	a0,s1,1004c <.LBB2_13+0x34>
   1003c:	0100000f          	fence	w,unknown
   10040:	008a3503          	ld	a0,8(s4)
   10044:	0230000f          	fence	r,rw
   10048:	fe951ae3          	bne	a0,s1,1003c <.LBB2_13+0x24>
   1004c:	010a0513          	add	a0,s4,16

0000000000010050 <.LBB2_14>:
   10050:	00006597          	auipc	a1,0x6
   10054:	32058593          	add	a1,a1,800 # 16370 <_ZN8user_lib10HEAP_SPACE17h90582ce43c94d355E>
   10058:	00032637          	lui	a2,0x32
   1005c:	00001097          	auipc	ra,0x1
   10060:	ab0080e7          	jalr	-1360(ra) # 10b0c <_ZN22buddy_system_allocator4Heap4init17h0387239a34ee507aE>
   10064:	00148513          	add	a0,s1,1
   10068:	0310000f          	fence	rw,w
   1006c:	00aa3423          	sd	a0,8(s4)
   10070:	4521                	li	a0,8
   10072:	f8a43023          	sd	a0,-128(s0)
   10076:	f8043423          	sd	zero,-120(s0)
   1007a:	f8043823          	sd	zero,-112(s0)
   1007e:	00091e63          	bnez	s2,1009a <.LBB2_14+0x4a>
   10082:	45a1                	li	a1,8
   10084:	4501                	li	a0,0
   10086:	4601                	li	a2,0
   10088:	00000097          	auipc	ra,0x0
   1008c:	440080e7          	jalr	1088(ra) # 104c8 <main>
   10090:	00000097          	auipc	ra,0x0
   10094:	7aa080e7          	jalr	1962(ra) # 1083a <_ZN8user_lib4exit17h20483dcbea918787E>
   10098:	0000                	unimp
   1009a:	4481                	li	s1,0
   1009c:	fa040a13          	add	s4,s0,-96
   100a0:	a015                	j	100c4 <.LBB2_14+0x74>
   100a2:	f8043503          	ld	a0,-128(s0)
   100a6:	0592                	sll	a1,a1,0x4
   100a8:	952e                	add	a0,a0,a1
   100aa:	01553023          	sd	s5,0(a0)
   100ae:	01653423          	sd	s6,8(a0)
   100b2:	f9043503          	ld	a0,-112(s0)
   100b6:	0485                	add	s1,s1,1
   100b8:	00150613          	add	a2,a0,1
   100bc:	f8c43823          	sd	a2,-112(s0)
   100c0:	05248963          	beq	s1,s2,10112 <.LBB2_14+0xc2>
   100c4:	00349513          	sll	a0,s1,0x3
   100c8:	954e                	add	a0,a0,s3
   100ca:	610c                	ld	a1,0(a0)
   100cc:	567d                	li	a2,-1
   100ce:	00c58533          	add	a0,a1,a2
   100d2:	00154503          	lbu	a0,1(a0)
   100d6:	0605                	add	a2,a2,1 # 32001 <_ZN8user_lib10HEAP_SPACE17h90582ce43c94d355E+0x1bc91>
   100d8:	f97d                	bnez	a0,100ce <.LBB2_14+0x7e>
   100da:	f9840513          	add	a0,s0,-104
   100de:	00002097          	auipc	ra,0x2
   100e2:	3ce080e7          	jalr	974(ra) # 124ac <_ZN4core3str8converts9from_utf817hbe620603d93abf90E>
   100e6:	f9843503          	ld	a0,-104(s0)
   100ea:	e121                	bnez	a0,1012a <.LBB2_14+0xda>
   100ec:	fa043a83          	ld	s5,-96(s0)
   100f0:	f9043583          	ld	a1,-112(s0)
   100f4:	f8843503          	ld	a0,-120(s0)
   100f8:	fa843b03          	ld	s6,-88(s0)
   100fc:	faa593e3          	bne	a1,a0,100a2 <.LBB2_14+0x52>
   10100:	f8040513          	add	a0,s0,-128
   10104:	00001097          	auipc	ra,0x1
   10108:	8ce080e7          	jalr	-1842(ra) # 109d2 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E>
   1010c:	f9043583          	ld	a1,-112(s0)
   10110:	bf49                	j	100a2 <.LBB2_14+0x52>
   10112:	f8043583          	ld	a1,-128(s0)
   10116:	854a                	mv	a0,s2
   10118:	00000097          	auipc	ra,0x0
   1011c:	3b0080e7          	jalr	944(ra) # 104c8 <main>
   10120:	00000097          	auipc	ra,0x0
   10124:	71a080e7          	jalr	1818(ra) # 1083a <_ZN8user_lib4exit17h20483dcbea918787E>
   10128:	0000                	unimp
   1012a:	008a3503          	ld	a0,8(s4)
   1012e:	000a3583          	ld	a1,0(s4)
   10132:	faa43c23          	sd	a0,-72(s0)
   10136:	fab43823          	sd	a1,-80(s0)

000000000001013a <.LBB2_15>:
   1013a:	00004517          	auipc	a0,0x4
   1013e:	3c650513          	add	a0,a0,966 # 14500 <.Lanon.24728be23fbcdb17895d264a60dac18d.4>

0000000000010142 <.LBB2_16>:
   10142:	00004697          	auipc	a3,0x4
   10146:	3ee68693          	add	a3,a3,1006 # 14530 <.Lanon.24728be23fbcdb17895d264a60dac18d.5>

000000000001014a <.LBB2_17>:
   1014a:	00004717          	auipc	a4,0x4
   1014e:	40670713          	add	a4,a4,1030 # 14550 <.Lanon.24728be23fbcdb17895d264a60dac18d.6>
   10152:	02b00593          	li	a1,43
   10156:	fb040613          	add	a2,s0,-80
   1015a:	00001097          	auipc	ra,0x1
   1015e:	298080e7          	jalr	664(ra) # 113f2 <_ZN4core6result13unwrap_failed17h3c2e5884ed497eadE>
	...

0000000000010164 <_ZN5alloc7raw_vec11finish_grow17ha87c3f2986f23f49E.llvm.768705636110488888>:
   10164:	7179                	add	sp,sp,-48
   10166:	f406                	sd	ra,40(sp)
   10168:	f022                	sd	s0,32(sp)
   1016a:	ec26                	sd	s1,24(sp)
   1016c:	e84a                	sd	s2,16(sp)
   1016e:	e44e                	sd	s3,8(sp)
   10170:	1800                	add	s0,sp,48
   10172:	892e                	mv	s2,a1
   10174:	89aa                	mv	s3,a0
   10176:	ce19                	beqz	a2,10194 <_ZN5alloc7raw_vec11finish_grow17ha87c3f2986f23f49E.llvm.768705636110488888+0x30>
   10178:	84b2                	mv	s1,a2
   1017a:	6a88                	ld	a0,16(a3)
   1017c:	c10d                	beqz	a0,1019e <_ZN5alloc7raw_vec11finish_grow17ha87c3f2986f23f49E.llvm.768705636110488888+0x3a>
   1017e:	668c                	ld	a1,8(a3)
   10180:	cd99                	beqz	a1,1019e <_ZN5alloc7raw_vec11finish_grow17ha87c3f2986f23f49E.llvm.768705636110488888+0x3a>
   10182:	6288                	ld	a0,0(a3)
   10184:	8626                	mv	a2,s1
   10186:	86ca                	mv	a3,s2
   10188:	00000097          	auipc	ra,0x0
   1018c:	470080e7          	jalr	1136(ra) # 105f8 <__rust_realloc>
   10190:	e11d                	bnez	a0,101b6 <_ZN5alloc7raw_vec11finish_grow17ha87c3f2986f23f49E.llvm.768705636110488888+0x52>
   10192:	a011                	j	10196 <_ZN5alloc7raw_vec11finish_grow17ha87c3f2986f23f49E.llvm.768705636110488888+0x32>
   10194:	4481                	li	s1,0
   10196:	0129b423          	sd	s2,8(s3)
   1019a:	4585                	li	a1,1
   1019c:	a00d                	j	101be <_ZN5alloc7raw_vec11finish_grow17ha87c3f2986f23f49E.llvm.768705636110488888+0x5a>
   1019e:	00090a63          	beqz	s2,101b2 <_ZN5alloc7raw_vec11finish_grow17ha87c3f2986f23f49E.llvm.768705636110488888+0x4e>
   101a2:	854a                	mv	a0,s2
   101a4:	85a6                	mv	a1,s1
   101a6:	00000097          	auipc	ra,0x0
   101aa:	442080e7          	jalr	1090(ra) # 105e8 <__rust_alloc>
   101ae:	e501                	bnez	a0,101b6 <_ZN5alloc7raw_vec11finish_grow17ha87c3f2986f23f49E.llvm.768705636110488888+0x52>
   101b0:	b7dd                	j	10196 <_ZN5alloc7raw_vec11finish_grow17ha87c3f2986f23f49E.llvm.768705636110488888+0x32>
   101b2:	4901                	li	s2,0
   101b4:	8526                	mv	a0,s1
   101b6:	4581                	li	a1,0
   101b8:	00a9b423          	sd	a0,8(s3)
   101bc:	84ca                	mv	s1,s2
   101be:	0099b823          	sd	s1,16(s3)
   101c2:	00b9b023          	sd	a1,0(s3)
   101c6:	70a2                	ld	ra,40(sp)
   101c8:	7402                	ld	s0,32(sp)
   101ca:	64e2                	ld	s1,24(sp)
   101cc:	6942                	ld	s2,16(sp)
   101ce:	69a2                	ld	s3,8(sp)
   101d0:	6145                	add	sp,sp,48
   101d2:	8082                	ret

00000000000101d4 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h6e085ec00cce29f2E>:
   101d4:	715d                	add	sp,sp,-80
   101d6:	e486                	sd	ra,72(sp)
   101d8:	e0a2                	sd	s0,64(sp)
   101da:	fc26                	sd	s1,56(sp)
   101dc:	f84a                	sd	s2,48(sp)
   101de:	0880                	add	s0,sp,80
   101e0:	962e                	add	a2,a2,a1
   101e2:	06b66d63          	bltu	a2,a1,1025c <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h6e085ec00cce29f2E+0x88>
   101e6:	892a                	mv	s2,a0
   101e8:	6508                	ld	a0,8(a0)
   101ea:	00151493          	sll	s1,a0,0x1
   101ee:	02967463          	bgeu	a2,s1,10216 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h6e085ec00cce29f2E+0x42>
   101f2:	45a1                	li	a1,8
   101f4:	0295f563          	bgeu	a1,s1,1021e <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h6e085ec00cce29f2E+0x4a>
   101f8:	fff4c593          	not	a1,s1
   101fc:	03f5d613          	srl	a2,a1,0x3f
   10200:	c50d                	beqz	a0,1022a <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h6e085ec00cce29f2E+0x56>
   10202:	00093683          	ld	a3,0(s2)
   10206:	fff54593          	not	a1,a0
   1020a:	91fd                	srl	a1,a1,0x3f
   1020c:	fcd43423          	sd	a3,-56(s0)
   10210:	fca43823          	sd	a0,-48(s0)
   10214:	a821                	j	1022c <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h6e085ec00cce29f2E+0x58>
   10216:	84b2                	mv	s1,a2
   10218:	45a1                	li	a1,8
   1021a:	fc95efe3          	bltu	a1,s1,101f8 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h6e085ec00cce29f2E+0x24>
   1021e:	44a1                	li	s1,8
   10220:	fff4c593          	not	a1,s1
   10224:	03f5d613          	srl	a2,a1,0x3f
   10228:	fd69                	bnez	a0,10202 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h6e085ec00cce29f2E+0x2e>
   1022a:	4581                	li	a1,0
   1022c:	fcb43c23          	sd	a1,-40(s0)
   10230:	fb040513          	add	a0,s0,-80
   10234:	fc840693          	add	a3,s0,-56
   10238:	85a6                	mv	a1,s1
   1023a:	00000097          	auipc	ra,0x0
   1023e:	f2a080e7          	jalr	-214(ra) # 10164 <_ZN5alloc7raw_vec11finish_grow17ha87c3f2986f23f49E.llvm.768705636110488888>
   10242:	fb043583          	ld	a1,-80(s0)
   10246:	fb843503          	ld	a0,-72(s0)
   1024a:	cd91                	beqz	a1,10266 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h6e085ec00cce29f2E+0x92>
   1024c:	fc043583          	ld	a1,-64(s0)
   10250:	567d                	li	a2,-1
   10252:	167e                	sll	a2,a2,0x3f
   10254:	0605                	add	a2,a2,1
   10256:	00c58c63          	beq	a1,a2,1026e <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h6e085ec00cce29f2E+0x9a>
   1025a:	e185                	bnez	a1,1027a <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h6e085ec00cce29f2E+0xa6>
   1025c:	00001097          	auipc	ra,0x1
   10260:	e8e080e7          	jalr	-370(ra) # 110ea <_ZN5alloc7raw_vec17capacity_overflow17h8c103c8c1cb34845E>
   10264:	0000                	unimp
   10266:	00a93023          	sd	a0,0(s2)
   1026a:	00993423          	sd	s1,8(s2)
   1026e:	60a6                	ld	ra,72(sp)
   10270:	6406                	ld	s0,64(sp)
   10272:	74e2                	ld	s1,56(sp)
   10274:	7942                	ld	s2,48(sp)
   10276:	6161                	add	sp,sp,80
   10278:	8082                	ret
   1027a:	00001097          	auipc	ra,0x1
   1027e:	ea4080e7          	jalr	-348(ra) # 1111e <_ZN5alloc5alloc18handle_alloc_error17h48d9534aaf1ab1f8E>
	...

0000000000010284 <_ZN5shell16preliminary_test17h62fc12545a82fb4dE>:
   10284:	d5010113          	add	sp,sp,-688
   10288:	2a113423          	sd	ra,680(sp)
   1028c:	2a813023          	sd	s0,672(sp)
   10290:	28913c23          	sd	s1,664(sp)
   10294:	29213823          	sd	s2,656(sp)
   10298:	29313423          	sd	s3,648(sp)
   1029c:	29413023          	sd	s4,640(sp)
   102a0:	27513c23          	sd	s5,632(sp)
   102a4:	27613823          	sd	s6,624(sp)
   102a8:	27713423          	sd	s7,616(sp)
   102ac:	27813023          	sd	s8,608(sp)
   102b0:	25913c23          	sd	s9,600(sp)
   102b4:	1d00                	add	s0,sp,688

00000000000102b6 <.LBB0_17>:
   102b6:	00004517          	auipc	a0,0x4
   102ba:	d8250513          	add	a0,a0,-638 # 14038 <.Lanon.fad58de7366495db4650cfefac2fcd61.2>
   102be:	d4a43823          	sd	a0,-688(s0)
   102c2:	4485                	li	s1,1
   102c4:	d4943c23          	sd	s1,-680(s0)
   102c8:	d6043023          	sd	zero,-672(s0)

00000000000102cc <.LBB0_18>:
   102cc:	00004b17          	auipc	s6,0x4
   102d0:	d34b0b13          	add	s6,s6,-716 # 14000 <.Lanon.fad58de7366495db4650cfefac2fcd61.1>
   102d4:	d7643823          	sd	s6,-656(s0)
   102d8:	d6043c23          	sd	zero,-648(s0)
   102dc:	d5040513          	add	a0,s0,-688
   102e0:	00000097          	auipc	ra,0x0
   102e4:	46e080e7          	jalr	1134(ra) # 1074e <_ZN8user_lib7console5print17h2df775f81e475c40E>

00000000000102e8 <.LBB0_19>:
   102e8:	00004517          	auipc	a0,0x4
   102ec:	d6850513          	add	a0,a0,-664 # 14050 <.Lanon.fad58de7366495db4650cfefac2fcd61.4>
   102f0:	d4a43823          	sd	a0,-688(s0)
   102f4:	d4943c23          	sd	s1,-680(s0)
   102f8:	d6043023          	sd	zero,-672(s0)
   102fc:	d7643823          	sd	s6,-656(s0)
   10300:	d6043c23          	sd	zero,-648(s0)
   10304:	d5040513          	add	a0,s0,-688
   10308:	00000097          	auipc	ra,0x0
   1030c:	446080e7          	jalr	1094(ra) # 1074e <_ZN8user_lib7console5print17h2df775f81e475c40E>

0000000000010310 <.LBB0_20>:
   10310:	00004597          	auipc	a1,0x4
   10314:	e2058593          	add	a1,a1,-480 # 14130 <.Lanon.fad58de7366495db4650cfefac2fcd61.37>
   10318:	d5040513          	add	a0,s0,-688
   1031c:	20000613          	li	a2,512
   10320:	00003097          	auipc	ra,0x3
   10324:	536080e7          	jalr	1334(ra) # 13856 <memcpy>
   10328:	02000513          	li	a0,32
   1032c:	f4a43c23          	sd	a0,-168(s0)
   10330:	d5840b93          	add	s7,s0,-680
   10334:	5c7d                	li	s8,-1

0000000000010336 <.LBB0_21>:
   10336:	00004917          	auipc	s2,0x4
   1033a:	d3a90913          	add	s2,s2,-710 # 14070 <.Lanon.fad58de7366495db4650cfefac2fcd61.4+0x20>
   1033e:	4cfd                	li	s9,31
   10340:	002c0513          	add	a0,s8,2
   10344:	f4a43823          	sd	a0,-176(s0)
   10348:	ff8bba83          	ld	s5,-8(s7)
   1034c:	100a8763          	beqz	s5,1045a <.LBB0_22+0x3c>
   10350:	000bb983          	ld	s3,0(s7)
   10354:	00000097          	auipc	ra,0x0
   10358:	4f8080e7          	jalr	1272(ra) # 1084c <_ZN8user_lib4fork17h4831b1e32e249225E>
   1035c:	f6a43023          	sd	a0,-160(s0)
   10360:	c11d                	beqz	a0,10386 <.LBB0_21+0x50>
   10362:	84aa                	mv	s1,a0
   10364:	f6042623          	sw	zero,-148(s0)
   10368:	f6c40593          	add	a1,s0,-148
   1036c:	00000097          	auipc	ra,0x0
   10370:	520080e7          	jalr	1312(ra) # 1088c <_ZN8user_lib7waitpid17hc88733ebf8ce9b70E>
   10374:	f6a43823          	sd	a0,-144(s0)
   10378:	10a49a63          	bne	s1,a0,1048c <.LBB0_22+0x6e>
   1037c:	0c05                	add	s8,s8,1
   1037e:	0bc1                	add	s7,s7,16
   10380:	fd9c60e3          	bltu	s8,s9,10340 <.LBB0_21+0xa>
   10384:	a8d9                	j	1045a <.LBB0_22+0x3c>
   10386:	4a05                	li	s4,1
   10388:	02098163          	beqz	s3,103aa <.LBB0_21+0x74>
   1038c:	1209c263          	bltz	s3,104b0 <.LBB0_23+0x20>
   10390:	fff9c513          	not	a0,s3
   10394:	03f55493          	srl	s1,a0,0x3f
   10398:	854e                	mv	a0,s3
   1039a:	85a6                	mv	a1,s1
   1039c:	00000097          	auipc	ra,0x0
   103a0:	24c080e7          	jalr	588(ra) # 105e8 <__rust_alloc>
   103a4:	8a2a                	mv	s4,a0
   103a6:	10050a63          	beqz	a0,104ba <.LBB0_23+0x2a>
   103aa:	8552                	mv	a0,s4
   103ac:	85d6                	mv	a1,s5
   103ae:	864e                	mv	a2,s3
   103b0:	00003097          	auipc	ra,0x3
   103b4:	4a6080e7          	jalr	1190(ra) # 13856 <memcpy>
   103b8:	f7443c23          	sd	s4,-136(s0)
   103bc:	f9343023          	sd	s3,-128(s0)
   103c0:	f9343423          	sd	s3,-120(s0)
   103c4:	f7840513          	add	a0,s0,-136
   103c8:	4605                	li	a2,1
   103ca:	85ce                	mv	a1,s3
   103cc:	00000097          	auipc	ra,0x0
   103d0:	e08080e7          	jalr	-504(ra) # 101d4 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h6e085ec00cce29f2E>
   103d4:	f8843503          	ld	a0,-120(s0)
   103d8:	f7843483          	ld	s1,-136(s0)
   103dc:	00a485b3          	add	a1,s1,a0
   103e0:	00058023          	sb	zero,0(a1)
   103e4:	f8043983          	ld	s3,-128(s0)
   103e8:	00150593          	add	a1,a0,1
   103ec:	f6943c23          	sd	s1,-136(s0)
   103f0:	f8043023          	sd	zero,-128(s0)
   103f4:	f7840613          	add	a2,s0,-136
   103f8:	4689                	li	a3,2
   103fa:	4785                	li	a5,1
   103fc:	8526                	mv	a0,s1
   103fe:	874a                	mv	a4,s2
   10400:	00000097          	auipc	ra,0x0
   10404:	470080e7          	jalr	1136(ra) # 10870 <_ZN8user_lib6execve17h3e0a48d92ed78303E>
   10408:	e919                	bnez	a0,1041e <.LBB0_22>
   1040a:	f73059e3          	blez	s3,1037c <.LBB0_21+0x46>
   1040e:	4605                	li	a2,1
   10410:	8526                	mv	a0,s1
   10412:	85ce                	mv	a1,s3
   10414:	00000097          	auipc	ra,0x0
   10418:	1dc080e7          	jalr	476(ra) # 105f0 <__rust_dealloc>
   1041c:	b785                	j	1037c <.LBB0_21+0x46>

000000000001041e <.LBB0_22>:
   1041e:	00004517          	auipc	a0,0x4
   10422:	f8250513          	add	a0,a0,-126 # 143a0 <.Lanon.fad58de7366495db4650cfefac2fcd61.43>
   10426:	f6a43c23          	sd	a0,-136(s0)
   1042a:	4505                	li	a0,1
   1042c:	f8a43023          	sd	a0,-128(s0)
   10430:	f8043423          	sd	zero,-120(s0)
   10434:	f9643c23          	sd	s6,-104(s0)
   10438:	fa043023          	sd	zero,-96(s0)
   1043c:	f7840513          	add	a0,s0,-136
   10440:	00000097          	auipc	ra,0x0
   10444:	30e080e7          	jalr	782(ra) # 1074e <_ZN8user_lib7console5print17h2df775f81e475c40E>
   10448:	01305963          	blez	s3,1045a <.LBB0_22+0x3c>
   1044c:	4605                	li	a2,1
   1044e:	8526                	mv	a0,s1
   10450:	85ce                	mv	a1,s3
   10452:	00000097          	auipc	ra,0x0
   10456:	19e080e7          	jalr	414(ra) # 105f0 <__rust_dealloc>
   1045a:	2a813083          	ld	ra,680(sp)
   1045e:	2a013403          	ld	s0,672(sp)
   10462:	29813483          	ld	s1,664(sp)
   10466:	29013903          	ld	s2,656(sp)
   1046a:	28813983          	ld	s3,648(sp)
   1046e:	28013a03          	ld	s4,640(sp)
   10472:	27813a83          	ld	s5,632(sp)
   10476:	27013b03          	ld	s6,624(sp)
   1047a:	26813b83          	ld	s7,616(sp)
   1047e:	26013c03          	ld	s8,608(sp)
   10482:	25813c83          	ld	s9,600(sp)
   10486:	2b010113          	add	sp,sp,688
   1048a:	8082                	ret
   1048c:	f6043c23          	sd	zero,-136(s0)

0000000000010490 <.LBB0_23>:
   10490:	00004717          	auipc	a4,0x4
   10494:	ee070713          	add	a4,a4,-288 # 14370 <.Lanon.fad58de7366495db4650cfefac2fcd61.39>
   10498:	f6040593          	add	a1,s0,-160
   1049c:	f7040613          	add	a2,s0,-144
   104a0:	f7840693          	add	a3,s0,-136
   104a4:	4501                	li	a0,0
   104a6:	00000097          	auipc	ra,0x0
   104aa:	0ec080e7          	jalr	236(ra) # 10592 <_ZN4core9panicking13assert_failed17h990cdfb487308d09E>
   104ae:	0000                	unimp
   104b0:	00001097          	auipc	ra,0x1
   104b4:	c3a080e7          	jalr	-966(ra) # 110ea <_ZN5alloc7raw_vec17capacity_overflow17h8c103c8c1cb34845E>
   104b8:	0000                	unimp
   104ba:	854e                	mv	a0,s3
   104bc:	85a6                	mv	a1,s1
   104be:	00001097          	auipc	ra,0x1
   104c2:	c60080e7          	jalr	-928(ra) # 1111e <_ZN5alloc5alloc18handle_alloc_error17h48d9534aaf1ab1f8E>
	...

00000000000104c8 <main>:
   104c8:	7139                	add	sp,sp,-64
   104ca:	fc06                	sd	ra,56(sp)
   104cc:	f822                	sd	s0,48(sp)
   104ce:	0080                	add	s0,sp,64

00000000000104d0 <.LBB1_1>:
   104d0:	00004517          	auipc	a0,0x4
   104d4:	ee050513          	add	a0,a0,-288 # 143b0 <.Lanon.fad58de7366495db4650cfefac2fcd61.45>
   104d8:	fca43023          	sd	a0,-64(s0)
   104dc:	4505                	li	a0,1
   104de:	fca43423          	sd	a0,-56(s0)
   104e2:	fc043823          	sd	zero,-48(s0)

00000000000104e6 <.LBB1_2>:
   104e6:	00004517          	auipc	a0,0x4
   104ea:	b1a50513          	add	a0,a0,-1254 # 14000 <.Lanon.fad58de7366495db4650cfefac2fcd61.1>
   104ee:	fea43023          	sd	a0,-32(s0)
   104f2:	fe043423          	sd	zero,-24(s0)
   104f6:	fc040513          	add	a0,s0,-64
   104fa:	00000097          	auipc	ra,0x0
   104fe:	254080e7          	jalr	596(ra) # 1074e <_ZN8user_lib7console5print17h2df775f81e475c40E>
   10502:	00000097          	auipc	ra,0x0
   10506:	d82080e7          	jalr	-638(ra) # 10284 <_ZN5shell16preliminary_test17h62fc12545a82fb4dE>
   1050a:	4501                	li	a0,0
   1050c:	00000097          	auipc	ra,0x0
   10510:	32e080e7          	jalr	814(ra) # 1083a <_ZN8user_lib4exit17h20483dcbea918787E>
	...

0000000000010516 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hc87c089af8817bd3E>:
   10516:	1101                	add	sp,sp,-32
   10518:	ec06                	sd	ra,24(sp)
   1051a:	e822                	sd	s0,16(sp)
   1051c:	e426                	sd	s1,8(sp)
   1051e:	e04a                	sd	s2,0(sp)
   10520:	1000                	add	s0,sp,32
   10522:	00053903          	ld	s2,0(a0)
   10526:	84ae                	mv	s1,a1
   10528:	852e                	mv	a0,a1
   1052a:	00002097          	auipc	ra,0x2
   1052e:	c90080e7          	jalr	-880(ra) # 121ba <_ZN4core3fmt9Formatter15debug_lower_hex17h62bc36bb9a6deaceE>
   10532:	cd01                	beqz	a0,1054a <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hc87c089af8817bd3E+0x34>
   10534:	854a                	mv	a0,s2
   10536:	85a6                	mv	a1,s1
   10538:	60e2                	ld	ra,24(sp)
   1053a:	6442                	ld	s0,16(sp)
   1053c:	64a2                	ld	s1,8(sp)
   1053e:	6902                	ld	s2,0(sp)
   10540:	6105                	add	sp,sp,32
   10542:	00003317          	auipc	t1,0x3
   10546:	84030067          	jr	-1984(t1) # 12d82 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i64$GT$3fmt17h55cbfb91d25d94efE>
   1054a:	8526                	mv	a0,s1
   1054c:	00002097          	auipc	ra,0x2
   10550:	c78080e7          	jalr	-904(ra) # 121c4 <_ZN4core3fmt9Formatter15debug_upper_hex17h8071f907d66aecd7E>
   10554:	cd01                	beqz	a0,1056c <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hc87c089af8817bd3E+0x56>
   10556:	854a                	mv	a0,s2
   10558:	85a6                	mv	a1,s1
   1055a:	60e2                	ld	ra,24(sp)
   1055c:	6442                	ld	s0,16(sp)
   1055e:	64a2                	ld	s1,8(sp)
   10560:	6902                	ld	s2,0(sp)
   10562:	6105                	add	sp,sp,32
   10564:	00003317          	auipc	t1,0x3
   10568:	89c30067          	jr	-1892(t1) # 12e00 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$3fmt17h1e56d5f13d948128E>
   1056c:	854a                	mv	a0,s2
   1056e:	85a6                	mv	a1,s1
   10570:	60e2                	ld	ra,24(sp)
   10572:	6442                	ld	s0,16(sp)
   10574:	64a2                	ld	s1,8(sp)
   10576:	6902                	ld	s2,0(sp)
   10578:	6105                	add	sp,sp,32
   1057a:	00003317          	auipc	t1,0x3
   1057e:	d0830067          	jr	-760(t1) # 13282 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i64$GT$3fmt17h23de1c985db3efdcE>

0000000000010582 <_ZN4core3ptr30drop_in_place$LT$$RF$isize$GT$17h8817d980ee83e0e1E.llvm.3823990424930666649>:
   10582:	1141                	add	sp,sp,-16
   10584:	e406                	sd	ra,8(sp)
   10586:	e022                	sd	s0,0(sp)
   10588:	0800                	add	s0,sp,16
   1058a:	60a2                	ld	ra,8(sp)
   1058c:	6402                	ld	s0,0(sp)
   1058e:	0141                	add	sp,sp,16
   10590:	8082                	ret

0000000000010592 <_ZN4core9panicking13assert_failed17h990cdfb487308d09E>:
   10592:	715d                	add	sp,sp,-80
   10594:	e486                	sd	ra,72(sp)
   10596:	e0a2                	sd	s0,64(sp)
   10598:	0880                	add	s0,sp,80
   1059a:	883a                	mv	a6,a4
   1059c:	7698                	ld	a4,40(a3)
   1059e:	729c                	ld	a5,32(a3)
   105a0:	fab43823          	sd	a1,-80(s0)
   105a4:	fac43c23          	sd	a2,-72(s0)
   105a8:	fee43423          	sd	a4,-24(s0)
   105ac:	fef43023          	sd	a5,-32(s0)
   105b0:	6e8c                	ld	a1,24(a3)
   105b2:	6a90                	ld	a2,16(a3)
   105b4:	6698                	ld	a4,8(a3)
   105b6:	6294                	ld	a3,0(a3)
   105b8:	fcb43c23          	sd	a1,-40(s0)
   105bc:	fcc43823          	sd	a2,-48(s0)
   105c0:	fce43423          	sd	a4,-56(s0)
   105c4:	fcd43023          	sd	a3,-64(s0)

00000000000105c8 <.LBB1_1>:
   105c8:	00004617          	auipc	a2,0x4
   105cc:	df860613          	add	a2,a2,-520 # 143c0 <anon.f6f366f898315358e847d8a5f664601a.0.llvm.3823990424930666649>
   105d0:	fb040593          	add	a1,s0,-80
   105d4:	fb840693          	add	a3,s0,-72
   105d8:	fc040793          	add	a5,s0,-64
   105dc:	8732                	mv	a4,a2
   105de:	00001097          	auipc	ra,0x1
   105e2:	d28080e7          	jalr	-728(ra) # 11306 <_ZN4core9panicking19assert_failed_inner17hf0fd0a6058ec5f00E>
	...

00000000000105e8 <__rust_alloc>:
   105e8:	00000317          	auipc	t1,0x0
   105ec:	2be30067          	jr	702(t1) # 108a6 <__rg_alloc>

00000000000105f0 <__rust_dealloc>:
   105f0:	00000317          	auipc	t1,0x0
   105f4:	2dc30067          	jr	732(t1) # 108cc <__rg_dealloc>

00000000000105f8 <__rust_realloc>:
   105f8:	00000317          	auipc	t1,0x0
   105fc:	2fc30067          	jr	764(t1) # 108f4 <__rg_realloc>

0000000000010600 <__rust_alloc_error_handler>:
   10600:	00001317          	auipc	t1,0x1
   10604:	b3a30067          	jr	-1222(t1) # 1113a <__rg_oom>

0000000000010608 <_ZN4core3ptr37drop_in_place$LT$core..fmt..Error$GT$17h43784471ba3f448dE.llvm.17959331584031496199>:
   10608:	1141                	add	sp,sp,-16
   1060a:	e406                	sd	ra,8(sp)
   1060c:	e022                	sd	s0,0(sp)
   1060e:	0800                	add	s0,sp,16
   10610:	60a2                	ld	ra,8(sp)
   10612:	6402                	ld	s0,0(sp)
   10614:	0141                	add	sp,sp,16
   10616:	8082                	ret

0000000000010618 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h66dec9b024693ddfE.llvm.17959331584031496199>:
   10618:	1101                	add	sp,sp,-32
   1061a:	ec06                	sd	ra,24(sp)
   1061c:	e822                	sd	s0,16(sp)
   1061e:	1000                	add	s0,sp,32
   10620:	0005851b          	sext.w	a0,a1
   10624:	08000613          	li	a2,128
   10628:	fe042623          	sw	zero,-20(s0)
   1062c:	00c57663          	bgeu	a0,a2,10638 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h66dec9b024693ddfE.llvm.17959331584031496199+0x20>
   10630:	feb40623          	sb	a1,-20(s0)
   10634:	4605                	li	a2,1
   10636:	a849                	j	106c8 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h66dec9b024693ddfE.llvm.17959331584031496199+0xb0>
   10638:	00b5d51b          	srlw	a0,a1,0xb
   1063c:	ed19                	bnez	a0,1065a <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h66dec9b024693ddfE.llvm.17959331584031496199+0x42>
   1063e:	0065d513          	srl	a0,a1,0x6
   10642:	0c056513          	or	a0,a0,192
   10646:	fea40623          	sb	a0,-20(s0)
   1064a:	03f5f513          	and	a0,a1,63
   1064e:	08056513          	or	a0,a0,128
   10652:	fea406a3          	sb	a0,-19(s0)
   10656:	4609                	li	a2,2
   10658:	a885                	j	106c8 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h66dec9b024693ddfE.llvm.17959331584031496199+0xb0>
   1065a:	0105d51b          	srlw	a0,a1,0x10
   1065e:	e51d                	bnez	a0,1068c <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h66dec9b024693ddfE.llvm.17959331584031496199+0x74>
   10660:	00c5d51b          	srlw	a0,a1,0xc
   10664:	0e056513          	or	a0,a0,224
   10668:	fea40623          	sb	a0,-20(s0)
   1066c:	0065d51b          	srlw	a0,a1,0x6
   10670:	03f57513          	and	a0,a0,63
   10674:	08056513          	or	a0,a0,128
   10678:	fea406a3          	sb	a0,-19(s0)
   1067c:	03f5f513          	and	a0,a1,63
   10680:	08056513          	or	a0,a0,128
   10684:	fea40723          	sb	a0,-18(s0)
   10688:	460d                	li	a2,3
   1068a:	a83d                	j	106c8 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h66dec9b024693ddfE.llvm.17959331584031496199+0xb0>
   1068c:	0125d51b          	srlw	a0,a1,0x12
   10690:	891d                	and	a0,a0,7
   10692:	0f056513          	or	a0,a0,240
   10696:	fea40623          	sb	a0,-20(s0)
   1069a:	00c5d51b          	srlw	a0,a1,0xc
   1069e:	03f57513          	and	a0,a0,63
   106a2:	08056513          	or	a0,a0,128
   106a6:	fea406a3          	sb	a0,-19(s0)
   106aa:	0065d51b          	srlw	a0,a1,0x6
   106ae:	03f57513          	and	a0,a0,63
   106b2:	08056513          	or	a0,a0,128
   106b6:	fea40723          	sb	a0,-18(s0)
   106ba:	03f5f513          	and	a0,a1,63
   106be:	08056513          	or	a0,a0,128
   106c2:	fea407a3          	sb	a0,-17(s0)
   106c6:	4611                	li	a2,4
   106c8:	fec40593          	add	a1,s0,-20
   106cc:	04000893          	li	a7,64
   106d0:	4505                	li	a0,1
   106d2:	00000073          	ecall
   106d6:	4501                	li	a0,0
   106d8:	60e2                	ld	ra,24(sp)
   106da:	6442                	ld	s0,16(sp)
   106dc:	6105                	add	sp,sp,32
   106de:	8082                	ret

00000000000106e0 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hcd6b00a3f2b58b1dE.llvm.17959331584031496199>:
   106e0:	715d                	add	sp,sp,-80
   106e2:	e486                	sd	ra,72(sp)
   106e4:	e0a2                	sd	s0,64(sp)
   106e6:	0880                	add	s0,sp,80
   106e8:	6108                	ld	a0,0(a0)
   106ea:	7590                	ld	a2,40(a1)
   106ec:	7194                	ld	a3,32(a1)
   106ee:	faa43c23          	sd	a0,-72(s0)
   106f2:	fec43423          	sd	a2,-24(s0)
   106f6:	fed43023          	sd	a3,-32(s0)
   106fa:	6d88                	ld	a0,24(a1)
   106fc:	6990                	ld	a2,16(a1)
   106fe:	6594                	ld	a3,8(a1)
   10700:	618c                	ld	a1,0(a1)
   10702:	fca43c23          	sd	a0,-40(s0)
   10706:	fcc43823          	sd	a2,-48(s0)
   1070a:	fcd43423          	sd	a3,-56(s0)
   1070e:	fcb43023          	sd	a1,-64(s0)

0000000000010712 <.LBB2_1>:
   10712:	00004597          	auipc	a1,0x4
   10716:	cce58593          	add	a1,a1,-818 # 143e0 <anon.cab5b07038618639c4e6406ab92cac85.0.llvm.17959331584031496199>
   1071a:	fb840513          	add	a0,s0,-72
   1071e:	fc040613          	add	a2,s0,-64
   10722:	00001097          	auipc	ra,0x1
   10726:	3e0080e7          	jalr	992(ra) # 11b02 <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   1072a:	60a6                	ld	ra,72(sp)
   1072c:	6406                	ld	s0,64(sp)
   1072e:	6161                	add	sp,sp,80
   10730:	8082                	ret

0000000000010732 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17he0a2184791a24869E.llvm.17959331584031496199>:
   10732:	1141                	add	sp,sp,-16
   10734:	e406                	sd	ra,8(sp)
   10736:	e022                	sd	s0,0(sp)
   10738:	0800                	add	s0,sp,16
   1073a:	04000893          	li	a7,64
   1073e:	4505                	li	a0,1
   10740:	00000073          	ecall
   10744:	4501                	li	a0,0
   10746:	60a2                	ld	ra,8(sp)
   10748:	6402                	ld	s0,0(sp)
   1074a:	0141                	add	sp,sp,16
   1074c:	8082                	ret

000000000001074e <_ZN8user_lib7console5print17h2df775f81e475c40E>:
   1074e:	715d                	add	sp,sp,-80
   10750:	e486                	sd	ra,72(sp)
   10752:	e0a2                	sd	s0,64(sp)
   10754:	0880                	add	s0,sp,80
   10756:	750c                	ld	a1,40(a0)
   10758:	7110                	ld	a2,32(a0)
   1075a:	fe840693          	add	a3,s0,-24
   1075e:	fad43823          	sd	a3,-80(s0)
   10762:	feb43023          	sd	a1,-32(s0)
   10766:	fcc43c23          	sd	a2,-40(s0)
   1076a:	6d0c                	ld	a1,24(a0)
   1076c:	6910                	ld	a2,16(a0)
   1076e:	6514                	ld	a3,8(a0)
   10770:	6108                	ld	a0,0(a0)
   10772:	fcb43823          	sd	a1,-48(s0)
   10776:	fcc43423          	sd	a2,-56(s0)
   1077a:	fcd43023          	sd	a3,-64(s0)
   1077e:	faa43c23          	sd	a0,-72(s0)

0000000000010782 <.LBB5_3>:
   10782:	00004597          	auipc	a1,0x4
   10786:	c5e58593          	add	a1,a1,-930 # 143e0 <anon.cab5b07038618639c4e6406ab92cac85.0.llvm.17959331584031496199>
   1078a:	fb040513          	add	a0,s0,-80
   1078e:	fb840613          	add	a2,s0,-72
   10792:	00001097          	auipc	ra,0x1
   10796:	370080e7          	jalr	880(ra) # 11b02 <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   1079a:	e509                	bnez	a0,107a4 <.LBB5_4>
   1079c:	60a6                	ld	ra,72(sp)
   1079e:	6406                	ld	s0,64(sp)
   107a0:	6161                	add	sp,sp,80
   107a2:	8082                	ret

00000000000107a4 <.LBB5_4>:
   107a4:	00004517          	auipc	a0,0x4
   107a8:	c6c50513          	add	a0,a0,-916 # 14410 <anon.cab5b07038618639c4e6406ab92cac85.1.llvm.17959331584031496199>

00000000000107ac <.LBB5_5>:
   107ac:	00004697          	auipc	a3,0x4
   107b0:	c9468693          	add	a3,a3,-876 # 14440 <anon.cab5b07038618639c4e6406ab92cac85.2.llvm.17959331584031496199>

00000000000107b4 <.LBB5_6>:
   107b4:	00004717          	auipc	a4,0x4
   107b8:	cbc70713          	add	a4,a4,-836 # 14470 <anon.cab5b07038618639c4e6406ab92cac85.4.llvm.17959331584031496199>
   107bc:	02b00593          	li	a1,43
   107c0:	fe840613          	add	a2,s0,-24
   107c4:	00001097          	auipc	ra,0x1
   107c8:	c2e080e7          	jalr	-978(ra) # 113f2 <_ZN4core6result13unwrap_failed17h3c2e5884ed497eadE>
	...

00000000000107ce <_ZN4core3ptr48drop_in_place$LT$core..str..error..Utf8Error$GT$17h6bc69a15aa1420d5E>:
   107ce:	1141                	add	sp,sp,-16
   107d0:	e406                	sd	ra,8(sp)
   107d2:	e022                	sd	s0,0(sp)
   107d4:	0800                	add	s0,sp,16
   107d6:	60a2                	ld	ra,8(sp)
   107d8:	6402                	ld	s0,0(sp)
   107da:	0141                	add	sp,sp,16
   107dc:	8082                	ret

00000000000107de <rust_oom>:
   107de:	711d                	add	sp,sp,-96
   107e0:	ec86                	sd	ra,88(sp)
   107e2:	e8a2                	sd	s0,80(sp)
   107e4:	1080                	add	s0,sp,96
   107e6:	faa43023          	sd	a0,-96(s0)
   107ea:	fab43423          	sd	a1,-88(s0)
   107ee:	fa040513          	add	a0,s0,-96
   107f2:	fea43023          	sd	a0,-32(s0)

00000000000107f6 <.LBB1_1>:
   107f6:	00003517          	auipc	a0,0x3
   107fa:	e5050513          	add	a0,a0,-432 # 13646 <_ZN64_$LT$core..alloc..layout..Layout$u20$as$u20$core..fmt..Debug$GT$3fmt17h86d7136df2fe6134E>
   107fe:	fea43423          	sd	a0,-24(s0)

0000000000010802 <.LBB1_2>:
   10802:	00004517          	auipc	a0,0x4
   10806:	cc650513          	add	a0,a0,-826 # 144c8 <.Lanon.24728be23fbcdb17895d264a60dac18d.1>
   1080a:	faa43823          	sd	a0,-80(s0)
   1080e:	4505                	li	a0,1
   10810:	faa43c23          	sd	a0,-72(s0)
   10814:	fc043023          	sd	zero,-64(s0)
   10818:	fe040593          	add	a1,s0,-32
   1081c:	fcb43823          	sd	a1,-48(s0)
   10820:	fca43c23          	sd	a0,-40(s0)

0000000000010824 <.LBB1_3>:
   10824:	00004597          	auipc	a1,0x4
   10828:	cc458593          	add	a1,a1,-828 # 144e8 <.Lanon.24728be23fbcdb17895d264a60dac18d.3>
   1082c:	fb040513          	add	a0,s0,-80
   10830:	00001097          	auipc	ra,0x1
   10834:	aa8080e7          	jalr	-1368(ra) # 112d8 <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

000000000001083a <_ZN8user_lib4exit17h20483dcbea918787E>:
   1083a:	1141                	add	sp,sp,-16
   1083c:	e406                	sd	ra,8(sp)
   1083e:	e022                	sd	s0,0(sp)
   10840:	0800                	add	s0,sp,16
   10842:	00000097          	auipc	ra,0x0
   10846:	298080e7          	jalr	664(ra) # 10ada <_ZN8user_lib7syscall8sys_exit17h641f41ab1e635210E>
	...

000000000001084c <_ZN8user_lib4fork17h4831b1e32e249225E>:
   1084c:	1141                	add	sp,sp,-16
   1084e:	e406                	sd	ra,8(sp)
   10850:	e022                	sd	s0,0(sp)
   10852:	0800                	add	s0,sp,16
   10854:	0dc00893          	li	a7,220
   10858:	4581                	li	a1,0
   1085a:	4601                	li	a2,0
   1085c:	4681                	li	a3,0
   1085e:	4701                	li	a4,0
   10860:	4781                	li	a5,0
   10862:	4501                	li	a0,0
   10864:	00000073          	ecall
   10868:	60a2                	ld	ra,8(sp)
   1086a:	6402                	ld	s0,0(sp)
   1086c:	0141                	add	sp,sp,16
   1086e:	8082                	ret

0000000000010870 <_ZN8user_lib6execve17h3e0a48d92ed78303E>:
   10870:	1141                	add	sp,sp,-16
   10872:	e406                	sd	ra,8(sp)
   10874:	e022                	sd	s0,0(sp)
   10876:	0800                	add	s0,sp,16
   10878:	85b2                	mv	a1,a2
   1087a:	0dd00893          	li	a7,221
   1087e:	863a                	mv	a2,a4
   10880:	00000073          	ecall
   10884:	60a2                	ld	ra,8(sp)
   10886:	6402                	ld	s0,0(sp)
   10888:	0141                	add	sp,sp,16
   1088a:	8082                	ret

000000000001088c <_ZN8user_lib7waitpid17hc88733ebf8ce9b70E>:
   1088c:	1141                	add	sp,sp,-16
   1088e:	e406                	sd	ra,8(sp)
   10890:	e022                	sd	s0,0(sp)
   10892:	0800                	add	s0,sp,16
   10894:	10400893          	li	a7,260
   10898:	4601                	li	a2,0
   1089a:	00000073          	ecall
   1089e:	60a2                	ld	ra,8(sp)
   108a0:	6402                	ld	s0,0(sp)
   108a2:	0141                	add	sp,sp,16
   108a4:	8082                	ret

00000000000108a6 <__rg_alloc>:
   108a6:	1141                	add	sp,sp,-16
   108a8:	e406                	sd	ra,8(sp)
   108aa:	e022                	sd	s0,0(sp)
   108ac:	0800                	add	s0,sp,16

00000000000108ae <.LBB30_1>:
   108ae:	00038617          	auipc	a2,0x38
   108b2:	ac260613          	add	a2,a2,-1342 # 48370 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E>
   108b6:	86ae                	mv	a3,a1
   108b8:	85aa                	mv	a1,a0
   108ba:	8532                	mv	a0,a2
   108bc:	8636                	mv	a2,a3
   108be:	60a2                	ld	ra,8(sp)
   108c0:	6402                	ld	s0,0(sp)
   108c2:	0141                	add	sp,sp,16
   108c4:	00000317          	auipc	t1,0x0
   108c8:	77c30067          	jr	1916(t1) # 11040 <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h6574ad5d6e524a4aE>

00000000000108cc <__rg_dealloc>:
   108cc:	1141                	add	sp,sp,-16
   108ce:	e406                	sd	ra,8(sp)
   108d0:	e022                	sd	s0,0(sp)
   108d2:	0800                	add	s0,sp,16

00000000000108d4 <.LBB31_1>:
   108d4:	00038697          	auipc	a3,0x38
   108d8:	a9c68693          	add	a3,a3,-1380 # 48370 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E>
   108dc:	8732                	mv	a4,a2
   108de:	862e                	mv	a2,a1
   108e0:	85aa                	mv	a1,a0
   108e2:	8536                	mv	a0,a3
   108e4:	86ba                	mv	a3,a4
   108e6:	60a2                	ld	ra,8(sp)
   108e8:	6402                	ld	s0,0(sp)
   108ea:	0141                	add	sp,sp,16
   108ec:	00000317          	auipc	t1,0x0
   108f0:	7a230067          	jr	1954(t1) # 1108e <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hbd168ca56515a3a0E>

00000000000108f4 <__rg_realloc>:
   108f4:	7139                	add	sp,sp,-64
   108f6:	fc06                	sd	ra,56(sp)
   108f8:	f822                	sd	s0,48(sp)
   108fa:	f426                	sd	s1,40(sp)
   108fc:	f04a                	sd	s2,32(sp)
   108fe:	ec4e                	sd	s3,24(sp)
   10900:	e852                	sd	s4,16(sp)
   10902:	e456                	sd	s5,8(sp)
   10904:	0080                	add	s0,sp,64
   10906:	84b6                	mv	s1,a3
   10908:	8932                	mv	s2,a2
   1090a:	8a2e                	mv	s4,a1
   1090c:	89aa                	mv	s3,a0

000000000001090e <.LBB32_5>:
   1090e:	00038517          	auipc	a0,0x38
   10912:	a6250513          	add	a0,a0,-1438 # 48370 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E>
   10916:	85b6                	mv	a1,a3
   10918:	00000097          	auipc	ra,0x0
   1091c:	728080e7          	jalr	1832(ra) # 11040 <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h6574ad5d6e524a4aE>
   10920:	8aaa                	mv	s5,a0
   10922:	c515                	beqz	a0,1094e <.LBB32_6+0x16>
   10924:	0144e363          	bltu	s1,s4,1092a <.LBB32_5+0x1c>
   10928:	84d2                	mv	s1,s4
   1092a:	8556                	mv	a0,s5
   1092c:	85ce                	mv	a1,s3
   1092e:	8626                	mv	a2,s1
   10930:	00003097          	auipc	ra,0x3
   10934:	f26080e7          	jalr	-218(ra) # 13856 <memcpy>

0000000000010938 <.LBB32_6>:
   10938:	00038517          	auipc	a0,0x38
   1093c:	a3850513          	add	a0,a0,-1480 # 48370 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E>
   10940:	85ce                	mv	a1,s3
   10942:	8652                	mv	a2,s4
   10944:	86ca                	mv	a3,s2
   10946:	00000097          	auipc	ra,0x0
   1094a:	748080e7          	jalr	1864(ra) # 1108e <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hbd168ca56515a3a0E>
   1094e:	8556                	mv	a0,s5
   10950:	70e2                	ld	ra,56(sp)
   10952:	7442                	ld	s0,48(sp)
   10954:	74a2                	ld	s1,40(sp)
   10956:	7902                	ld	s2,32(sp)
   10958:	69e2                	ld	s3,24(sp)
   1095a:	6a42                	ld	s4,16(sp)
   1095c:	6aa2                	ld	s5,8(sp)
   1095e:	6121                	add	sp,sp,64
   10960:	8082                	ret

0000000000010962 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE>:
   10962:	7179                	add	sp,sp,-48
   10964:	f406                	sd	ra,40(sp)
   10966:	f022                	sd	s0,32(sp)
   10968:	ec26                	sd	s1,24(sp)
   1096a:	e84a                	sd	s2,16(sp)
   1096c:	e44e                	sd	s3,8(sp)
   1096e:	1800                	add	s0,sp,48
   10970:	892e                	mv	s2,a1
   10972:	89aa                	mv	s3,a0
   10974:	ce19                	beqz	a2,10992 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x30>
   10976:	84b2                	mv	s1,a2
   10978:	6a88                	ld	a0,16(a3)
   1097a:	c10d                	beqz	a0,1099c <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x3a>
   1097c:	668c                	ld	a1,8(a3)
   1097e:	cd99                	beqz	a1,1099c <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x3a>
   10980:	6288                	ld	a0,0(a3)
   10982:	8626                	mv	a2,s1
   10984:	86ca                	mv	a3,s2
   10986:	00000097          	auipc	ra,0x0
   1098a:	c72080e7          	jalr	-910(ra) # 105f8 <__rust_realloc>
   1098e:	e11d                	bnez	a0,109b4 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x52>
   10990:	a011                	j	10994 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x32>
   10992:	4481                	li	s1,0
   10994:	0129b423          	sd	s2,8(s3)
   10998:	4585                	li	a1,1
   1099a:	a00d                	j	109bc <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x5a>
   1099c:	00090a63          	beqz	s2,109b0 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x4e>
   109a0:	854a                	mv	a0,s2
   109a2:	85a6                	mv	a1,s1
   109a4:	00000097          	auipc	ra,0x0
   109a8:	c44080e7          	jalr	-956(ra) # 105e8 <__rust_alloc>
   109ac:	e501                	bnez	a0,109b4 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x52>
   109ae:	b7dd                	j	10994 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x32>
   109b0:	4901                	li	s2,0
   109b2:	8526                	mv	a0,s1
   109b4:	4581                	li	a1,0
   109b6:	00a9b423          	sd	a0,8(s3)
   109ba:	84ca                	mv	s1,s2
   109bc:	0099b823          	sd	s1,16(s3)
   109c0:	00b9b023          	sd	a1,0(s3)
   109c4:	70a2                	ld	ra,40(sp)
   109c6:	7402                	ld	s0,32(sp)
   109c8:	64e2                	ld	s1,24(sp)
   109ca:	6942                	ld	s2,16(sp)
   109cc:	69a2                	ld	s3,8(sp)
   109ce:	6145                	add	sp,sp,48
   109d0:	8082                	ret

00000000000109d2 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E>:
   109d2:	715d                	add	sp,sp,-80
   109d4:	e486                	sd	ra,72(sp)
   109d6:	e0a2                	sd	s0,64(sp)
   109d8:	fc26                	sd	s1,56(sp)
   109da:	f84a                	sd	s2,48(sp)
   109dc:	0880                	add	s0,sp,80
   109de:	00158613          	add	a2,a1,1
   109e2:	08b66b63          	bltu	a2,a1,10a78 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0xa6>
   109e6:	892a                	mv	s2,a0
   109e8:	6508                	ld	a0,8(a0)
   109ea:	00151493          	sll	s1,a0,0x1
   109ee:	00966363          	bltu	a2,s1,109f4 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0x22>
   109f2:	84b2                	mv	s1,a2
   109f4:	4591                	li	a1,4
   109f6:	0095e363          	bltu	a1,s1,109fc <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0x2a>
   109fa:	4491                	li	s1,4
   109fc:	00449593          	sll	a1,s1,0x4
   10a00:	0045d613          	srl	a2,a1,0x4
   10a04:	8e25                	xor	a2,a2,s1
   10a06:	00c03633          	snez	a2,a2
   10a0a:	56c5                	li	a3,-15
   10a0c:	8285                	srl	a3,a3,0x1
   10a0e:	00b6b6b3          	sltu	a3,a3,a1
   10a12:	8ed1                	or	a3,a3,a2
   10a14:	4601                	li	a2,0
   10a16:	e291                	bnez	a3,10a1a <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0x48>
   10a18:	4621                	li	a2,8
   10a1a:	c51d                	beqz	a0,10a48 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0x76>
   10a1c:	00451693          	sll	a3,a0,0x4
   10a20:	0046d713          	srl	a4,a3,0x4
   10a24:	8d39                	xor	a0,a0,a4
   10a26:	00a03533          	snez	a0,a0
   10a2a:	5745                	li	a4,-15
   10a2c:	8305                	srl	a4,a4,0x1
   10a2e:	00d73733          	sltu	a4,a4,a3
   10a32:	8f49                	or	a4,a4,a0
   10a34:	4501                	li	a0,0
   10a36:	e311                	bnez	a4,10a3a <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0x68>
   10a38:	4521                	li	a0,8
   10a3a:	00093703          	ld	a4,0(s2)
   10a3e:	fce43423          	sd	a4,-56(s0)
   10a42:	fcd43823          	sd	a3,-48(s0)
   10a46:	a011                	j	10a4a <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0x78>
   10a48:	4501                	li	a0,0
   10a4a:	fca43c23          	sd	a0,-40(s0)
   10a4e:	fb040513          	add	a0,s0,-80
   10a52:	fc840693          	add	a3,s0,-56
   10a56:	00000097          	auipc	ra,0x0
   10a5a:	f0c080e7          	jalr	-244(ra) # 10962 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE>
   10a5e:	fb043583          	ld	a1,-80(s0)
   10a62:	fb843503          	ld	a0,-72(s0)
   10a66:	cd91                	beqz	a1,10a82 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0xb0>
   10a68:	fc043583          	ld	a1,-64(s0)
   10a6c:	567d                	li	a2,-1
   10a6e:	167e                	sll	a2,a2,0x3f
   10a70:	0605                	add	a2,a2,1
   10a72:	00c58c63          	beq	a1,a2,10a8a <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0xb8>
   10a76:	e185                	bnez	a1,10a96 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0xc4>
   10a78:	00000097          	auipc	ra,0x0
   10a7c:	672080e7          	jalr	1650(ra) # 110ea <_ZN5alloc7raw_vec17capacity_overflow17h8c103c8c1cb34845E>
   10a80:	0000                	unimp
   10a82:	00a93023          	sd	a0,0(s2)
   10a86:	00993423          	sd	s1,8(s2)
   10a8a:	60a6                	ld	ra,72(sp)
   10a8c:	6406                	ld	s0,64(sp)
   10a8e:	74e2                	ld	s1,56(sp)
   10a90:	7942                	ld	s2,48(sp)
   10a92:	6161                	add	sp,sp,80
   10a94:	8082                	ret
   10a96:	00000097          	auipc	ra,0x0
   10a9a:	688080e7          	jalr	1672(ra) # 1111e <_ZN5alloc5alloc18handle_alloc_error17h48d9534aaf1ab1f8E>
	...

0000000000010aa0 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h64d226d320fa66bfE>:
   10aa0:	1141                	add	sp,sp,-16
   10aa2:	e406                	sd	ra,8(sp)
   10aa4:	e022                	sd	s0,0(sp)
   10aa6:	0800                	add	s0,sp,16
   10aa8:	6108                	ld	a0,0(a0)
   10aaa:	60a2                	ld	ra,8(sp)
   10aac:	6402                	ld	s0,0(sp)
   10aae:	0141                	add	sp,sp,16
   10ab0:	00001317          	auipc	t1,0x1
   10ab4:	02030067          	jr	32(t1) # 11ad0 <_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17h4d3bee89ff077165E>

0000000000010ab8 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hd4b1075b6d8c8353E>:
   10ab8:	1141                	add	sp,sp,-16
   10aba:	e406                	sd	ra,8(sp)
   10abc:	e022                	sd	s0,0(sp)
   10abe:	0800                	add	s0,sp,16
   10ac0:	6110                	ld	a2,0(a0)
   10ac2:	6514                	ld	a3,8(a0)
   10ac4:	872e                	mv	a4,a1
   10ac6:	8532                	mv	a0,a2
   10ac8:	85b6                	mv	a1,a3
   10aca:	863a                	mv	a2,a4
   10acc:	60a2                	ld	ra,8(sp)
   10ace:	6402                	ld	s0,0(sp)
   10ad0:	0141                	add	sp,sp,16
   10ad2:	00001317          	auipc	t1,0x1
   10ad6:	6fc30067          	jr	1788(t1) # 121ce <_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hd677dbeaf8ccd015E>

0000000000010ada <_ZN8user_lib7syscall8sys_exit17h641f41ab1e635210E>:
   10ada:	1141                	add	sp,sp,-16
   10adc:	e406                	sd	ra,8(sp)
   10ade:	e022                	sd	s0,0(sp)
   10ae0:	0800                	add	s0,sp,16
   10ae2:	2501                	sext.w	a0,a0
   10ae4:	05d00893          	li	a7,93
   10ae8:	4581                	li	a1,0
   10aea:	4601                	li	a2,0
   10aec:	00000073          	ecall

0000000000010af0 <.LBB8_1>:
   10af0:	00004517          	auipc	a0,0x4
   10af4:	a7850513          	add	a0,a0,-1416 # 14568 <anon.c95dffcbf20f28c4475d7f1508a8896b.0.llvm.11064437996114815796>

0000000000010af8 <.LBB8_2>:
   10af8:	00004617          	auipc	a2,0x4
   10afc:	a9860613          	add	a2,a2,-1384 # 14590 <anon.c95dffcbf20f28c4475d7f1508a8896b.2.llvm.11064437996114815796>
   10b00:	45dd                	li	a1,23
   10b02:	00000097          	auipc	ra,0x0
   10b06:	722080e7          	jalr	1826(ra) # 11224 <_ZN4core9panicking5panic17h92f54f473578363dE>
	...

0000000000010b0c <_ZN22buddy_system_allocator4Heap4init17h0387239a34ee507aE>:
   10b0c:	1141                	add	sp,sp,-16
   10b0e:	e406                	sd	ra,8(sp)
   10b10:	e022                	sd	s0,0(sp)
   10b12:	0800                	add	s0,sp,16
   10b14:	962e                	add	a2,a2,a1
   10b16:	059d                	add	a1,a1,7
   10b18:	99e1                	and	a1,a1,-8
   10b1a:	ff867e93          	and	t4,a2,-8
   10b1e:	12beea63          	bltu	t4,a1,10c52 <.LBB4_20>
   10b22:	4701                	li	a4,0
   10b24:	00858613          	add	a2,a1,8
   10b28:	10cee063          	bltu	t4,a2,10c28 <.LBB4_18+0xd8>

0000000000010b2c <.LBB4_15>:
   10b2c:	00005617          	auipc	a2,0x5
   10b30:	4f460613          	add	a2,a2,1268 # 16020 <.LCPI4_0>
   10b34:	00063803          	ld	a6,0(a2)

0000000000010b38 <.LBB4_16>:
   10b38:	00005617          	auipc	a2,0x5
   10b3c:	4f060613          	add	a2,a2,1264 # 16028 <.LCPI4_1>
   10b40:	00063f03          	ld	t5,0(a2)

0000000000010b44 <.LBB4_17>:
   10b44:	00005617          	auipc	a2,0x5
   10b48:	4ec60613          	add	a2,a2,1260 # 16030 <.LCPI4_2>
   10b4c:	00063883          	ld	a7,0(a2)

0000000000010b50 <.LBB4_18>:
   10b50:	00005617          	auipc	a2,0x5
   10b54:	4e860613          	add	a2,a2,1256 # 16038 <.LCPI4_3>
   10b58:	00063283          	ld	t0,0(a2)
   10b5c:	03f00313          	li	t1,63
   10b60:	4385                	li	t2,1
   10b62:	4e7d                	li	t3,31
   10b64:	40be8633          	sub	a2,t4,a1
   10b68:	ca29                	beqz	a2,10bba <.LBB4_18+0x6a>
   10b6a:	00165693          	srl	a3,a2,0x1
   10b6e:	8e55                	or	a2,a2,a3
   10b70:	00265693          	srl	a3,a2,0x2
   10b74:	8e55                	or	a2,a2,a3
   10b76:	00465693          	srl	a3,a2,0x4
   10b7a:	8e55                	or	a2,a2,a3
   10b7c:	00865693          	srl	a3,a2,0x8
   10b80:	8e55                	or	a2,a2,a3
   10b82:	01065693          	srl	a3,a2,0x10
   10b86:	8e55                	or	a2,a2,a3
   10b88:	02065693          	srl	a3,a2,0x20
   10b8c:	8e55                	or	a2,a2,a3
   10b8e:	fff64613          	not	a2,a2
   10b92:	00165693          	srl	a3,a2,0x1
   10b96:	0106f6b3          	and	a3,a3,a6
   10b9a:	8e15                	sub	a2,a2,a3
   10b9c:	01e676b3          	and	a3,a2,t5
   10ba0:	8209                	srl	a2,a2,0x2
   10ba2:	01e67633          	and	a2,a2,t5
   10ba6:	9636                	add	a2,a2,a3
   10ba8:	00465693          	srl	a3,a2,0x4
   10bac:	9636                	add	a2,a2,a3
   10bae:	01167633          	and	a2,a2,a7
   10bb2:	02560633          	mul	a2,a2,t0
   10bb6:	9261                	srl	a2,a2,0x38
   10bb8:	a019                	j	10bbe <.LBB4_18+0x6e>
   10bba:	04000613          	li	a2,64
   10bbe:	40b006b3          	neg	a3,a1
   10bc2:	8eed                	and	a3,a3,a1
   10bc4:	40c30633          	sub	a2,t1,a2
   10bc8:	00c39633          	sll	a2,t2,a2
   10bcc:	00d66363          	bltu	a2,a3,10bd2 <.LBB4_18+0x82>
   10bd0:	8636                	mv	a2,a3
   10bd2:	ce05                	beqz	a2,10c0a <.LBB4_18+0xba>
   10bd4:	fff60693          	add	a3,a2,-1
   10bd8:	fff64793          	not	a5,a2
   10bdc:	8efd                	and	a3,a3,a5
   10bde:	0016d793          	srl	a5,a3,0x1
   10be2:	0107f7b3          	and	a5,a5,a6
   10be6:	8e9d                	sub	a3,a3,a5
   10be8:	01e6f7b3          	and	a5,a3,t5
   10bec:	8289                	srl	a3,a3,0x2
   10bee:	01e6f6b3          	and	a3,a3,t5
   10bf2:	96be                	add	a3,a3,a5
   10bf4:	0046d793          	srl	a5,a3,0x4
   10bf8:	96be                	add	a3,a3,a5
   10bfa:	0116f6b3          	and	a3,a3,a7
   10bfe:	025686b3          	mul	a3,a3,t0
   10c02:	92e1                	srl	a3,a3,0x38
   10c04:	00de7763          	bgeu	t3,a3,10c12 <.LBB4_18+0xc2>
   10c08:	a80d                	j	10c3a <.LBB4_19>
   10c0a:	04000693          	li	a3,64
   10c0e:	02de6663          	bltu	t3,a3,10c3a <.LBB4_19>
   10c12:	068e                	sll	a3,a3,0x3
   10c14:	96aa                	add	a3,a3,a0
   10c16:	629c                	ld	a5,0(a3)
   10c18:	e19c                	sd	a5,0(a1)
   10c1a:	e28c                	sd	a1,0(a3)
   10c1c:	95b2                	add	a1,a1,a2
   10c1e:	00858693          	add	a3,a1,8
   10c22:	9732                	add	a4,a4,a2
   10c24:	f4def0e3          	bgeu	t4,a3,10b64 <.LBB4_18+0x14>
   10c28:	11053583          	ld	a1,272(a0)
   10c2c:	95ba                	add	a1,a1,a4
   10c2e:	10b53823          	sd	a1,272(a0)
   10c32:	60a2                	ld	ra,8(sp)
   10c34:	6402                	ld	s0,0(sp)
   10c36:	0141                	add	sp,sp,16
   10c38:	8082                	ret

0000000000010c3a <.LBB4_19>:
   10c3a:	00004617          	auipc	a2,0x4
   10c3e:	a1e60613          	add	a2,a2,-1506 # 14658 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.3>
   10c42:	02000593          	li	a1,32
   10c46:	8536                	mv	a0,a3
   10c48:	00000097          	auipc	ra,0x0
   10c4c:	650080e7          	jalr	1616(ra) # 11298 <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>
	...

0000000000010c52 <.LBB4_20>:
   10c52:	00004517          	auipc	a0,0x4
   10c56:	95650513          	add	a0,a0,-1706 # 145a8 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.0>

0000000000010c5a <.LBB4_21>:
   10c5a:	00004617          	auipc	a2,0x4
   10c5e:	9e660613          	add	a2,a2,-1562 # 14640 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.2>
   10c62:	45f9                	li	a1,30
   10c64:	00000097          	auipc	ra,0x0
   10c68:	5c0080e7          	jalr	1472(ra) # 11224 <_ZN4core9panicking5panic17h92f54f473578363dE>
	...

0000000000010c6e <_ZN22buddy_system_allocator4Heap5alloc17hcac490460a84cc76E>:
   10c6e:	1141                	add	sp,sp,-16
   10c70:	e406                	sd	ra,8(sp)
   10c72:	e022                	sd	s0,0(sp)
   10c74:	0800                	add	s0,sp,16
   10c76:	4285                	li	t0,1
   10c78:	4709                	li	a4,2

0000000000010c7a <.LBB5_28>:
   10c7a:	00005397          	auipc	t2,0x5
   10c7e:	3c638393          	add	t2,t2,966 # 16040 <.LCPI5_0>

0000000000010c82 <.LBB5_29>:
   10c82:	00005317          	auipc	t1,0x5
   10c86:	3c630313          	add	t1,t1,966 # 16048 <.LCPI5_1>

0000000000010c8a <.LBB5_30>:
   10c8a:	00005897          	auipc	a7,0x5
   10c8e:	3c688893          	add	a7,a7,966 # 16050 <.LCPI5_2>

0000000000010c92 <.LBB5_31>:
   10c92:	00005817          	auipc	a6,0x5
   10c96:	3c680813          	add	a6,a6,966 # 16058 <.LCPI5_3>
   10c9a:	0ee5f963          	bgeu	a1,a4,10d8c <.LBB5_31+0xfa>
   10c9e:	46a1                	li	a3,8
   10ca0:	14c6fe63          	bgeu	a3,a2,10dfc <.LBB5_31+0x16a>
   10ca4:	14567f63          	bgeu	a2,t0,10e02 <.LBB5_31+0x170>
   10ca8:	16028063          	beqz	t0,10e08 <.LBB5_31+0x176>
   10cac:	fff28613          	add	a2,t0,-1
   10cb0:	fff2c693          	not	a3,t0
   10cb4:	0003b703          	ld	a4,0(t2)
   10cb8:	8e75                	and	a2,a2,a3
   10cba:	00033683          	ld	a3,0(t1)
   10cbe:	00165793          	srl	a5,a2,0x1
   10cc2:	8f7d                	and	a4,a4,a5
   10cc4:	8e19                	sub	a2,a2,a4
   10cc6:	00d67733          	and	a4,a2,a3
   10cca:	8209                	srl	a2,a2,0x2
   10ccc:	8e75                	and	a2,a2,a3
   10cce:	963a                	add	a2,a2,a4
   10cd0:	0008b683          	ld	a3,0(a7)
   10cd4:	00083703          	ld	a4,0(a6)
   10cd8:	00465793          	srl	a5,a2,0x4
   10cdc:	963e                	add	a2,a2,a5
   10cde:	8e75                	and	a2,a2,a3
   10ce0:	02e60633          	mul	a2,a2,a4
   10ce4:	03865893          	srl	a7,a2,0x38
   10ce8:	02000613          	li	a2,32
   10cec:	8746                	mv	a4,a7
   10cee:	01166463          	bltu	a2,a7,10cf6 <.LBB5_31+0x64>
   10cf2:	02000713          	li	a4,32
   10cf6:	00389813          	sll	a6,a7,0x3
   10cfa:	00a80633          	add	a2,a6,a0
   10cfe:	ff060793          	add	a5,a2,-16
   10d02:	86c6                	mv	a3,a7
   10d04:	06d70e63          	beq	a4,a3,10d80 <.LBB5_31+0xee>
   10d08:	6b90                	ld	a2,16(a5)
   10d0a:	0685                	add	a3,a3,1
   10d0c:	07a1                	add	a5,a5,8
   10d0e:	da7d                	beqz	a2,10d04 <.LBB5_31+0x72>
   10d10:	fff68713          	add	a4,a3,-1
   10d14:	04e8f063          	bgeu	a7,a4,10d54 <.LBB5_31+0xc2>
   10d18:	00188e93          	add	t4,a7,1
   10d1c:	02000313          	li	t1,32
   10d20:	43fd                	li	t2,31
   10d22:	4e05                	li	t3,1
   10d24:	fff68713          	add	a4,a3,-1
   10d28:	0e677863          	bgeu	a4,t1,10e18 <.LBB5_31+0x186>
   10d2c:	00063f03          	ld	t5,0(a2)
   10d30:	16f9                	add	a3,a3,-2
   10d32:	01e7b423          	sd	t5,8(a5)
   10d36:	0ed3e863          	bltu	t2,a3,10e26 <.LBB5_33>
   10d3a:	0007bf03          	ld	t5,0(a5)
   10d3e:	00de16b3          	sll	a3,t3,a3
   10d42:	96b2                	add	a3,a3,a2
   10d44:	01e6b023          	sd	t5,0(a3)
   10d48:	e214                	sd	a3,0(a2)
   10d4a:	e390                	sd	a2,0(a5)
   10d4c:	17e1                	add	a5,a5,-8
   10d4e:	86ba                	mv	a3,a4
   10d50:	fceeeae3          	bltu	t4,a4,10d24 <.LBB5_31+0x92>
   10d54:	02000613          	li	a2,32
   10d58:	0ec8f363          	bgeu	a7,a2,10e3e <.LBB5_34>
   10d5c:	010506b3          	add	a3,a0,a6
   10d60:	6290                	ld	a2,0(a3)
   10d62:	ca75                	beqz	a2,10e56 <.LBB5_35>
   10d64:	6218                	ld	a4,0(a2)
   10d66:	e298                	sd	a4,0(a3)
   10d68:	10053683          	ld	a3,256(a0)
   10d6c:	10853703          	ld	a4,264(a0)
   10d70:	95b6                	add	a1,a1,a3
   10d72:	10b53023          	sd	a1,256(a0)
   10d76:	005705b3          	add	a1,a4,t0
   10d7a:	10b53423          	sd	a1,264(a0)
   10d7e:	a011                	j	10d82 <.LBB5_31+0xf0>
   10d80:	4601                	li	a2,0
   10d82:	8532                	mv	a0,a2
   10d84:	60a2                	ld	ra,8(sp)
   10d86:	6402                	ld	s0,0(sp)
   10d88:	0141                	add	sp,sp,16
   10d8a:	8082                	ret
   10d8c:	fff58713          	add	a4,a1,-1
   10d90:	00175693          	srl	a3,a4,0x1
   10d94:	8ed9                	or	a3,a3,a4
   10d96:	0026d713          	srl	a4,a3,0x2
   10d9a:	8ed9                	or	a3,a3,a4
   10d9c:	0046d713          	srl	a4,a3,0x4
   10da0:	8ed9                	or	a3,a3,a4
   10da2:	0086d713          	srl	a4,a3,0x8
   10da6:	8ed9                	or	a3,a3,a4
   10da8:	0106d713          	srl	a4,a3,0x10
   10dac:	8ed9                	or	a3,a3,a4
   10dae:	0206d713          	srl	a4,a3,0x20
   10db2:	8ed9                	or	a3,a3,a4
   10db4:	0003b283          	ld	t0,0(t2)
   10db8:	fff6c693          	not	a3,a3
   10dbc:	00033783          	ld	a5,0(t1)
   10dc0:	0016d713          	srl	a4,a3,0x1
   10dc4:	00577733          	and	a4,a4,t0
   10dc8:	8e99                	sub	a3,a3,a4
   10dca:	00f6f733          	and	a4,a3,a5
   10dce:	8289                	srl	a3,a3,0x2
   10dd0:	8efd                	and	a3,a3,a5
   10dd2:	96ba                	add	a3,a3,a4
   10dd4:	0008b283          	ld	t0,0(a7)
   10dd8:	00083783          	ld	a5,0(a6)
   10ddc:	0046d713          	srl	a4,a3,0x4
   10de0:	96ba                	add	a3,a3,a4
   10de2:	0056f6b3          	and	a3,a3,t0
   10de6:	02f686b3          	mul	a3,a3,a5
   10dea:	92e1                	srl	a3,a3,0x38
   10dec:	577d                	li	a4,-1
   10dee:	00d756b3          	srl	a3,a4,a3
   10df2:	00168293          	add	t0,a3,1
   10df6:	46a1                	li	a3,8
   10df8:	eac6e6e3          	bltu	a3,a2,10ca4 <.LBB5_31+0x12>
   10dfc:	4621                	li	a2,8
   10dfe:	ea5665e3          	bltu	a2,t0,10ca8 <.LBB5_31+0x16>
   10e02:	82b2                	mv	t0,a2
   10e04:	ea0294e3          	bnez	t0,10cac <.LBB5_31+0x1a>
   10e08:	04000893          	li	a7,64
   10e0c:	02000613          	li	a2,32
   10e10:	8746                	mv	a4,a7
   10e12:	ef1670e3          	bgeu	a2,a7,10cf2 <.LBB5_31+0x60>
   10e16:	b5c5                	j	10cf6 <.LBB5_31+0x64>
   10e18:	fff68513          	add	a0,a3,-1

0000000000010e1c <.LBB5_32>:
   10e1c:	00004617          	auipc	a2,0x4
   10e20:	85460613          	add	a2,a2,-1964 # 14670 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.4>
   10e24:	a031                	j	10e30 <.LBB5_33+0xa>

0000000000010e26 <.LBB5_33>:
   10e26:	00004617          	auipc	a2,0x4
   10e2a:	86260613          	add	a2,a2,-1950 # 14688 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.5>
   10e2e:	557d                	li	a0,-1
   10e30:	02000593          	li	a1,32
   10e34:	00000097          	auipc	ra,0x0
   10e38:	464080e7          	jalr	1124(ra) # 11298 <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>
	...

0000000000010e3e <.LBB5_34>:
   10e3e:	00004617          	auipc	a2,0x4
   10e42:	86260613          	add	a2,a2,-1950 # 146a0 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.6>
   10e46:	02000593          	li	a1,32
   10e4a:	8546                	mv	a0,a7
   10e4c:	00000097          	auipc	ra,0x0
   10e50:	44c080e7          	jalr	1100(ra) # 11298 <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>
	...

0000000000010e56 <.LBB5_35>:
   10e56:	00004517          	auipc	a0,0x4
   10e5a:	86250513          	add	a0,a0,-1950 # 146b8 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.7>

0000000000010e5e <.LBB5_36>:
   10e5e:	00004617          	auipc	a2,0x4
   10e62:	84260613          	add	a2,a2,-1982 # 146a0 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.6>
   10e66:	02800593          	li	a1,40
   10e6a:	00000097          	auipc	ra,0x0
   10e6e:	3a4080e7          	jalr	932(ra) # 1120e <_ZN4core6option13expect_failed17h35a4d77224f066bbE>
	...

0000000000010e74 <_ZN22buddy_system_allocator4Heap7dealloc17h73addb8809d46fa4E>:
   10e74:	1141                	add	sp,sp,-16
   10e76:	e406                	sd	ra,8(sp)
   10e78:	e022                	sd	s0,0(sp)
   10e7a:	0800                	add	s0,sp,16
   10e7c:	4305                	li	t1,1
   10e7e:	4709                	li	a4,2

0000000000010e80 <.LBB6_22>:
   10e80:	00005397          	auipc	t2,0x5
   10e84:	1e038393          	add	t2,t2,480 # 16060 <.LCPI6_0>

0000000000010e88 <.LBB6_23>:
   10e88:	00005297          	auipc	t0,0x5
   10e8c:	1e028293          	add	t0,t0,480 # 16068 <.LCPI6_1>

0000000000010e90 <.LBB6_24>:
   10e90:	00005897          	auipc	a7,0x5
   10e94:	1e088893          	add	a7,a7,480 # 16070 <.LCPI6_2>

0000000000010e98 <.LBB6_25>:
   10e98:	00005817          	auipc	a6,0x5
   10e9c:	1e080813          	add	a6,a6,480 # 16078 <.LCPI6_3>
   10ea0:	0ce67b63          	bgeu	a2,a4,10f76 <.LBB6_25+0xde>
   10ea4:	4721                	li	a4,8
   10ea6:	14d77163          	bgeu	a4,a3,10fe8 <.LBB6_25+0x150>
   10eaa:	1466f263          	bgeu	a3,t1,10fee <.LBB6_25+0x156>
   10eae:	14030363          	beqz	t1,10ff4 <.LBB6_25+0x15c>
   10eb2:	fff30693          	add	a3,t1,-1
   10eb6:	fff34713          	not	a4,t1
   10eba:	0003b383          	ld	t2,0(t2)
   10ebe:	8ef9                	and	a3,a3,a4
   10ec0:	0002b703          	ld	a4,0(t0)
   10ec4:	0016d793          	srl	a5,a3,0x1
   10ec8:	0077f7b3          	and	a5,a5,t2
   10ecc:	8e9d                	sub	a3,a3,a5
   10ece:	00e6f7b3          	and	a5,a3,a4
   10ed2:	8289                	srl	a3,a3,0x2
   10ed4:	8ef9                	and	a3,a3,a4
   10ed6:	96be                	add	a3,a3,a5
   10ed8:	0008b883          	ld	a7,0(a7)
   10edc:	00083783          	ld	a5,0(a6)
   10ee0:	0046d713          	srl	a4,a3,0x4
   10ee4:	96ba                	add	a3,a3,a4
   10ee6:	0116f6b3          	and	a3,a3,a7
   10eea:	02f686b3          	mul	a3,a3,a5
   10eee:	0386d393          	srl	t2,a3,0x38
   10ef2:	487d                	li	a6,31
   10ef4:	10786563          	bltu	a6,t2,10ffe <.LBB6_26>
   10ef8:	00339693          	sll	a3,t2,0x3
   10efc:	96aa                	add	a3,a3,a0
   10efe:	6298                	ld	a4,0(a3)
   10f00:	e198                	sd	a4,0(a1)
   10f02:	e28c                	sd	a1,0(a3)
   10f04:	4885                	li	a7,1
   10f06:	82ae                	mv	t0,a1
   10f08:	007896b3          	sll	a3,a7,t2
   10f0c:	00339713          	sll	a4,t2,0x3
   10f10:	00e50e33          	add	t3,a0,a4
   10f14:	00d2ceb3          	xor	t4,t0,a3
   10f18:	86f2                	mv	a3,t3
   10f1a:	cd9d                	beqz	a1,10f58 <.LBB6_25+0xc0>
   10f1c:	872e                	mv	a4,a1
   10f1e:	87b6                	mv	a5,a3
   10f20:	618c                	ld	a1,0(a1)
   10f22:	86ba                	mv	a3,a4
   10f24:	feee9be3          	bne	t4,a4,10f1a <.LBB6_25+0x82>
   10f28:	e38c                	sd	a1,0(a5)
   10f2a:	000e3583          	ld	a1,0(t3)
   10f2e:	c581                	beqz	a1,10f36 <.LBB6_25+0x9e>
   10f30:	618c                	ld	a1,0(a1)
   10f32:	00be3023          	sd	a1,0(t3)
   10f36:	005ee363          	bltu	t4,t0,10f3c <.LBB6_25+0xa4>
   10f3a:	8e96                	mv	t4,t0
   10f3c:	0d038d63          	beq	t2,a6,11016 <.LBB6_27>
   10f40:	0385                	add	t2,t2,1
   10f42:	00339593          	sll	a1,t2,0x3
   10f46:	95aa                	add	a1,a1,a0
   10f48:	6194                	ld	a3,0(a1)
   10f4a:	00deb023          	sd	a3,0(t4)
   10f4e:	01d5b023          	sd	t4,0(a1)
   10f52:	85f6                	mv	a1,t4
   10f54:	82f6                	mv	t0,t4
   10f56:	bf4d                	j	10f08 <.LBB6_25+0x70>
   10f58:	10053583          	ld	a1,256(a0)
   10f5c:	10853683          	ld	a3,264(a0)
   10f60:	8d91                	sub	a1,a1,a2
   10f62:	10b53023          	sd	a1,256(a0)
   10f66:	406685b3          	sub	a1,a3,t1
   10f6a:	10b53423          	sd	a1,264(a0)
   10f6e:	60a2                	ld	ra,8(sp)
   10f70:	6402                	ld	s0,0(sp)
   10f72:	0141                	add	sp,sp,16
   10f74:	8082                	ret
   10f76:	fff60713          	add	a4,a2,-1
   10f7a:	00175793          	srl	a5,a4,0x1
   10f7e:	8f5d                	or	a4,a4,a5
   10f80:	00275793          	srl	a5,a4,0x2
   10f84:	8f5d                	or	a4,a4,a5
   10f86:	00475793          	srl	a5,a4,0x4
   10f8a:	8f5d                	or	a4,a4,a5
   10f8c:	00875793          	srl	a5,a4,0x8
   10f90:	8f5d                	or	a4,a4,a5
   10f92:	01075793          	srl	a5,a4,0x10
   10f96:	8f5d                	or	a4,a4,a5
   10f98:	02075793          	srl	a5,a4,0x20
   10f9c:	8f5d                	or	a4,a4,a5
   10f9e:	0003b303          	ld	t1,0(t2)
   10fa2:	fff74e13          	not	t3,a4
   10fa6:	0002b783          	ld	a5,0(t0)
   10faa:	001e5713          	srl	a4,t3,0x1
   10fae:	00677733          	and	a4,a4,t1
   10fb2:	40ee0733          	sub	a4,t3,a4
   10fb6:	00f77333          	and	t1,a4,a5
   10fba:	8309                	srl	a4,a4,0x2
   10fbc:	8f7d                	and	a4,a4,a5
   10fbe:	971a                	add	a4,a4,t1
   10fc0:	0008b303          	ld	t1,0(a7)
   10fc4:	00083e03          	ld	t3,0(a6)
   10fc8:	00475793          	srl	a5,a4,0x4
   10fcc:	973e                	add	a4,a4,a5
   10fce:	00677733          	and	a4,a4,t1
   10fd2:	03c70733          	mul	a4,a4,t3
   10fd6:	9361                	srl	a4,a4,0x38
   10fd8:	57fd                	li	a5,-1
   10fda:	00e7d733          	srl	a4,a5,a4
   10fde:	00170313          	add	t1,a4,1
   10fe2:	4721                	li	a4,8
   10fe4:	ecd763e3          	bltu	a4,a3,10eaa <.LBB6_25+0x12>
   10fe8:	46a1                	li	a3,8
   10fea:	ec66e2e3          	bltu	a3,t1,10eae <.LBB6_25+0x16>
   10fee:	8336                	mv	t1,a3
   10ff0:	ec0311e3          	bnez	t1,10eb2 <.LBB6_25+0x1a>
   10ff4:	04000393          	li	t2,64
   10ff8:	487d                	li	a6,31
   10ffa:	ee787fe3          	bgeu	a6,t2,10ef8 <.LBB6_25+0x60>

0000000000010ffe <.LBB6_26>:
   10ffe:	00003617          	auipc	a2,0x3
   11002:	6e260613          	add	a2,a2,1762 # 146e0 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.8>
   11006:	02000593          	li	a1,32
   1100a:	851e                	mv	a0,t2
   1100c:	00000097          	auipc	ra,0x0
   11010:	28c080e7          	jalr	652(ra) # 11298 <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>
	...

0000000000011016 <.LBB6_27>:
   11016:	00003617          	auipc	a2,0x3
   1101a:	6e260613          	add	a2,a2,1762 # 146f8 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.9>
   1101e:	02000513          	li	a0,32
   11022:	02000593          	li	a1,32
   11026:	00000097          	auipc	ra,0x0
   1102a:	272080e7          	jalr	626(ra) # 11298 <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>
	...

0000000000011030 <_ZN78_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..ops..deref..Deref$GT$5deref17h07e5175ddf4fbb7aE>:
   11030:	1141                	add	sp,sp,-16
   11032:	e406                	sd	ra,8(sp)
   11034:	e022                	sd	s0,0(sp)
   11036:	0800                	add	s0,sp,16
   11038:	60a2                	ld	ra,8(sp)
   1103a:	6402                	ld	s0,0(sp)
   1103c:	0141                	add	sp,sp,16
   1103e:	8082                	ret

0000000000011040 <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h6574ad5d6e524a4aE>:
   11040:	1101                	add	sp,sp,-32
   11042:	ec06                	sd	ra,24(sp)
   11044:	e822                	sd	s0,16(sp)
   11046:	e426                	sd	s1,8(sp)
   11048:	e04a                	sd	s2,0(sp)
   1104a:	1000                	add	s0,sp,32
   1104c:	84aa                	mv	s1,a0
   1104e:	4505                	li	a0,1
   11050:	00a4b92f          	amoadd.d	s2,a0,(s1)
   11054:	6488                	ld	a0,8(s1)
   11056:	0230000f          	fence	r,rw
   1105a:	01250963          	beq	a0,s2,1106c <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h6574ad5d6e524a4aE+0x2c>
   1105e:	0100000f          	fence	w,unknown
   11062:	6488                	ld	a0,8(s1)
   11064:	0230000f          	fence	r,rw
   11068:	ff251be3          	bne	a0,s2,1105e <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h6574ad5d6e524a4aE+0x1e>
   1106c:	01048513          	add	a0,s1,16
   11070:	00000097          	auipc	ra,0x0
   11074:	bfe080e7          	jalr	-1026(ra) # 10c6e <_ZN22buddy_system_allocator4Heap5alloc17hcac490460a84cc76E>
   11078:	00190593          	add	a1,s2,1
   1107c:	0310000f          	fence	rw,w
   11080:	e48c                	sd	a1,8(s1)
   11082:	60e2                	ld	ra,24(sp)
   11084:	6442                	ld	s0,16(sp)
   11086:	64a2                	ld	s1,8(sp)
   11088:	6902                	ld	s2,0(sp)
   1108a:	6105                	add	sp,sp,32
   1108c:	8082                	ret

000000000001108e <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hbd168ca56515a3a0E>:
   1108e:	1101                	add	sp,sp,-32
   11090:	ec06                	sd	ra,24(sp)
   11092:	e822                	sd	s0,16(sp)
   11094:	e426                	sd	s1,8(sp)
   11096:	e04a                	sd	s2,0(sp)
   11098:	1000                	add	s0,sp,32
   1109a:	84aa                	mv	s1,a0
   1109c:	4505                	li	a0,1
   1109e:	00a4b92f          	amoadd.d	s2,a0,(s1)
   110a2:	6488                	ld	a0,8(s1)
   110a4:	0230000f          	fence	r,rw
   110a8:	01250963          	beq	a0,s2,110ba <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hbd168ca56515a3a0E+0x2c>
   110ac:	0100000f          	fence	w,unknown
   110b0:	6488                	ld	a0,8(s1)
   110b2:	0230000f          	fence	r,rw
   110b6:	ff251be3          	bne	a0,s2,110ac <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hbd168ca56515a3a0E+0x1e>
   110ba:	01048513          	add	a0,s1,16
   110be:	00000097          	auipc	ra,0x0
   110c2:	db6080e7          	jalr	-586(ra) # 10e74 <_ZN22buddy_system_allocator4Heap7dealloc17h73addb8809d46fa4E>
   110c6:	00190513          	add	a0,s2,1
   110ca:	0310000f          	fence	rw,w
   110ce:	e488                	sd	a0,8(s1)
   110d0:	60e2                	ld	ra,24(sp)
   110d2:	6442                	ld	s0,16(sp)
   110d4:	64a2                	ld	s1,8(sp)
   110d6:	6902                	ld	s2,0(sp)
   110d8:	6105                	add	sp,sp,32
   110da:	8082                	ret

00000000000110dc <_ZN4core3ops8function6FnOnce9call_once17h6ee4dec5eed49e53E>:
   110dc:	1141                	add	sp,sp,-16
   110de:	e406                	sd	ra,8(sp)
   110e0:	00000097          	auipc	ra,0x0
   110e4:	04c080e7          	jalr	76(ra) # 1112c <_ZN5alloc5alloc18handle_alloc_error8rt_error17h3e76e7b1bd103949E>
	...

00000000000110ea <_ZN5alloc7raw_vec17capacity_overflow17h8c103c8c1cb34845E>:
   110ea:	7139                	add	sp,sp,-64
   110ec:	fc06                	sd	ra,56(sp)

00000000000110ee <.LBB18_1>:
   110ee:	00003517          	auipc	a0,0x3
   110f2:	65250513          	add	a0,a0,1618 # 14740 <.Lanon.bc962af17d66e1a6bfbd1e5dd004f443.6>
   110f6:	e42a                	sd	a0,8(sp)
   110f8:	4505                	li	a0,1
   110fa:	e82a                	sd	a0,16(sp)
   110fc:	ec02                	sd	zero,24(sp)

00000000000110fe <.LBB18_2>:
   110fe:	00003517          	auipc	a0,0x3
   11102:	61250513          	add	a0,a0,1554 # 14710 <.Lanon.bc962af17d66e1a6bfbd1e5dd004f443.4>
   11106:	f42a                	sd	a0,40(sp)
   11108:	f802                	sd	zero,48(sp)

000000000001110a <.LBB18_3>:
   1110a:	00003597          	auipc	a1,0x3
   1110e:	64658593          	add	a1,a1,1606 # 14750 <.Lanon.bc962af17d66e1a6bfbd1e5dd004f443.7>
   11112:	0028                	add	a0,sp,8
   11114:	00000097          	auipc	ra,0x0
   11118:	1c4080e7          	jalr	452(ra) # 112d8 <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

000000000001111e <_ZN5alloc5alloc18handle_alloc_error17h48d9534aaf1ab1f8E>:
   1111e:	1141                	add	sp,sp,-16
   11120:	e406                	sd	ra,8(sp)
   11122:	00000097          	auipc	ra,0x0
   11126:	fba080e7          	jalr	-70(ra) # 110dc <_ZN4core3ops8function6FnOnce9call_once17h6ee4dec5eed49e53E>
	...

000000000001112c <_ZN5alloc5alloc18handle_alloc_error8rt_error17h3e76e7b1bd103949E>:
   1112c:	1141                	add	sp,sp,-16
   1112e:	e406                	sd	ra,8(sp)
   11130:	fffff097          	auipc	ra,0xfffff
   11134:	4d0080e7          	jalr	1232(ra) # 10600 <__rust_alloc_error_handler>
	...

000000000001113a <__rg_oom>:
   1113a:	1141                	add	sp,sp,-16
   1113c:	e406                	sd	ra,8(sp)
   1113e:	fffff097          	auipc	ra,0xfffff
   11142:	6a0080e7          	jalr	1696(ra) # 107de <rust_oom>
	...

0000000000011148 <_ZN4core3ops8function6FnOnce9call_once17h0008a32bd325903dE>:
   11148:	1141                	add	sp,sp,-16
   1114a:	e406                	sd	ra,8(sp)
   1114c:	00001097          	auipc	ra,0x1
   11150:	272080e7          	jalr	626(ra) # 123be <_ZN4core5slice5index29slice_start_index_len_fail_rt17h66247b7e841f83e5E>
	...

0000000000011156 <_ZN4core3ops8function6FnOnce9call_once17h0ccd98de653a7264E>:
   11156:	1141                	add	sp,sp,-16
   11158:	e406                	sd	ra,8(sp)
   1115a:	00001097          	auipc	ra,0x1
   1115e:	30c080e7          	jalr	780(ra) # 12466 <_ZN4core5slice5index25slice_index_order_fail_rt17h814668a4a9208686E>
	...

0000000000011164 <_ZN4core3ops8function6FnOnce9call_once17h6b85840bc58c33c1E>:
   11164:	6108                	ld	a0,0(a0)
   11166:	a001                	j	11166 <_ZN4core3ops8function6FnOnce9call_once17h6b85840bc58c33c1E+0x2>

0000000000011168 <_ZN4core3ops8function6FnOnce9call_once17had1f8e39903f1947E>:
   11168:	1141                	add	sp,sp,-16
   1116a:	e406                	sd	ra,8(sp)
   1116c:	00001097          	auipc	ra,0x1
   11170:	2a6080e7          	jalr	678(ra) # 12412 <_ZN4core5slice5index27slice_end_index_len_fail_rt17h3a149a007ccdb3bbE>
	...

0000000000011176 <_ZN4core3ops8function6FnOnce9call_once17hbd59230e70bde5e6E>:
   11176:	1141                	add	sp,sp,-16
   11178:	e406                	sd	ra,8(sp)
   1117a:	00001097          	auipc	ra,0x1
   1117e:	6f4080e7          	jalr	1780(ra) # 1286e <_ZN4core3str19slice_error_fail_rt17hcb246852ed3ab8e1E>
	...

0000000000011184 <_ZN4core3ptr102drop_in_place$LT$$RF$core..iter..adapters..copied..Copied$LT$core..slice..iter..Iter$LT$u8$GT$$GT$$GT$17hea562d0102c22270E>:
   11184:	8082                	ret

0000000000011186 <_ZN4core10intrinsics17const_eval_select17h4d2f7b41c60bf971E>:
   11186:	1141                	add	sp,sp,-16
   11188:	e406                	sd	ra,8(sp)
   1118a:	6118                	ld	a4,0(a0)
   1118c:	650c                	ld	a1,8(a0)
   1118e:	6910                	ld	a2,16(a0)
   11190:	6d14                	ld	a3,24(a0)
   11192:	853a                	mv	a0,a4
   11194:	00000097          	auipc	ra,0x0
   11198:	fe2080e7          	jalr	-30(ra) # 11176 <_ZN4core3ops8function6FnOnce9call_once17hbd59230e70bde5e6E>
	...

000000000001119e <_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h59f5c39c772cdb32E>:
   1119e:	715d                	add	sp,sp,-80
   111a0:	e486                	sd	ra,72(sp)
   111a2:	e0a2                	sd	s0,64(sp)
   111a4:	fc26                	sd	s1,56(sp)
   111a6:	842e                	mv	s0,a1
   111a8:	84aa                	mv	s1,a0
   111aa:	00002097          	auipc	ra,0x2
   111ae:	e8c080e7          	jalr	-372(ra) # 13036 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E>
   111b2:	e51d                	bnez	a0,111e0 <.LBB84_5+0x18>
   111b4:	7008                	ld	a0,32(s0)
   111b6:	740c                	ld	a1,40(s0)

00000000000111b8 <.LBB84_4>:
   111b8:	00003617          	auipc	a2,0x3
   111bc:	79060613          	add	a2,a2,1936 # 14948 <.Lanon.442aba94db1f841cd37d39ada1516238.140>
   111c0:	e432                	sd	a2,8(sp)
   111c2:	4605                	li	a2,1
   111c4:	e832                	sd	a2,16(sp)
   111c6:	ec02                	sd	zero,24(sp)

00000000000111c8 <.LBB84_5>:
   111c8:	00003617          	auipc	a2,0x3
   111cc:	71060613          	add	a2,a2,1808 # 148d8 <.Lanon.442aba94db1f841cd37d39ada1516238.83>
   111d0:	f432                	sd	a2,40(sp)
   111d2:	f802                	sd	zero,48(sp)
   111d4:	0030                	add	a2,sp,8
   111d6:	00001097          	auipc	ra,0x1
   111da:	92c080e7          	jalr	-1748(ra) # 11b02 <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   111de:	c519                	beqz	a0,111ec <.LBB84_5+0x24>
   111e0:	4505                	li	a0,1
   111e2:	60a6                	ld	ra,72(sp)
   111e4:	6406                	ld	s0,64(sp)
   111e6:	74e2                	ld	s1,56(sp)
   111e8:	6161                	add	sp,sp,80
   111ea:	8082                	ret
   111ec:	00848513          	add	a0,s1,8
   111f0:	85a2                	mv	a1,s0
   111f2:	60a6                	ld	ra,72(sp)
   111f4:	6406                	ld	s0,64(sp)
   111f6:	74e2                	ld	s1,56(sp)
   111f8:	6161                	add	sp,sp,80
   111fa:	00002317          	auipc	t1,0x2
   111fe:	e3c30067          	jr	-452(t1) # 13036 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E>

0000000000011202 <_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h34db90cd42bdf307E>:
   11202:	00005517          	auipc	a0,0x5
   11206:	f8650513          	add	a0,a0,-122 # 16188 <.LCPI85_0>
   1120a:	6108                	ld	a0,0(a0)
   1120c:	8082                	ret

000000000001120e <_ZN4core6option13expect_failed17h35a4d77224f066bbE>:
   1120e:	1141                	add	sp,sp,-16
   11210:	e406                	sd	ra,8(sp)
   11212:	00000097          	auipc	ra,0x0
   11216:	03e080e7          	jalr	62(ra) # 11250 <_ZN4core9panicking9panic_str17hf10af3bd6a6f7e35E>
	...

000000000001121c <_ZN4core5panic10panic_info9PanicInfo7message17h4abe68e22d422758E>:
   1121c:	6908                	ld	a0,16(a0)
   1121e:	8082                	ret

0000000000011220 <_ZN4core5panic10panic_info9PanicInfo8location17h873d58c3c1958ff8E>:
   11220:	6d08                	ld	a0,24(a0)
   11222:	8082                	ret

0000000000011224 <_ZN4core9panicking5panic17h92f54f473578363dE>:
   11224:	715d                	add	sp,sp,-80
   11226:	e486                	sd	ra,72(sp)
   11228:	fc2a                	sd	a0,56(sp)
   1122a:	e0ae                	sd	a1,64(sp)
   1122c:	1828                	add	a0,sp,56
   1122e:	e42a                	sd	a0,8(sp)
   11230:	4505                	li	a0,1
   11232:	e82a                	sd	a0,16(sp)
   11234:	ec02                	sd	zero,24(sp)

0000000000011236 <.LBB150_1>:
   11236:	00003517          	auipc	a0,0x3
   1123a:	6a250513          	add	a0,a0,1698 # 148d8 <.Lanon.442aba94db1f841cd37d39ada1516238.83>
   1123e:	f42a                	sd	a0,40(sp)
   11240:	f802                	sd	zero,48(sp)
   11242:	0028                	add	a0,sp,8
   11244:	85b2                	mv	a1,a2
   11246:	00000097          	auipc	ra,0x0
   1124a:	092080e7          	jalr	146(ra) # 112d8 <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

0000000000011250 <_ZN4core9panicking9panic_str17hf10af3bd6a6f7e35E>:
   11250:	1101                	add	sp,sp,-32
   11252:	ec06                	sd	ra,24(sp)
   11254:	e42a                	sd	a0,8(sp)
   11256:	e82e                	sd	a1,16(sp)
   11258:	0028                	add	a0,sp,8
   1125a:	85b2                	mv	a1,a2
   1125c:	00000097          	auipc	ra,0x0
   11260:	00a080e7          	jalr	10(ra) # 11266 <_ZN4core9panicking13panic_display17h2406e68106876c5fE>
	...

0000000000011266 <_ZN4core9panicking13panic_display17h2406e68106876c5fE>:
   11266:	715d                	add	sp,sp,-80
   11268:	e486                	sd	ra,72(sp)
   1126a:	fc2a                	sd	a0,56(sp)

000000000001126c <.LBB152_1>:
   1126c:	00002517          	auipc	a0,0x2
   11270:	1dc50513          	add	a0,a0,476 # 13448 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdd6e30ce7f47bbb8E>
   11274:	e0aa                	sd	a0,64(sp)

0000000000011276 <.LBB152_2>:
   11276:	00003517          	auipc	a0,0x3
   1127a:	71a50513          	add	a0,a0,1818 # 14990 <.Lanon.442aba94db1f841cd37d39ada1516238.203>
   1127e:	e42a                	sd	a0,8(sp)
   11280:	4505                	li	a0,1
   11282:	e82a                	sd	a0,16(sp)
   11284:	ec02                	sd	zero,24(sp)
   11286:	1830                	add	a2,sp,56
   11288:	f432                	sd	a2,40(sp)
   1128a:	f82a                	sd	a0,48(sp)
   1128c:	0028                	add	a0,sp,8
   1128e:	00000097          	auipc	ra,0x0
   11292:	04a080e7          	jalr	74(ra) # 112d8 <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

0000000000011298 <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>:
   11298:	7159                	add	sp,sp,-112
   1129a:	f486                	sd	ra,104(sp)
   1129c:	e42a                	sd	a0,8(sp)
   1129e:	e82e                	sd	a1,16(sp)
   112a0:	0808                	add	a0,sp,16
   112a2:	e4aa                	sd	a0,72(sp)

00000000000112a4 <.LBB153_1>:
   112a4:	00002517          	auipc	a0,0x2
   112a8:	ffe50513          	add	a0,a0,-2 # 132a2 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h68be92e0167cbc01E>
   112ac:	e8aa                	sd	a0,80(sp)
   112ae:	002c                	add	a1,sp,8
   112b0:	ecae                	sd	a1,88(sp)
   112b2:	f0aa                	sd	a0,96(sp)

00000000000112b4 <.LBB153_2>:
   112b4:	00003517          	auipc	a0,0x3
   112b8:	6bc50513          	add	a0,a0,1724 # 14970 <.Lanon.442aba94db1f841cd37d39ada1516238.178>
   112bc:	ec2a                	sd	a0,24(sp)
   112be:	4509                	li	a0,2
   112c0:	f02a                	sd	a0,32(sp)
   112c2:	f402                	sd	zero,40(sp)
   112c4:	00ac                	add	a1,sp,72
   112c6:	fc2e                	sd	a1,56(sp)
   112c8:	e0aa                	sd	a0,64(sp)
   112ca:	0828                	add	a0,sp,24
   112cc:	85b2                	mv	a1,a2
   112ce:	00000097          	auipc	ra,0x0
   112d2:	00a080e7          	jalr	10(ra) # 112d8 <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

00000000000112d8 <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>:
   112d8:	7179                	add	sp,sp,-48
   112da:	f406                	sd	ra,40(sp)

00000000000112dc <.LBB155_1>:
   112dc:	00003617          	auipc	a2,0x3
   112e0:	5fc60613          	add	a2,a2,1532 # 148d8 <.Lanon.442aba94db1f841cd37d39ada1516238.83>
   112e4:	e032                	sd	a2,0(sp)

00000000000112e6 <.LBB155_2>:
   112e6:	00003617          	auipc	a2,0x3
   112ea:	6ba60613          	add	a2,a2,1722 # 149a0 <.Lanon.442aba94db1f841cd37d39ada1516238.208>
   112ee:	e432                	sd	a2,8(sp)
   112f0:	e82a                	sd	a0,16(sp)
   112f2:	ec2e                	sd	a1,24(sp)
   112f4:	4505                	li	a0,1
   112f6:	02a10023          	sb	a0,32(sp)
   112fa:	850a                	mv	a0,sp
   112fc:	00002097          	auipc	ra,0x2
   11300:	3fa080e7          	jalr	1018(ra) # 136f6 <rust_begin_unwind>
	...

0000000000011306 <_ZN4core9panicking19assert_failed_inner17hf0fd0a6058ec5f00E>:
   11306:	7115                	add	sp,sp,-224
   11308:	ed86                	sd	ra,216(sp)
   1130a:	e42e                	sd	a1,8(sp)
   1130c:	e832                	sd	a2,16(sp)
   1130e:	ec36                	sd	a3,24(sp)
   11310:	0ff57513          	zext.b	a0,a0
   11314:	f03a                	sd	a4,32(sp)
   11316:	c909                	beqz	a0,11328 <.LBB160_10>
   11318:	4585                	li	a1,1
   1131a:	04b51d63          	bne	a0,a1,11374 <.LBB160_14>

000000000001131e <.LBB160_9>:
   1131e:	00003517          	auipc	a0,0x3
   11322:	6c950513          	add	a0,a0,1737 # 149e7 <.Lanon.442aba94db1f841cd37d39ada1516238.213>
   11326:	a029                	j	11330 <.LBB160_10+0x8>

0000000000011328 <.LBB160_10>:
   11328:	00003517          	auipc	a0,0x3
   1132c:	6c150513          	add	a0,a0,1729 # 149e9 <.Lanon.442aba94db1f841cd37d39ada1516238.214>
   11330:	f42a                	sd	a0,40(sp)
   11332:	4509                	li	a0,2
   11334:	638c                	ld	a1,0(a5)
   11336:	f82a                	sd	a0,48(sp)
   11338:	e5b9                	bnez	a1,11386 <.LBB160_14+0x12>
   1133a:	1028                	add	a0,sp,40
   1133c:	f4aa                	sd	a0,104(sp)

000000000001133e <.LBB160_11>:
   1133e:	00002517          	auipc	a0,0x2
   11342:	10a50513          	add	a0,a0,266 # 13448 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdd6e30ce7f47bbb8E>
   11346:	f8aa                	sd	a0,112(sp)
   11348:	0028                	add	a0,sp,8
   1134a:	fcaa                	sd	a0,120(sp)

000000000001134c <.LBB160_12>:
   1134c:	00002517          	auipc	a0,0x2
   11350:	0f450513          	add	a0,a0,244 # 13440 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf12cc52608b51daeE>
   11354:	e12a                	sd	a0,128(sp)
   11356:	082c                	add	a1,sp,24
   11358:	e52e                	sd	a1,136(sp)
   1135a:	e92a                	sd	a0,144(sp)

000000000001135c <.LBB160_13>:
   1135c:	00003517          	auipc	a0,0x3
   11360:	71450513          	add	a0,a0,1812 # 14a70 <.Lanon.442aba94db1f841cd37d39ada1516238.221>
   11364:	f52a                	sd	a0,168(sp)
   11366:	4511                	li	a0,4
   11368:	f92a                	sd	a0,176(sp)
   1136a:	fd02                	sd	zero,184(sp)
   1136c:	10a8                	add	a0,sp,104
   1136e:	e5aa                	sd	a0,200(sp)
   11370:	450d                	li	a0,3
   11372:	a885                	j	113e2 <.LBB160_18+0x14>

0000000000011374 <.LBB160_14>:
   11374:	00003517          	auipc	a0,0x3
   11378:	66c50513          	add	a0,a0,1644 # 149e0 <.Lanon.442aba94db1f841cd37d39ada1516238.212>
   1137c:	f42a                	sd	a0,40(sp)
   1137e:	451d                	li	a0,7
   11380:	638c                	ld	a1,0(a5)
   11382:	f82a                	sd	a0,48(sp)
   11384:	d9dd                	beqz	a1,1133a <.LBB160_10+0x12>
   11386:	7788                	ld	a0,40(a5)
   11388:	738c                	ld	a1,32(a5)
   1138a:	f0aa                	sd	a0,96(sp)
   1138c:	ecae                	sd	a1,88(sp)
   1138e:	6f88                	ld	a0,24(a5)
   11390:	6b8c                	ld	a1,16(a5)
   11392:	6790                	ld	a2,8(a5)
   11394:	6394                	ld	a3,0(a5)
   11396:	e8aa                	sd	a0,80(sp)
   11398:	e4ae                	sd	a1,72(sp)
   1139a:	e0b2                	sd	a2,64(sp)
   1139c:	fc36                	sd	a3,56(sp)
   1139e:	1028                	add	a0,sp,40
   113a0:	f4aa                	sd	a0,104(sp)

00000000000113a2 <.LBB160_15>:
   113a2:	00002517          	auipc	a0,0x2
   113a6:	0a650513          	add	a0,a0,166 # 13448 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdd6e30ce7f47bbb8E>
   113aa:	f8aa                	sd	a0,112(sp)
   113ac:	0028                	add	a0,sp,8
   113ae:	fcaa                	sd	a0,120(sp)

00000000000113b0 <.LBB160_16>:
   113b0:	00002517          	auipc	a0,0x2
   113b4:	09050513          	add	a0,a0,144 # 13440 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf12cc52608b51daeE>
   113b8:	e12a                	sd	a0,128(sp)
   113ba:	082c                	add	a1,sp,24
   113bc:	e52e                	sd	a1,136(sp)
   113be:	e92a                	sd	a0,144(sp)
   113c0:	1828                	add	a0,sp,56
   113c2:	ed2a                	sd	a0,152(sp)

00000000000113c4 <.LBB160_17>:
   113c4:	00000517          	auipc	a0,0x0
   113c8:	70c50513          	add	a0,a0,1804 # 11ad0 <_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17h4d3bee89ff077165E>
   113cc:	f12a                	sd	a0,160(sp)

00000000000113ce <.LBB160_18>:
   113ce:	00003517          	auipc	a0,0x3
   113d2:	65a50513          	add	a0,a0,1626 # 14a28 <.Lanon.442aba94db1f841cd37d39ada1516238.219>
   113d6:	f52a                	sd	a0,168(sp)
   113d8:	4511                	li	a0,4
   113da:	f92a                	sd	a0,176(sp)
   113dc:	fd02                	sd	zero,184(sp)
   113de:	10ac                	add	a1,sp,104
   113e0:	e5ae                	sd	a1,200(sp)
   113e2:	e9aa                	sd	a0,208(sp)
   113e4:	1128                	add	a0,sp,168
   113e6:	85c2                	mv	a1,a6
   113e8:	00000097          	auipc	ra,0x0
   113ec:	ef0080e7          	jalr	-272(ra) # 112d8 <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

00000000000113f2 <_ZN4core6result13unwrap_failed17h3c2e5884ed497eadE>:
   113f2:	7119                	add	sp,sp,-128
   113f4:	fc86                	sd	ra,120(sp)
   113f6:	e42a                	sd	a0,8(sp)
   113f8:	e82e                	sd	a1,16(sp)
   113fa:	ec32                	sd	a2,24(sp)
   113fc:	f036                	sd	a3,32(sp)
   113fe:	0028                	add	a0,sp,8
   11400:	ecaa                	sd	a0,88(sp)

0000000000011402 <.LBB161_1>:
   11402:	00002517          	auipc	a0,0x2
   11406:	04650513          	add	a0,a0,70 # 13448 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdd6e30ce7f47bbb8E>
   1140a:	f0aa                	sd	a0,96(sp)
   1140c:	0828                	add	a0,sp,24
   1140e:	f4aa                	sd	a0,104(sp)

0000000000011410 <.LBB161_2>:
   11410:	00002517          	auipc	a0,0x2
   11414:	03050513          	add	a0,a0,48 # 13440 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf12cc52608b51daeE>
   11418:	f8aa                	sd	a0,112(sp)

000000000001141a <.LBB161_3>:
   1141a:	00003517          	auipc	a0,0x3
   1141e:	69e50513          	add	a0,a0,1694 # 14ab8 <.Lanon.442aba94db1f841cd37d39ada1516238.223>
   11422:	f42a                	sd	a0,40(sp)
   11424:	4509                	li	a0,2
   11426:	f82a                	sd	a0,48(sp)
   11428:	fc02                	sd	zero,56(sp)
   1142a:	08ac                	add	a1,sp,88
   1142c:	e4ae                	sd	a1,72(sp)
   1142e:	e8aa                	sd	a0,80(sp)
   11430:	1028                	add	a0,sp,40
   11432:	85ba                	mv	a1,a4
   11434:	00000097          	auipc	ra,0x0
   11438:	ea4080e7          	jalr	-348(ra) # 112d8 <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

000000000001143e <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h3f75591b6059cd6cE>:
   1143e:	7159                	add	sp,sp,-112
   11440:	f486                	sd	ra,104(sp)
   11442:	f0a2                	sd	s0,96(sp)
   11444:	eca6                	sd	s1,88(sp)
   11446:	e8ca                	sd	s2,80(sp)
   11448:	e4ce                	sd	s3,72(sp)
   1144a:	e0d2                	sd	s4,64(sp)
   1144c:	fc56                	sd	s5,56(sp)
   1144e:	f85a                	sd	s6,48(sp)
   11450:	f45e                	sd	s7,40(sp)
   11452:	f062                	sd	s8,32(sp)
   11454:	ec66                	sd	s9,24(sp)
   11456:	e86a                	sd	s10,16(sp)
   11458:	e46e                	sd	s11,8(sp)
   1145a:	1a060263          	beqz	a2,115fe <.LBB163_49+0x142>
   1145e:	8d32                	mv	s10,a2
   11460:	89ae                	mv	s3,a1

0000000000011462 <.LBB163_46>:
   11462:	00005597          	auipc	a1,0x5
   11466:	d3658593          	add	a1,a1,-714 # 16198 <.LCPI163_0>
   1146a:	0005ba03          	ld	s4,0(a1)

000000000001146e <.LBB163_47>:
   1146e:	00005597          	auipc	a1,0x5
   11472:	d3258593          	add	a1,a1,-718 # 161a0 <.LCPI163_1>
   11476:	0005ba83          	ld	s5,0(a1)

000000000001147a <.LBB163_48>:
   1147a:	00005597          	auipc	a1,0x5
   1147e:	d2e58593          	add	a1,a1,-722 # 161a8 <.LCPI163_2>
   11482:	0005bb03          	ld	s6,0(a1)
   11486:	01053b83          	ld	s7,16(a0)
   1148a:	00053903          	ld	s2,0(a0)
   1148e:	00853c03          	ld	s8,8(a0)
   11492:	4cc1                	li	s9,16
   11494:	4da9                	li	s11,10
   11496:	a831                	j	114b2 <.LBB163_48+0x38>
   11498:	00898533          	add	a0,s3,s0
   1149c:	00050503          	lb	a0,0(a0)
   114a0:	fbf00593          	li	a1,-65
   114a4:	16a5df63          	bge	a1,a0,11622 <.LBB163_49+0x166>
   114a8:	408d0d33          	sub	s10,s10,s0
   114ac:	99a2                	add	s3,s3,s0
   114ae:	140d0863          	beqz	s10,115fe <.LBB163_49+0x142>
   114b2:	000bc503          	lbu	a0,0(s7)
   114b6:	cd01                	beqz	a0,114ce <.LBB163_49+0x12>
   114b8:	018c3683          	ld	a3,24(s8)

00000000000114bc <.LBB163_49>:
   114bc:	00003597          	auipc	a1,0x3
   114c0:	bd058593          	add	a1,a1,-1072 # 1408c <.Lanon.fad58de7366495db4650cfefac2fcd61.9+0x7>
   114c4:	4611                	li	a2,4
   114c6:	854a                	mv	a0,s2
   114c8:	9682                	jalr	a3
   114ca:	12051c63          	bnez	a0,11602 <.LBB163_49+0x146>
   114ce:	4681                	li	a3,0
   114d0:	85ea                	mv	a1,s10
   114d2:	a029                	j	114dc <.LBB163_49+0x20>
   114d4:	40dd05b3          	sub	a1,s10,a3
   114d8:	0edd6963          	bltu	s10,a3,115ca <.LBB163_49+0x10e>
   114dc:	00d98633          	add	a2,s3,a3
   114e0:	0195fe63          	bgeu	a1,s9,114fc <.LBB163_49+0x40>
   114e4:	c1fd                	beqz	a1,115ca <.LBB163_49+0x10e>
   114e6:	4701                	li	a4,0
   114e8:	00e60533          	add	a0,a2,a4
   114ec:	00054503          	lbu	a0,0(a0)
   114f0:	09b50f63          	beq	a0,s11,1158e <.LBB163_49+0xd2>
   114f4:	0705                	add	a4,a4,1
   114f6:	fee599e3          	bne	a1,a4,114e8 <.LBB163_49+0x2c>
   114fa:	a8c1                	j	115ca <.LBB163_49+0x10e>
   114fc:	00760513          	add	a0,a2,7
   11500:	9961                	and	a0,a0,-8
   11502:	40c50733          	sub	a4,a0,a2
   11506:	c705                	beqz	a4,1152e <.LBB163_49+0x72>
   11508:	852e                	mv	a0,a1
   1150a:	00e5e363          	bltu	a1,a4,11510 <.LBB163_49+0x54>
   1150e:	853a                	mv	a0,a4
   11510:	4701                	li	a4,0
   11512:	00e607b3          	add	a5,a2,a4
   11516:	0007c783          	lbu	a5,0(a5)
   1151a:	07b78a63          	beq	a5,s11,1158e <.LBB163_49+0xd2>
   1151e:	0705                	add	a4,a4,1
   11520:	fee519e3          	bne	a0,a4,11512 <.LBB163_49+0x56>
   11524:	ff058713          	add	a4,a1,-16
   11528:	00a77663          	bgeu	a4,a0,11534 <.LBB163_49+0x78>
   1152c:	a83d                	j	1156a <.LBB163_49+0xae>
   1152e:	4501                	li	a0,0
   11530:	ff058713          	add	a4,a1,-16
   11534:	00a607b3          	add	a5,a2,a0
   11538:	6380                	ld	s0,0(a5)
   1153a:	fff44493          	not	s1,s0
   1153e:	01644433          	xor	s0,s0,s6
   11542:	9452                	add	s0,s0,s4
   11544:	0154f4b3          	and	s1,s1,s5
   11548:	8ce1                	and	s1,s1,s0
   1154a:	ec91                	bnez	s1,11566 <.LBB163_49+0xaa>
   1154c:	679c                	ld	a5,8(a5)
   1154e:	0167c4b3          	xor	s1,a5,s6
   11552:	fff7c793          	not	a5,a5
   11556:	94d2                	add	s1,s1,s4
   11558:	0157f7b3          	and	a5,a5,s5
   1155c:	8fe5                	and	a5,a5,s1
   1155e:	e781                	bnez	a5,11566 <.LBB163_49+0xaa>
   11560:	0541                	add	a0,a0,16
   11562:	fca779e3          	bgeu	a4,a0,11534 <.LBB163_49+0x78>
   11566:	0ca5e763          	bltu	a1,a0,11634 <.LBB163_49+0x178>
   1156a:	06b50063          	beq	a0,a1,115ca <.LBB163_49+0x10e>
   1156e:	4701                	li	a4,0
   11570:	962a                	add	a2,a2,a0
   11572:	40b505b3          	sub	a1,a0,a1
   11576:	00e607b3          	add	a5,a2,a4
   1157a:	0007c783          	lbu	a5,0(a5)
   1157e:	01b78763          	beq	a5,s11,1158c <.LBB163_49+0xd0>
   11582:	0705                	add	a4,a4,1
   11584:	00e587b3          	add	a5,a1,a4
   11588:	f7fd                	bnez	a5,11576 <.LBB163_49+0xba>
   1158a:	a081                	j	115ca <.LBB163_49+0x10e>
   1158c:	972a                	add	a4,a4,a0
   1158e:	00d70533          	add	a0,a4,a3
   11592:	00150693          	add	a3,a0,1
   11596:	00a6b5b3          	sltu	a1,a3,a0
   1159a:	00dd3633          	sltu	a2,s10,a3
   1159e:	8dd1                	or	a1,a1,a2
   115a0:	f995                	bnez	a1,114d4 <.LBB163_49+0x18>
   115a2:	954e                	add	a0,a0,s3
   115a4:	00054503          	lbu	a0,0(a0)
   115a8:	f3b516e3          	bne	a0,s11,114d4 <.LBB163_49+0x18>
   115ac:	4505                	li	a0,1
   115ae:	00ab8023          	sb	a0,0(s7)
   115b2:	03a6fb63          	bgeu	a3,s10,115e8 <.LBB163_49+0x12c>
   115b6:	00d98533          	add	a0,s3,a3
   115ba:	00050503          	lb	a0,0(a0)
   115be:	fbf00593          	li	a1,-65
   115c2:	02a5d663          	bge	a1,a0,115ee <.LBB163_49+0x132>
   115c6:	8436                	mv	s0,a3
   115c8:	a021                	j	115d0 <.LBB163_49+0x114>
   115ca:	000b8023          	sb	zero,0(s7)
   115ce:	846a                	mv	s0,s10
   115d0:	018c3683          	ld	a3,24(s8)
   115d4:	854a                	mv	a0,s2
   115d6:	85ce                	mv	a1,s3
   115d8:	8622                	mv	a2,s0
   115da:	9682                	jalr	a3
   115dc:	e11d                	bnez	a0,11602 <.LBB163_49+0x146>
   115de:	eba46de3          	bltu	s0,s10,11498 <.LBB163_48+0x1e>
   115e2:	ec8d03e3          	beq	s10,s0,114a8 <.LBB163_48+0x2e>
   115e6:	a835                	j	11622 <.LBB163_49+0x166>
   115e8:	846a                	mv	s0,s10
   115ea:	fedd03e3          	beq	s10,a3,115d0 <.LBB163_49+0x114>
   115ee:	854e                	mv	a0,s3
   115f0:	85ea                	mv	a1,s10
   115f2:	4601                	li	a2,0
   115f4:	00001097          	auipc	ra,0x1
   115f8:	262080e7          	jalr	610(ra) # 12856 <_ZN4core3str16slice_error_fail17h0f23970489177861E>
   115fc:	0000                	unimp
   115fe:	4501                	li	a0,0
   11600:	a011                	j	11604 <.LBB163_49+0x148>
   11602:	4505                	li	a0,1
   11604:	70a6                	ld	ra,104(sp)
   11606:	7406                	ld	s0,96(sp)
   11608:	64e6                	ld	s1,88(sp)
   1160a:	6946                	ld	s2,80(sp)
   1160c:	69a6                	ld	s3,72(sp)
   1160e:	6a06                	ld	s4,64(sp)
   11610:	7ae2                	ld	s5,56(sp)
   11612:	7b42                	ld	s6,48(sp)
   11614:	7ba2                	ld	s7,40(sp)
   11616:	7c02                	ld	s8,32(sp)
   11618:	6ce2                	ld	s9,24(sp)
   1161a:	6d42                	ld	s10,16(sp)
   1161c:	6da2                	ld	s11,8(sp)
   1161e:	6165                	add	sp,sp,112
   11620:	8082                	ret
   11622:	854e                	mv	a0,s3
   11624:	85ea                	mv	a1,s10
   11626:	8622                	mv	a2,s0
   11628:	86ea                	mv	a3,s10
   1162a:	00001097          	auipc	ra,0x1
   1162e:	22c080e7          	jalr	556(ra) # 12856 <_ZN4core3str16slice_error_fail17h0f23970489177861E>
   11632:	0000                	unimp
   11634:	00001097          	auipc	ra,0x1
   11638:	d7c080e7          	jalr	-644(ra) # 123b0 <_ZN4core5slice5index26slice_start_index_len_fail17hc36fc5bb321621f5E>
	...

000000000001163e <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E>:
   1163e:	7171                	add	sp,sp,-176
   11640:	f506                	sd	ra,168(sp)
   11642:	f122                	sd	s0,160(sp)
   11644:	ed26                	sd	s1,152(sp)
   11646:	e94a                	sd	s2,144(sp)
   11648:	e54e                	sd	s3,136(sp)
   1164a:	e152                	sd	s4,128(sp)
   1164c:	fcd6                	sd	s5,120(sp)
   1164e:	f8da                	sd	s6,112(sp)
   11650:	f4de                	sd	s7,104(sp)
   11652:	842a                	mv	s0,a0
   11654:	00854503          	lbu	a0,8(a0)
   11658:	4b85                	li	s7,1
   1165a:	4485                	li	s1,1
   1165c:	c10d                	beqz	a0,1167e <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x40>
   1165e:	00940423          	sb	s1,8(s0)
   11662:	017404a3          	sb	s7,9(s0)
   11666:	8522                	mv	a0,s0
   11668:	70aa                	ld	ra,168(sp)
   1166a:	740a                	ld	s0,160(sp)
   1166c:	64ea                	ld	s1,152(sp)
   1166e:	694a                	ld	s2,144(sp)
   11670:	69aa                	ld	s3,136(sp)
   11672:	6a0a                	ld	s4,128(sp)
   11674:	7ae6                	ld	s5,120(sp)
   11676:	7b46                	ld	s6,112(sp)
   11678:	7ba6                	ld	s7,104(sp)
   1167a:	614d                	add	sp,sp,176
   1167c:	8082                	ret
   1167e:	89ba                	mv	s3,a4
   11680:	8936                	mv	s2,a3
   11682:	8a32                	mv	s4,a2
   11684:	8aae                	mv	s5,a1
   11686:	00043b03          	ld	s6,0(s0)
   1168a:	030b6503          	lwu	a0,48(s6)
   1168e:	00944583          	lbu	a1,9(s0)
   11692:	00457613          	and	a2,a0,4
   11696:	ea09                	bnez	a2,116a8 <.LBB164_18+0xa>
   11698:	0015b613          	seqz	a2,a1
   1169c:	c1e1                	beqz	a1,1175c <.LBB164_23>

000000000001169e <.LBB164_18>:
   1169e:	00003597          	auipc	a1,0x3
   116a2:	46f58593          	add	a1,a1,1135 # 14b0d <.Lanon.442aba94db1f841cd37d39ada1516238.229>
   116a6:	a87d                	j	11764 <.LBB164_23+0x8>
   116a8:	e185                	bnez	a1,116c8 <.LBB164_19+0x14>
   116aa:	028b3583          	ld	a1,40(s6)
   116ae:	020b3503          	ld	a0,32(s6)
   116b2:	6d94                	ld	a3,24(a1)

00000000000116b4 <.LBB164_19>:
   116b4:	00003597          	auipc	a1,0x3
   116b8:	45458593          	add	a1,a1,1108 # 14b08 <.Lanon.442aba94db1f841cd37d39ada1516238.227>
   116bc:	460d                	li	a2,3
   116be:	9682                	jalr	a3
   116c0:	4485                	li	s1,1
   116c2:	fd51                	bnez	a0,1165e <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x20>
   116c4:	030b2503          	lw	a0,48(s6)
   116c8:	4485                	li	s1,1
   116ca:	029103a3          	sb	s1,39(sp)
   116ce:	020b3583          	ld	a1,32(s6)
   116d2:	028b3603          	ld	a2,40(s6)
   116d6:	e42e                	sd	a1,8(sp)
   116d8:	e832                	sd	a2,16(sp)
   116da:	02710593          	add	a1,sp,39
   116de:	ec2e                	sd	a1,24(sp)
   116e0:	034b2583          	lw	a1,52(s6)
   116e4:	038b0603          	lb	a2,56(s6)
   116e8:	000b3683          	ld	a3,0(s6)
   116ec:	008b3703          	ld	a4,8(s6)
   116f0:	010b3783          	ld	a5,16(s6)
   116f4:	018b3803          	ld	a6,24(s6)
   116f8:	ccaa                	sw	a0,88(sp)
   116fa:	ceae                	sw	a1,92(sp)
   116fc:	06c10023          	sb	a2,96(sp)
   11700:	f436                	sd	a3,40(sp)
   11702:	f83a                	sd	a4,48(sp)
   11704:	fc3e                	sd	a5,56(sp)
   11706:	e0c2                	sd	a6,64(sp)
   11708:	0028                	add	a0,sp,8
   1170a:	e4aa                	sd	a0,72(sp)

000000000001170c <.LBB164_20>:
   1170c:	00003517          	auipc	a0,0x3
   11710:	3cc50513          	add	a0,a0,972 # 14ad8 <.Lanon.442aba94db1f841cd37d39ada1516238.224>
   11714:	e8aa                	sd	a0,80(sp)
   11716:	0028                	add	a0,sp,8
   11718:	85d6                	mv	a1,s5
   1171a:	8652                	mv	a2,s4
   1171c:	00000097          	auipc	ra,0x0
   11720:	d22080e7          	jalr	-734(ra) # 1143e <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h3f75591b6059cd6cE>
   11724:	fd0d                	bnez	a0,1165e <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x20>

0000000000011726 <.LBB164_21>:
   11726:	00003597          	auipc	a1,0x3
   1172a:	38a58593          	add	a1,a1,906 # 14ab0 <.Lanon.442aba94db1f841cd37d39ada1516238.222>
   1172e:	0028                	add	a0,sp,8
   11730:	4609                	li	a2,2
   11732:	00000097          	auipc	ra,0x0
   11736:	d0c080e7          	jalr	-756(ra) # 1143e <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h3f75591b6059cd6cE>
   1173a:	f115                	bnez	a0,1165e <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x20>
   1173c:	0189b603          	ld	a2,24(s3)
   11740:	102c                	add	a1,sp,40
   11742:	854a                	mv	a0,s2
   11744:	9602                	jalr	a2
   11746:	fd01                	bnez	a0,1165e <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x20>
   11748:	65c6                	ld	a1,80(sp)
   1174a:	6526                	ld	a0,72(sp)
   1174c:	6d94                	ld	a3,24(a1)

000000000001174e <.LBB164_22>:
   1174e:	00003597          	auipc	a1,0x3
   11752:	3bd58593          	add	a1,a1,957 # 14b0b <.Lanon.442aba94db1f841cd37d39ada1516238.228>
   11756:	4609                	li	a2,2
   11758:	9682                	jalr	a3
   1175a:	a8b1                	j	117b6 <.LBB164_24+0x1c>

000000000001175c <.LBB164_23>:
   1175c:	00003597          	auipc	a1,0x3
   11760:	3b358593          	add	a1,a1,947 # 14b0f <.Lanon.442aba94db1f841cd37d39ada1516238.230>
   11764:	028b3683          	ld	a3,40(s6)
   11768:	020b3503          	ld	a0,32(s6)
   1176c:	6e94                	ld	a3,24(a3)
   1176e:	00266613          	or	a2,a2,2
   11772:	9682                	jalr	a3
   11774:	4485                	li	s1,1
   11776:	ee0514e3          	bnez	a0,1165e <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x20>
   1177a:	028b3583          	ld	a1,40(s6)
   1177e:	020b3503          	ld	a0,32(s6)
   11782:	6d94                	ld	a3,24(a1)
   11784:	85d6                	mv	a1,s5
   11786:	8652                	mv	a2,s4
   11788:	9682                	jalr	a3
   1178a:	4485                	li	s1,1
   1178c:	ec0519e3          	bnez	a0,1165e <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x20>
   11790:	028b3583          	ld	a1,40(s6)
   11794:	020b3503          	ld	a0,32(s6)
   11798:	6d94                	ld	a3,24(a1)

000000000001179a <.LBB164_24>:
   1179a:	00003597          	auipc	a1,0x3
   1179e:	31658593          	add	a1,a1,790 # 14ab0 <.Lanon.442aba94db1f841cd37d39ada1516238.222>
   117a2:	4609                	li	a2,2
   117a4:	9682                	jalr	a3
   117a6:	4485                	li	s1,1
   117a8:	ea051be3          	bnez	a0,1165e <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x20>
   117ac:	0189b603          	ld	a2,24(s3)
   117b0:	854a                	mv	a0,s2
   117b2:	85da                	mv	a1,s6
   117b4:	9602                	jalr	a2
   117b6:	84aa                	mv	s1,a0
   117b8:	b55d                	j	1165e <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x20>

00000000000117ba <_ZN4core3fmt8builders10DebugTuple5field17hb4c5d4885bb0d25dE>:
   117ba:	7135                	add	sp,sp,-160
   117bc:	ed06                	sd	ra,152(sp)
   117be:	e922                	sd	s0,144(sp)
   117c0:	e526                	sd	s1,136(sp)
   117c2:	e14a                	sd	s2,128(sp)
   117c4:	fcce                	sd	s3,120(sp)
   117c6:	f8d2                	sd	s4,112(sp)
   117c8:	f4d6                	sd	s5,104(sp)
   117ca:	842a                	mv	s0,a0
   117cc:	01054503          	lbu	a0,16(a0)
   117d0:	c509                	beqz	a0,117da <_ZN4core3fmt8builders10DebugTuple5field17hb4c5d4885bb0d25dE+0x20>
   117d2:	00843a83          	ld	s5,8(s0)
   117d6:	4485                	li	s1,1
   117d8:	a0e5                	j	118c0 <.LBB167_20+0xe>
   117da:	89b2                	mv	s3,a2
   117dc:	892e                	mv	s2,a1
   117de:	00043a03          	ld	s4,0(s0)
   117e2:	030a6503          	lwu	a0,48(s4)
   117e6:	00843a83          	ld	s5,8(s0)
   117ea:	00457593          	and	a1,a0,4
   117ee:	e991                	bnez	a1,11802 <.LBB167_16+0xa>
   117f0:	001ab613          	seqz	a2,s5
   117f4:	020a8863          	beqz	s5,11824 <.LBB167_18>

00000000000117f8 <.LBB167_16>:
   117f8:	00003597          	auipc	a1,0x3
   117fc:	31558593          	add	a1,a1,789 # 14b0d <.Lanon.442aba94db1f841cd37d39ada1516238.229>
   11800:	a035                	j	1182c <.LBB167_18+0x8>
   11802:	040a9863          	bnez	s5,11852 <.LBB167_18+0x2e>
   11806:	028a3583          	ld	a1,40(s4)
   1180a:	020a3503          	ld	a0,32(s4)
   1180e:	6d94                	ld	a3,24(a1)

0000000000011810 <.LBB167_17>:
   11810:	00003597          	auipc	a1,0x3
   11814:	30558593          	add	a1,a1,773 # 14b15 <.Lanon.442aba94db1f841cd37d39ada1516238.236>
   11818:	4609                	li	a2,2
   1181a:	9682                	jalr	a3
   1181c:	c90d                	beqz	a0,1184e <.LBB167_18+0x2a>
   1181e:	4a81                	li	s5,0
   11820:	4485                	li	s1,1
   11822:	a879                	j	118c0 <.LBB167_20+0xe>

0000000000011824 <.LBB167_18>:
   11824:	00003597          	auipc	a1,0x3
   11828:	2f358593          	add	a1,a1,755 # 14b17 <.Lanon.442aba94db1f841cd37d39ada1516238.237>
   1182c:	028a3683          	ld	a3,40(s4)
   11830:	020a3503          	ld	a0,32(s4)
   11834:	6e94                	ld	a3,24(a3)
   11836:	4709                	li	a4,2
   11838:	40c70633          	sub	a2,a4,a2
   1183c:	9682                	jalr	a3
   1183e:	4485                	li	s1,1
   11840:	e141                	bnez	a0,118c0 <.LBB167_20+0xe>
   11842:	0189b603          	ld	a2,24(s3)
   11846:	854a                	mv	a0,s2
   11848:	85d2                	mv	a1,s4
   1184a:	9602                	jalr	a2
   1184c:	a88d                	j	118be <.LBB167_20+0xc>
   1184e:	030a2503          	lw	a0,48(s4)
   11852:	4485                	li	s1,1
   11854:	029103a3          	sb	s1,39(sp)
   11858:	020a3583          	ld	a1,32(s4)
   1185c:	028a3603          	ld	a2,40(s4)
   11860:	e42e                	sd	a1,8(sp)
   11862:	e832                	sd	a2,16(sp)
   11864:	02710593          	add	a1,sp,39
   11868:	ec2e                	sd	a1,24(sp)
   1186a:	034a2583          	lw	a1,52(s4)
   1186e:	038a0603          	lb	a2,56(s4)
   11872:	000a3683          	ld	a3,0(s4)
   11876:	008a3703          	ld	a4,8(s4)
   1187a:	010a3783          	ld	a5,16(s4)
   1187e:	018a3803          	ld	a6,24(s4)
   11882:	ccaa                	sw	a0,88(sp)
   11884:	ceae                	sw	a1,92(sp)
   11886:	06c10023          	sb	a2,96(sp)
   1188a:	f436                	sd	a3,40(sp)
   1188c:	f83a                	sd	a4,48(sp)
   1188e:	fc3e                	sd	a5,56(sp)
   11890:	e0c2                	sd	a6,64(sp)
   11892:	0028                	add	a0,sp,8
   11894:	0189b603          	ld	a2,24(s3)
   11898:	e4aa                	sd	a0,72(sp)

000000000001189a <.LBB167_19>:
   1189a:	00003517          	auipc	a0,0x3
   1189e:	23e50513          	add	a0,a0,574 # 14ad8 <.Lanon.442aba94db1f841cd37d39ada1516238.224>
   118a2:	e8aa                	sd	a0,80(sp)
   118a4:	102c                	add	a1,sp,40
   118a6:	854a                	mv	a0,s2
   118a8:	9602                	jalr	a2
   118aa:	e919                	bnez	a0,118c0 <.LBB167_20+0xe>
   118ac:	65c6                	ld	a1,80(sp)
   118ae:	6526                	ld	a0,72(sp)
   118b0:	6d94                	ld	a3,24(a1)

00000000000118b2 <.LBB167_20>:
   118b2:	00003597          	auipc	a1,0x3
   118b6:	25958593          	add	a1,a1,601 # 14b0b <.Lanon.442aba94db1f841cd37d39ada1516238.228>
   118ba:	4609                	li	a2,2
   118bc:	9682                	jalr	a3
   118be:	84aa                	mv	s1,a0
   118c0:	00940823          	sb	s1,16(s0)
   118c4:	001a8513          	add	a0,s5,1
   118c8:	e408                	sd	a0,8(s0)
   118ca:	8522                	mv	a0,s0
   118cc:	60ea                	ld	ra,152(sp)
   118ce:	644a                	ld	s0,144(sp)
   118d0:	64aa                	ld	s1,136(sp)
   118d2:	690a                	ld	s2,128(sp)
   118d4:	79e6                	ld	s3,120(sp)
   118d6:	7a46                	ld	s4,112(sp)
   118d8:	7aa6                	ld	s5,104(sp)
   118da:	610d                	add	sp,sp,160
   118dc:	8082                	ret

00000000000118de <_ZN4core3fmt5Write10write_char17hebf5064c644471f8E>:
   118de:	1141                	add	sp,sp,-16
   118e0:	e406                	sd	ra,8(sp)
   118e2:	0005861b          	sext.w	a2,a1
   118e6:	08000693          	li	a3,128
   118ea:	c202                	sw	zero,4(sp)
   118ec:	00d67663          	bgeu	a2,a3,118f8 <_ZN4core3fmt5Write10write_char17hebf5064c644471f8E+0x1a>
   118f0:	00b10223          	sb	a1,4(sp)
   118f4:	4605                	li	a2,1
   118f6:	a849                	j	11988 <_ZN4core3fmt5Write10write_char17hebf5064c644471f8E+0xaa>
   118f8:	00b5d61b          	srlw	a2,a1,0xb
   118fc:	ee19                	bnez	a2,1191a <_ZN4core3fmt5Write10write_char17hebf5064c644471f8E+0x3c>
   118fe:	0065d613          	srl	a2,a1,0x6
   11902:	0c066613          	or	a2,a2,192
   11906:	00c10223          	sb	a2,4(sp)
   1190a:	03f5f593          	and	a1,a1,63
   1190e:	0805e593          	or	a1,a1,128
   11912:	00b102a3          	sb	a1,5(sp)
   11916:	4609                	li	a2,2
   11918:	a885                	j	11988 <_ZN4core3fmt5Write10write_char17hebf5064c644471f8E+0xaa>
   1191a:	0105d61b          	srlw	a2,a1,0x10
   1191e:	e61d                	bnez	a2,1194c <_ZN4core3fmt5Write10write_char17hebf5064c644471f8E+0x6e>
   11920:	00c5d61b          	srlw	a2,a1,0xc
   11924:	0e066613          	or	a2,a2,224
   11928:	00c10223          	sb	a2,4(sp)
   1192c:	0065d61b          	srlw	a2,a1,0x6
   11930:	03f67613          	and	a2,a2,63
   11934:	08066613          	or	a2,a2,128
   11938:	00c102a3          	sb	a2,5(sp)
   1193c:	03f5f593          	and	a1,a1,63
   11940:	0805e593          	or	a1,a1,128
   11944:	00b10323          	sb	a1,6(sp)
   11948:	460d                	li	a2,3
   1194a:	a83d                	j	11988 <_ZN4core3fmt5Write10write_char17hebf5064c644471f8E+0xaa>
   1194c:	0125d61b          	srlw	a2,a1,0x12
   11950:	8a1d                	and	a2,a2,7
   11952:	0f066613          	or	a2,a2,240
   11956:	00c10223          	sb	a2,4(sp)
   1195a:	00c5d61b          	srlw	a2,a1,0xc
   1195e:	03f67613          	and	a2,a2,63
   11962:	08066613          	or	a2,a2,128
   11966:	00c102a3          	sb	a2,5(sp)
   1196a:	0065d61b          	srlw	a2,a1,0x6
   1196e:	03f67613          	and	a2,a2,63
   11972:	08066613          	or	a2,a2,128
   11976:	00c10323          	sb	a2,6(sp)
   1197a:	03f5f593          	and	a1,a1,63
   1197e:	0805e593          	or	a1,a1,128
   11982:	00b103a3          	sb	a1,7(sp)
   11986:	4611                	li	a2,4
   11988:	004c                	add	a1,sp,4
   1198a:	00000097          	auipc	ra,0x0
   1198e:	ab4080e7          	jalr	-1356(ra) # 1143e <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h3f75591b6059cd6cE>
   11992:	60a2                	ld	ra,8(sp)
   11994:	0141                	add	sp,sp,16
   11996:	8082                	ret

0000000000011998 <_ZN4core3fmt5Write9write_fmt17h4daaf3148a56cafcE>:
   11998:	7139                	add	sp,sp,-64
   1199a:	fc06                	sd	ra,56(sp)
   1199c:	7590                	ld	a2,40(a1)
   1199e:	7194                	ld	a3,32(a1)
   119a0:	e02a                	sd	a0,0(sp)
   119a2:	f832                	sd	a2,48(sp)
   119a4:	f436                	sd	a3,40(sp)
   119a6:	6d88                	ld	a0,24(a1)
   119a8:	6990                	ld	a2,16(a1)
   119aa:	6594                	ld	a3,8(a1)
   119ac:	618c                	ld	a1,0(a1)
   119ae:	f02a                	sd	a0,32(sp)
   119b0:	ec32                	sd	a2,24(sp)
   119b2:	e836                	sd	a3,16(sp)
   119b4:	e42e                	sd	a1,8(sp)

00000000000119b6 <.LBB190_1>:
   119b6:	00003597          	auipc	a1,0x3
   119ba:	25a58593          	add	a1,a1,602 # 14c10 <.Lanon.442aba94db1f841cd37d39ada1516238.262>
   119be:	850a                	mv	a0,sp
   119c0:	0030                	add	a2,sp,8
   119c2:	00000097          	auipc	ra,0x0
   119c6:	140080e7          	jalr	320(ra) # 11b02 <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   119ca:	70e2                	ld	ra,56(sp)
   119cc:	6121                	add	sp,sp,64
   119ce:	8082                	ret

00000000000119d0 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h438723c400931e56E>:
   119d0:	6108                	ld	a0,0(a0)
   119d2:	00000317          	auipc	t1,0x0
   119d6:	a6c30067          	jr	-1428(t1) # 1143e <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h3f75591b6059cd6cE>

00000000000119da <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9e9bbdc7252b47dcE>:
   119da:	1141                	add	sp,sp,-16
   119dc:	e406                	sd	ra,8(sp)
   119de:	6108                	ld	a0,0(a0)
   119e0:	0005861b          	sext.w	a2,a1
   119e4:	08000693          	li	a3,128
   119e8:	c202                	sw	zero,4(sp)
   119ea:	00d67663          	bgeu	a2,a3,119f6 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9e9bbdc7252b47dcE+0x1c>
   119ee:	00b10223          	sb	a1,4(sp)
   119f2:	4605                	li	a2,1
   119f4:	a849                	j	11a86 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9e9bbdc7252b47dcE+0xac>
   119f6:	00b5d61b          	srlw	a2,a1,0xb
   119fa:	ee19                	bnez	a2,11a18 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9e9bbdc7252b47dcE+0x3e>
   119fc:	0065d613          	srl	a2,a1,0x6
   11a00:	0c066613          	or	a2,a2,192
   11a04:	00c10223          	sb	a2,4(sp)
   11a08:	03f5f593          	and	a1,a1,63
   11a0c:	0805e593          	or	a1,a1,128
   11a10:	00b102a3          	sb	a1,5(sp)
   11a14:	4609                	li	a2,2
   11a16:	a885                	j	11a86 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9e9bbdc7252b47dcE+0xac>
   11a18:	0105d61b          	srlw	a2,a1,0x10
   11a1c:	e61d                	bnez	a2,11a4a <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9e9bbdc7252b47dcE+0x70>
   11a1e:	00c5d61b          	srlw	a2,a1,0xc
   11a22:	0e066613          	or	a2,a2,224
   11a26:	00c10223          	sb	a2,4(sp)
   11a2a:	0065d61b          	srlw	a2,a1,0x6
   11a2e:	03f67613          	and	a2,a2,63
   11a32:	08066613          	or	a2,a2,128
   11a36:	00c102a3          	sb	a2,5(sp)
   11a3a:	03f5f593          	and	a1,a1,63
   11a3e:	0805e593          	or	a1,a1,128
   11a42:	00b10323          	sb	a1,6(sp)
   11a46:	460d                	li	a2,3
   11a48:	a83d                	j	11a86 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9e9bbdc7252b47dcE+0xac>
   11a4a:	0125d61b          	srlw	a2,a1,0x12
   11a4e:	8a1d                	and	a2,a2,7
   11a50:	0f066613          	or	a2,a2,240
   11a54:	00c10223          	sb	a2,4(sp)
   11a58:	00c5d61b          	srlw	a2,a1,0xc
   11a5c:	03f67613          	and	a2,a2,63
   11a60:	08066613          	or	a2,a2,128
   11a64:	00c102a3          	sb	a2,5(sp)
   11a68:	0065d61b          	srlw	a2,a1,0x6
   11a6c:	03f67613          	and	a2,a2,63
   11a70:	08066613          	or	a2,a2,128
   11a74:	00c10323          	sb	a2,6(sp)
   11a78:	03f5f593          	and	a1,a1,63
   11a7c:	0805e593          	or	a1,a1,128
   11a80:	00b103a3          	sb	a1,7(sp)
   11a84:	4611                	li	a2,4
   11a86:	004c                	add	a1,sp,4
   11a88:	00000097          	auipc	ra,0x0
   11a8c:	9b6080e7          	jalr	-1610(ra) # 1143e <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h3f75591b6059cd6cE>
   11a90:	60a2                	ld	ra,8(sp)
   11a92:	0141                	add	sp,sp,16
   11a94:	8082                	ret

0000000000011a96 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17he338dd4ffa451fd9E>:
   11a96:	7139                	add	sp,sp,-64
   11a98:	fc06                	sd	ra,56(sp)
   11a9a:	6108                	ld	a0,0(a0)
   11a9c:	7590                	ld	a2,40(a1)
   11a9e:	7194                	ld	a3,32(a1)
   11aa0:	e02a                	sd	a0,0(sp)
   11aa2:	f832                	sd	a2,48(sp)
   11aa4:	f436                	sd	a3,40(sp)
   11aa6:	6d88                	ld	a0,24(a1)
   11aa8:	6990                	ld	a2,16(a1)
   11aaa:	6594                	ld	a3,8(a1)
   11aac:	618c                	ld	a1,0(a1)
   11aae:	f02a                	sd	a0,32(sp)
   11ab0:	ec32                	sd	a2,24(sp)
   11ab2:	e836                	sd	a3,16(sp)
   11ab4:	e42e                	sd	a1,8(sp)

0000000000011ab6 <.LBB193_1>:
   11ab6:	00003597          	auipc	a1,0x3
   11aba:	15a58593          	add	a1,a1,346 # 14c10 <.Lanon.442aba94db1f841cd37d39ada1516238.262>
   11abe:	850a                	mv	a0,sp
   11ac0:	0030                	add	a2,sp,8
   11ac2:	00000097          	auipc	ra,0x0
   11ac6:	040080e7          	jalr	64(ra) # 11b02 <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   11aca:	70e2                	ld	ra,56(sp)
   11acc:	6121                	add	sp,sp,64
   11ace:	8082                	ret

0000000000011ad0 <_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17h4d3bee89ff077165E>:
   11ad0:	7139                	add	sp,sp,-64
   11ad2:	fc06                	sd	ra,56(sp)
   11ad4:	7510                	ld	a2,40(a0)
   11ad6:	7118                	ld	a4,32(a0)
   11ad8:	6d1c                	ld	a5,24(a0)
   11ada:	f832                	sd	a2,48(sp)
   11adc:	7194                	ld	a3,32(a1)
   11ade:	f43a                	sd	a4,40(sp)
   11ae0:	f03e                	sd	a5,32(sp)
   11ae2:	6910                	ld	a2,16(a0)
   11ae4:	6518                	ld	a4,8(a0)
   11ae6:	6108                	ld	a0,0(a0)
   11ae8:	758c                	ld	a1,40(a1)
   11aea:	ec32                	sd	a2,24(sp)
   11aec:	e83a                	sd	a4,16(sp)
   11aee:	e42a                	sd	a0,8(sp)
   11af0:	0030                	add	a2,sp,8
   11af2:	8536                	mv	a0,a3
   11af4:	00000097          	auipc	ra,0x0
   11af8:	00e080e7          	jalr	14(ra) # 11b02 <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   11afc:	70e2                	ld	ra,56(sp)
   11afe:	6121                	add	sp,sp,64
   11b00:	8082                	ret

0000000000011b02 <_ZN4core3fmt5write17ha708f69ea5ad27e5E>:
   11b02:	7119                	add	sp,sp,-128
   11b04:	fc86                	sd	ra,120(sp)
   11b06:	f8a2                	sd	s0,112(sp)
   11b08:	f4a6                	sd	s1,104(sp)
   11b0a:	f0ca                	sd	s2,96(sp)
   11b0c:	ecce                	sd	s3,88(sp)
   11b0e:	e8d2                	sd	s4,80(sp)
   11b10:	e4d6                	sd	s5,72(sp)
   11b12:	e0da                	sd	s6,64(sp)
   11b14:	89b2                	mv	s3,a2
   11b16:	4605                	li	a2,1
   11b18:	1616                	sll	a2,a2,0x25
   11b1a:	f832                	sd	a2,48(sp)
   11b1c:	460d                	li	a2,3
   11b1e:	02c10c23          	sb	a2,56(sp)
   11b22:	0109b603          	ld	a2,16(s3)
   11b26:	e002                	sd	zero,0(sp)
   11b28:	e802                	sd	zero,16(sp)
   11b2a:	f02a                	sd	a0,32(sp)
   11b2c:	f42e                	sd	a1,40(sp)
   11b2e:	ca69                	beqz	a2,11c00 <.LBB199_34+0xa6>
   11b30:	0189b503          	ld	a0,24(s3)
   11b34:	10050c63          	beqz	a0,11c4c <.LBB199_34+0xf2>
   11b38:	0009b583          	ld	a1,0(s3)
   11b3c:	fff50693          	add	a3,a0,-1
   11b40:	068e                	sll	a3,a3,0x3
   11b42:	828d                	srl	a3,a3,0x3
   11b44:	00168913          	add	s2,a3,1
   11b48:	00858493          	add	s1,a1,8
   11b4c:	03800593          	li	a1,56
   11b50:	02b50a33          	mul	s4,a0,a1
   11b54:	03060413          	add	s0,a2,48
   11b58:	4a85                	li	s5,1

0000000000011b5a <.LBB199_34>:
   11b5a:	fffffb17          	auipc	s6,0xfffff
   11b5e:	60ab0b13          	add	s6,s6,1546 # 11164 <_ZN4core3ops8function6FnOnce9call_once17h6b85840bc58c33c1E>
   11b62:	6090                	ld	a2,0(s1)
   11b64:	ca09                	beqz	a2,11b76 <.LBB199_34+0x1c>
   11b66:	76a2                	ld	a3,40(sp)
   11b68:	7502                	ld	a0,32(sp)
   11b6a:	ff84b583          	ld	a1,-8(s1)
   11b6e:	6e94                	ld	a3,24(a3)
   11b70:	9682                	jalr	a3
   11b72:	10051963          	bnez	a0,11c84 <.LBB199_34+0x12a>
   11b76:	ff842503          	lw	a0,-8(s0)
   11b7a:	da2a                	sw	a0,52(sp)
   11b7c:	00040503          	lb	a0,0(s0)
   11b80:	02a10c23          	sb	a0,56(sp)
   11b84:	ffc42583          	lw	a1,-4(s0)
   11b88:	0209b503          	ld	a0,32(s3)
   11b8c:	d82e                	sw	a1,48(sp)
   11b8e:	fe843683          	ld	a3,-24(s0)
   11b92:	ff043583          	ld	a1,-16(s0)
   11b96:	ce89                	beqz	a3,11bb0 <.LBB199_34+0x56>
   11b98:	4601                	li	a2,0
   11b9a:	01569c63          	bne	a3,s5,11bb2 <.LBB199_34+0x58>
   11b9e:	0592                	sll	a1,a1,0x4
   11ba0:	95aa                	add	a1,a1,a0
   11ba2:	6590                	ld	a2,8(a1)
   11ba4:	01660463          	beq	a2,s6,11bac <.LBB199_34+0x52>
   11ba8:	4601                	li	a2,0
   11baa:	a021                	j	11bb2 <.LBB199_34+0x58>
   11bac:	618c                	ld	a1,0(a1)
   11bae:	618c                	ld	a1,0(a1)
   11bb0:	4605                	li	a2,1
   11bb2:	e032                	sd	a2,0(sp)
   11bb4:	e42e                	sd	a1,8(sp)
   11bb6:	fd843683          	ld	a3,-40(s0)
   11bba:	fe043583          	ld	a1,-32(s0)
   11bbe:	ce89                	beqz	a3,11bd8 <.LBB199_34+0x7e>
   11bc0:	4601                	li	a2,0
   11bc2:	01569c63          	bne	a3,s5,11bda <.LBB199_34+0x80>
   11bc6:	0592                	sll	a1,a1,0x4
   11bc8:	95aa                	add	a1,a1,a0
   11bca:	6590                	ld	a2,8(a1)
   11bcc:	01660463          	beq	a2,s6,11bd4 <.LBB199_34+0x7a>
   11bd0:	4601                	li	a2,0
   11bd2:	a021                	j	11bda <.LBB199_34+0x80>
   11bd4:	618c                	ld	a1,0(a1)
   11bd6:	618c                	ld	a1,0(a1)
   11bd8:	4605                	li	a2,1
   11bda:	e832                	sd	a2,16(sp)
   11bdc:	ec2e                	sd	a1,24(sp)
   11bde:	fd043583          	ld	a1,-48(s0)
   11be2:	0592                	sll	a1,a1,0x4
   11be4:	952e                	add	a0,a0,a1
   11be6:	6510                	ld	a2,8(a0)
   11be8:	6108                	ld	a0,0(a0)
   11bea:	858a                	mv	a1,sp
   11bec:	9602                	jalr	a2
   11bee:	e959                	bnez	a0,11c84 <.LBB199_34+0x12a>
   11bf0:	04c1                	add	s1,s1,16
   11bf2:	fc8a0a13          	add	s4,s4,-56
   11bf6:	03840413          	add	s0,s0,56
   11bfa:	f60a14e3          	bnez	s4,11b62 <.LBB199_34+0x8>
   11bfe:	a881                	j	11c4e <.LBB199_34+0xf4>
   11c00:	0289b503          	ld	a0,40(s3)
   11c04:	cd29                	beqz	a0,11c5e <.LBB199_34+0x104>
   11c06:	0209b583          	ld	a1,32(s3)
   11c0a:	0009b603          	ld	a2,0(s3)
   11c0e:	157d                	add	a0,a0,-1
   11c10:	0512                	sll	a0,a0,0x4
   11c12:	8111                	srl	a0,a0,0x4
   11c14:	00150913          	add	s2,a0,1
   11c18:	00860413          	add	s0,a2,8
   11c1c:	00858493          	add	s1,a1,8
   11c20:	8a4a                	mv	s4,s2
   11c22:	6010                	ld	a2,0(s0)
   11c24:	ca01                	beqz	a2,11c34 <.LBB199_34+0xda>
   11c26:	76a2                	ld	a3,40(sp)
   11c28:	7502                	ld	a0,32(sp)
   11c2a:	ff843583          	ld	a1,-8(s0)
   11c2e:	6e94                	ld	a3,24(a3)
   11c30:	9682                	jalr	a3
   11c32:	e929                	bnez	a0,11c84 <.LBB199_34+0x12a>
   11c34:	6090                	ld	a2,0(s1)
   11c36:	ff84b503          	ld	a0,-8(s1)
   11c3a:	858a                	mv	a1,sp
   11c3c:	9602                	jalr	a2
   11c3e:	e139                	bnez	a0,11c84 <.LBB199_34+0x12a>
   11c40:	1a7d                	add	s4,s4,-1
   11c42:	0441                	add	s0,s0,16
   11c44:	04c1                	add	s1,s1,16
   11c46:	fc0a1ee3          	bnez	s4,11c22 <.LBB199_34+0xc8>
   11c4a:	a011                	j	11c4e <.LBB199_34+0xf4>
   11c4c:	4901                	li	s2,0
   11c4e:	0089b503          	ld	a0,8(s3)
   11c52:	00a96b63          	bltu	s2,a0,11c68 <.LBB199_34+0x10e>
   11c56:	4601                	li	a2,0
   11c58:	00a96f63          	bltu	s2,a0,11c76 <.LBB199_34+0x11c>
   11c5c:	a035                	j	11c88 <.LBB199_34+0x12e>
   11c5e:	4901                	li	s2,0
   11c60:	0089b503          	ld	a0,8(s3)
   11c64:	fea979e3          	bgeu	s2,a0,11c56 <.LBB199_34+0xfc>
   11c68:	0009b583          	ld	a1,0(s3)
   11c6c:	00491613          	sll	a2,s2,0x4
   11c70:	962e                	add	a2,a2,a1
   11c72:	00a97b63          	bgeu	s2,a0,11c88 <.LBB199_34+0x12e>
   11c76:	76a2                	ld	a3,40(sp)
   11c78:	7502                	ld	a0,32(sp)
   11c7a:	620c                	ld	a1,0(a2)
   11c7c:	6610                	ld	a2,8(a2)
   11c7e:	6e94                	ld	a3,24(a3)
   11c80:	9682                	jalr	a3
   11c82:	c119                	beqz	a0,11c88 <.LBB199_34+0x12e>
   11c84:	4505                	li	a0,1
   11c86:	a011                	j	11c8a <.LBB199_34+0x130>
   11c88:	4501                	li	a0,0
   11c8a:	70e6                	ld	ra,120(sp)
   11c8c:	7446                	ld	s0,112(sp)
   11c8e:	74a6                	ld	s1,104(sp)
   11c90:	7906                	ld	s2,96(sp)
   11c92:	69e6                	ld	s3,88(sp)
   11c94:	6a46                	ld	s4,80(sp)
   11c96:	6aa6                	ld	s5,72(sp)
   11c98:	6b06                	ld	s6,64(sp)
   11c9a:	6109                	add	sp,sp,128
   11c9c:	8082                	ret

0000000000011c9e <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E>:
   11c9e:	711d                	add	sp,sp,-96
   11ca0:	ec86                	sd	ra,88(sp)
   11ca2:	e8a2                	sd	s0,80(sp)
   11ca4:	e4a6                	sd	s1,72(sp)
   11ca6:	e0ca                	sd	s2,64(sp)
   11ca8:	fc4e                	sd	s3,56(sp)
   11caa:	f852                	sd	s4,48(sp)
   11cac:	f456                	sd	s5,40(sp)
   11cae:	f05a                	sd	s6,32(sp)
   11cb0:	ec5e                	sd	s7,24(sp)
   11cb2:	e862                	sd	s8,16(sp)
   11cb4:	e466                	sd	s9,8(sp)
   11cb6:	e06a                	sd	s10,0(sp)
   11cb8:	89be                	mv	s3,a5
   11cba:	893a                	mv	s2,a4
   11cbc:	8b36                	mv	s6,a3
   11cbe:	8ab2                	mv	s5,a2
   11cc0:	8c2a                	mv	s8,a0
   11cc2:	c5b9                	beqz	a1,11d10 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x72>
   11cc4:	030c6503          	lwu	a0,48(s8)
   11cc8:	00157593          	and	a1,a0,1
   11ccc:	00110a37          	lui	s4,0x110
   11cd0:	c199                	beqz	a1,11cd6 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x38>
   11cd2:	02b00a13          	li	s4,43
   11cd6:	01358433          	add	s0,a1,s3
   11cda:	8911                	and	a0,a0,4
   11cdc:	c131                	beqz	a0,11d20 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x82>
   11cde:	02000513          	li	a0,32
   11ce2:	04ab7463          	bgeu	s6,a0,11d2a <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x8c>
   11ce6:	4501                	li	a0,0
   11ce8:	000b0e63          	beqz	s6,11d04 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x66>
   11cec:	fbf00593          	li	a1,-65
   11cf0:	865a                	mv	a2,s6
   11cf2:	86d6                	mv	a3,s5
   11cf4:	00068703          	lb	a4,0(a3)
   11cf8:	0685                	add	a3,a3,1
   11cfa:	00e5a733          	slt	a4,a1,a4
   11cfe:	167d                	add	a2,a2,-1
   11d00:	953a                	add	a0,a0,a4
   11d02:	fa6d                	bnez	a2,11cf4 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x56>
   11d04:	942a                	add	s0,s0,a0
   11d06:	8bd6                	mv	s7,s5
   11d08:	000c3503          	ld	a0,0(s8)
   11d0c:	e915                	bnez	a0,11d40 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0xa2>
   11d0e:	a095                	j	11d72 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0xd4>
   11d10:	030c2503          	lw	a0,48(s8)
   11d14:	00198413          	add	s0,s3,1
   11d18:	02d00a13          	li	s4,45
   11d1c:	8911                	and	a0,a0,4
   11d1e:	f161                	bnez	a0,11cde <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x40>
   11d20:	4b81                	li	s7,0
   11d22:	000c3503          	ld	a0,0(s8)
   11d26:	ed09                	bnez	a0,11d40 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0xa2>
   11d28:	a0a9                	j	11d72 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0xd4>
   11d2a:	8556                	mv	a0,s5
   11d2c:	85da                	mv	a1,s6
   11d2e:	00001097          	auipc	ra,0x1
   11d32:	992080e7          	jalr	-1646(ra) # 126c0 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E>
   11d36:	942a                	add	s0,s0,a0
   11d38:	8bd6                	mv	s7,s5
   11d3a:	000c3503          	ld	a0,0(s8)
   11d3e:	c915                	beqz	a0,11d72 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0xd4>
   11d40:	008c3483          	ld	s1,8(s8)
   11d44:	02947763          	bgeu	s0,s1,11d72 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0xd4>
   11d48:	030c4503          	lbu	a0,48(s8)
   11d4c:	8921                	and	a0,a0,8
   11d4e:	e135                	bnez	a0,11db2 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x114>
   11d50:	038c4583          	lbu	a1,56(s8)
   11d54:	460d                	li	a2,3
   11d56:	4505                	li	a0,1
   11d58:	00c58363          	beq	a1,a2,11d5e <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0xc0>
   11d5c:	852e                	mv	a0,a1
   11d5e:	00357593          	and	a1,a0,3
   11d62:	40848533          	sub	a0,s1,s0
   11d66:	c9d9                	beqz	a1,11dfc <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x15e>
   11d68:	4605                	li	a2,1
   11d6a:	08c59c63          	bne	a1,a2,11e02 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x164>
   11d6e:	4c81                	li	s9,0
   11d70:	a871                	j	11e0c <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x16e>
   11d72:	8562                	mv	a0,s8
   11d74:	85d2                	mv	a1,s4
   11d76:	865e                	mv	a2,s7
   11d78:	86da                	mv	a3,s6
   11d7a:	00000097          	auipc	ra,0x0
   11d7e:	1a0080e7          	jalr	416(ra) # 11f1a <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h7777cb7311890bc3E>
   11d82:	4a85                	li	s5,1
   11d84:	12051363          	bnez	a0,11eaa <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   11d88:	028c3583          	ld	a1,40(s8)
   11d8c:	020c3503          	ld	a0,32(s8)
   11d90:	6d9c                	ld	a5,24(a1)
   11d92:	85ca                	mv	a1,s2
   11d94:	864e                	mv	a2,s3
   11d96:	60e6                	ld	ra,88(sp)
   11d98:	6446                	ld	s0,80(sp)
   11d9a:	64a6                	ld	s1,72(sp)
   11d9c:	6906                	ld	s2,64(sp)
   11d9e:	79e2                	ld	s3,56(sp)
   11da0:	7a42                	ld	s4,48(sp)
   11da2:	7aa2                	ld	s5,40(sp)
   11da4:	7b02                	ld	s6,32(sp)
   11da6:	6be2                	ld	s7,24(sp)
   11da8:	6c42                	ld	s8,16(sp)
   11daa:	6ca2                	ld	s9,8(sp)
   11dac:	6d02                	ld	s10,0(sp)
   11dae:	6125                	add	sp,sp,96
   11db0:	8782                	jr	a5
   11db2:	034c2c83          	lw	s9,52(s8)
   11db6:	03000513          	li	a0,48
   11dba:	038c4d03          	lbu	s10,56(s8)
   11dbe:	02ac2a23          	sw	a0,52(s8)
   11dc2:	4a85                	li	s5,1
   11dc4:	035c0c23          	sb	s5,56(s8)
   11dc8:	8562                	mv	a0,s8
   11dca:	85d2                	mv	a1,s4
   11dcc:	865e                	mv	a2,s7
   11dce:	86da                	mv	a3,s6
   11dd0:	00000097          	auipc	ra,0x0
   11dd4:	14a080e7          	jalr	330(ra) # 11f1a <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h7777cb7311890bc3E>
   11dd8:	e969                	bnez	a0,11eaa <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   11dda:	038c4583          	lbu	a1,56(s8)
   11dde:	460d                	li	a2,3
   11de0:	4505                	li	a0,1
   11de2:	00c58363          	beq	a1,a2,11de8 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x14a>
   11de6:	852e                	mv	a0,a1
   11de8:	00357593          	and	a1,a0,3
   11dec:	40848533          	sub	a0,s1,s0
   11df0:	c5c9                	beqz	a1,11e7a <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x1dc>
   11df2:	4605                	li	a2,1
   11df4:	08c59663          	bne	a1,a2,11e80 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x1e2>
   11df8:	4a01                	li	s4,0
   11dfa:	a841                	j	11e8a <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x1ec>
   11dfc:	8caa                	mv	s9,a0
   11dfe:	4501                	li	a0,0
   11e00:	a031                	j	11e0c <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x16e>
   11e02:	00150593          	add	a1,a0,1
   11e06:	8105                	srl	a0,a0,0x1
   11e08:	0015dc93          	srl	s9,a1,0x1
   11e0c:	020c3a83          	ld	s5,32(s8)
   11e10:	028c3483          	ld	s1,40(s8)
   11e14:	034c2d03          	lw	s10,52(s8)
   11e18:	00150413          	add	s0,a0,1
   11e1c:	147d                	add	s0,s0,-1
   11e1e:	c419                	beqz	s0,11e2c <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x18e>
   11e20:	7090                	ld	a2,32(s1)
   11e22:	8556                	mv	a0,s5
   11e24:	85ea                	mv	a1,s10
   11e26:	9602                	jalr	a2
   11e28:	d975                	beqz	a0,11e1c <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x17e>
   11e2a:	a8bd                	j	11ea8 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20a>
   11e2c:	00110537          	lui	a0,0x110
   11e30:	4a85                	li	s5,1
   11e32:	06ad0c63          	beq	s10,a0,11eaa <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   11e36:	8562                	mv	a0,s8
   11e38:	85d2                	mv	a1,s4
   11e3a:	865e                	mv	a2,s7
   11e3c:	86da                	mv	a3,s6
   11e3e:	00000097          	auipc	ra,0x0
   11e42:	0dc080e7          	jalr	220(ra) # 11f1a <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h7777cb7311890bc3E>
   11e46:	e135                	bnez	a0,11eaa <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   11e48:	028c3583          	ld	a1,40(s8)
   11e4c:	020c3503          	ld	a0,32(s8)
   11e50:	6d94                	ld	a3,24(a1)
   11e52:	85ca                	mv	a1,s2
   11e54:	864e                	mv	a2,s3
   11e56:	9682                	jalr	a3
   11e58:	e929                	bnez	a0,11eaa <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   11e5a:	020c3903          	ld	s2,32(s8)
   11e5e:	028c3483          	ld	s1,40(s8)
   11e62:	4401                	li	s0,0
   11e64:	0a8c8763          	beq	s9,s0,11f12 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x274>
   11e68:	7090                	ld	a2,32(s1)
   11e6a:	0405                	add	s0,s0,1
   11e6c:	854a                	mv	a0,s2
   11e6e:	85ea                	mv	a1,s10
   11e70:	9602                	jalr	a2
   11e72:	d96d                	beqz	a0,11e64 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x1c6>
   11e74:	fff40513          	add	a0,s0,-1
   11e78:	a871                	j	11f14 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x276>
   11e7a:	8a2a                	mv	s4,a0
   11e7c:	4501                	li	a0,0
   11e7e:	a031                	j	11e8a <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x1ec>
   11e80:	00150593          	add	a1,a0,1 # 110001 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xc7c91>
   11e84:	8105                	srl	a0,a0,0x1
   11e86:	0015da13          	srl	s4,a1,0x1
   11e8a:	020c3a83          	ld	s5,32(s8)
   11e8e:	028c3483          	ld	s1,40(s8)
   11e92:	034c2b03          	lw	s6,52(s8)
   11e96:	00150413          	add	s0,a0,1
   11e9a:	147d                	add	s0,s0,-1
   11e9c:	c415                	beqz	s0,11ec8 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x22a>
   11e9e:	7090                	ld	a2,32(s1)
   11ea0:	8556                	mv	a0,s5
   11ea2:	85da                	mv	a1,s6
   11ea4:	9602                	jalr	a2
   11ea6:	d975                	beqz	a0,11e9a <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x1fc>
   11ea8:	4a85                	li	s5,1
   11eaa:	8556                	mv	a0,s5
   11eac:	60e6                	ld	ra,88(sp)
   11eae:	6446                	ld	s0,80(sp)
   11eb0:	64a6                	ld	s1,72(sp)
   11eb2:	6906                	ld	s2,64(sp)
   11eb4:	79e2                	ld	s3,56(sp)
   11eb6:	7a42                	ld	s4,48(sp)
   11eb8:	7aa2                	ld	s5,40(sp)
   11eba:	7b02                	ld	s6,32(sp)
   11ebc:	6be2                	ld	s7,24(sp)
   11ebe:	6c42                	ld	s8,16(sp)
   11ec0:	6ca2                	ld	s9,8(sp)
   11ec2:	6d02                	ld	s10,0(sp)
   11ec4:	6125                	add	sp,sp,96
   11ec6:	8082                	ret
   11ec8:	00110537          	lui	a0,0x110
   11ecc:	4a85                	li	s5,1
   11ece:	fcab0ee3          	beq	s6,a0,11eaa <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   11ed2:	028c3583          	ld	a1,40(s8)
   11ed6:	020c3503          	ld	a0,32(s8)
   11eda:	6d94                	ld	a3,24(a1)
   11edc:	85ca                	mv	a1,s2
   11ede:	864e                	mv	a2,s3
   11ee0:	9682                	jalr	a3
   11ee2:	f561                	bnez	a0,11eaa <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   11ee4:	020c3903          	ld	s2,32(s8)
   11ee8:	028c3483          	ld	s1,40(s8)
   11eec:	4401                	li	s0,0
   11eee:	008a0c63          	beq	s4,s0,11f06 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x268>
   11ef2:	7090                	ld	a2,32(s1)
   11ef4:	0405                	add	s0,s0,1
   11ef6:	854a                	mv	a0,s2
   11ef8:	85da                	mv	a1,s6
   11efa:	9602                	jalr	a2
   11efc:	d96d                	beqz	a0,11eee <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x250>
   11efe:	fff40513          	add	a0,s0,-1
   11f02:	fb4564e3          	bltu	a0,s4,11eaa <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   11f06:	4a81                	li	s5,0
   11f08:	039c2a23          	sw	s9,52(s8)
   11f0c:	03ac0c23          	sb	s10,56(s8)
   11f10:	bf69                	j	11eaa <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   11f12:	8566                	mv	a0,s9
   11f14:	01953ab3          	sltu	s5,a0,s9
   11f18:	bf49                	j	11eaa <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>

0000000000011f1a <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h7777cb7311890bc3E>:
   11f1a:	1101                	add	sp,sp,-32
   11f1c:	ec06                	sd	ra,24(sp)
   11f1e:	e822                	sd	s0,16(sp)
   11f20:	e426                	sd	s1,8(sp)
   11f22:	e04a                	sd	s2,0(sp)
   11f24:	0005871b          	sext.w	a4,a1
   11f28:	001107b7          	lui	a5,0x110
   11f2c:	8936                	mv	s2,a3
   11f2e:	84b2                	mv	s1,a2
   11f30:	842a                	mv	s0,a0
   11f32:	00f70963          	beq	a4,a5,11f44 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h7777cb7311890bc3E+0x2a>
   11f36:	7410                	ld	a2,40(s0)
   11f38:	7008                	ld	a0,32(s0)
   11f3a:	7210                	ld	a2,32(a2)
   11f3c:	9602                	jalr	a2
   11f3e:	85aa                	mv	a1,a0
   11f40:	4505                	li	a0,1
   11f42:	ed91                	bnez	a1,11f5e <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h7777cb7311890bc3E+0x44>
   11f44:	cc81                	beqz	s1,11f5c <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h7777cb7311890bc3E+0x42>
   11f46:	740c                	ld	a1,40(s0)
   11f48:	7008                	ld	a0,32(s0)
   11f4a:	6d9c                	ld	a5,24(a1)
   11f4c:	85a6                	mv	a1,s1
   11f4e:	864a                	mv	a2,s2
   11f50:	60e2                	ld	ra,24(sp)
   11f52:	6442                	ld	s0,16(sp)
   11f54:	64a2                	ld	s1,8(sp)
   11f56:	6902                	ld	s2,0(sp)
   11f58:	6105                	add	sp,sp,32
   11f5a:	8782                	jr	a5
   11f5c:	4501                	li	a0,0
   11f5e:	60e2                	ld	ra,24(sp)
   11f60:	6442                	ld	s0,16(sp)
   11f62:	64a2                	ld	s1,8(sp)
   11f64:	6902                	ld	s2,0(sp)
   11f66:	6105                	add	sp,sp,32
   11f68:	8082                	ret

0000000000011f6a <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E>:
   11f6a:	715d                	add	sp,sp,-80
   11f6c:	e486                	sd	ra,72(sp)
   11f6e:	e0a2                	sd	s0,64(sp)
   11f70:	fc26                	sd	s1,56(sp)
   11f72:	f84a                	sd	s2,48(sp)
   11f74:	f44e                	sd	s3,40(sp)
   11f76:	f052                	sd	s4,32(sp)
   11f78:	ec56                	sd	s5,24(sp)
   11f7a:	e85a                	sd	s6,16(sp)
   11f7c:	e45e                	sd	s7,8(sp)
   11f7e:	84aa                	mv	s1,a0
   11f80:	00053303          	ld	t1,0(a0) # 110000 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xc7c90>
   11f84:	6914                	ld	a3,16(a0)
   11f86:	fff30713          	add	a4,t1,-1
   11f8a:	00e03733          	snez	a4,a4
   11f8e:	fff68793          	add	a5,a3,-1
   11f92:	00f037b3          	snez	a5,a5
   11f96:	8f7d                	and	a4,a4,a5
   11f98:	89b2                	mv	s3,a2
   11f9a:	892e                	mv	s2,a1
   11f9c:	16071b63          	bnez	a4,12112 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1a8>
   11fa0:	4585                	li	a1,1
   11fa2:	10b69763          	bne	a3,a1,120b0 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   11fa6:	6c94                	ld	a3,24(s1)
   11fa8:	01390633          	add	a2,s2,s3
   11fac:	4581                	li	a1,0
   11fae:	cea5                	beqz	a3,12026 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0xbc>
   11fb0:	0e000293          	li	t0,224
   11fb4:	0f000893          	li	a7,240
   11fb8:	00110837          	lui	a6,0x110
   11fbc:	844a                	mv	s0,s2
   11fbe:	a811                	j	11fd2 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x68>
   11fc0:	00140793          	add	a5,s0,1
   11fc4:	40858533          	sub	a0,a1,s0
   11fc8:	16fd                	add	a3,a3,-1
   11fca:	00f505b3          	add	a1,a0,a5
   11fce:	843e                	mv	s0,a5
   11fd0:	cea1                	beqz	a3,12028 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0xbe>
   11fd2:	0cc40f63          	beq	s0,a2,120b0 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   11fd6:	00040783          	lb	a5,0(s0)
   11fda:	fe07d3e3          	bgez	a5,11fc0 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x56>
   11fde:	0ff7f793          	zext.b	a5,a5
   11fe2:	0257ec63          	bltu	a5,t0,1201a <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0xb0>
   11fe6:	0317ed63          	bltu	a5,a7,12020 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0xb6>
   11fea:	00144703          	lbu	a4,1(s0)
   11fee:	00244503          	lbu	a0,2(s0)
   11ff2:	03f77713          	and	a4,a4,63
   11ff6:	03f57513          	and	a0,a0,63
   11ffa:	00344383          	lbu	t2,3(s0)
   11ffe:	17f6                	sll	a5,a5,0x3d
   12000:	93ad                	srl	a5,a5,0x2b
   12002:	0732                	sll	a4,a4,0xc
   12004:	051a                	sll	a0,a0,0x6
   12006:	8d59                	or	a0,a0,a4
   12008:	03f3f713          	and	a4,t2,63
   1200c:	8d59                	or	a0,a0,a4
   1200e:	8d5d                	or	a0,a0,a5
   12010:	0b050063          	beq	a0,a6,120b0 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   12014:	00440793          	add	a5,s0,4
   12018:	b775                	j	11fc4 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x5a>
   1201a:	00240793          	add	a5,s0,2
   1201e:	b75d                	j	11fc4 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x5a>
   12020:	00340793          	add	a5,s0,3
   12024:	b745                	j	11fc4 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x5a>
   12026:	87ca                	mv	a5,s2
   12028:	08c78463          	beq	a5,a2,120b0 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   1202c:	00078603          	lb	a2,0(a5) # 110000 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xc7c90>
   12030:	04065363          	bgez	a2,12076 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x10c>
   12034:	0ff67613          	zext.b	a2,a2
   12038:	0e000513          	li	a0,224
   1203c:	02a66d63          	bltu	a2,a0,12076 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x10c>
   12040:	0f000513          	li	a0,240
   12044:	02a66963          	bltu	a2,a0,12076 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x10c>
   12048:	0017c503          	lbu	a0,1(a5)
   1204c:	0027c683          	lbu	a3,2(a5)
   12050:	03f57513          	and	a0,a0,63
   12054:	03f6f693          	and	a3,a3,63
   12058:	0037c703          	lbu	a4,3(a5)
   1205c:	1676                	sll	a2,a2,0x3d
   1205e:	922d                	srl	a2,a2,0x2b
   12060:	0532                	sll	a0,a0,0xc
   12062:	069a                	sll	a3,a3,0x6
   12064:	8d55                	or	a0,a0,a3
   12066:	03f77693          	and	a3,a4,63
   1206a:	8d55                	or	a0,a0,a3
   1206c:	8d51                	or	a0,a0,a2
   1206e:	00110637          	lui	a2,0x110
   12072:	02c50f63          	beq	a0,a2,120b0 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   12076:	c185                	beqz	a1,12096 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x12c>
   12078:	0335f463          	bgeu	a1,s3,120a0 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x136>
   1207c:	00b90533          	add	a0,s2,a1
   12080:	00050503          	lb	a0,0(a0)
   12084:	fc000613          	li	a2,-64
   12088:	10c54763          	blt	a0,a2,12196 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x22c>
   1208c:	862e                	mv	a2,a1
   1208e:	85b2                	mv	a1,a2
   12090:	864a                	mv	a2,s2
   12092:	ce19                	beqz	a2,120b0 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   12094:	a821                	j	120ac <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x142>
   12096:	4601                	li	a2,0
   12098:	85b2                	mv	a1,a2
   1209a:	864a                	mv	a2,s2
   1209c:	ca11                	beqz	a2,120b0 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   1209e:	a039                	j	120ac <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x142>
   120a0:	864e                	mv	a2,s3
   120a2:	0f359a63          	bne	a1,s3,12196 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x22c>
   120a6:	85b2                	mv	a1,a2
   120a8:	864a                	mv	a2,s2
   120aa:	c219                	beqz	a2,120b0 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   120ac:	89ae                	mv	s3,a1
   120ae:	8932                	mv	s2,a2
   120b0:	06030163          	beqz	t1,12112 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1a8>
   120b4:	6480                	ld	s0,8(s1)
   120b6:	02000513          	li	a0,32
   120ba:	04a9f463          	bgeu	s3,a0,12102 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x198>
   120be:	4501                	li	a0,0
   120c0:	00098e63          	beqz	s3,120dc <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x172>
   120c4:	fbf00593          	li	a1,-65
   120c8:	864e                	mv	a2,s3
   120ca:	86ca                	mv	a3,s2
   120cc:	00068703          	lb	a4,0(a3)
   120d0:	0685                	add	a3,a3,1
   120d2:	00e5a733          	slt	a4,a1,a4
   120d6:	167d                	add	a2,a2,-1 # 10ffff <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xc7c8f>
   120d8:	953a                	add	a0,a0,a4
   120da:	fa6d                	bnez	a2,120cc <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x162>
   120dc:	02857b63          	bgeu	a0,s0,12112 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1a8>
   120e0:	0384c583          	lbu	a1,56(s1)
   120e4:	468d                	li	a3,3
   120e6:	4601                	li	a2,0
   120e8:	00d58363          	beq	a1,a3,120ee <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x184>
   120ec:	862e                	mv	a2,a1
   120ee:	00367593          	and	a1,a2,3
   120f2:	40a40533          	sub	a0,s0,a0
   120f6:	cd95                	beqz	a1,12132 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1c8>
   120f8:	4605                	li	a2,1
   120fa:	02c59f63          	bne	a1,a2,12138 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1ce>
   120fe:	4a81                	li	s5,0
   12100:	a089                	j	12142 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1d8>
   12102:	854a                	mv	a0,s2
   12104:	85ce                	mv	a1,s3
   12106:	00000097          	auipc	ra,0x0
   1210a:	5ba080e7          	jalr	1466(ra) # 126c0 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E>
   1210e:	fc8569e3          	bltu	a0,s0,120e0 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x176>
   12112:	748c                	ld	a1,40(s1)
   12114:	7088                	ld	a0,32(s1)
   12116:	6d9c                	ld	a5,24(a1)
   12118:	85ca                	mv	a1,s2
   1211a:	864e                	mv	a2,s3
   1211c:	60a6                	ld	ra,72(sp)
   1211e:	6406                	ld	s0,64(sp)
   12120:	74e2                	ld	s1,56(sp)
   12122:	7942                	ld	s2,48(sp)
   12124:	79a2                	ld	s3,40(sp)
   12126:	7a02                	ld	s4,32(sp)
   12128:	6ae2                	ld	s5,24(sp)
   1212a:	6b42                	ld	s6,16(sp)
   1212c:	6ba2                	ld	s7,8(sp)
   1212e:	6161                	add	sp,sp,80
   12130:	8782                	jr	a5
   12132:	8aaa                	mv	s5,a0
   12134:	4501                	li	a0,0
   12136:	a031                	j	12142 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1d8>
   12138:	00150593          	add	a1,a0,1
   1213c:	8105                	srl	a0,a0,0x1
   1213e:	0015da93          	srl	s5,a1,0x1
   12142:	0204bb03          	ld	s6,32(s1)
   12146:	0284bb83          	ld	s7,40(s1)
   1214a:	58c4                	lw	s1,52(s1)
   1214c:	00150413          	add	s0,a0,1
   12150:	147d                	add	s0,s0,-1
   12152:	c809                	beqz	s0,12164 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1fa>
   12154:	020bb603          	ld	a2,32(s7)
   12158:	855a                	mv	a0,s6
   1215a:	85a6                	mv	a1,s1
   1215c:	9602                	jalr	a2
   1215e:	d96d                	beqz	a0,12150 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1e6>
   12160:	4a05                	li	s4,1
   12162:	a081                	j	121a2 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x238>
   12164:	00110537          	lui	a0,0x110
   12168:	4a05                	li	s4,1
   1216a:	02a48c63          	beq	s1,a0,121a2 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x238>
   1216e:	018bb683          	ld	a3,24(s7)
   12172:	855a                	mv	a0,s6
   12174:	85ca                	mv	a1,s2
   12176:	864e                	mv	a2,s3
   12178:	9682                	jalr	a3
   1217a:	e505                	bnez	a0,121a2 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x238>
   1217c:	4401                	li	s0,0
   1217e:	008a8f63          	beq	s5,s0,1219c <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x232>
   12182:	020bb603          	ld	a2,32(s7)
   12186:	0405                	add	s0,s0,1
   12188:	855a                	mv	a0,s6
   1218a:	85a6                	mv	a1,s1
   1218c:	9602                	jalr	a2
   1218e:	d965                	beqz	a0,1217e <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x214>
   12190:	fff40513          	add	a0,s0,-1
   12194:	a029                	j	1219e <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x234>
   12196:	4601                	li	a2,0
   12198:	de01                	beqz	a2,120b0 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   1219a:	bf09                	j	120ac <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x142>
   1219c:	8556                	mv	a0,s5
   1219e:	01553a33          	sltu	s4,a0,s5
   121a2:	8552                	mv	a0,s4
   121a4:	60a6                	ld	ra,72(sp)
   121a6:	6406                	ld	s0,64(sp)
   121a8:	74e2                	ld	s1,56(sp)
   121aa:	7942                	ld	s2,48(sp)
   121ac:	79a2                	ld	s3,40(sp)
   121ae:	7a02                	ld	s4,32(sp)
   121b0:	6ae2                	ld	s5,24(sp)
   121b2:	6b42                	ld	s6,16(sp)
   121b4:	6ba2                	ld	s7,8(sp)
   121b6:	6161                	add	sp,sp,80
   121b8:	8082                	ret

00000000000121ba <_ZN4core3fmt9Formatter15debug_lower_hex17h62bc36bb9a6deaceE>:
   121ba:	03054503          	lbu	a0,48(a0) # 110030 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xc7cc0>
   121be:	156e                	sll	a0,a0,0x3b
   121c0:	917d                	srl	a0,a0,0x3f
   121c2:	8082                	ret

00000000000121c4 <_ZN4core3fmt9Formatter15debug_upper_hex17h8071f907d66aecd7E>:
   121c4:	03054503          	lbu	a0,48(a0)
   121c8:	156a                	sll	a0,a0,0x3a
   121ca:	917d                	srl	a0,a0,0x3f
   121cc:	8082                	ret

00000000000121ce <_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hd677dbeaf8ccd015E>:
   121ce:	86ae                	mv	a3,a1
   121d0:	85aa                	mv	a1,a0
   121d2:	8532                	mv	a0,a2
   121d4:	8636                	mv	a2,a3
   121d6:	00000317          	auipc	t1,0x0
   121da:	d9430067          	jr	-620(t1) # 11f6a <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E>

00000000000121de <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hdf624fa96493adceE>:
   121de:	711d                	add	sp,sp,-96
   121e0:	ec86                	sd	ra,88(sp)
   121e2:	e8a2                	sd	s0,80(sp)
   121e4:	e4a6                	sd	s1,72(sp)
   121e6:	e0ca                	sd	s2,64(sp)
   121e8:	fc4e                	sd	s3,56(sp)
   121ea:	f852                	sd	s4,48(sp)
   121ec:	f456                	sd	s5,40(sp)
   121ee:	f05a                	sd	s6,32(sp)
   121f0:	ec5e                	sd	s7,24(sp)
   121f2:	e862                	sd	s8,16(sp)
   121f4:	e466                	sd	s9,8(sp)
   121f6:	7590                	ld	a2,40(a1)
   121f8:	0205ba83          	ld	s5,32(a1)
   121fc:	721c                	ld	a5,32(a2)
   121fe:	84aa                	mv	s1,a0
   12200:	02700593          	li	a1,39
   12204:	02700413          	li	s0,39
   12208:	8556                	mv	a0,s5
   1220a:	e03e                	sd	a5,0(sp)
   1220c:	9782                	jalr	a5
   1220e:	cd19                	beqz	a0,1222c <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hdf624fa96493adceE+0x4e>
   12210:	4505                	li	a0,1
   12212:	60e6                	ld	ra,88(sp)
   12214:	6446                	ld	s0,80(sp)
   12216:	64a6                	ld	s1,72(sp)
   12218:	6906                	ld	s2,64(sp)
   1221a:	79e2                	ld	s3,56(sp)
   1221c:	7a42                	ld	s4,48(sp)
   1221e:	7aa2                	ld	s5,40(sp)
   12220:	7b02                	ld	s6,32(sp)
   12222:	6be2                	ld	s7,24(sp)
   12224:	6c42                	ld	s8,16(sp)
   12226:	6ca2                	ld	s9,8(sp)
   12228:	6125                	add	sp,sp,96
   1222a:	8082                	ret
   1222c:	0004e903          	lwu	s2,0(s1)
   12230:	02b91513          	sll	a0,s2,0x2b
   12234:	912d                	srl	a0,a0,0x2b
   12236:	4489                	li	s1,2
   12238:	02a46063          	bltu	s0,a0,12258 <.LBB242_4+0x6>
   1223c:	00391513          	sll	a0,s2,0x3

0000000000012240 <.LBB242_31>:
   12240:	00002597          	auipc	a1,0x2
   12244:	52858593          	add	a1,a1,1320 # 14768 <.LJTI242_0>
   12248:	952e                	add	a0,a0,a1
   1224a:	6108                	ld	a0,0(a0)
   1224c:	03000993          	li	s3,48
   12250:	8502                	jr	a0

0000000000012252 <.LBB242_4>:
   12252:	07400993          	li	s3,116
   12256:	a065                	j	122fe <.LBB242_14>
   12258:	05c00993          	li	s3,92
   1225c:	03390363          	beq	s2,s3,12282 <.LBB242_9+0x4>

0000000000012260 <.LBB242_6>:
   12260:	854a                	mv	a0,s2
   12262:	00001097          	auipc	ra,0x1
   12266:	2a6080e7          	jalr	678(ra) # 13508 <_ZN4core7unicode12unicode_data15grapheme_extend6lookup17h412ef2d588f4e502E>
   1226a:	e11d                	bnez	a0,12290 <.LBB242_12+0x6>
   1226c:	854a                	mv	a0,s2
   1226e:	00001097          	auipc	ra,0x1
   12272:	9e2080e7          	jalr	-1566(ra) # 12c50 <_ZN4core7unicode9printable12is_printable17h190ab7f3e4a90aabE>
   12276:	cd09                	beqz	a0,12290 <.LBB242_12+0x6>
   12278:	4485                	li	s1,1
   1227a:	89ca                	mv	s3,s2
   1227c:	a049                	j	122fe <.LBB242_14>

000000000001227e <.LBB242_9>:
   1227e:	06e00993          	li	s3,110
   12282:	a8b5                	j	122fe <.LBB242_14>

0000000000012284 <.LBB242_11>:
   12284:	07200993          	li	s3,114
   12288:	a89d                	j	122fe <.LBB242_14>

000000000001228a <.LBB242_12>:
   1228a:	02700993          	li	s3,39
   1228e:	a885                	j	122fe <.LBB242_14>
   12290:	00196513          	or	a0,s2,1
   12294:	00155593          	srl	a1,a0,0x1
   12298:	8d4d                	or	a0,a0,a1
   1229a:	00255593          	srl	a1,a0,0x2
   1229e:	8d4d                	or	a0,a0,a1
   122a0:	00455593          	srl	a1,a0,0x4
   122a4:	8d4d                	or	a0,a0,a1
   122a6:	00855593          	srl	a1,a0,0x8
   122aa:	8d4d                	or	a0,a0,a1
   122ac:	0105559b          	srlw	a1,a0,0x10
   122b0:	8d4d                	or	a0,a0,a1
   122b2:	fff54513          	not	a0,a0
   122b6:	00155593          	srl	a1,a0,0x1
   122ba:	55555637          	lui	a2,0x55555
   122be:	5556061b          	addw	a2,a2,1365 # 55555555 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0x5550d1e5>
   122c2:	8df1                	and	a1,a1,a2
   122c4:	9d0d                	subw	a0,a0,a1
   122c6:	333335b7          	lui	a1,0x33333
   122ca:	3335859b          	addw	a1,a1,819 # 33333333 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0x332eafc3>
   122ce:	00b57633          	and	a2,a0,a1
   122d2:	8109                	srl	a0,a0,0x2
   122d4:	8d6d                	and	a0,a0,a1
   122d6:	9532                	add	a0,a0,a2
   122d8:	00455593          	srl	a1,a0,0x4
   122dc:	952e                	add	a0,a0,a1
   122de:	0f0f15b7          	lui	a1,0xf0f1
   122e2:	f0f5859b          	addw	a1,a1,-241 # f0f0f0f <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xf0a8b9f>
   122e6:	8d6d                	and	a0,a0,a1
   122e8:	010105b7          	lui	a1,0x1010
   122ec:	1015859b          	addw	a1,a1,257 # 1010101 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xfc7d91>
   122f0:	02b5053b          	mulw	a0,a0,a1
   122f4:	01a5551b          	srlw	a0,a0,0x1a
   122f8:	00754a13          	xor	s4,a0,7
   122fc:	448d                	li	s1,3

00000000000122fe <.LBB242_14>:
   122fe:	4415                	li	s0,5
   12300:	4c85                	li	s9,1
   12302:	4b09                	li	s6,2

0000000000012304 <.LBB242_32>:
   12304:	00002b97          	auipc	s7,0x2
   12308:	5a4b8b93          	add	s7,s7,1444 # 148a8 <.LJTI242_1>
   1230c:	4c29                	li	s8,10
   1230e:	a039                	j	1231c <.LBB242_16+0xa>
   12310:	4485                	li	s1,1

0000000000012312 <.LBB242_16>:
   12312:	8556                	mv	a0,s5
   12314:	6782                	ld	a5,0(sp)
   12316:	9782                	jalr	a5
   12318:	ee051ce3          	bnez	a0,12210 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hdf624fa96493adceE+0x32>
   1231c:	049cd763          	bge	s9,s1,1236a <.LBB242_20+0x2e>
   12320:	05c00593          	li	a1,92
   12324:	ff6486e3          	beq	s1,s6,12310 <.LBB242_32+0xc>
   12328:	0ff47513          	zext.b	a0,s0
   1232c:	050e                	sll	a0,a0,0x3
   1232e:	955e                	add	a0,a0,s7
   12330:	6108                	ld	a0,0(a0)
   12332:	448d                	li	s1,3
   12334:	07d00593          	li	a1,125
   12338:	4401                	li	s0,0
   1233a:	8502                	jr	a0

000000000001233c <.LBB242_20>:
   1233c:	002a151b          	sllw	a0,s4,0x2
   12340:	00a9553b          	srlw	a0,s2,a0
   12344:	893d                	and	a0,a0,15
   12346:	03000593          	li	a1,48
   1234a:	01856463          	bltu	a0,s8,12352 <.LBB242_20+0x16>
   1234e:	05700593          	li	a1,87
   12352:	4601                	li	a2,0
   12354:	000a0463          	beqz	s4,1235c <.LBB242_20+0x20>
   12358:	fffa0613          	add	a2,s4,-1 # 10ffff <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xc7c8f>
   1235c:	95aa                	add	a1,a1,a0
   1235e:	001a3513          	seqz	a0,s4
   12362:	40ab0433          	sub	s0,s6,a0
   12366:	8a32                	mv	s4,a2
   12368:	b76d                	j	12312 <.LBB242_16>
   1236a:	03949263          	bne	s1,s9,1238e <.LBB242_30>
   1236e:	4481                	li	s1,0
   12370:	85ce                	mv	a1,s3
   12372:	b745                	j	12312 <.LBB242_16>

0000000000012374 <.LBB242_27>:
   12374:	4409                	li	s0,2
   12376:	07b00593          	li	a1,123
   1237a:	bf61                	j	12312 <.LBB242_16>

000000000001237c <.LBB242_28>:
   1237c:	440d                	li	s0,3
   1237e:	07500593          	li	a1,117
   12382:	448d                	li	s1,3
   12384:	b779                	j	12312 <.LBB242_16>

0000000000012386 <.LBB242_29>:
   12386:	4411                	li	s0,4
   12388:	05c00593          	li	a1,92
   1238c:	b759                	j	12312 <.LBB242_16>

000000000001238e <.LBB242_30>:
   1238e:	02700593          	li	a1,39
   12392:	8556                	mv	a0,s5
   12394:	6782                	ld	a5,0(sp)
   12396:	60e6                	ld	ra,88(sp)
   12398:	6446                	ld	s0,80(sp)
   1239a:	64a6                	ld	s1,72(sp)
   1239c:	6906                	ld	s2,64(sp)
   1239e:	79e2                	ld	s3,56(sp)
   123a0:	7a42                	ld	s4,48(sp)
   123a2:	7aa2                	ld	s5,40(sp)
   123a4:	7b02                	ld	s6,32(sp)
   123a6:	6be2                	ld	s7,24(sp)
   123a8:	6c42                	ld	s8,16(sp)
   123aa:	6ca2                	ld	s9,8(sp)
   123ac:	6125                	add	sp,sp,96
   123ae:	8782                	jr	a5

00000000000123b0 <_ZN4core5slice5index26slice_start_index_len_fail17hc36fc5bb321621f5E>:
   123b0:	1141                	add	sp,sp,-16
   123b2:	e406                	sd	ra,8(sp)
   123b4:	fffff097          	auipc	ra,0xfffff
   123b8:	d94080e7          	jalr	-620(ra) # 11148 <_ZN4core3ops8function6FnOnce9call_once17h0008a32bd325903dE>
	...

00000000000123be <_ZN4core5slice5index29slice_start_index_len_fail_rt17h66247b7e841f83e5E>:
   123be:	7159                	add	sp,sp,-112
   123c0:	f486                	sd	ra,104(sp)
   123c2:	e42a                	sd	a0,8(sp)
   123c4:	e82e                	sd	a1,16(sp)
   123c6:	0028                	add	a0,sp,8
   123c8:	e4aa                	sd	a0,72(sp)

00000000000123ca <.LBB259_1>:
   123ca:	00001517          	auipc	a0,0x1
   123ce:	ed850513          	add	a0,a0,-296 # 132a2 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h68be92e0167cbc01E>
   123d2:	e8aa                	sd	a0,80(sp)
   123d4:	080c                	add	a1,sp,16
   123d6:	ecae                	sd	a1,88(sp)
   123d8:	f0aa                	sd	a0,96(sp)

00000000000123da <.LBB259_2>:
   123da:	00003517          	auipc	a0,0x3
   123de:	89e50513          	add	a0,a0,-1890 # 14c78 <.Lanon.442aba94db1f841cd37d39ada1516238.276>
   123e2:	ec2a                	sd	a0,24(sp)
   123e4:	4509                	li	a0,2
   123e6:	f02a                	sd	a0,32(sp)
   123e8:	f402                	sd	zero,40(sp)
   123ea:	00ac                	add	a1,sp,72
   123ec:	fc2e                	sd	a1,56(sp)
   123ee:	e0aa                	sd	a0,64(sp)

00000000000123f0 <.LBB259_3>:
   123f0:	00003597          	auipc	a1,0x3
   123f4:	8c858593          	add	a1,a1,-1848 # 14cb8 <.Lanon.442aba94db1f841cd37d39ada1516238.278>
   123f8:	0828                	add	a0,sp,24
   123fa:	fffff097          	auipc	ra,0xfffff
   123fe:	ede080e7          	jalr	-290(ra) # 112d8 <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

0000000000012404 <_ZN4core5slice5index24slice_end_index_len_fail17h5d1e1d044f43082eE>:
   12404:	1141                	add	sp,sp,-16
   12406:	e406                	sd	ra,8(sp)
   12408:	fffff097          	auipc	ra,0xfffff
   1240c:	d60080e7          	jalr	-672(ra) # 11168 <_ZN4core3ops8function6FnOnce9call_once17had1f8e39903f1947E>
	...

0000000000012412 <_ZN4core5slice5index27slice_end_index_len_fail_rt17h3a149a007ccdb3bbE>:
   12412:	7159                	add	sp,sp,-112
   12414:	f486                	sd	ra,104(sp)
   12416:	e42a                	sd	a0,8(sp)
   12418:	e82e                	sd	a1,16(sp)
   1241a:	0028                	add	a0,sp,8
   1241c:	e4aa                	sd	a0,72(sp)

000000000001241e <.LBB262_1>:
   1241e:	00001517          	auipc	a0,0x1
   12422:	e8450513          	add	a0,a0,-380 # 132a2 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h68be92e0167cbc01E>
   12426:	e8aa                	sd	a0,80(sp)
   12428:	080c                	add	a1,sp,16
   1242a:	ecae                	sd	a1,88(sp)
   1242c:	f0aa                	sd	a0,96(sp)

000000000001242e <.LBB262_2>:
   1242e:	00003517          	auipc	a0,0x3
   12432:	8a250513          	add	a0,a0,-1886 # 14cd0 <.Lanon.442aba94db1f841cd37d39ada1516238.283>
   12436:	ec2a                	sd	a0,24(sp)
   12438:	4509                	li	a0,2
   1243a:	f02a                	sd	a0,32(sp)
   1243c:	f402                	sd	zero,40(sp)
   1243e:	00ac                	add	a1,sp,72
   12440:	fc2e                	sd	a1,56(sp)
   12442:	e0aa                	sd	a0,64(sp)

0000000000012444 <.LBB262_3>:
   12444:	00003597          	auipc	a1,0x3
   12448:	8ac58593          	add	a1,a1,-1876 # 14cf0 <.Lanon.442aba94db1f841cd37d39ada1516238.284>
   1244c:	0828                	add	a0,sp,24
   1244e:	fffff097          	auipc	ra,0xfffff
   12452:	e8a080e7          	jalr	-374(ra) # 112d8 <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

0000000000012458 <_ZN4core5slice5index22slice_index_order_fail17h5b8db1271a95aea8E>:
   12458:	1141                	add	sp,sp,-16
   1245a:	e406                	sd	ra,8(sp)
   1245c:	fffff097          	auipc	ra,0xfffff
   12460:	cfa080e7          	jalr	-774(ra) # 11156 <_ZN4core3ops8function6FnOnce9call_once17h0ccd98de653a7264E>
	...

0000000000012466 <_ZN4core5slice5index25slice_index_order_fail_rt17h814668a4a9208686E>:
   12466:	7159                	add	sp,sp,-112
   12468:	f486                	sd	ra,104(sp)
   1246a:	e42a                	sd	a0,8(sp)
   1246c:	e82e                	sd	a1,16(sp)
   1246e:	0028                	add	a0,sp,8
   12470:	e4aa                	sd	a0,72(sp)

0000000000012472 <.LBB265_1>:
   12472:	00001517          	auipc	a0,0x1
   12476:	e3050513          	add	a0,a0,-464 # 132a2 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h68be92e0167cbc01E>
   1247a:	e8aa                	sd	a0,80(sp)
   1247c:	080c                	add	a1,sp,16
   1247e:	ecae                	sd	a1,88(sp)
   12480:	f0aa                	sd	a0,96(sp)

0000000000012482 <.LBB265_2>:
   12482:	00003517          	auipc	a0,0x3
   12486:	8ae50513          	add	a0,a0,-1874 # 14d30 <.Lanon.442aba94db1f841cd37d39ada1516238.290>
   1248a:	ec2a                	sd	a0,24(sp)
   1248c:	4509                	li	a0,2
   1248e:	f02a                	sd	a0,32(sp)
   12490:	f402                	sd	zero,40(sp)
   12492:	00ac                	add	a1,sp,72
   12494:	fc2e                	sd	a1,56(sp)
   12496:	e0aa                	sd	a0,64(sp)

0000000000012498 <.LBB265_3>:
   12498:	00003597          	auipc	a1,0x3
   1249c:	8b858593          	add	a1,a1,-1864 # 14d50 <.Lanon.442aba94db1f841cd37d39ada1516238.291>
   124a0:	0828                	add	a0,sp,24
   124a2:	fffff097          	auipc	ra,0xfffff
   124a6:	e36080e7          	jalr	-458(ra) # 112d8 <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

00000000000124ac <_ZN4core3str8converts9from_utf817hbe620603d93abf90E>:
   124ac:	715d                	add	sp,sp,-80
   124ae:	e4a2                	sd	s0,72(sp)
   124b0:	e0a6                	sd	s1,64(sp)
   124b2:	fc4a                	sd	s2,56(sp)
   124b4:	f84e                	sd	s3,48(sp)
   124b6:	f452                	sd	s4,40(sp)
   124b8:	f056                	sd	s5,32(sp)
   124ba:	ec5a                	sd	s6,24(sp)
   124bc:	e85e                	sd	s7,16(sp)
   124be:	e462                	sd	s8,8(sp)
   124c0:	e066                	sd	s9,0(sp)
   124c2:	ff160693          	add	a3,a2,-15
   124c6:	4c81                	li	s9,0
   124c8:	00d66363          	bltu	a2,a3,124ce <_ZN4core3str8converts9from_utf817hbe620603d93abf90E+0x22>
   124cc:	8cb6                	mv	s9,a3
   124ce:	1a060a63          	beqz	a2,12682 <.LBB274_50+0x19a>
   124d2:	4681                	li	a3,0
   124d4:	00758713          	add	a4,a1,7
   124d8:	9b61                	and	a4,a4,-8
   124da:	40b70833          	sub	a6,a4,a1

00000000000124de <.LBB274_49>:
   124de:	00004717          	auipc	a4,0x4
   124e2:	d8a70713          	add	a4,a4,-630 # 16268 <.LCPI274_0>
   124e6:	631c                	ld	a5,0(a4)

00000000000124e8 <.LBB274_50>:
   124e8:	00003897          	auipc	a7,0x3
   124ec:	88088893          	add	a7,a7,-1920 # 14d68 <.Lanon.442aba94db1f841cd37d39ada1516238.311>
   124f0:	4291                	li	t0,4
   124f2:	0f000313          	li	t1,240
   124f6:	03000393          	li	t2,48
   124fa:	fbf00e13          	li	t3,-65
   124fe:	0f400e93          	li	t4,244
   12502:	f8f00f13          	li	t5,-113
   12506:	4f8d                	li	t6,3
   12508:	0e000a93          	li	s5,224
   1250c:	fa000913          	li	s2,-96
   12510:	0ed00a13          	li	s4,237
   12514:	49b1                	li	s3,12
   12516:	4b09                	li	s6,2
   12518:	4b85                	li	s7,1
   1251a:	a021                	j	12522 <.LBB274_50+0x3a>
   1251c:	0685                	add	a3,a3,1
   1251e:	16c6f263          	bgeu	a3,a2,12682 <.LBB274_50+0x19a>
   12522:	00d58733          	add	a4,a1,a3
   12526:	00074403          	lbu	s0,0(a4)
   1252a:	03841713          	sll	a4,s0,0x38
   1252e:	43875493          	sra	s1,a4,0x38
   12532:	0404c663          	bltz	s1,1257e <.LBB274_50+0x96>
   12536:	00180713          	add	a4,a6,1 # 110001 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xc7c91>
   1253a:	00e03733          	snez	a4,a4
   1253e:	40d8043b          	subw	s0,a6,a3
   12542:	881d                	and	s0,s0,7
   12544:	00143413          	seqz	s0,s0
   12548:	8f61                	and	a4,a4,s0
   1254a:	db69                	beqz	a4,1251c <.LBB274_50+0x34>
   1254c:	0196fd63          	bgeu	a3,s9,12566 <.LBB274_50+0x7e>
   12550:	00d58733          	add	a4,a1,a3
   12554:	6300                	ld	s0,0(a4)
   12556:	8c7d                	and	s0,s0,a5
   12558:	e419                	bnez	s0,12566 <.LBB274_50+0x7e>
   1255a:	6718                	ld	a4,8(a4)
   1255c:	8f7d                	and	a4,a4,a5
   1255e:	e701                	bnez	a4,12566 <.LBB274_50+0x7e>
   12560:	06c1                	add	a3,a3,16
   12562:	ff96e7e3          	bltu	a3,s9,12550 <.LBB274_50+0x68>
   12566:	10c6fc63          	bgeu	a3,a2,1267e <.LBB274_50+0x196>
   1256a:	00d58733          	add	a4,a1,a3
   1256e:	00070703          	lb	a4,0(a4)
   12572:	10074663          	bltz	a4,1267e <.LBB274_50+0x196>
   12576:	0685                	add	a3,a3,1
   12578:	fed619e3          	bne	a2,a3,1256a <.LBB274_50+0x82>
   1257c:	a219                	j	12682 <.LBB274_50+0x19a>
   1257e:	01140733          	add	a4,s0,a7
   12582:	00074703          	lbu	a4,0(a4)
   12586:	02570263          	beq	a4,t0,125aa <.LBB274_50+0xc2>
   1258a:	03f70e63          	beq	a4,t6,125c6 <.LBB274_50+0xde>
   1258e:	11671d63          	bne	a4,s6,126a8 <.LBB274_50+0x1c0>
   12592:	00168413          	add	s0,a3,1
   12596:	10c47663          	bgeu	s0,a2,126a2 <.LBB274_50+0x1ba>
   1259a:	00858733          	add	a4,a1,s0
   1259e:	00070703          	lb	a4,0(a4)
   125a2:	4485                	li	s1,1
   125a4:	0cee5b63          	bge	t3,a4,1267a <.LBB274_50+0x192>
   125a8:	a229                	j	126b2 <.LBB274_50+0x1ca>
   125aa:	00168713          	add	a4,a3,1
   125ae:	0ec77c63          	bgeu	a4,a2,126a6 <.LBB274_50+0x1be>
   125b2:	972e                	add	a4,a4,a1
   125b4:	00070c03          	lb	s8,0(a4)
   125b8:	02640563          	beq	s0,t1,125e2 <.LBB274_50+0xfa>
   125bc:	03d41a63          	bne	s0,t4,125f0 <.LBB274_50+0x108>
   125c0:	058f5563          	bge	t5,s8,1260a <.LBB274_50+0x122>
   125c4:	a0d5                	j	126a8 <.LBB274_50+0x1c0>
   125c6:	00168713          	add	a4,a3,1
   125ca:	0cc77e63          	bgeu	a4,a2,126a6 <.LBB274_50+0x1be>
   125ce:	972e                	add	a4,a4,a1
   125d0:	00070c03          	lb	s8,0(a4)
   125d4:	05540f63          	beq	s0,s5,12632 <.LBB274_50+0x14a>
   125d8:	07441263          	bne	s0,s4,1263c <.LBB274_50+0x154>
   125dc:	092c4563          	blt	s8,s2,12666 <.LBB274_50+0x17e>
   125e0:	a0e1                	j	126a8 <.LBB274_50+0x1c0>
   125e2:	070c071b          	addw	a4,s8,112
   125e6:	0ff77713          	zext.b	a4,a4
   125ea:	02776063          	bltu	a4,t2,1260a <.LBB274_50+0x122>
   125ee:	a86d                	j	126a8 <.LBB274_50+0x1c0>
   125f0:	00f4871b          	addw	a4,s1,15
   125f4:	0ff77713          	zext.b	a4,a4
   125f8:	00373713          	sltiu	a4,a4,3
   125fc:	000c2413          	slti	s0,s8,0
   12600:	8f61                	and	a4,a4,s0
   12602:	fc0c3413          	sltiu	s0,s8,-64
   12606:	8f61                	and	a4,a4,s0
   12608:	c345                	beqz	a4,126a8 <.LBB274_50+0x1c0>
   1260a:	00268713          	add	a4,a3,2
   1260e:	08c77a63          	bgeu	a4,a2,126a2 <.LBB274_50+0x1ba>
   12612:	972e                	add	a4,a4,a1
   12614:	00070703          	lb	a4,0(a4)
   12618:	08ee4a63          	blt	t3,a4,126ac <.LBB274_50+0x1c4>
   1261c:	00368413          	add	s0,a3,3
   12620:	08c47163          	bgeu	s0,a2,126a2 <.LBB274_50+0x1ba>
   12624:	00858733          	add	a4,a1,s0
   12628:	00070703          	lb	a4,0(a4)
   1262c:	04ee5763          	bge	t3,a4,1267a <.LBB274_50+0x192>
   12630:	a041                	j	126b0 <.LBB274_50+0x1c8>
   12632:	fe0c7713          	and	a4,s8,-32
   12636:	03270863          	beq	a4,s2,12666 <.LBB274_50+0x17e>
   1263a:	a0bd                	j	126a8 <.LBB274_50+0x1c0>
   1263c:	01f4871b          	addw	a4,s1,31
   12640:	0ff77713          	zext.b	a4,a4
   12644:	01377563          	bgeu	a4,s3,1264e <.LBB274_50+0x166>
   12648:	018e5f63          	bge	t3,s8,12666 <.LBB274_50+0x17e>
   1264c:	a8b1                	j	126a8 <.LBB274_50+0x1c0>
   1264e:	ffe4f713          	and	a4,s1,-2
   12652:	0749                	add	a4,a4,18
   12654:	00173713          	seqz	a4,a4
   12658:	000c2413          	slti	s0,s8,0
   1265c:	8f61                	and	a4,a4,s0
   1265e:	fc0c3413          	sltiu	s0,s8,-64
   12662:	8f61                	and	a4,a4,s0
   12664:	c331                	beqz	a4,126a8 <.LBB274_50+0x1c0>
   12666:	00268413          	add	s0,a3,2
   1266a:	02c47c63          	bgeu	s0,a2,126a2 <.LBB274_50+0x1ba>
   1266e:	00858733          	add	a4,a1,s0
   12672:	00070703          	lb	a4,0(a4)
   12676:	02ee4b63          	blt	t3,a4,126ac <.LBB274_50+0x1c4>
   1267a:	00140693          	add	a3,s0,1
   1267e:	eac6e2e3          	bltu	a3,a2,12522 <.LBB274_50+0x3a>
   12682:	4681                	li	a3,0
   12684:	e50c                	sd	a1,8(a0)
   12686:	e910                	sd	a2,16(a0)
   12688:	e114                	sd	a3,0(a0)
   1268a:	6426                	ld	s0,72(sp)
   1268c:	6486                	ld	s1,64(sp)
   1268e:	7962                	ld	s2,56(sp)
   12690:	79c2                	ld	s3,48(sp)
   12692:	7a22                	ld	s4,40(sp)
   12694:	7a82                	ld	s5,32(sp)
   12696:	6b62                	ld	s6,24(sp)
   12698:	6bc2                	ld	s7,16(sp)
   1269a:	6c22                	ld	s8,8(sp)
   1269c:	6c82                	ld	s9,0(sp)
   1269e:	6161                	add	sp,sp,80
   126a0:	8082                	ret
   126a2:	4b81                	li	s7,0
   126a4:	a039                	j	126b2 <.LBB274_50+0x1ca>
   126a6:	4b81                	li	s7,0
   126a8:	4485                	li	s1,1
   126aa:	a021                	j	126b2 <.LBB274_50+0x1ca>
   126ac:	4489                	li	s1,2
   126ae:	a011                	j	126b2 <.LBB274_50+0x1ca>
   126b0:	448d                	li	s1,3
   126b2:	e514                	sd	a3,8(a0)
   126b4:	01750823          	sb	s7,16(a0)
   126b8:	009508a3          	sb	s1,17(a0)
   126bc:	4685                	li	a3,1
   126be:	b7e9                	j	12688 <.LBB274_50+0x1a0>

00000000000126c0 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E>:
   126c0:	862a                	mv	a2,a0
   126c2:	051d                	add	a0,a0,7
   126c4:	9961                	and	a0,a0,-8
   126c6:	40c508b3          	sub	a7,a0,a2
   126ca:	0115eb63          	bltu	a1,a7,126e0 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x20>
   126ce:	41158833          	sub	a6,a1,a7
   126d2:	00883693          	sltiu	a3,a6,8
   126d6:	4721                	li	a4,8
   126d8:	01173733          	sltu	a4,a4,a7
   126dc:	8ed9                	or	a3,a3,a4
   126de:	ce91                	beqz	a3,126fa <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x3a>
   126e0:	4501                	li	a0,0
   126e2:	c999                	beqz	a1,126f8 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x38>
   126e4:	fbf00693          	li	a3,-65
   126e8:	00060703          	lb	a4,0(a2)
   126ec:	0605                	add	a2,a2,1
   126ee:	00e6a733          	slt	a4,a3,a4
   126f2:	15fd                	add	a1,a1,-1
   126f4:	953a                	add	a0,a0,a4
   126f6:	f9ed                	bnez	a1,126e8 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x28>
   126f8:	8082                	ret
   126fa:	00787693          	and	a3,a6,7
   126fe:	4701                	li	a4,0
   12700:	00088f63          	beqz	a7,1271e <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x5e>
   12704:	40a60533          	sub	a0,a2,a0
   12708:	fbf00293          	li	t0,-65
   1270c:	87b2                	mv	a5,a2
   1270e:	00078583          	lb	a1,0(a5)
   12712:	0785                	add	a5,a5,1
   12714:	00b2a5b3          	slt	a1,t0,a1
   12718:	0505                	add	a0,a0,1
   1271a:	972e                	add	a4,a4,a1
   1271c:	f96d                	bnez	a0,1270e <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x4e>
   1271e:	011602b3          	add	t0,a2,a7
   12722:	4581                	li	a1,0
   12724:	ce99                	beqz	a3,12742 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x82>
   12726:	ff887513          	and	a0,a6,-8
   1272a:	00a28633          	add	a2,t0,a0
   1272e:	fbf00513          	li	a0,-65
   12732:	00060783          	lb	a5,0(a2)
   12736:	0605                	add	a2,a2,1
   12738:	00f527b3          	slt	a5,a0,a5
   1273c:	16fd                	add	a3,a3,-1
   1273e:	95be                	add	a1,a1,a5
   12740:	faed                	bnez	a3,12732 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x72>
   12742:	00385613          	srl	a2,a6,0x3

0000000000012746 <.LBB276_25>:
   12746:	00004517          	auipc	a0,0x4
   1274a:	b3250513          	add	a0,a0,-1230 # 16278 <.LCPI276_0>
   1274e:	00053e83          	ld	t4,0(a0)

0000000000012752 <.LBB276_26>:
   12752:	00004517          	auipc	a0,0x4
   12756:	b2e50513          	add	a0,a0,-1234 # 16280 <.LCPI276_1>
   1275a:	00053883          	ld	a7,0(a0)
   1275e:	10001537          	lui	a0,0x10001
   12762:	0512                	sll	a0,a0,0x4
   12764:	0505                	add	a0,a0,1 # 10001001 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xffb8c91>
   12766:	0542                	sll	a0,a0,0x10
   12768:	00150813          	add	a6,a0,1
   1276c:	00e58533          	add	a0,a1,a4
   12770:	a025                	j	12798 <.LBB276_26+0x46>
   12772:	00339593          	sll	a1,t2,0x3
   12776:	92ae                	add	t0,t0,a1
   12778:	40730633          	sub	a2,t1,t2
   1277c:	0033f393          	and	t2,t2,3
   12780:	0117f5b3          	and	a1,a5,a7
   12784:	83a1                	srl	a5,a5,0x8
   12786:	0117f7b3          	and	a5,a5,a7
   1278a:	95be                	add	a1,a1,a5
   1278c:	030585b3          	mul	a1,a1,a6
   12790:	91c1                	srl	a1,a1,0x30
   12792:	952e                	add	a0,a0,a1
   12794:	06039f63          	bnez	t2,12812 <.LBB276_26+0xc0>
   12798:	d225                	beqz	a2,126f8 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x38>
   1279a:	8332                	mv	t1,a2
   1279c:	0c000593          	li	a1,192
   127a0:	83b2                	mv	t2,a2
   127a2:	00b66463          	bltu	a2,a1,127aa <.LBB276_26+0x58>
   127a6:	0c000393          	li	t2,192
   127aa:	0fc3f593          	and	a1,t2,252
   127ae:	058e                	sll	a1,a1,0x3
   127b0:	00b28e33          	add	t3,t0,a1
   127b4:	4781                	li	a5,0
   127b6:	ddd5                	beqz	a1,12772 <.LBB276_26+0x20>
   127b8:	8596                	mv	a1,t0
   127ba:	6190                	ld	a2,0(a1)
   127bc:	fff64713          	not	a4,a2
   127c0:	831d                	srl	a4,a4,0x7
   127c2:	8219                	srl	a2,a2,0x6
   127c4:	6594                	ld	a3,8(a1)
   127c6:	8e59                	or	a2,a2,a4
   127c8:	01d67633          	and	a2,a2,t4
   127cc:	963e                	add	a2,a2,a5
   127ce:	fff6c713          	not	a4,a3
   127d2:	831d                	srl	a4,a4,0x7
   127d4:	8299                	srl	a3,a3,0x6
   127d6:	699c                	ld	a5,16(a1)
   127d8:	8ed9                	or	a3,a3,a4
   127da:	01d6f6b3          	and	a3,a3,t4
   127de:	9636                	add	a2,a2,a3
   127e0:	fff7c693          	not	a3,a5
   127e4:	829d                	srl	a3,a3,0x7
   127e6:	0067d713          	srl	a4,a5,0x6
   127ea:	6d9c                	ld	a5,24(a1)
   127ec:	8ed9                	or	a3,a3,a4
   127ee:	01d6f6b3          	and	a3,a3,t4
   127f2:	9636                	add	a2,a2,a3
   127f4:	fff7c693          	not	a3,a5
   127f8:	829d                	srl	a3,a3,0x7
   127fa:	0067d713          	srl	a4,a5,0x6
   127fe:	8ed9                	or	a3,a3,a4
   12800:	01d6f6b3          	and	a3,a3,t4
   12804:	02058593          	add	a1,a1,32
   12808:	00c687b3          	add	a5,a3,a2
   1280c:	fabe17e3          	bne	t3,a1,127ba <.LBB276_26+0x68>
   12810:	b78d                	j	12772 <.LBB276_26+0x20>
   12812:	0c000593          	li	a1,192
   12816:	00b36463          	bltu	t1,a1,1281e <.LBB276_26+0xcc>
   1281a:	0c000313          	li	t1,192
   1281e:	4581                	li	a1,0
   12820:	00337613          	and	a2,t1,3
   12824:	060e                	sll	a2,a2,0x3
   12826:	000e3683          	ld	a3,0(t3)
   1282a:	0e21                	add	t3,t3,8
   1282c:	fff6c713          	not	a4,a3
   12830:	831d                	srl	a4,a4,0x7
   12832:	8299                	srl	a3,a3,0x6
   12834:	8ed9                	or	a3,a3,a4
   12836:	01d6f6b3          	and	a3,a3,t4
   1283a:	1661                	add	a2,a2,-8
   1283c:	95b6                	add	a1,a1,a3
   1283e:	f665                	bnez	a2,12826 <.LBB276_26+0xd4>
   12840:	0115f633          	and	a2,a1,a7
   12844:	81a1                	srl	a1,a1,0x8
   12846:	0115f5b3          	and	a1,a1,a7
   1284a:	95b2                	add	a1,a1,a2
   1284c:	030585b3          	mul	a1,a1,a6
   12850:	91c1                	srl	a1,a1,0x30
   12852:	952e                	add	a0,a0,a1
   12854:	8082                	ret

0000000000012856 <_ZN4core3str16slice_error_fail17h0f23970489177861E>:
   12856:	7179                	add	sp,sp,-48
   12858:	f406                	sd	ra,40(sp)
   1285a:	e42a                	sd	a0,8(sp)
   1285c:	e82e                	sd	a1,16(sp)
   1285e:	ec32                	sd	a2,24(sp)
   12860:	f036                	sd	a3,32(sp)
   12862:	0028                	add	a0,sp,8
   12864:	fffff097          	auipc	ra,0xfffff
   12868:	922080e7          	jalr	-1758(ra) # 11186 <_ZN4core10intrinsics17const_eval_select17h4d2f7b41c60bf971E>
	...

000000000001286e <_ZN4core3str19slice_error_fail_rt17hcb246852ed3ab8e1E>:
   1286e:	7115                	add	sp,sp,-224
   12870:	ed86                	sd	ra,216(sp)
   12872:	e432                	sd	a2,8(sp)
   12874:	10100713          	li	a4,257
   12878:	e836                	sd	a3,16(sp)
   1287a:	04e5eb63          	bltu	a1,a4,128d0 <.LBB293_48+0x12>
   1287e:	10050783          	lb	a5,256(a0)
   12882:	fbf00813          	li	a6,-65
   12886:	470d                	li	a4,3
   12888:	00f84d63          	blt	a6,a5,128a2 <_ZN4core3str19slice_error_fail_rt17hcb246852ed3ab8e1E+0x34>
   1288c:	0ff50783          	lb	a5,255(a0)
   12890:	4709                	li	a4,2
   12892:	00f84863          	blt	a6,a5,128a2 <_ZN4core3str19slice_error_fail_rt17hcb246852ed3ab8e1E+0x34>
   12896:	0fe50703          	lb	a4,254(a0)
   1289a:	fbf00793          	li	a5,-65
   1289e:	00e7a733          	slt	a4,a5,a4
   128a2:	0fd70713          	add	a4,a4,253
   128a6:	02b77363          	bgeu	a4,a1,128cc <.LBB293_48+0xe>
   128aa:	00e507b3          	add	a5,a0,a4
   128ae:	00078803          	lb	a6,0(a5)
   128b2:	fbf00793          	li	a5,-65
   128b6:	0f07d163          	bge	a5,a6,12998 <.LBB293_57+0x14>
   128ba:	ec2a                	sd	a0,24(sp)
   128bc:	f03a                	sd	a4,32(sp)

00000000000128be <.LBB293_48>:
   128be:	00002717          	auipc	a4,0x2
   128c2:	5c570713          	add	a4,a4,1477 # 14e83 <.Lanon.442aba94db1f841cd37d39ada1516238.351>
   128c6:	f43a                	sd	a4,40(sp)
   128c8:	4815                	li	a6,5
   128ca:	a819                	j	128e0 <.LBB293_49+0xa>
   128cc:	0cb71663          	bne	a4,a1,12998 <.LBB293_57+0x14>
   128d0:	ec2a                	sd	a0,24(sp)
   128d2:	f02e                	sd	a1,32(sp)
   128d4:	4801                	li	a6,0

00000000000128d6 <.LBB293_49>:
   128d6:	00002797          	auipc	a5,0x2
   128da:	00278793          	add	a5,a5,2 # 148d8 <.Lanon.442aba94db1f841cd37d39ada1516238.83>
   128de:	f43e                	sd	a5,40(sp)
   128e0:	00c5b7b3          	sltu	a5,a1,a2
   128e4:	00d5b733          	sltu	a4,a1,a3
   128e8:	8f5d                	or	a4,a4,a5
   128ea:	f842                	sd	a6,48(sp)
   128ec:	cb39                	beqz	a4,12942 <.LBB293_53+0x14>
   128ee:	00c5e363          	bltu	a1,a2,128f4 <.LBB293_49+0x1e>
   128f2:	8636                	mv	a2,a3
   128f4:	e4b2                	sd	a2,72(sp)
   128f6:	00a8                	add	a0,sp,72
   128f8:	e52a                	sd	a0,136(sp)

00000000000128fa <.LBB293_50>:
   128fa:	00001517          	auipc	a0,0x1
   128fe:	9a850513          	add	a0,a0,-1624 # 132a2 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h68be92e0167cbc01E>
   12902:	e92a                	sd	a0,144(sp)
   12904:	0828                	add	a0,sp,24
   12906:	ed2a                	sd	a0,152(sp)

0000000000012908 <.LBB293_51>:
   12908:	00001517          	auipc	a0,0x1
   1290c:	b4050513          	add	a0,a0,-1216 # 13448 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdd6e30ce7f47bbb8E>
   12910:	f12a                	sd	a0,160(sp)
   12912:	102c                	add	a1,sp,40
   12914:	f52e                	sd	a1,168(sp)
   12916:	f92a                	sd	a0,176(sp)

0000000000012918 <.LBB293_52>:
   12918:	00002517          	auipc	a0,0x2
   1291c:	59850513          	add	a0,a0,1432 # 14eb0 <.Lanon.442aba94db1f841cd37d39ada1516238.354>
   12920:	ecaa                	sd	a0,88(sp)
   12922:	450d                	li	a0,3
   12924:	f0aa                	sd	a0,96(sp)
   12926:	f482                	sd	zero,104(sp)
   12928:	012c                	add	a1,sp,136
   1292a:	fcae                	sd	a1,120(sp)
   1292c:	e12a                	sd	a0,128(sp)

000000000001292e <.LBB293_53>:
   1292e:	00002597          	auipc	a1,0x2
   12932:	5b258593          	add	a1,a1,1458 # 14ee0 <.Lanon.442aba94db1f841cd37d39ada1516238.355>
   12936:	08a8                	add	a0,sp,88
   12938:	fffff097          	auipc	ra,0xfffff
   1293c:	9a0080e7          	jalr	-1632(ra) # 112d8 <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
   12940:	0000                	unimp
   12942:	06c6f263          	bgeu	a3,a2,129a6 <.LBB293_57+0x22>
   12946:	0028                	add	a0,sp,8
   12948:	e52a                	sd	a0,136(sp)

000000000001294a <.LBB293_54>:
   1294a:	00001517          	auipc	a0,0x1
   1294e:	95850513          	add	a0,a0,-1704 # 132a2 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h68be92e0167cbc01E>
   12952:	e92a                	sd	a0,144(sp)
   12954:	080c                	add	a1,sp,16
   12956:	ed2e                	sd	a1,152(sp)
   12958:	f12a                	sd	a0,160(sp)
   1295a:	0828                	add	a0,sp,24
   1295c:	f52a                	sd	a0,168(sp)

000000000001295e <.LBB293_55>:
   1295e:	00001517          	auipc	a0,0x1
   12962:	aea50513          	add	a0,a0,-1302 # 13448 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdd6e30ce7f47bbb8E>
   12966:	f92a                	sd	a0,176(sp)
   12968:	102c                	add	a1,sp,40
   1296a:	fd2e                	sd	a1,184(sp)
   1296c:	e1aa                	sd	a0,192(sp)

000000000001296e <.LBB293_56>:
   1296e:	00002517          	auipc	a0,0x2
   12972:	59a50513          	add	a0,a0,1434 # 14f08 <.Lanon.442aba94db1f841cd37d39ada1516238.359>
   12976:	ecaa                	sd	a0,88(sp)
   12978:	4511                	li	a0,4
   1297a:	f0aa                	sd	a0,96(sp)
   1297c:	f482                	sd	zero,104(sp)
   1297e:	012c                	add	a1,sp,136
   12980:	fcae                	sd	a1,120(sp)
   12982:	e12a                	sd	a0,128(sp)

0000000000012984 <.LBB293_57>:
   12984:	00002597          	auipc	a1,0x2
   12988:	5c458593          	add	a1,a1,1476 # 14f48 <.Lanon.442aba94db1f841cd37d39ada1516238.360>
   1298c:	08a8                	add	a0,sp,88
   1298e:	fffff097          	auipc	ra,0xfffff
   12992:	94a080e7          	jalr	-1718(ra) # 112d8 <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
   12996:	0000                	unimp
   12998:	4601                	li	a2,0
   1299a:	86ba                	mv	a3,a4
   1299c:	00000097          	auipc	ra,0x0
   129a0:	eba080e7          	jalr	-326(ra) # 12856 <_ZN4core3str16slice_error_fail17h0f23970489177861E>
   129a4:	0000                	unimp
   129a6:	e61d                	bnez	a2,129d4 <.LBB293_57+0x50>
   129a8:	8636                	mv	a2,a3
   129aa:	fc32                	sd	a2,56(sp)
   129ac:	86ae                	mv	a3,a1
   129ae:	04b67e63          	bgeu	a2,a1,12a0a <.LBB293_57+0x86>
   129b2:	ffd60713          	add	a4,a2,-3
   129b6:	4801                	li	a6,0
   129b8:	00e66363          	bltu	a2,a4,129be <.LBB293_57+0x3a>
   129bc:	883a                	mv	a6,a4
   129be:	00160713          	add	a4,a2,1
   129c2:	03077463          	bgeu	a4,a6,129ea <.LBB293_57+0x66>
   129c6:	8542                	mv	a0,a6
   129c8:	85ba                	mv	a1,a4
   129ca:	00000097          	auipc	ra,0x0
   129ce:	a8e080e7          	jalr	-1394(ra) # 12458 <_ZN4core5slice5index22slice_index_order_fail17h5b8db1271a95aea8E>
   129d2:	0000                	unimp
   129d4:	04b67d63          	bgeu	a2,a1,12a2e <.LBB293_57+0xaa>
   129d8:	00c50733          	add	a4,a0,a2
   129dc:	00070703          	lb	a4,0(a4)
   129e0:	fc000793          	li	a5,-64
   129e4:	fcf752e3          	bge	a4,a5,129a8 <.LBB293_57+0x24>
   129e8:	b7c9                	j	129aa <.LBB293_57+0x26>
   129ea:	010507b3          	add	a5,a0,a6
   129ee:	972a                	add	a4,a4,a0
   129f0:	8f1d                	sub	a4,a4,a5
   129f2:	962a                	add	a2,a2,a0
   129f4:	fc000793          	li	a5,-64
   129f8:	c719                	beqz	a4,12a06 <.LBB293_57+0x82>
   129fa:	00060683          	lb	a3,0(a2)
   129fe:	177d                	add	a4,a4,-1
   12a00:	167d                	add	a2,a2,-1
   12a02:	fef6cbe3          	blt	a3,a5,129f8 <.LBB293_57+0x74>
   12a06:	010706b3          	add	a3,a4,a6
   12a0a:	c69d                	beqz	a3,12a38 <.LBB293_57+0xb4>
   12a0c:	02b6f463          	bgeu	a3,a1,12a34 <.LBB293_57+0xb0>
   12a10:	00d50633          	add	a2,a0,a3
   12a14:	00060603          	lb	a2,0(a2)
   12a18:	fbf00713          	li	a4,-65
   12a1c:	00c74e63          	blt	a4,a2,12a38 <.LBB293_57+0xb4>
   12a20:	8636                	mv	a2,a3
   12a22:	86ae                	mv	a3,a1
   12a24:	00000097          	auipc	ra,0x0
   12a28:	e32080e7          	jalr	-462(ra) # 12856 <_ZN4core3str16slice_error_fail17h0f23970489177861E>
   12a2c:	0000                	unimp
   12a2e:	f6b60de3          	beq	a2,a1,129a8 <.LBB293_57+0x24>
   12a32:	bfa5                	j	129aa <.LBB293_57+0x26>
   12a34:	feb696e3          	bne	a3,a1,12a20 <.LBB293_57+0x9c>
   12a38:	02b69163          	bne	a3,a1,12a5a <.LBB293_59+0x16>

0000000000012a3c <.LBB293_58>:
   12a3c:	00002517          	auipc	a0,0x2
   12a40:	e9c50513          	add	a0,a0,-356 # 148d8 <.Lanon.442aba94db1f841cd37d39ada1516238.83>

0000000000012a44 <.LBB293_59>:
   12a44:	00002617          	auipc	a2,0x2
   12a48:	51c60613          	add	a2,a2,1308 # 14f60 <.Lanon.442aba94db1f841cd37d39ada1516238.361>
   12a4c:	02b00593          	li	a1,43
   12a50:	ffffe097          	auipc	ra,0xffffe
   12a54:	7d4080e7          	jalr	2004(ra) # 11224 <_ZN4core9panicking5panic17h92f54f473578363dE>
   12a58:	0000                	unimp
   12a5a:	9536                	add	a0,a0,a3
   12a5c:	00050603          	lb	a2,0(a0)
   12a60:	0ff67593          	zext.b	a1,a2
   12a64:	00064563          	bltz	a2,12a6e <.LBB293_59+0x2a>
   12a68:	c2ae                	sw	a1,68(sp)
   12a6a:	4585                	li	a1,1
   12a6c:	a89d                	j	12ae2 <.LBB293_59+0x9e>
   12a6e:	00154703          	lbu	a4,1(a0)
   12a72:	01f5f613          	and	a2,a1,31
   12a76:	0df00793          	li	a5,223
   12a7a:	03f77713          	and	a4,a4,63
   12a7e:	02b7fc63          	bgeu	a5,a1,12ab6 <.LBB293_59+0x72>
   12a82:	00254783          	lbu	a5,2(a0)
   12a86:	071a                	sll	a4,a4,0x6
   12a88:	03f7f793          	and	a5,a5,63
   12a8c:	0f000813          	li	a6,240
   12a90:	8f5d                	or	a4,a4,a5
   12a92:	0305e663          	bltu	a1,a6,12abe <.LBB293_59+0x7a>
   12a96:	00354503          	lbu	a0,3(a0)
   12a9a:	03d61593          	sll	a1,a2,0x3d
   12a9e:	91ad                	srl	a1,a1,0x2b
   12aa0:	00671613          	sll	a2,a4,0x6
   12aa4:	03f57513          	and	a0,a0,63
   12aa8:	8d51                	or	a0,a0,a2
   12aaa:	8d4d                	or	a0,a0,a1
   12aac:	001105b7          	lui	a1,0x110
   12ab0:	f8b506e3          	beq	a0,a1,12a3c <.LBB293_58>
   12ab4:	a801                	j	12ac4 <.LBB293_59+0x80>
   12ab6:	00661513          	sll	a0,a2,0x6
   12aba:	8d59                	or	a0,a0,a4
   12abc:	a021                	j	12ac4 <.LBB293_59+0x80>
   12abe:	00c61513          	sll	a0,a2,0xc
   12ac2:	8d59                	or	a0,a0,a4
   12ac4:	c2aa                	sw	a0,68(sp)
   12ac6:	08000613          	li	a2,128
   12aca:	4585                	li	a1,1
   12acc:	00c56b63          	bltu	a0,a2,12ae2 <.LBB293_59+0x9e>
   12ad0:	00b55613          	srl	a2,a0,0xb
   12ad4:	4589                	li	a1,2
   12ad6:	c611                	beqz	a2,12ae2 <.LBB293_59+0x9e>
   12ad8:	8141                	srl	a0,a0,0x10
   12ada:	00153513          	seqz	a0,a0
   12ade:	4591                	li	a1,4
   12ae0:	8d89                	sub	a1,a1,a0
   12ae2:	00d58533          	add	a0,a1,a3
   12ae6:	e4b6                	sd	a3,72(sp)
   12ae8:	e8aa                	sd	a0,80(sp)
   12aea:	1828                	add	a0,sp,56
   12aec:	e52a                	sd	a0,136(sp)

0000000000012aee <.LBB293_60>:
   12aee:	00000517          	auipc	a0,0x0
   12af2:	7b450513          	add	a0,a0,1972 # 132a2 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h68be92e0167cbc01E>
   12af6:	e92a                	sd	a0,144(sp)
   12af8:	00c8                	add	a0,sp,68
   12afa:	ed2a                	sd	a0,152(sp)

0000000000012afc <.LBB293_61>:
   12afc:	fffff517          	auipc	a0,0xfffff
   12b00:	6e250513          	add	a0,a0,1762 # 121de <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hdf624fa96493adceE>
   12b04:	f12a                	sd	a0,160(sp)
   12b06:	00a8                	add	a0,sp,72
   12b08:	f52a                	sd	a0,168(sp)

0000000000012b0a <.LBB293_62>:
   12b0a:	ffffe517          	auipc	a0,0xffffe
   12b0e:	69450513          	add	a0,a0,1684 # 1119e <_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h59f5c39c772cdb32E>
   12b12:	f92a                	sd	a0,176(sp)
   12b14:	0828                	add	a0,sp,24
   12b16:	fd2a                	sd	a0,184(sp)

0000000000012b18 <.LBB293_63>:
   12b18:	00001517          	auipc	a0,0x1
   12b1c:	93050513          	add	a0,a0,-1744 # 13448 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdd6e30ce7f47bbb8E>
   12b20:	e1aa                	sd	a0,192(sp)
   12b22:	102c                	add	a1,sp,40
   12b24:	e5ae                	sd	a1,200(sp)
   12b26:	e9aa                	sd	a0,208(sp)

0000000000012b28 <.LBB293_64>:
   12b28:	00002517          	auipc	a0,0x2
   12b2c:	48050513          	add	a0,a0,1152 # 14fa8 <.Lanon.442aba94db1f841cd37d39ada1516238.365>
   12b30:	ecaa                	sd	a0,88(sp)
   12b32:	4515                	li	a0,5
   12b34:	f0aa                	sd	a0,96(sp)
   12b36:	f482                	sd	zero,104(sp)
   12b38:	012c                	add	a1,sp,136
   12b3a:	fcae                	sd	a1,120(sp)
   12b3c:	e12a                	sd	a0,128(sp)

0000000000012b3e <.LBB293_65>:
   12b3e:	00002597          	auipc	a1,0x2
   12b42:	4ba58593          	add	a1,a1,1210 # 14ff8 <.Lanon.442aba94db1f841cd37d39ada1516238.366>
   12b46:	08a8                	add	a0,sp,88
   12b48:	ffffe097          	auipc	ra,0xffffe
   12b4c:	790080e7          	jalr	1936(ra) # 112d8 <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

0000000000012b52 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E>:
   12b52:	1141                	add	sp,sp,-16
   12b54:	e406                	sd	ra,8(sp)
   12b56:	03051293          	sll	t0,a0,0x30
   12b5a:	ce29                	beqz	a2,12bb4 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x62>
   12b5c:	88aa                	mv	a7,a0
   12b5e:	4501                	li	a0,0
   12b60:	0382d313          	srl	t1,t0,0x38
   12b64:	0606                	sll	a2,a2,0x1
   12b66:	00c583b3          	add	t2,a1,a2
   12b6a:	0ff8fe13          	zext.b	t3,a7
   12b6e:	a811                	j	12b82 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x30>
   12b70:	01d33533          	sltu	a0,t1,t4
   12b74:	00b3c633          	xor	a2,t2,a1
   12b78:	00163613          	seqz	a2,a2
   12b7c:	8e49                	or	a2,a2,a0
   12b7e:	8546                	mv	a0,a7
   12b80:	ea15                	bnez	a2,12bb4 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x62>
   12b82:	0005ce83          	lbu	t4,0(a1)
   12b86:	0015c603          	lbu	a2,1(a1)
   12b8a:	0589                	add	a1,a1,2
   12b8c:	00c508b3          	add	a7,a0,a2
   12b90:	fe6e90e3          	bne	t4,t1,12b70 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x1e>
   12b94:	0aa8e163          	bltu	a7,a0,12c36 <.LBB312_24+0x16>
   12b98:	0b176563          	bltu	a4,a7,12c42 <.LBB312_24+0x22>
   12b9c:	9536                	add	a0,a0,a3
   12b9e:	ca01                	beqz	a2,12bae <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x5c>
   12ba0:	00054e83          	lbu	t4,0(a0)
   12ba4:	0505                	add	a0,a0,1
   12ba6:	167d                	add	a2,a2,-1
   12ba8:	ffce9be3          	bne	t4,t3,12b9e <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x4c>
   12bac:	a8a1                	j	12c04 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0xb2>
   12bae:	8546                	mv	a0,a7
   12bb0:	fcb399e3          	bne	t2,a1,12b82 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x30>
   12bb4:	04080d63          	beqz	a6,12c0e <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0xbc>
   12bb8:	010785b3          	add	a1,a5,a6
   12bbc:	0302d613          	srl	a2,t0,0x30
   12bc0:	4505                	li	a0,1
   12bc2:	00078683          	lb	a3,0(a5)
   12bc6:	00178713          	add	a4,a5,1
   12bca:	0006c963          	bltz	a3,12bdc <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x8a>
   12bce:	0ff6f693          	zext.b	a3,a3
   12bd2:	87ba                	mv	a5,a4
   12bd4:	9e15                	subw	a2,a2,a3
   12bd6:	00065f63          	bgez	a2,12bf4 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0xa2>
   12bda:	a00d                	j	12bfc <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0xaa>
   12bdc:	02b70e63          	beq	a4,a1,12c18 <.LBB312_23>
   12be0:	0017c703          	lbu	a4,1(a5)
   12be4:	0789                	add	a5,a5,2
   12be6:	07f6f693          	and	a3,a3,127
   12bea:	06a2                	sll	a3,a3,0x8
   12bec:	8ed9                	or	a3,a3,a4
   12bee:	9e15                	subw	a2,a2,a3
   12bf0:	00064663          	bltz	a2,12bfc <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0xaa>
   12bf4:	00154513          	xor	a0,a0,1
   12bf8:	fcb795e3          	bne	a5,a1,12bc2 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x70>
   12bfc:	8905                	and	a0,a0,1
   12bfe:	60a2                	ld	ra,8(sp)
   12c00:	0141                	add	sp,sp,16
   12c02:	8082                	ret
   12c04:	4501                	li	a0,0
   12c06:	8905                	and	a0,a0,1
   12c08:	60a2                	ld	ra,8(sp)
   12c0a:	0141                	add	sp,sp,16
   12c0c:	8082                	ret
   12c0e:	4505                	li	a0,1
   12c10:	8905                	and	a0,a0,1
   12c12:	60a2                	ld	ra,8(sp)
   12c14:	0141                	add	sp,sp,16
   12c16:	8082                	ret

0000000000012c18 <.LBB312_23>:
   12c18:	00002517          	auipc	a0,0x2
   12c1c:	cc050513          	add	a0,a0,-832 # 148d8 <.Lanon.442aba94db1f841cd37d39ada1516238.83>

0000000000012c20 <.LBB312_24>:
   12c20:	00002617          	auipc	a2,0x2
   12c24:	41860613          	add	a2,a2,1048 # 15038 <.Lanon.442aba94db1f841cd37d39ada1516238.388>
   12c28:	02b00593          	li	a1,43
   12c2c:	ffffe097          	auipc	ra,0xffffe
   12c30:	5f8080e7          	jalr	1528(ra) # 11224 <_ZN4core9panicking5panic17h92f54f473578363dE>
   12c34:	0000                	unimp
   12c36:	85c6                	mv	a1,a7
   12c38:	00000097          	auipc	ra,0x0
   12c3c:	820080e7          	jalr	-2016(ra) # 12458 <_ZN4core5slice5index22slice_index_order_fail17h5b8db1271a95aea8E>
   12c40:	0000                	unimp
   12c42:	8546                	mv	a0,a7
   12c44:	85ba                	mv	a1,a4
   12c46:	fffff097          	auipc	ra,0xfffff
   12c4a:	7be080e7          	jalr	1982(ra) # 12404 <_ZN4core5slice5index24slice_end_index_len_fail17h5d1e1d044f43082eE>
	...

0000000000012c50 <_ZN4core7unicode9printable12is_printable17h190ab7f3e4a90aabE>:
   12c50:	0005061b          	sext.w	a2,a0
   12c54:	02000593          	li	a1,32
   12c58:	00b67463          	bgeu	a2,a1,12c60 <_ZN4core7unicode9printable12is_printable17h190ab7f3e4a90aabE+0x10>
   12c5c:	4501                	li	a0,0
   12c5e:	8082                	ret
   12c60:	07f00693          	li	a3,127
   12c64:	4585                	li	a1,1
   12c66:	00d67463          	bgeu	a2,a3,12c6e <_ZN4core7unicode9printable12is_printable17h190ab7f3e4a90aabE+0x1e>
   12c6a:	852e                	mv	a0,a1
   12c6c:	8082                	ret
   12c6e:	0105559b          	srlw	a1,a0,0x10
   12c72:	e59d                	bnez	a1,12ca0 <.LBB313_12+0x1c>

0000000000012c74 <.LBB313_10>:
   12c74:	00002597          	auipc	a1,0x2
   12c78:	3dc58593          	add	a1,a1,988 # 15050 <.Lanon.442aba94db1f841cd37d39ada1516238.389>

0000000000012c7c <.LBB313_11>:
   12c7c:	00002697          	auipc	a3,0x2
   12c80:	42468693          	add	a3,a3,1060 # 150a0 <.Lanon.442aba94db1f841cd37d39ada1516238.390>

0000000000012c84 <.LBB313_12>:
   12c84:	00002797          	auipc	a5,0x2
   12c88:	53c78793          	add	a5,a5,1340 # 151c0 <.Lanon.442aba94db1f841cd37d39ada1516238.391>
   12c8c:	02800613          	li	a2,40
   12c90:	12000713          	li	a4,288
   12c94:	12f00813          	li	a6,303
   12c98:	00000317          	auipc	t1,0x0
   12c9c:	eba30067          	jr	-326(t1) # 12b52 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E>
   12ca0:	0115559b          	srlw	a1,a0,0x11
   12ca4:	e59d                	bnez	a1,12cd2 <.LBB313_15+0x1c>

0000000000012ca6 <.LBB313_13>:
   12ca6:	00002597          	auipc	a1,0x2
   12caa:	64958593          	add	a1,a1,1609 # 152ef <.Lanon.442aba94db1f841cd37d39ada1516238.392>

0000000000012cae <.LBB313_14>:
   12cae:	00002697          	auipc	a3,0x2
   12cb2:	69568693          	add	a3,a3,1685 # 15343 <.Lanon.442aba94db1f841cd37d39ada1516238.393>

0000000000012cb6 <.LBB313_15>:
   12cb6:	00002797          	auipc	a5,0x2
   12cba:	74d78793          	add	a5,a5,1869 # 15403 <.Lanon.442aba94db1f841cd37d39ada1516238.394>
   12cbe:	02a00613          	li	a2,42
   12cc2:	0c000713          	li	a4,192
   12cc6:	1b600813          	li	a6,438
   12cca:	00000317          	auipc	t1,0x0
   12cce:	e8830067          	jr	-376(t1) # 12b52 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E>
   12cd2:	0055559b          	srlw	a1,a0,0x5
   12cd6:	6605                	lui	a2,0x1
   12cd8:	5376069b          	addw	a3,a2,1335 # 1537 <_start-0xeac9>
   12cdc:	8db5                	xor	a1,a1,a3
   12cde:	0015b593          	seqz	a1,a1
   12ce2:	fffd56b7          	lui	a3,0xfffd5
   12ce6:	8c76869b          	addw	a3,a3,-1849 # fffffffffffd48c7 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xfffffffffff8c557>
   12cea:	9ea9                	addw	a3,a3,a0
   12cec:	0076b693          	sltiu	a3,a3,7
   12cf0:	8dd5                	or	a1,a1,a3
   12cf2:	0015569b          	srlw	a3,a0,0x1
   12cf6:	6759                	lui	a4,0x16
   12cf8:	c0f7071b          	addw	a4,a4,-1009 # 15c0f <.Lanon.86a3613c128665d32fc75176e6ae67c2.14+0x177>
   12cfc:	8eb9                	xor	a3,a3,a4
   12cfe:	0016b693          	seqz	a3,a3
   12d02:	8dd5                	or	a1,a1,a3
   12d04:	fffd36b7          	lui	a3,0xfffd3
   12d08:	15e6869b          	addw	a3,a3,350 # fffffffffffd315e <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xfffffffffff8adee>
   12d0c:	9ea9                	addw	a3,a3,a0
   12d0e:	00e6b693          	sltiu	a3,a3,14
   12d12:	8dd5                	or	a1,a1,a3
   12d14:	fffd16b7          	lui	a3,0xfffd1
   12d18:	41f6869b          	addw	a3,a3,1055 # fffffffffffd141f <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xfffffffffff890af>
   12d1c:	9ea9                	addw	a3,a3,a0
   12d1e:	c1f6061b          	addw	a2,a2,-993
   12d22:	00c6b633          	sltu	a2,a3,a2
   12d26:	8dd1                	or	a1,a1,a2
   12d28:	fffd0637          	lui	a2,0xfffd0
   12d2c:	5e26069b          	addw	a3,a2,1506 # fffffffffffd05e2 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xfffffffffff88272>
   12d30:	9ea9                	addw	a3,a3,a0
   12d32:	5e26b693          	sltiu	a3,a3,1506
   12d36:	8dd5                	or	a1,a1,a3
   12d38:	fffcf6b7          	lui	a3,0xfffcf
   12d3c:	cb56869b          	addw	a3,a3,-843 # fffffffffffcecb5 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xfffffffffff86945>
   12d40:	9ea9                	addw	a3,a3,a0
   12d42:	000af737          	lui	a4,0xaf
   12d46:	db57071b          	addw	a4,a4,-587 # aedb5 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0x66a45>
   12d4a:	00e6b6b3          	sltu	a3,a3,a4
   12d4e:	8dd5                	or	a1,a1,a3
   12d50:	0015f693          	and	a3,a1,1
   12d54:	4581                	li	a1,0
   12d56:	fa91                	bnez	a3,12c6a <_ZN4core7unicode9printable12is_printable17h190ab7f3e4a90aabE+0x1a>
   12d58:	ffef05b7          	lui	a1,0xffef0
   12d5c:	9d2d                	addw	a0,a0,a1
   12d5e:	1f06059b          	addw	a1,a2,496
   12d62:	00b535b3          	sltu	a1,a0,a1
   12d66:	852e                	mv	a0,a1
   12d68:	8082                	ret

0000000000012d6a <_ZN69_$LT$core..num..nonzero..NonZeroUsize$u20$as$u20$core..fmt..Debug$GT$3fmt17hcd3cf5bdc6888583E>:
   12d6a:	1141                	add	sp,sp,-16
   12d6c:	e406                	sd	ra,8(sp)
   12d6e:	6108                	ld	a0,0(a0)
   12d70:	e02a                	sd	a0,0(sp)
   12d72:	850a                	mv	a0,sp
   12d74:	00000097          	auipc	ra,0x0
   12d78:	2c2080e7          	jalr	706(ra) # 13036 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E>
   12d7c:	60a2                	ld	ra,8(sp)
   12d7e:	0141                	add	sp,sp,16
   12d80:	8082                	ret

0000000000012d82 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i64$GT$3fmt17h55cbfb91d25d94efE>:
   12d82:	7175                	add	sp,sp,-144
   12d84:	e506                	sd	ra,136(sp)
   12d86:	882e                	mv	a6,a1
   12d88:	4581                	li	a1,0
   12d8a:	6118                	ld	a4,0(a0)
   12d8c:	00810893          	add	a7,sp,8
   12d90:	42a9                	li	t0,10
   12d92:	433d                	li	t1,15
   12d94:	a819                	j	12daa <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i64$GT$3fmt17h55cbfb91d25d94efE+0x28>
   12d96:	00b886b3          	add	a3,a7,a1
   12d9a:	00455713          	srl	a4,a0,0x4
   12d9e:	9e3d                	addw	a2,a2,a5
   12da0:	06c68fa3          	sb	a2,127(a3)
   12da4:	15fd                	add	a1,a1,-1 # ffffffffffeeffff <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xffffffffffea7c8f>
   12da6:	00a37c63          	bgeu	t1,a0,12dbe <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i64$GT$3fmt17h55cbfb91d25d94efE+0x3c>
   12daa:	853a                	mv	a0,a4
   12dac:	00f77793          	and	a5,a4,15
   12db0:	03000613          	li	a2,48
   12db4:	fe57e1e3          	bltu	a5,t0,12d96 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i64$GT$3fmt17h55cbfb91d25d94efE+0x14>
   12db8:	05700613          	li	a2,87
   12dbc:	bfe9                	j	12d96 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i64$GT$3fmt17h55cbfb91d25d94efE+0x14>
   12dbe:	08058513          	add	a0,a1,128
   12dc2:	08100613          	li	a2,129
   12dc6:	02c57663          	bgeu	a0,a2,12df2 <.LBB528_7+0x1c>
   12dca:	40b007b3          	neg	a5,a1
   12dce:	0028                	add	a0,sp,8
   12dd0:	952e                	add	a0,a0,a1
   12dd2:	08050713          	add	a4,a0,128

0000000000012dd6 <.LBB528_7>:
   12dd6:	00002617          	auipc	a2,0x2
   12dda:	d6a60613          	add	a2,a2,-662 # 14b40 <.Lanon.442aba94db1f841cd37d39ada1516238.257>
   12dde:	4585                	li	a1,1
   12de0:	4689                	li	a3,2
   12de2:	8542                	mv	a0,a6
   12de4:	fffff097          	auipc	ra,0xfffff
   12de8:	eba080e7          	jalr	-326(ra) # 11c9e <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E>
   12dec:	60aa                	ld	ra,136(sp)
   12dee:	6149                	add	sp,sp,144
   12df0:	8082                	ret
   12df2:	08000593          	li	a1,128
   12df6:	fffff097          	auipc	ra,0xfffff
   12dfa:	5ba080e7          	jalr	1466(ra) # 123b0 <_ZN4core5slice5index26slice_start_index_len_fail17hc36fc5bb321621f5E>
	...

0000000000012e00 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$3fmt17h1e56d5f13d948128E>:
   12e00:	7175                	add	sp,sp,-144
   12e02:	e506                	sd	ra,136(sp)
   12e04:	882e                	mv	a6,a1
   12e06:	4581                	li	a1,0
   12e08:	6118                	ld	a4,0(a0)
   12e0a:	00810893          	add	a7,sp,8
   12e0e:	42a9                	li	t0,10
   12e10:	433d                	li	t1,15
   12e12:	a819                	j	12e28 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$3fmt17h1e56d5f13d948128E+0x28>
   12e14:	00b886b3          	add	a3,a7,a1
   12e18:	00455713          	srl	a4,a0,0x4
   12e1c:	9e3d                	addw	a2,a2,a5
   12e1e:	06c68fa3          	sb	a2,127(a3)
   12e22:	15fd                	add	a1,a1,-1
   12e24:	00a37c63          	bgeu	t1,a0,12e3c <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$3fmt17h1e56d5f13d948128E+0x3c>
   12e28:	853a                	mv	a0,a4
   12e2a:	00f77793          	and	a5,a4,15
   12e2e:	03000613          	li	a2,48
   12e32:	fe57e1e3          	bltu	a5,t0,12e14 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$3fmt17h1e56d5f13d948128E+0x14>
   12e36:	03700613          	li	a2,55
   12e3a:	bfe9                	j	12e14 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$3fmt17h1e56d5f13d948128E+0x14>
   12e3c:	08058513          	add	a0,a1,128
   12e40:	08100613          	li	a2,129
   12e44:	02c57663          	bgeu	a0,a2,12e70 <.LBB529_7+0x1c>
   12e48:	40b007b3          	neg	a5,a1
   12e4c:	0028                	add	a0,sp,8
   12e4e:	952e                	add	a0,a0,a1
   12e50:	08050713          	add	a4,a0,128

0000000000012e54 <.LBB529_7>:
   12e54:	00002617          	auipc	a2,0x2
   12e58:	cec60613          	add	a2,a2,-788 # 14b40 <.Lanon.442aba94db1f841cd37d39ada1516238.257>
   12e5c:	4585                	li	a1,1
   12e5e:	4689                	li	a3,2
   12e60:	8542                	mv	a0,a6
   12e62:	fffff097          	auipc	ra,0xfffff
   12e66:	e3c080e7          	jalr	-452(ra) # 11c9e <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E>
   12e6a:	60aa                	ld	ra,136(sp)
   12e6c:	6149                	add	sp,sp,144
   12e6e:	8082                	ret
   12e70:	08000593          	li	a1,128
   12e74:	fffff097          	auipc	ra,0xfffff
   12e78:	53c080e7          	jalr	1340(ra) # 123b0 <_ZN4core5slice5index26slice_start_index_len_fail17hc36fc5bb321621f5E>
	...

0000000000012e7e <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E>:
   12e7e:	7175                	add	sp,sp,-144
   12e80:	e506                	sd	ra,136(sp)
   12e82:	882e                	mv	a6,a1
   12e84:	0305e583          	lwu	a1,48(a1)
   12e88:	0105f613          	and	a2,a1,16
   12e8c:	ee11                	bnez	a2,12ea8 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x2a>
   12e8e:	0205f593          	and	a1,a1,32
   12e92:	e9a1                	bnez	a1,12ee2 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x64>
   12e94:	00054503          	lbu	a0,0(a0)
   12e98:	4585                	li	a1,1
   12e9a:	8642                	mv	a2,a6
   12e9c:	60aa                	ld	ra,136(sp)
   12e9e:	6149                	add	sp,sp,144
   12ea0:	00000317          	auipc	t1,0x0
   12ea4:	26c30067          	jr	620(t1) # 1310c <_ZN4core3fmt3num3imp7fmt_u6417h5e0a68673bc51c93E>
   12ea8:	4581                	li	a1,0
   12eaa:	00054683          	lbu	a3,0(a0)
   12eae:	00810893          	add	a7,sp,8
   12eb2:	42a9                	li	t0,10
   12eb4:	433d                	li	t1,15
   12eb6:	a829                	j	12ed0 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x52>
   12eb8:	00b88633          	add	a2,a7,a1
   12ebc:	0ff6f713          	zext.b	a4,a3
   12ec0:	00475693          	srl	a3,a4,0x4
   12ec4:	9d3d                	addw	a0,a0,a5
   12ec6:	06a60fa3          	sb	a0,127(a2)
   12eca:	15fd                	add	a1,a1,-1
   12ecc:	04e37863          	bgeu	t1,a4,12f1c <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x9e>
   12ed0:	00f6f793          	and	a5,a3,15
   12ed4:	03000513          	li	a0,48
   12ed8:	fe57e0e3          	bltu	a5,t0,12eb8 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x3a>
   12edc:	05700513          	li	a0,87
   12ee0:	bfe1                	j	12eb8 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x3a>
   12ee2:	4581                	li	a1,0
   12ee4:	00054683          	lbu	a3,0(a0)
   12ee8:	00810893          	add	a7,sp,8
   12eec:	42a9                	li	t0,10
   12eee:	433d                	li	t1,15
   12ef0:	a829                	j	12f0a <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x8c>
   12ef2:	00b88633          	add	a2,a7,a1
   12ef6:	0ff6f713          	zext.b	a4,a3
   12efa:	00475693          	srl	a3,a4,0x4
   12efe:	9d3d                	addw	a0,a0,a5
   12f00:	06a60fa3          	sb	a0,127(a2)
   12f04:	15fd                	add	a1,a1,-1
   12f06:	00e37b63          	bgeu	t1,a4,12f1c <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x9e>
   12f0a:	00f6f793          	and	a5,a3,15
   12f0e:	03000513          	li	a0,48
   12f12:	fe57e0e3          	bltu	a5,t0,12ef2 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x74>
   12f16:	03700513          	li	a0,55
   12f1a:	bfe1                	j	12ef2 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x74>
   12f1c:	08058513          	add	a0,a1,128
   12f20:	08100613          	li	a2,129
   12f24:	02c57663          	bgeu	a0,a2,12f50 <.LBB538_14+0x1c>
   12f28:	40b007b3          	neg	a5,a1
   12f2c:	0028                	add	a0,sp,8
   12f2e:	952e                	add	a0,a0,a1
   12f30:	08050713          	add	a4,a0,128

0000000000012f34 <.LBB538_14>:
   12f34:	00002617          	auipc	a2,0x2
   12f38:	c0c60613          	add	a2,a2,-1012 # 14b40 <.Lanon.442aba94db1f841cd37d39ada1516238.257>
   12f3c:	4585                	li	a1,1
   12f3e:	4689                	li	a3,2
   12f40:	8542                	mv	a0,a6
   12f42:	fffff097          	auipc	ra,0xfffff
   12f46:	d5c080e7          	jalr	-676(ra) # 11c9e <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E>
   12f4a:	60aa                	ld	ra,136(sp)
   12f4c:	6149                	add	sp,sp,144
   12f4e:	8082                	ret
   12f50:	08000593          	li	a1,128
   12f54:	fffff097          	auipc	ra,0xfffff
   12f58:	45c080e7          	jalr	1116(ra) # 123b0 <_ZN4core5slice5index26slice_start_index_len_fail17hc36fc5bb321621f5E>
	...

0000000000012f5e <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E>:
   12f5e:	7175                	add	sp,sp,-144
   12f60:	e506                	sd	ra,136(sp)
   12f62:	882e                	mv	a6,a1
   12f64:	0305e583          	lwu	a1,48(a1)
   12f68:	0105f613          	and	a2,a1,16
   12f6c:	ee11                	bnez	a2,12f88 <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x2a>
   12f6e:	0205f593          	and	a1,a1,32
   12f72:	e5b1                	bnez	a1,12fbe <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x60>
   12f74:	00056503          	lwu	a0,0(a0)
   12f78:	4585                	li	a1,1
   12f7a:	8642                	mv	a2,a6
   12f7c:	60aa                	ld	ra,136(sp)
   12f7e:	6149                	add	sp,sp,144
   12f80:	00000317          	auipc	t1,0x0
   12f84:	18c30067          	jr	396(t1) # 1310c <_ZN4core3fmt3num3imp7fmt_u6417h5e0a68673bc51c93E>
   12f88:	4581                	li	a1,0
   12f8a:	4118                	lw	a4,0(a0)
   12f8c:	00810893          	add	a7,sp,8
   12f90:	42a9                	li	t0,10
   12f92:	433d                	li	t1,15
   12f94:	a819                	j	12faa <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x4c>
   12f96:	00b886b3          	add	a3,a7,a1
   12f9a:	0045571b          	srlw	a4,a0,0x4
   12f9e:	9e3d                	addw	a2,a2,a5
   12fa0:	06c68fa3          	sb	a2,127(a3)
   12fa4:	15fd                	add	a1,a1,-1
   12fa6:	04a37763          	bgeu	t1,a0,12ff4 <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x96>
   12faa:	853a                	mv	a0,a4
   12fac:	00f77793          	and	a5,a4,15
   12fb0:	03000613          	li	a2,48
   12fb4:	fe57e1e3          	bltu	a5,t0,12f96 <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x38>
   12fb8:	05700613          	li	a2,87
   12fbc:	bfe9                	j	12f96 <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x38>
   12fbe:	4581                	li	a1,0
   12fc0:	4118                	lw	a4,0(a0)
   12fc2:	00810893          	add	a7,sp,8
   12fc6:	42a9                	li	t0,10
   12fc8:	433d                	li	t1,15
   12fca:	a819                	j	12fe0 <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x82>
   12fcc:	00b886b3          	add	a3,a7,a1
   12fd0:	0045571b          	srlw	a4,a0,0x4
   12fd4:	9e3d                	addw	a2,a2,a5
   12fd6:	06c68fa3          	sb	a2,127(a3)
   12fda:	15fd                	add	a1,a1,-1
   12fdc:	00a37c63          	bgeu	t1,a0,12ff4 <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x96>
   12fe0:	853a                	mv	a0,a4
   12fe2:	00f77793          	and	a5,a4,15
   12fe6:	03000613          	li	a2,48
   12fea:	fe57e1e3          	bltu	a5,t0,12fcc <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x6e>
   12fee:	03700613          	li	a2,55
   12ff2:	bfe9                	j	12fcc <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x6e>
   12ff4:	08058513          	add	a0,a1,128
   12ff8:	08100613          	li	a2,129
   12ffc:	02c57663          	bgeu	a0,a2,13028 <.LBB540_14+0x1c>
   13000:	40b007b3          	neg	a5,a1
   13004:	0028                	add	a0,sp,8
   13006:	952e                	add	a0,a0,a1
   13008:	08050713          	add	a4,a0,128

000000000001300c <.LBB540_14>:
   1300c:	00002617          	auipc	a2,0x2
   13010:	b3460613          	add	a2,a2,-1228 # 14b40 <.Lanon.442aba94db1f841cd37d39ada1516238.257>
   13014:	4585                	li	a1,1
   13016:	4689                	li	a3,2
   13018:	8542                	mv	a0,a6
   1301a:	fffff097          	auipc	ra,0xfffff
   1301e:	c84080e7          	jalr	-892(ra) # 11c9e <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E>
   13022:	60aa                	ld	ra,136(sp)
   13024:	6149                	add	sp,sp,144
   13026:	8082                	ret
   13028:	08000593          	li	a1,128
   1302c:	fffff097          	auipc	ra,0xfffff
   13030:	384080e7          	jalr	900(ra) # 123b0 <_ZN4core5slice5index26slice_start_index_len_fail17hc36fc5bb321621f5E>
	...

0000000000013036 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E>:
   13036:	7175                	add	sp,sp,-144
   13038:	e506                	sd	ra,136(sp)
   1303a:	882e                	mv	a6,a1
   1303c:	0305e583          	lwu	a1,48(a1)
   13040:	0105f613          	and	a2,a1,16
   13044:	ee09                	bnez	a2,1305e <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x28>
   13046:	0205f593          	and	a1,a1,32
   1304a:	e5a9                	bnez	a1,13094 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x5e>
   1304c:	6108                	ld	a0,0(a0)
   1304e:	4585                	li	a1,1
   13050:	8642                	mv	a2,a6
   13052:	60aa                	ld	ra,136(sp)
   13054:	6149                	add	sp,sp,144
   13056:	00000317          	auipc	t1,0x0
   1305a:	0b630067          	jr	182(t1) # 1310c <_ZN4core3fmt3num3imp7fmt_u6417h5e0a68673bc51c93E>
   1305e:	4581                	li	a1,0
   13060:	6118                	ld	a4,0(a0)
   13062:	00810893          	add	a7,sp,8
   13066:	42a9                	li	t0,10
   13068:	433d                	li	t1,15
   1306a:	a819                	j	13080 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x4a>
   1306c:	00b886b3          	add	a3,a7,a1
   13070:	00455713          	srl	a4,a0,0x4
   13074:	9e3d                	addw	a2,a2,a5
   13076:	06c68fa3          	sb	a2,127(a3)
   1307a:	15fd                	add	a1,a1,-1
   1307c:	04a37763          	bgeu	t1,a0,130ca <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x94>
   13080:	853a                	mv	a0,a4
   13082:	00f77793          	and	a5,a4,15
   13086:	03000613          	li	a2,48
   1308a:	fe57e1e3          	bltu	a5,t0,1306c <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x36>
   1308e:	05700613          	li	a2,87
   13092:	bfe9                	j	1306c <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x36>
   13094:	4581                	li	a1,0
   13096:	6118                	ld	a4,0(a0)
   13098:	00810893          	add	a7,sp,8
   1309c:	42a9                	li	t0,10
   1309e:	433d                	li	t1,15
   130a0:	a819                	j	130b6 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x80>
   130a2:	00b886b3          	add	a3,a7,a1
   130a6:	00455713          	srl	a4,a0,0x4
   130aa:	9e3d                	addw	a2,a2,a5
   130ac:	06c68fa3          	sb	a2,127(a3)
   130b0:	15fd                	add	a1,a1,-1
   130b2:	00a37c63          	bgeu	t1,a0,130ca <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x94>
   130b6:	853a                	mv	a0,a4
   130b8:	00f77793          	and	a5,a4,15
   130bc:	03000613          	li	a2,48
   130c0:	fe57e1e3          	bltu	a5,t0,130a2 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x6c>
   130c4:	03700613          	li	a2,55
   130c8:	bfe9                	j	130a2 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x6c>
   130ca:	08058513          	add	a0,a1,128
   130ce:	08100613          	li	a2,129
   130d2:	02c57663          	bgeu	a0,a2,130fe <.LBB542_14+0x1c>
   130d6:	40b007b3          	neg	a5,a1
   130da:	0028                	add	a0,sp,8
   130dc:	952e                	add	a0,a0,a1
   130de:	08050713          	add	a4,a0,128

00000000000130e2 <.LBB542_14>:
   130e2:	00002617          	auipc	a2,0x2
   130e6:	a5e60613          	add	a2,a2,-1442 # 14b40 <.Lanon.442aba94db1f841cd37d39ada1516238.257>
   130ea:	4585                	li	a1,1
   130ec:	4689                	li	a3,2
   130ee:	8542                	mv	a0,a6
   130f0:	fffff097          	auipc	ra,0xfffff
   130f4:	bae080e7          	jalr	-1106(ra) # 11c9e <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E>
   130f8:	60aa                	ld	ra,136(sp)
   130fa:	6149                	add	sp,sp,144
   130fc:	8082                	ret
   130fe:	08000593          	li	a1,128
   13102:	fffff097          	auipc	ra,0xfffff
   13106:	2ae080e7          	jalr	686(ra) # 123b0 <_ZN4core5slice5index26slice_start_index_len_fail17hc36fc5bb321621f5E>
	...

000000000001310c <_ZN4core3fmt3num3imp7fmt_u6417h5e0a68673bc51c93E>:
   1310c:	7139                	add	sp,sp,-64
   1310e:	fc06                	sd	ra,56(sp)
   13110:	f822                	sd	s0,48(sp)
   13112:	f426                	sd	s1,40(sp)
   13114:	8832                	mv	a6,a2
   13116:	00455693          	srl	a3,a0,0x4
   1311a:	02700713          	li	a4,39
   1311e:	27100793          	li	a5,625

0000000000013122 <.LBB543_10>:
   13122:	00002e17          	auipc	t3,0x2
   13126:	a20e0e13          	add	t3,t3,-1504 # 14b42 <.Lanon.442aba94db1f841cd37d39ada1516238.259>
   1312a:	02f6f363          	bgeu	a3,a5,13150 <.LBB543_10+0x2e>
   1312e:	06300693          	li	a3,99
   13132:	0aa6e963          	bltu	a3,a0,131e4 <.LBB543_11+0x92>
   13136:	4629                	li	a2,10
   13138:	0ec57763          	bgeu	a0,a2,13226 <.LBB543_11+0xd4>
   1313c:	fff70693          	add	a3,a4,-1
   13140:	00110613          	add	a2,sp,1
   13144:	9636                	add	a2,a2,a3
   13146:	0305051b          	addw	a0,a0,48
   1314a:	00a60023          	sb	a0,0(a2)
   1314e:	a8dd                	j	13244 <.LBB543_11+0xf2>
   13150:	4701                	li	a4,0

0000000000013152 <.LBB543_11>:
   13152:	00003697          	auipc	a3,0x3
   13156:	19668693          	add	a3,a3,406 # 162e8 <.LCPI543_0>
   1315a:	0006b883          	ld	a7,0(a3)
   1315e:	6689                	lui	a3,0x2
   13160:	7106839b          	addw	t2,a3,1808 # 2710 <_start-0xd8f0>
   13164:	6685                	lui	a3,0x1
   13166:	47b68e9b          	addw	t4,a3,1147 # 147b <_start-0xeb85>
   1316a:	06400293          	li	t0,100
   1316e:	00110313          	add	t1,sp,1
   13172:	05f5e6b7          	lui	a3,0x5f5e
   13176:	0ff68f1b          	addw	t5,a3,255 # 5f5e0ff <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0x5f15d8f>
   1317a:	862a                	mv	a2,a0
   1317c:	03153533          	mulhu	a0,a0,a7
   13180:	812d                	srl	a0,a0,0xb
   13182:	027507bb          	mulw	a5,a0,t2
   13186:	40f607bb          	subw	a5,a2,a5
   1318a:	03079693          	sll	a3,a5,0x30
   1318e:	92c9                	srl	a3,a3,0x32
   13190:	03d686b3          	mul	a3,a3,t4
   13194:	82c5                	srl	a3,a3,0x11
   13196:	00169f93          	sll	t6,a3,0x1
   1319a:	025686bb          	mulw	a3,a3,t0
   1319e:	40d786bb          	subw	a3,a5,a3
   131a2:	16c6                	sll	a3,a3,0x31
   131a4:	0306d413          	srl	s0,a3,0x30
   131a8:	01cf87b3          	add	a5,t6,t3
   131ac:	00e306b3          	add	a3,t1,a4
   131b0:	0007cf83          	lbu	t6,0(a5)
   131b4:	00178783          	lb	a5,1(a5)
   131b8:	9472                	add	s0,s0,t3
   131ba:	00140483          	lb	s1,1(s0)
   131be:	00044403          	lbu	s0,0(s0)
   131c2:	02f68223          	sb	a5,36(a3)
   131c6:	03f681a3          	sb	t6,35(a3)
   131ca:	02968323          	sb	s1,38(a3)
   131ce:	028682a3          	sb	s0,37(a3)
   131d2:	1771                	add	a4,a4,-4
   131d4:	facf63e3          	bltu	t5,a2,1317a <.LBB543_11+0x28>
   131d8:	02770713          	add	a4,a4,39
   131dc:	06300693          	li	a3,99
   131e0:	f4a6fbe3          	bgeu	a3,a0,13136 <.LBB543_10+0x14>
   131e4:	03051613          	sll	a2,a0,0x30
   131e8:	9249                	srl	a2,a2,0x32
   131ea:	6685                	lui	a3,0x1
   131ec:	47b6869b          	addw	a3,a3,1147 # 147b <_start-0xeb85>
   131f0:	02d60633          	mul	a2,a2,a3
   131f4:	8245                	srl	a2,a2,0x11
   131f6:	06400693          	li	a3,100
   131fa:	02d606bb          	mulw	a3,a2,a3
   131fe:	9d15                	subw	a0,a0,a3
   13200:	1546                	sll	a0,a0,0x31
   13202:	9141                	srl	a0,a0,0x30
   13204:	1779                	add	a4,a4,-2
   13206:	9572                	add	a0,a0,t3
   13208:	00150683          	lb	a3,1(a0)
   1320c:	00054503          	lbu	a0,0(a0)
   13210:	00110793          	add	a5,sp,1
   13214:	97ba                	add	a5,a5,a4
   13216:	00d780a3          	sb	a3,1(a5)
   1321a:	00a78023          	sb	a0,0(a5)
   1321e:	8532                	mv	a0,a2
   13220:	4629                	li	a2,10
   13222:	f0c56de3          	bltu	a0,a2,1313c <.LBB543_10+0x1a>
   13226:	0506                	sll	a0,a0,0x1
   13228:	ffe70693          	add	a3,a4,-2
   1322c:	9572                	add	a0,a0,t3
   1322e:	00150603          	lb	a2,1(a0)
   13232:	00054503          	lbu	a0,0(a0)
   13236:	00110713          	add	a4,sp,1
   1323a:	9736                	add	a4,a4,a3
   1323c:	00c700a3          	sb	a2,1(a4)
   13240:	00a70023          	sb	a0,0(a4)
   13244:	00110513          	add	a0,sp,1
   13248:	00d50733          	add	a4,a0,a3
   1324c:	02700513          	li	a0,39
   13250:	40d507b3          	sub	a5,a0,a3

0000000000013254 <.LBB543_12>:
   13254:	00001617          	auipc	a2,0x1
   13258:	68460613          	add	a2,a2,1668 # 148d8 <.Lanon.442aba94db1f841cd37d39ada1516238.83>
   1325c:	8542                	mv	a0,a6
   1325e:	4681                	li	a3,0
   13260:	fffff097          	auipc	ra,0xfffff
   13264:	a3e080e7          	jalr	-1474(ra) # 11c9e <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E>
   13268:	70e2                	ld	ra,56(sp)
   1326a:	7442                	ld	s0,48(sp)
   1326c:	74a2                	ld	s1,40(sp)
   1326e:	6121                	add	sp,sp,64
   13270:	8082                	ret

0000000000013272 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h2cede4cfd0806b52E>:
   13272:	00056503          	lwu	a0,0(a0)
   13276:	862e                	mv	a2,a1
   13278:	4585                	li	a1,1
   1327a:	00000317          	auipc	t1,0x0
   1327e:	e9230067          	jr	-366(t1) # 1310c <_ZN4core3fmt3num3imp7fmt_u6417h5e0a68673bc51c93E>

0000000000013282 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i64$GT$3fmt17h23de1c985db3efdcE>:
   13282:	6110                	ld	a2,0(a0)
   13284:	43f65513          	sra	a0,a2,0x3f
   13288:	00a606b3          	add	a3,a2,a0
   1328c:	8d35                	xor	a0,a0,a3
   1328e:	fff64613          	not	a2,a2
   13292:	927d                	srl	a2,a2,0x3f
   13294:	86ae                	mv	a3,a1
   13296:	85b2                	mv	a1,a2
   13298:	8636                	mv	a2,a3
   1329a:	00000317          	auipc	t1,0x0
   1329e:	e7230067          	jr	-398(t1) # 1310c <_ZN4core3fmt3num3imp7fmt_u6417h5e0a68673bc51c93E>

00000000000132a2 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h68be92e0167cbc01E>:
   132a2:	6108                	ld	a0,0(a0)
   132a4:	862e                	mv	a2,a1
   132a6:	4585                	li	a1,1
   132a8:	00000317          	auipc	t1,0x0
   132ac:	e6430067          	jr	-412(t1) # 1310c <_ZN4core3fmt3num3imp7fmt_u6417h5e0a68673bc51c93E>

00000000000132b0 <_ZN53_$LT$core..fmt..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17ha7da3f83b0a295bbE>:
   132b0:	7590                	ld	a2,40(a1)
   132b2:	7188                	ld	a0,32(a1)
   132b4:	6e1c                	ld	a5,24(a2)

00000000000132b6 <.LBB575_1>:
   132b6:	00002597          	auipc	a1,0x2
   132ba:	34a58593          	add	a1,a1,842 # 15600 <.Lanon.442aba94db1f841cd37d39ada1516238.584>
   132be:	4615                	li	a2,5
   132c0:	8782                	jr	a5

00000000000132c2 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h0c2533d9de598038E>:
   132c2:	6108                	ld	a0,0(a0)
   132c4:	00000317          	auipc	t1,0x0
   132c8:	d7230067          	jr	-654(t1) # 13036 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E>

00000000000132cc <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h2e9275016a084c2bE>:
   132cc:	7139                	add	sp,sp,-64
   132ce:	fc06                	sd	ra,56(sp)
   132d0:	f822                	sd	s0,48(sp)
   132d2:	f426                	sd	s1,40(sp)
   132d4:	6108                	ld	a0,0(a0)
   132d6:	00054603          	lbu	a2,0(a0)
   132da:	842e                	mv	s0,a1
   132dc:	ce49                	beqz	a2,13376 <.LBB588_13+0x1e>
   132de:	740c                	ld	a1,40(s0)
   132e0:	0505                	add	a0,a0,1
   132e2:	e42a                	sd	a0,8(sp)
   132e4:	7008                	ld	a0,32(s0)
   132e6:	6d94                	ld	a3,24(a1)

00000000000132e8 <.LBB588_10>:
   132e8:	00001597          	auipc	a1,0x1
   132ec:	dac58593          	add	a1,a1,-596 # 14094 <.Lanon.fad58de7366495db4650cfefac2fcd61.9+0xf>
   132f0:	4611                	li	a2,4
   132f2:	9682                	jalr	a3
   132f4:	e822                	sd	s0,16(sp)
   132f6:	02a10023          	sb	a0,32(sp)
   132fa:	ec02                	sd	zero,24(sp)
   132fc:	020100a3          	sb	zero,33(sp)

0000000000013300 <.LBB588_11>:
   13300:	00002617          	auipc	a2,0x2
   13304:	82060613          	add	a2,a2,-2016 # 14b20 <.Lanon.442aba94db1f841cd37d39ada1516238.243>
   13308:	0808                	add	a0,sp,16
   1330a:	002c                	add	a1,sp,8
   1330c:	ffffe097          	auipc	ra,0xffffe
   13310:	4ae080e7          	jalr	1198(ra) # 117ba <_ZN4core3fmt8builders10DebugTuple5field17hb4c5d4885bb0d25dE>
   13314:	6562                	ld	a0,24(sp)
   13316:	02014583          	lbu	a1,32(sp)
   1331a:	c539                	beqz	a0,13368 <.LBB588_13+0x10>
   1331c:	4405                	li	s0,1
   1331e:	e5a1                	bnez	a1,13366 <.LBB588_13+0xe>
   13320:	02114583          	lbu	a1,33(sp)
   13324:	157d                	add	a0,a0,-1
   13326:	00153513          	seqz	a0,a0
   1332a:	64c2                	ld	s1,16(sp)
   1332c:	00b035b3          	snez	a1,a1
   13330:	8d6d                	and	a0,a0,a1
   13332:	c105                	beqz	a0,13352 <.LBB588_12+0x10>
   13334:	0304c503          	lbu	a0,48(s1)
   13338:	8911                	and	a0,a0,4
   1333a:	ed01                	bnez	a0,13352 <.LBB588_12+0x10>
   1333c:	748c                	ld	a1,40(s1)
   1333e:	7088                	ld	a0,32(s1)
   13340:	6d94                	ld	a3,24(a1)

0000000000013342 <.LBB588_12>:
   13342:	00001597          	auipc	a1,0x1
   13346:	7d658593          	add	a1,a1,2006 # 14b18 <.Lanon.442aba94db1f841cd37d39ada1516238.238>
   1334a:	4605                	li	a2,1
   1334c:	4405                	li	s0,1
   1334e:	9682                	jalr	a3
   13350:	e919                	bnez	a0,13366 <.LBB588_13+0xe>
   13352:	748c                	ld	a1,40(s1)
   13354:	7088                	ld	a0,32(s1)
   13356:	6d94                	ld	a3,24(a1)

0000000000013358 <.LBB588_13>:
   13358:	00001597          	auipc	a1,0x1
   1335c:	5b258593          	add	a1,a1,1458 # 1490a <.Lanon.442aba94db1f841cd37d39ada1516238.137>
   13360:	4605                	li	a2,1
   13362:	9682                	jalr	a3
   13364:	842a                	mv	s0,a0
   13366:	85a2                	mv	a1,s0
   13368:	00b03533          	snez	a0,a1
   1336c:	70e2                	ld	ra,56(sp)
   1336e:	7442                	ld	s0,48(sp)
   13370:	74a2                	ld	s1,40(sp)
   13372:	6121                	add	sp,sp,64
   13374:	8082                	ret
   13376:	740c                	ld	a1,40(s0)
   13378:	7008                	ld	a0,32(s0)
   1337a:	6d9c                	ld	a5,24(a1)

000000000001337c <.LBB588_14>:
   1337c:	00001597          	auipc	a1,0x1
   13380:	d3c58593          	add	a1,a1,-708 # 140b8 <.Lanon.fad58de7366495db4650cfefac2fcd61.9+0x33>
   13384:	4611                	li	a2,4
   13386:	70e2                	ld	ra,56(sp)
   13388:	7442                	ld	s0,48(sp)
   1338a:	74a2                	ld	s1,40(sp)
   1338c:	6121                	add	sp,sp,64
   1338e:	8782                	jr	a5

0000000000013390 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h3c4bf84e4a8f4da0E>:
   13390:	6108                	ld	a0,0(a0)
   13392:	00000317          	auipc	t1,0x0
   13396:	aec30067          	jr	-1300(t1) # 12e7e <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E>

000000000001339a <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hdf0bbe7209f97e5eE>:
   1339a:	7159                	add	sp,sp,-112
   1339c:	f486                	sd	ra,104(sp)
   1339e:	6108                	ld	a0,0(a0)
   133a0:	6108                	ld	a0,0(a0)
   133a2:	f42a                	sd	a0,40(sp)
   133a4:	fff50613          	add	a2,a0,-1
   133a8:	fff54513          	not	a0,a0
   133ac:	8d71                	and	a0,a0,a2

00000000000133ae <.LBB647_1>:
   133ae:	00003617          	auipc	a2,0x3
   133b2:	fa260613          	add	a2,a2,-94 # 16350 <.LCPI647_0>
   133b6:	6210                	ld	a2,0(a2)

00000000000133b8 <.LBB647_2>:
   133b8:	00003697          	auipc	a3,0x3
   133bc:	fa068693          	add	a3,a3,-96 # 16358 <.LCPI647_1>
   133c0:	6294                	ld	a3,0(a3)
   133c2:	00155713          	srl	a4,a0,0x1
   133c6:	8e79                	and	a2,a2,a4
   133c8:	8d11                	sub	a0,a0,a2
   133ca:	00d57633          	and	a2,a0,a3
   133ce:	8109                	srl	a0,a0,0x2
   133d0:	8d75                	and	a0,a0,a3
   133d2:	9532                	add	a0,a0,a2

00000000000133d4 <.LBB647_3>:
   133d4:	00003617          	auipc	a2,0x3
   133d8:	f8c60613          	add	a2,a2,-116 # 16360 <.LCPI647_2>
   133dc:	6210                	ld	a2,0(a2)

00000000000133de <.LBB647_4>:
   133de:	00003697          	auipc	a3,0x3
   133e2:	f8a68693          	add	a3,a3,-118 # 16368 <.LCPI647_3>
   133e6:	6294                	ld	a3,0(a3)
   133e8:	00455713          	srl	a4,a0,0x4
   133ec:	953a                	add	a0,a0,a4
   133ee:	8d71                	and	a0,a0,a2
   133f0:	02d50533          	mul	a0,a0,a3
   133f4:	9161                	srl	a0,a0,0x38
   133f6:	da2a                	sw	a0,52(sp)
   133f8:	1028                	add	a0,sp,40
   133fa:	e42a                	sd	a0,8(sp)

00000000000133fc <.LBB647_5>:
   133fc:	00000517          	auipc	a0,0x0
   13400:	96e50513          	add	a0,a0,-1682 # 12d6a <_ZN69_$LT$core..num..nonzero..NonZeroUsize$u20$as$u20$core..fmt..Debug$GT$3fmt17hcd3cf5bdc6888583E>
   13404:	e82a                	sd	a0,16(sp)
   13406:	1848                	add	a0,sp,52
   13408:	ec2a                	sd	a0,24(sp)

000000000001340a <.LBB647_6>:
   1340a:	00000517          	auipc	a0,0x0
   1340e:	b5450513          	add	a0,a0,-1196 # 12f5e <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E>
   13412:	f02a                	sd	a0,32(sp)
   13414:	7188                	ld	a0,32(a1)
   13416:	758c                	ld	a1,40(a1)

0000000000013418 <.LBB647_7>:
   13418:	00001617          	auipc	a2,0x1
   1341c:	4f860613          	add	a2,a2,1272 # 14910 <.Lanon.442aba94db1f841cd37d39ada1516238.138>
   13420:	fc32                	sd	a2,56(sp)
   13422:	460d                	li	a2,3
   13424:	e0b2                	sd	a2,64(sp)
   13426:	e482                	sd	zero,72(sp)
   13428:	0030                	add	a2,sp,8
   1342a:	ecb2                	sd	a2,88(sp)
   1342c:	4609                	li	a2,2
   1342e:	f0b2                	sd	a2,96(sp)
   13430:	1830                	add	a2,sp,56
   13432:	ffffe097          	auipc	ra,0xffffe
   13436:	6d0080e7          	jalr	1744(ra) # 11b02 <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   1343a:	70a6                	ld	ra,104(sp)
   1343c:	6165                	add	sp,sp,112
   1343e:	8082                	ret

0000000000013440 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf12cc52608b51daeE>:
   13440:	6510                	ld	a2,8(a0)
   13442:	6108                	ld	a0,0(a0)
   13444:	6e1c                	ld	a5,24(a2)
   13446:	8782                	jr	a5

0000000000013448 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdd6e30ce7f47bbb8E>:
   13448:	6114                	ld	a3,0(a0)
   1344a:	6510                	ld	a2,8(a0)
   1344c:	852e                	mv	a0,a1
   1344e:	85b6                	mv	a1,a3
   13450:	fffff317          	auipc	t1,0xfffff
   13454:	b1a30067          	jr	-1254(t1) # 11f6a <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E>

0000000000013458 <_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h0090dc94d9494495E>:
   13458:	7179                	add	sp,sp,-48
   1345a:	f406                	sd	ra,40(sp)
   1345c:	f022                	sd	s0,32(sp)
   1345e:	842e                	mv	s0,a1
   13460:	e02a                	sd	a0,0(sp)
   13462:	758c                	ld	a1,40(a1)
   13464:	0521                	add	a0,a0,8
   13466:	e42a                	sd	a0,8(sp)
   13468:	7008                	ld	a0,32(s0)
   1346a:	6d94                	ld	a3,24(a1)

000000000001346c <.LBB669_8>:
   1346c:	00002597          	auipc	a1,0x2
   13470:	19958593          	add	a1,a1,409 # 15605 <.Lanon.442aba94db1f841cd37d39ada1516238.608>
   13474:	4625                	li	a2,9
   13476:	9682                	jalr	a3
   13478:	e822                	sd	s0,16(sp)
   1347a:	00a10c23          	sb	a0,24(sp)
   1347e:	00010ca3          	sb	zero,25(sp)

0000000000013482 <.LBB669_9>:
   13482:	00002597          	auipc	a1,0x2
   13486:	18c58593          	add	a1,a1,396 # 1560e <.Lanon.442aba94db1f841cd37d39ada1516238.609>

000000000001348a <.LBB669_10>:
   1348a:	00001717          	auipc	a4,0x1
   1348e:	53670713          	add	a4,a4,1334 # 149c0 <.Lanon.442aba94db1f841cd37d39ada1516238.210>
   13492:	0808                	add	a0,sp,16
   13494:	462d                	li	a2,11
   13496:	868a                	mv	a3,sp
   13498:	ffffe097          	auipc	ra,0xffffe
   1349c:	1a6080e7          	jalr	422(ra) # 1163e <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E>

00000000000134a0 <.LBB669_11>:
   134a0:	00002597          	auipc	a1,0x2
   134a4:	17958593          	add	a1,a1,377 # 15619 <.Lanon.442aba94db1f841cd37d39ada1516238.610>

00000000000134a8 <.LBB669_12>:
   134a8:	00002717          	auipc	a4,0x2
   134ac:	18070713          	add	a4,a4,384 # 15628 <.Lanon.442aba94db1f841cd37d39ada1516238.611>
   134b0:	0808                	add	a0,sp,16
   134b2:	4625                	li	a2,9
   134b4:	0034                	add	a3,sp,8
   134b6:	ffffe097          	auipc	ra,0xffffe
   134ba:	188080e7          	jalr	392(ra) # 1163e <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E>
   134be:	01914503          	lbu	a0,25(sp)
   134c2:	01814583          	lbu	a1,24(sp)
   134c6:	c91d                	beqz	a0,134fc <.LBB669_14+0xe>
   134c8:	4505                	li	a0,1
   134ca:	e985                	bnez	a1,134fa <.LBB669_14+0xc>
   134cc:	6542                	ld	a0,16(sp)
   134ce:	03054583          	lbu	a1,48(a0)
   134d2:	8991                	and	a1,a1,4
   134d4:	e991                	bnez	a1,134e8 <.LBB669_13+0xc>
   134d6:	750c                	ld	a1,40(a0)
   134d8:	7108                	ld	a0,32(a0)
   134da:	6d94                	ld	a3,24(a1)

00000000000134dc <.LBB669_13>:
   134dc:	00001597          	auipc	a1,0x1
   134e0:	63758593          	add	a1,a1,1591 # 14b13 <.Lanon.442aba94db1f841cd37d39ada1516238.235>
   134e4:	4609                	li	a2,2
   134e6:	a809                	j	134f8 <.LBB669_14+0xa>
   134e8:	750c                	ld	a1,40(a0)
   134ea:	7108                	ld	a0,32(a0)
   134ec:	6d94                	ld	a3,24(a1)

00000000000134ee <.LBB669_14>:
   134ee:	00001597          	auipc	a1,0x1
   134f2:	62458593          	add	a1,a1,1572 # 14b12 <.Lanon.442aba94db1f841cd37d39ada1516238.232>
   134f6:	4605                	li	a2,1
   134f8:	9682                	jalr	a3
   134fa:	85aa                	mv	a1,a0
   134fc:	00b03533          	snez	a0,a1
   13500:	70a2                	ld	ra,40(sp)
   13502:	7402                	ld	s0,32(sp)
   13504:	6145                	add	sp,sp,48
   13506:	8082                	ret

0000000000013508 <_ZN4core7unicode12unicode_data15grapheme_extend6lookup17h412ef2d588f4e502E>:
   13508:	1141                	add	sp,sp,-16
   1350a:	e406                	sd	ra,8(sp)
   1350c:	4701                	li	a4,0
   1350e:	00b5131b          	sllw	t1,a0,0xb
   13512:	02000793          	li	a5,32

0000000000013516 <.LBB699_26>:
   13516:	00002297          	auipc	t0,0x2
   1351a:	13228293          	add	t0,t0,306 # 15648 <_ZN4core7unicode12unicode_data15grapheme_extend17SHORT_OFFSET_RUNS17h0644dacd678a09b8E>
   1351e:	4885                	li	a7,1
   13520:	0ff00813          	li	a6,255
   13524:	02000693          	li	a3,32
   13528:	a031                	j	13534 <.LBB699_26+0x1e>
   1352a:	86be                	mv	a3,a5
   1352c:	40e687b3          	sub	a5,a3,a4
   13530:	02d77b63          	bgeu	a4,a3,13566 <.LBB699_26+0x50>
   13534:	0017d593          	srl	a1,a5,0x1
   13538:	00e587b3          	add	a5,a1,a4
   1353c:	00279593          	sll	a1,a5,0x2
   13540:	9596                	add	a1,a1,t0
   13542:	418c                	lw	a1,0(a1)
   13544:	00b5961b          	sllw	a2,a1,0xb
   13548:	55fd                	li	a1,-1
   1354a:	00666663          	bltu	a2,t1,13556 <.LBB699_26+0x40>
   1354e:	006645b3          	xor	a1,a2,t1
   13552:	00b035b3          	snez	a1,a1
   13556:	fd158ae3          	beq	a1,a7,1352a <.LBB699_26+0x14>
   1355a:	00178713          	add	a4,a5,1
   1355e:	0ff5f593          	zext.b	a1,a1
   13562:	fd0585e3          	beq	a1,a6,1352c <.LBB699_26+0x16>
   13566:	45fd                	li	a1,31
   13568:	0ae5e763          	bltu	a1,a4,13616 <.LBB699_29>
   1356c:	00271793          	sll	a5,a4,0x2
   13570:	2c300613          	li	a2,707
   13574:	00b70863          	beq	a4,a1,13584 <.LBB699_26+0x6e>
   13578:	00f285b3          	add	a1,t0,a5
   1357c:	0045e583          	lwu	a1,4(a1)
   13580:	0155d613          	srl	a2,a1,0x15
   13584:	fff70693          	add	a3,a4,-1
   13588:	00d77463          	bgeu	a4,a3,13590 <.LBB699_26+0x7a>
   1358c:	4701                	li	a4,0
   1358e:	a829                	j	135a8 <.LBB699_26+0x92>
   13590:	02000593          	li	a1,32
   13594:	08b6fd63          	bgeu	a3,a1,1362e <.LBB699_30>
   13598:	00269593          	sll	a1,a3,0x2
   1359c:	9596                	add	a1,a1,t0
   1359e:	0005e583          	lwu	a1,0(a1)
   135a2:	15ae                	sll	a1,a1,0x2b
   135a4:	02b5d713          	srl	a4,a1,0x2b
   135a8:	005785b3          	add	a1,a5,t0
   135ac:	0005e583          	lwu	a1,0(a1)
   135b0:	81d5                	srl	a1,a1,0x15
   135b2:	fff5c693          	not	a3,a1
   135b6:	96b2                	add	a3,a3,a2
   135b8:	ce95                	beqz	a3,135f4 <.LBB699_27+0x22>
   135ba:	2c300793          	li	a5,707
   135be:	882e                	mv	a6,a1
   135c0:	00b7e463          	bltu	a5,a1,135c8 <.LBB699_26+0xb2>
   135c4:	2c300813          	li	a6,707
   135c8:	4781                	li	a5,0
   135ca:	40e5073b          	subw	a4,a0,a4
   135ce:	fff60513          	add	a0,a2,-1

00000000000135d2 <.LBB699_27>:
   135d2:	00002617          	auipc	a2,0x2
   135d6:	0f660613          	add	a2,a2,246 # 156c8 <_ZN4core7unicode12unicode_data15grapheme_extend7OFFSETS17h3c7ea72c87f82ab8E>
   135da:	02b80263          	beq	a6,a1,135fe <.LBB699_28>
   135de:	00c586b3          	add	a3,a1,a2
   135e2:	0006c683          	lbu	a3,0(a3)
   135e6:	9fb5                	addw	a5,a5,a3
   135e8:	00f76663          	bltu	a4,a5,135f4 <.LBB699_27+0x22>
   135ec:	0585                	add	a1,a1,1
   135ee:	feb516e3          	bne	a0,a1,135da <.LBB699_27+0x8>
   135f2:	85aa                	mv	a1,a0
   135f4:	0015f513          	and	a0,a1,1
   135f8:	60a2                	ld	ra,8(sp)
   135fa:	0141                	add	sp,sp,16
   135fc:	8082                	ret

00000000000135fe <.LBB699_28>:
   135fe:	00002617          	auipc	a2,0x2
   13602:	39260613          	add	a2,a2,914 # 15990 <.Lanon.442aba94db1f841cd37d39ada1516238.677>
   13606:	2c300593          	li	a1,707
   1360a:	8542                	mv	a0,a6
   1360c:	ffffe097          	auipc	ra,0xffffe
   13610:	c8c080e7          	jalr	-884(ra) # 11298 <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>
	...

0000000000013616 <.LBB699_29>:
   13616:	00002617          	auipc	a2,0x2
   1361a:	37a60613          	add	a2,a2,890 # 15990 <.Lanon.442aba94db1f841cd37d39ada1516238.677>
   1361e:	02000593          	li	a1,32
   13622:	853a                	mv	a0,a4
   13624:	ffffe097          	auipc	ra,0xffffe
   13628:	c74080e7          	jalr	-908(ra) # 11298 <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>
	...

000000000001362e <.LBB699_30>:
   1362e:	00002617          	auipc	a2,0x2
   13632:	fba60613          	add	a2,a2,-70 # 155e8 <.Lanon.442aba94db1f841cd37d39ada1516238.396>
   13636:	02000593          	li	a1,32
   1363a:	8536                	mv	a0,a3
   1363c:	ffffe097          	auipc	ra,0xffffe
   13640:	c5c080e7          	jalr	-932(ra) # 11298 <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>
	...

0000000000013646 <_ZN64_$LT$core..alloc..layout..Layout$u20$as$u20$core..fmt..Debug$GT$3fmt17h86d7136df2fe6134E>:
   13646:	7179                	add	sp,sp,-48
   13648:	f406                	sd	ra,40(sp)
   1364a:	f022                	sd	s0,32(sp)
   1364c:	842e                	mv	s0,a1
   1364e:	e02a                	sd	a0,0(sp)
   13650:	758c                	ld	a1,40(a1)
   13652:	0521                	add	a0,a0,8
   13654:	e42a                	sd	a0,8(sp)
   13656:	7008                	ld	a0,32(s0)
   13658:	6d94                	ld	a3,24(a1)

000000000001365a <.LBB709_8>:
   1365a:	00002597          	auipc	a1,0x2
   1365e:	34e58593          	add	a1,a1,846 # 159a8 <.Lanon.442aba94db1f841cd37d39ada1516238.695>
   13662:	4619                	li	a2,6
   13664:	9682                	jalr	a3
   13666:	e822                	sd	s0,16(sp)
   13668:	00a10c23          	sb	a0,24(sp)
   1366c:	00010ca3          	sb	zero,25(sp)

0000000000013670 <.LBB709_9>:
   13670:	00001597          	auipc	a1,0x1
   13674:	a2058593          	add	a1,a1,-1504 # 14090 <.Lanon.fad58de7366495db4650cfefac2fcd61.9+0xb>

0000000000013678 <.LBB709_10>:
   13678:	00001717          	auipc	a4,0x1
   1367c:	34870713          	add	a4,a4,840 # 149c0 <.Lanon.442aba94db1f841cd37d39ada1516238.210>
   13680:	0808                	add	a0,sp,16
   13682:	4611                	li	a2,4
   13684:	868a                	mv	a3,sp
   13686:	ffffe097          	auipc	ra,0xffffe
   1368a:	fb8080e7          	jalr	-72(ra) # 1163e <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E>

000000000001368e <.LBB709_11>:
   1368e:	00002597          	auipc	a1,0x2
   13692:	32058593          	add	a1,a1,800 # 159ae <.Lanon.442aba94db1f841cd37d39ada1516238.697>

0000000000013696 <.LBB709_12>:
   13696:	00002717          	auipc	a4,0x2
   1369a:	32270713          	add	a4,a4,802 # 159b8 <.Lanon.442aba94db1f841cd37d39ada1516238.698>
   1369e:	0808                	add	a0,sp,16
   136a0:	4615                	li	a2,5
   136a2:	0034                	add	a3,sp,8
   136a4:	ffffe097          	auipc	ra,0xffffe
   136a8:	f9a080e7          	jalr	-102(ra) # 1163e <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E>
   136ac:	01914503          	lbu	a0,25(sp)
   136b0:	01814583          	lbu	a1,24(sp)
   136b4:	c91d                	beqz	a0,136ea <.LBB709_14+0xe>
   136b6:	4505                	li	a0,1
   136b8:	e985                	bnez	a1,136e8 <.LBB709_14+0xc>
   136ba:	6542                	ld	a0,16(sp)
   136bc:	03054583          	lbu	a1,48(a0)
   136c0:	8991                	and	a1,a1,4
   136c2:	e991                	bnez	a1,136d6 <.LBB709_13+0xc>
   136c4:	750c                	ld	a1,40(a0)
   136c6:	7108                	ld	a0,32(a0)
   136c8:	6d94                	ld	a3,24(a1)

00000000000136ca <.LBB709_13>:
   136ca:	00001597          	auipc	a1,0x1
   136ce:	44958593          	add	a1,a1,1097 # 14b13 <.Lanon.442aba94db1f841cd37d39ada1516238.235>
   136d2:	4609                	li	a2,2
   136d4:	a809                	j	136e6 <.LBB709_14+0xa>
   136d6:	750c                	ld	a1,40(a0)
   136d8:	7108                	ld	a0,32(a0)
   136da:	6d94                	ld	a3,24(a1)

00000000000136dc <.LBB709_14>:
   136dc:	00001597          	auipc	a1,0x1
   136e0:	43658593          	add	a1,a1,1078 # 14b12 <.Lanon.442aba94db1f841cd37d39ada1516238.232>
   136e4:	4605                	li	a2,1
   136e6:	9682                	jalr	a3
   136e8:	85aa                	mv	a1,a0
   136ea:	00b03533          	snez	a0,a1
   136ee:	70a2                	ld	ra,40(sp)
   136f0:	7402                	ld	s0,32(sp)
   136f2:	6145                	add	sp,sp,48
   136f4:	8082                	ret

00000000000136f6 <rust_begin_unwind>:
   136f6:	7171                	add	sp,sp,-176
   136f8:	f506                	sd	ra,168(sp)
   136fa:	f122                	sd	s0,160(sp)
   136fc:	ed26                	sd	s1,152(sp)
   136fe:	1900                	add	s0,sp,176
   13700:	84aa                	mv	s1,a0
   13702:	ffffe097          	auipc	ra,0xffffe
   13706:	b1a080e7          	jalr	-1254(ra) # 1121c <_ZN4core5panic10panic_info9PanicInfo7message17h4abe68e22d422758E>
   1370a:	12050763          	beqz	a0,13838 <.LBB0_18>
   1370e:	f4a43c23          	sd	a0,-168(s0)
   13712:	8526                	mv	a0,s1
   13714:	ffffe097          	auipc	ra,0xffffe
   13718:	b0c080e7          	jalr	-1268(ra) # 11220 <_ZN4core5panic10panic_info9PanicInfo8location17h873d58c3c1958ff8E>
   1371c:	cd41                	beqz	a0,137b4 <.LBB0_11+0x1c>
   1371e:	610c                	ld	a1,0(a0)
   13720:	6510                	ld	a2,8(a0)
   13722:	f8b43823          	sd	a1,-112(s0)
   13726:	f8c43c23          	sd	a2,-104(s0)
   1372a:	4908                	lw	a0,16(a0)
   1372c:	faa42223          	sw	a0,-92(s0)
   13730:	f9040513          	add	a0,s0,-112
   13734:	f6a43023          	sd	a0,-160(s0)

0000000000013738 <.LBB0_7>:
   13738:	ffffd517          	auipc	a0,0xffffd
   1373c:	38050513          	add	a0,a0,896 # 10ab8 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hd4b1075b6d8c8353E>
   13740:	f6a43423          	sd	a0,-152(s0)
   13744:	fa440513          	add	a0,s0,-92
   13748:	f6a43823          	sd	a0,-144(s0)

000000000001374c <.LBB0_8>:
   1374c:	00000517          	auipc	a0,0x0
   13750:	b2650513          	add	a0,a0,-1242 # 13272 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h2cede4cfd0806b52E>
   13754:	f6a43c23          	sd	a0,-136(s0)
   13758:	f5840513          	add	a0,s0,-168
   1375c:	f8a43023          	sd	a0,-128(s0)

0000000000013760 <.LBB0_9>:
   13760:	ffffd517          	auipc	a0,0xffffd
   13764:	34050513          	add	a0,a0,832 # 10aa0 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h64d226d320fa66bfE>
   13768:	f8a43423          	sd	a0,-120(s0)
   1376c:	fe040513          	add	a0,s0,-32
   13770:	faa43423          	sd	a0,-88(s0)

0000000000013774 <.LBB0_10>:
   13774:	00002517          	auipc	a0,0x2
   13778:	2a450513          	add	a0,a0,676 # 15a18 <.Lanon.86a3613c128665d32fc75176e6ae67c2.11>
   1377c:	faa43823          	sd	a0,-80(s0)
   13780:	4511                	li	a0,4
   13782:	faa43c23          	sd	a0,-72(s0)
   13786:	fc043023          	sd	zero,-64(s0)
   1378a:	f6040513          	add	a0,s0,-160
   1378e:	fca43823          	sd	a0,-48(s0)
   13792:	450d                	li	a0,3
   13794:	fca43c23          	sd	a0,-40(s0)

0000000000013798 <.LBB0_11>:
   13798:	00001597          	auipc	a1,0x1
   1379c:	c4858593          	add	a1,a1,-952 # 143e0 <anon.cab5b07038618639c4e6406ab92cac85.0.llvm.17959331584031496199>
   137a0:	fa840513          	add	a0,s0,-88
   137a4:	fb040613          	add	a2,s0,-80
   137a8:	ffffe097          	auipc	ra,0xffffe
   137ac:	35a080e7          	jalr	858(ra) # 11b02 <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   137b0:	ed39                	bnez	a0,1380e <.LBB0_15>
   137b2:	a001                	j	137b2 <.LBB0_11+0x1a>
   137b4:	f5840513          	add	a0,s0,-168
   137b8:	f6a43023          	sd	a0,-160(s0)

00000000000137bc <.LBB0_12>:
   137bc:	ffffd517          	auipc	a0,0xffffd
   137c0:	2e450513          	add	a0,a0,740 # 10aa0 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h64d226d320fa66bfE>
   137c4:	f6a43423          	sd	a0,-152(s0)
   137c8:	fe040513          	add	a0,s0,-32
   137cc:	f8a43823          	sd	a0,-112(s0)

00000000000137d0 <.LBB0_13>:
   137d0:	00002517          	auipc	a0,0x2
   137d4:	21850513          	add	a0,a0,536 # 159e8 <.Lanon.86a3613c128665d32fc75176e6ae67c2.7>
   137d8:	faa43823          	sd	a0,-80(s0)
   137dc:	4509                	li	a0,2
   137de:	faa43c23          	sd	a0,-72(s0)
   137e2:	fc043023          	sd	zero,-64(s0)
   137e6:	f6040513          	add	a0,s0,-160
   137ea:	fca43823          	sd	a0,-48(s0)
   137ee:	4505                	li	a0,1
   137f0:	fca43c23          	sd	a0,-40(s0)

00000000000137f4 <.LBB0_14>:
   137f4:	00001597          	auipc	a1,0x1
   137f8:	bec58593          	add	a1,a1,-1044 # 143e0 <anon.cab5b07038618639c4e6406ab92cac85.0.llvm.17959331584031496199>
   137fc:	f9040513          	add	a0,s0,-112
   13800:	fb040613          	add	a2,s0,-80
   13804:	ffffe097          	auipc	ra,0xffffe
   13808:	2fe080e7          	jalr	766(ra) # 11b02 <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   1380c:	d15d                	beqz	a0,137b2 <.LBB0_11+0x1a>

000000000001380e <.LBB0_15>:
   1380e:	00001517          	auipc	a0,0x1
   13812:	c0250513          	add	a0,a0,-1022 # 14410 <anon.cab5b07038618639c4e6406ab92cac85.1.llvm.17959331584031496199>

0000000000013816 <.LBB0_16>:
   13816:	00001697          	auipc	a3,0x1
   1381a:	c2a68693          	add	a3,a3,-982 # 14440 <anon.cab5b07038618639c4e6406ab92cac85.2.llvm.17959331584031496199>

000000000001381e <.LBB0_17>:
   1381e:	00001717          	auipc	a4,0x1
   13822:	c5270713          	add	a4,a4,-942 # 14470 <anon.cab5b07038618639c4e6406ab92cac85.4.llvm.17959331584031496199>
   13826:	02b00593          	li	a1,43
   1382a:	fe040613          	add	a2,s0,-32
   1382e:	ffffe097          	auipc	ra,0xffffe
   13832:	bc4080e7          	jalr	-1084(ra) # 113f2 <_ZN4core6result13unwrap_failed17h3c2e5884ed497eadE>
	...

0000000000013838 <.LBB0_18>:
   13838:	00002517          	auipc	a0,0x2
   1383c:	22050513          	add	a0,a0,544 # 15a58 <.Lanon.86a3613c128665d32fc75176e6ae67c2.12>

0000000000013840 <.LBB0_19>:
   13840:	00002617          	auipc	a2,0x2
   13844:	25860613          	add	a2,a2,600 # 15a98 <.Lanon.86a3613c128665d32fc75176e6ae67c2.14>
   13848:	02b00593          	li	a1,43
   1384c:	ffffe097          	auipc	ra,0xffffe
   13850:	9d8080e7          	jalr	-1576(ra) # 11224 <_ZN4core9panicking5panic17h92f54f473578363dE>
	...

0000000000013856 <memcpy>:
   13856:	00000317          	auipc	t1,0x0
   1385a:	00830067          	jr	8(t1) # 1385e <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE>

000000000001385e <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE>:
   1385e:	46bd                	li	a3,15
   13860:	06c6fa63          	bgeu	a3,a2,138d4 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x76>
   13864:	40a006bb          	negw	a3,a0
   13868:	0076f813          	and	a6,a3,7
   1386c:	010503b3          	add	t2,a0,a6
   13870:	00080c63          	beqz	a6,13888 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x2a>
   13874:	87aa                	mv	a5,a0
   13876:	86ae                	mv	a3,a1
   13878:	00068703          	lb	a4,0(a3)
   1387c:	00e78023          	sb	a4,0(a5)
   13880:	0785                	add	a5,a5,1
   13882:	0685                	add	a3,a3,1
   13884:	fe77eae3          	bltu	a5,t2,13878 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x1a>
   13888:	010588b3          	add	a7,a1,a6
   1388c:	41060833          	sub	a6,a2,a6
   13890:	ff887293          	and	t0,a6,-8
   13894:	0078f593          	and	a1,a7,7
   13898:	005386b3          	add	a3,t2,t0
   1389c:	cd9d                	beqz	a1,138da <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x7c>
   1389e:	04505863          	blez	t0,138ee <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x90>
   138a2:	00359313          	sll	t1,a1,0x3
   138a6:	ff88f713          	and	a4,a7,-8
   138aa:	6310                	ld	a2,0(a4)
   138ac:	406005bb          	negw	a1,t1
   138b0:	0385fe13          	and	t3,a1,56
   138b4:	00870793          	add	a5,a4,8
   138b8:	6398                	ld	a4,0(a5)
   138ba:	00665633          	srl	a2,a2,t1
   138be:	01c715b3          	sll	a1,a4,t3
   138c2:	8dd1                	or	a1,a1,a2
   138c4:	00b3b023          	sd	a1,0(t2)
   138c8:	03a1                	add	t2,t2,8
   138ca:	07a1                	add	a5,a5,8
   138cc:	863a                	mv	a2,a4
   138ce:	fed3e5e3          	bltu	t2,a3,138b8 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x5a>
   138d2:	a831                	j	138ee <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x90>
   138d4:	86aa                	mv	a3,a0
   138d6:	e20d                	bnez	a2,138f8 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x9a>
   138d8:	a80d                	j	1390a <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0xac>
   138da:	00505a63          	blez	t0,138ee <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x90>
   138de:	85c6                	mv	a1,a7
   138e0:	6190                	ld	a2,0(a1)
   138e2:	00c3b023          	sd	a2,0(t2)
   138e6:	03a1                	add	t2,t2,8
   138e8:	05a1                	add	a1,a1,8
   138ea:	fed3ebe3          	bltu	t2,a3,138e0 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x82>
   138ee:	005885b3          	add	a1,a7,t0
   138f2:	00787613          	and	a2,a6,7
   138f6:	ca11                	beqz	a2,1390a <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0xac>
   138f8:	9636                	add	a2,a2,a3
   138fa:	00058703          	lb	a4,0(a1)
   138fe:	00e68023          	sb	a4,0(a3)
   13902:	0685                	add	a3,a3,1
   13904:	0585                	add	a1,a1,1
   13906:	fec6eae3          	bltu	a3,a2,138fa <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x9c>
   1390a:	8082                	ret
