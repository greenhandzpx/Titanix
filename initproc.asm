
initproc:     file format elf64-littleriscv


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
   10024:	c34080e7          	jalr	-972(ra) # 10c54 <_ZN78_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..ops..deref..Deref$GT$5deref17h07e5175ddf4fbb7aE>
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
   1005c:	00000097          	auipc	ra,0x0
   10060:	6d4080e7          	jalr	1748(ra) # 10730 <_ZN22buddy_system_allocator4Heap4init17h0387239a34ee507aE>
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
   1008c:	0dc080e7          	jalr	220(ra) # 10164 <main>
   10090:	00000097          	auipc	ra,0x0
   10094:	3f2080e7          	jalr	1010(ra) # 10482 <_ZN8user_lib4exit17h20483dcbea918787E>
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
   100e2:	ef2080e7          	jalr	-270(ra) # 11fd0 <_ZN4core3str8converts9from_utf817hbe620603d93abf90E>
   100e6:	f9843503          	ld	a0,-104(s0)
   100ea:	e121                	bnez	a0,1012a <.LBB2_14+0xda>
   100ec:	fa043a83          	ld	s5,-96(s0)
   100f0:	f9043583          	ld	a1,-112(s0)
   100f4:	f8843503          	ld	a0,-120(s0)
   100f8:	fa843b03          	ld	s6,-88(s0)
   100fc:	faa593e3          	bne	a1,a0,100a2 <.LBB2_14+0x52>
   10100:	f8040513          	add	a0,s0,-128
   10104:	00000097          	auipc	ra,0x0
   10108:	4f2080e7          	jalr	1266(ra) # 105f6 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E>
   1010c:	f9043583          	ld	a1,-112(s0)
   10110:	bf49                	j	100a2 <.LBB2_14+0x52>
   10112:	f8043583          	ld	a1,-128(s0)
   10116:	854a                	mv	a0,s2
   10118:	00000097          	auipc	ra,0x0
   1011c:	04c080e7          	jalr	76(ra) # 10164 <main>
   10120:	00000097          	auipc	ra,0x0
   10124:	362080e7          	jalr	866(ra) # 10482 <_ZN8user_lib4exit17h20483dcbea918787E>
   10128:	0000                	unimp
   1012a:	008a3503          	ld	a0,8(s4)
   1012e:	000a3583          	ld	a1,0(s4)
   10132:	faa43c23          	sd	a0,-72(s0)
   10136:	fab43823          	sd	a1,-80(s0)

000000000001013a <.LBB2_15>:
   1013a:	00004517          	auipc	a0,0x4
   1013e:	06650513          	add	a0,a0,102 # 141a0 <.Lanon.24728be23fbcdb17895d264a60dac18d.4>

0000000000010142 <.LBB2_16>:
   10142:	00004697          	auipc	a3,0x4
   10146:	08e68693          	add	a3,a3,142 # 141d0 <.Lanon.24728be23fbcdb17895d264a60dac18d.5>

000000000001014a <.LBB2_17>:
   1014a:	00004717          	auipc	a4,0x4
   1014e:	0a670713          	add	a4,a4,166 # 141f0 <.Lanon.24728be23fbcdb17895d264a60dac18d.6>
   10152:	02b00593          	li	a1,43
   10156:	fb040613          	add	a2,s0,-80
   1015a:	00001097          	auipc	ra,0x1
   1015e:	dd0080e7          	jalr	-560(ra) # 10f2a <_ZN4core6result13unwrap_failed17h3c2e5884ed497eadE>
	...

0000000000010164 <main>:
   10164:	7171                	add	sp,sp,-176
   10166:	f506                	sd	ra,168(sp)
   10168:	f122                	sd	s0,160(sp)
   1016a:	ed26                	sd	s1,152(sp)
   1016c:	e94a                	sd	s2,144(sp)
   1016e:	e54e                	sd	s3,136(sp)
   10170:	e152                	sd	s4,128(sp)
   10172:	fcd6                	sd	s5,120(sp)
   10174:	f8da                	sd	s6,112(sp)
   10176:	f4de                	sd	s7,104(sp)
   10178:	f0e2                	sd	s8,96(sp)
   1017a:	1900                	add	s0,sp,176
   1017c:	00000097          	auipc	ra,0x0
   10180:	318080e7          	jalr	792(ra) # 10494 <_ZN8user_lib4fork17h4831b1e32e249225E>
   10184:	c925                	beqz	a0,101f4 <.LBB0_7>
   10186:	f5440913          	add	s2,s0,-172
   1018a:	f5840993          	add	s3,s0,-168

000000000001018e <.LBB0_4>:
   1018e:	00003a17          	auipc	s4,0x3
   10192:	b3ca0a13          	add	s4,s4,-1220 # 12cca <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i64$GT$3fmt17h23de1c985db3efdcE>

0000000000010196 <.LBB0_5>:
   10196:	00003a97          	auipc	s5,0x3
   1019a:	b04a8a93          	add	s5,s5,-1276 # 12c9a <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h629811fb06cc9421E>

000000000001019e <.LBB0_6>:
   1019e:	00004497          	auipc	s1,0x4
   101a2:	e9a48493          	add	s1,s1,-358 # 14038 <.Lanon.fad58de7366495db4650cfefac2fcd61.3>
   101a6:	4b0d                	li	s6,3
   101a8:	f9040b93          	add	s7,s0,-112
   101ac:	4c09                	li	s8,2
   101ae:	f4042a23          	sw	zero,-172(s0)
   101b2:	f5440513          	add	a0,s0,-172
   101b6:	00000097          	auipc	ra,0x0
   101ba:	31e080e7          	jalr	798(ra) # 104d4 <_ZN8user_lib4wait17h8e46acc080b2d5feE>
   101be:	f4a43c23          	sd	a0,-168(s0)
   101c2:	f9343823          	sd	s3,-112(s0)
   101c6:	f9443c23          	sd	s4,-104(s0)
   101ca:	fb243023          	sd	s2,-96(s0)
   101ce:	fb543423          	sd	s5,-88(s0)
   101d2:	f6943023          	sd	s1,-160(s0)
   101d6:	f7643423          	sd	s6,-152(s0)
   101da:	f6043823          	sd	zero,-144(s0)
   101de:	f9743023          	sd	s7,-128(s0)
   101e2:	f9843423          	sd	s8,-120(s0)
   101e6:	f6040513          	add	a0,s0,-160
   101ea:	00000097          	auipc	ra,0x0
   101ee:	1ac080e7          	jalr	428(ra) # 10396 <_ZN8user_lib7console5print17h2df775f81e475c40E>
   101f2:	bf75                	j	101ae <.LBB0_6+0x10>

00000000000101f4 <.LBB0_7>:
   101f4:	00004517          	auipc	a0,0x4
   101f8:	e7450513          	add	a0,a0,-396 # 14068 <.Lanon.fad58de7366495db4650cfefac2fcd61.4>
   101fc:	f6a43023          	sd	a0,-160(s0)
   10200:	f6043423          	sd	zero,-152(s0)

0000000000010204 <.LBB0_8>:
   10204:	00004717          	auipc	a4,0x4
   10208:	e7470713          	add	a4,a4,-396 # 14078 <.Lanon.fad58de7366495db4650cfefac2fcd61.4+0x10>
   1020c:	4599                	li	a1,6
   1020e:	f6040613          	add	a2,s0,-160
   10212:	4689                	li	a3,2
   10214:	4785                	li	a5,1
   10216:	00000097          	auipc	ra,0x0
   1021a:	2a2080e7          	jalr	674(ra) # 104b8 <_ZN8user_lib6execve17h3e0a48d92ed78303E>
   1021e:	4501                	li	a0,0
   10220:	70aa                	ld	ra,168(sp)
   10222:	740a                	ld	s0,160(sp)
   10224:	64ea                	ld	s1,152(sp)
   10226:	694a                	ld	s2,144(sp)
   10228:	69aa                	ld	s3,136(sp)
   1022a:	6a0a                	ld	s4,128(sp)
   1022c:	7ae6                	ld	s5,120(sp)
   1022e:	7b46                	ld	s6,112(sp)
   10230:	7ba6                	ld	s7,104(sp)
   10232:	7c06                	ld	s8,96(sp)
   10234:	614d                	add	sp,sp,176
   10236:	8082                	ret

0000000000010238 <__rust_alloc>:
   10238:	00000317          	auipc	t1,0x0
   1023c:	2ba30067          	jr	698(t1) # 104f2 <__rg_alloc>

0000000000010240 <__rust_realloc>:
   10240:	00000317          	auipc	t1,0x0
   10244:	2d830067          	jr	728(t1) # 10518 <__rg_realloc>

0000000000010248 <__rust_alloc_error_handler>:
   10248:	00001317          	auipc	t1,0x1
   1024c:	b1630067          	jr	-1258(t1) # 10d5e <__rg_oom>

0000000000010250 <_ZN4core3ptr37drop_in_place$LT$core..fmt..Error$GT$17h43784471ba3f448dE.llvm.17959331584031496199>:
   10250:	1141                	add	sp,sp,-16
   10252:	e406                	sd	ra,8(sp)
   10254:	e022                	sd	s0,0(sp)
   10256:	0800                	add	s0,sp,16
   10258:	60a2                	ld	ra,8(sp)
   1025a:	6402                	ld	s0,0(sp)
   1025c:	0141                	add	sp,sp,16
   1025e:	8082                	ret

0000000000010260 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h66dec9b024693ddfE.llvm.17959331584031496199>:
   10260:	1101                	add	sp,sp,-32
   10262:	ec06                	sd	ra,24(sp)
   10264:	e822                	sd	s0,16(sp)
   10266:	1000                	add	s0,sp,32
   10268:	0005851b          	sext.w	a0,a1
   1026c:	08000613          	li	a2,128
   10270:	fe042623          	sw	zero,-20(s0)
   10274:	00c57663          	bgeu	a0,a2,10280 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h66dec9b024693ddfE.llvm.17959331584031496199+0x20>
   10278:	feb40623          	sb	a1,-20(s0)
   1027c:	4605                	li	a2,1
   1027e:	a849                	j	10310 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h66dec9b024693ddfE.llvm.17959331584031496199+0xb0>
   10280:	00b5d51b          	srlw	a0,a1,0xb
   10284:	ed19                	bnez	a0,102a2 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h66dec9b024693ddfE.llvm.17959331584031496199+0x42>
   10286:	0065d513          	srl	a0,a1,0x6
   1028a:	0c056513          	or	a0,a0,192
   1028e:	fea40623          	sb	a0,-20(s0)
   10292:	03f5f513          	and	a0,a1,63
   10296:	08056513          	or	a0,a0,128
   1029a:	fea406a3          	sb	a0,-19(s0)
   1029e:	4609                	li	a2,2
   102a0:	a885                	j	10310 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h66dec9b024693ddfE.llvm.17959331584031496199+0xb0>
   102a2:	0105d51b          	srlw	a0,a1,0x10
   102a6:	e51d                	bnez	a0,102d4 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h66dec9b024693ddfE.llvm.17959331584031496199+0x74>
   102a8:	00c5d51b          	srlw	a0,a1,0xc
   102ac:	0e056513          	or	a0,a0,224
   102b0:	fea40623          	sb	a0,-20(s0)
   102b4:	0065d51b          	srlw	a0,a1,0x6
   102b8:	03f57513          	and	a0,a0,63
   102bc:	08056513          	or	a0,a0,128
   102c0:	fea406a3          	sb	a0,-19(s0)
   102c4:	03f5f513          	and	a0,a1,63
   102c8:	08056513          	or	a0,a0,128
   102cc:	fea40723          	sb	a0,-18(s0)
   102d0:	460d                	li	a2,3
   102d2:	a83d                	j	10310 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h66dec9b024693ddfE.llvm.17959331584031496199+0xb0>
   102d4:	0125d51b          	srlw	a0,a1,0x12
   102d8:	891d                	and	a0,a0,7
   102da:	0f056513          	or	a0,a0,240
   102de:	fea40623          	sb	a0,-20(s0)
   102e2:	00c5d51b          	srlw	a0,a1,0xc
   102e6:	03f57513          	and	a0,a0,63
   102ea:	08056513          	or	a0,a0,128
   102ee:	fea406a3          	sb	a0,-19(s0)
   102f2:	0065d51b          	srlw	a0,a1,0x6
   102f6:	03f57513          	and	a0,a0,63
   102fa:	08056513          	or	a0,a0,128
   102fe:	fea40723          	sb	a0,-18(s0)
   10302:	03f5f513          	and	a0,a1,63
   10306:	08056513          	or	a0,a0,128
   1030a:	fea407a3          	sb	a0,-17(s0)
   1030e:	4611                	li	a2,4
   10310:	fec40593          	add	a1,s0,-20
   10314:	04000893          	li	a7,64
   10318:	4505                	li	a0,1
   1031a:	00000073          	ecall
   1031e:	4501                	li	a0,0
   10320:	60e2                	ld	ra,24(sp)
   10322:	6442                	ld	s0,16(sp)
   10324:	6105                	add	sp,sp,32
   10326:	8082                	ret

0000000000010328 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hcd6b00a3f2b58b1dE.llvm.17959331584031496199>:
   10328:	715d                	add	sp,sp,-80
   1032a:	e486                	sd	ra,72(sp)
   1032c:	e0a2                	sd	s0,64(sp)
   1032e:	0880                	add	s0,sp,80
   10330:	6108                	ld	a0,0(a0)
   10332:	7590                	ld	a2,40(a1)
   10334:	7194                	ld	a3,32(a1)
   10336:	faa43c23          	sd	a0,-72(s0)
   1033a:	fec43423          	sd	a2,-24(s0)
   1033e:	fed43023          	sd	a3,-32(s0)
   10342:	6d88                	ld	a0,24(a1)
   10344:	6990                	ld	a2,16(a1)
   10346:	6594                	ld	a3,8(a1)
   10348:	618c                	ld	a1,0(a1)
   1034a:	fca43c23          	sd	a0,-40(s0)
   1034e:	fcc43823          	sd	a2,-48(s0)
   10352:	fcd43423          	sd	a3,-56(s0)
   10356:	fcb43023          	sd	a1,-64(s0)

000000000001035a <.LBB2_1>:
   1035a:	00004597          	auipc	a1,0x4
   1035e:	d2658593          	add	a1,a1,-730 # 14080 <anon.cab5b07038618639c4e6406ab92cac85.0.llvm.17959331584031496199>
   10362:	fb840513          	add	a0,s0,-72
   10366:	fc040613          	add	a2,s0,-64
   1036a:	00001097          	auipc	ra,0x1
   1036e:	2d0080e7          	jalr	720(ra) # 1163a <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   10372:	60a6                	ld	ra,72(sp)
   10374:	6406                	ld	s0,64(sp)
   10376:	6161                	add	sp,sp,80
   10378:	8082                	ret

000000000001037a <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17he0a2184791a24869E.llvm.17959331584031496199>:
   1037a:	1141                	add	sp,sp,-16
   1037c:	e406                	sd	ra,8(sp)
   1037e:	e022                	sd	s0,0(sp)
   10380:	0800                	add	s0,sp,16
   10382:	04000893          	li	a7,64
   10386:	4505                	li	a0,1
   10388:	00000073          	ecall
   1038c:	4501                	li	a0,0
   1038e:	60a2                	ld	ra,8(sp)
   10390:	6402                	ld	s0,0(sp)
   10392:	0141                	add	sp,sp,16
   10394:	8082                	ret

0000000000010396 <_ZN8user_lib7console5print17h2df775f81e475c40E>:
   10396:	715d                	add	sp,sp,-80
   10398:	e486                	sd	ra,72(sp)
   1039a:	e0a2                	sd	s0,64(sp)
   1039c:	0880                	add	s0,sp,80
   1039e:	750c                	ld	a1,40(a0)
   103a0:	7110                	ld	a2,32(a0)
   103a2:	fe840693          	add	a3,s0,-24
   103a6:	fad43823          	sd	a3,-80(s0)
   103aa:	feb43023          	sd	a1,-32(s0)
   103ae:	fcc43c23          	sd	a2,-40(s0)
   103b2:	6d0c                	ld	a1,24(a0)
   103b4:	6910                	ld	a2,16(a0)
   103b6:	6514                	ld	a3,8(a0)
   103b8:	6108                	ld	a0,0(a0)
   103ba:	fcb43823          	sd	a1,-48(s0)
   103be:	fcc43423          	sd	a2,-56(s0)
   103c2:	fcd43023          	sd	a3,-64(s0)
   103c6:	faa43c23          	sd	a0,-72(s0)

00000000000103ca <.LBB5_3>:
   103ca:	00004597          	auipc	a1,0x4
   103ce:	cb658593          	add	a1,a1,-842 # 14080 <anon.cab5b07038618639c4e6406ab92cac85.0.llvm.17959331584031496199>
   103d2:	fb040513          	add	a0,s0,-80
   103d6:	fb840613          	add	a2,s0,-72
   103da:	00001097          	auipc	ra,0x1
   103de:	260080e7          	jalr	608(ra) # 1163a <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   103e2:	e509                	bnez	a0,103ec <.LBB5_4>
   103e4:	60a6                	ld	ra,72(sp)
   103e6:	6406                	ld	s0,64(sp)
   103e8:	6161                	add	sp,sp,80
   103ea:	8082                	ret

00000000000103ec <.LBB5_4>:
   103ec:	00004517          	auipc	a0,0x4
   103f0:	cc450513          	add	a0,a0,-828 # 140b0 <anon.cab5b07038618639c4e6406ab92cac85.1.llvm.17959331584031496199>

00000000000103f4 <.LBB5_5>:
   103f4:	00004697          	auipc	a3,0x4
   103f8:	cec68693          	add	a3,a3,-788 # 140e0 <anon.cab5b07038618639c4e6406ab92cac85.2.llvm.17959331584031496199>

00000000000103fc <.LBB5_6>:
   103fc:	00004717          	auipc	a4,0x4
   10400:	d1470713          	add	a4,a4,-748 # 14110 <anon.cab5b07038618639c4e6406ab92cac85.4.llvm.17959331584031496199>
   10404:	02b00593          	li	a1,43
   10408:	fe840613          	add	a2,s0,-24
   1040c:	00001097          	auipc	ra,0x1
   10410:	b1e080e7          	jalr	-1250(ra) # 10f2a <_ZN4core6result13unwrap_failed17h3c2e5884ed497eadE>
	...

0000000000010416 <_ZN4core3ptr48drop_in_place$LT$core..str..error..Utf8Error$GT$17h6bc69a15aa1420d5E>:
   10416:	1141                	add	sp,sp,-16
   10418:	e406                	sd	ra,8(sp)
   1041a:	e022                	sd	s0,0(sp)
   1041c:	0800                	add	s0,sp,16
   1041e:	60a2                	ld	ra,8(sp)
   10420:	6402                	ld	s0,0(sp)
   10422:	0141                	add	sp,sp,16
   10424:	8082                	ret

0000000000010426 <rust_oom>:
   10426:	711d                	add	sp,sp,-96
   10428:	ec86                	sd	ra,88(sp)
   1042a:	e8a2                	sd	s0,80(sp)
   1042c:	1080                	add	s0,sp,96
   1042e:	faa43023          	sd	a0,-96(s0)
   10432:	fab43423          	sd	a1,-88(s0)
   10436:	fa040513          	add	a0,s0,-96
   1043a:	fea43023          	sd	a0,-32(s0)

000000000001043e <.LBB1_1>:
   1043e:	00003517          	auipc	a0,0x3
   10442:	c5050513          	add	a0,a0,-944 # 1308e <_ZN64_$LT$core..alloc..layout..Layout$u20$as$u20$core..fmt..Debug$GT$3fmt17h86d7136df2fe6134E>
   10446:	fea43423          	sd	a0,-24(s0)

000000000001044a <.LBB1_2>:
   1044a:	00004517          	auipc	a0,0x4
   1044e:	d1e50513          	add	a0,a0,-738 # 14168 <.Lanon.24728be23fbcdb17895d264a60dac18d.1>
   10452:	faa43823          	sd	a0,-80(s0)
   10456:	4505                	li	a0,1
   10458:	faa43c23          	sd	a0,-72(s0)
   1045c:	fc043023          	sd	zero,-64(s0)
   10460:	fe040593          	add	a1,s0,-32
   10464:	fcb43823          	sd	a1,-48(s0)
   10468:	fca43c23          	sd	a0,-40(s0)

000000000001046c <.LBB1_3>:
   1046c:	00004597          	auipc	a1,0x4
   10470:	d1c58593          	add	a1,a1,-740 # 14188 <.Lanon.24728be23fbcdb17895d264a60dac18d.3>
   10474:	fb040513          	add	a0,s0,-80
   10478:	00001097          	auipc	ra,0x1
   1047c:	a84080e7          	jalr	-1404(ra) # 10efc <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

0000000000010482 <_ZN8user_lib4exit17h20483dcbea918787E>:
   10482:	1141                	add	sp,sp,-16
   10484:	e406                	sd	ra,8(sp)
   10486:	e022                	sd	s0,0(sp)
   10488:	0800                	add	s0,sp,16
   1048a:	00000097          	auipc	ra,0x0
   1048e:	274080e7          	jalr	628(ra) # 106fe <_ZN8user_lib7syscall8sys_exit17h641f41ab1e635210E>
	...

0000000000010494 <_ZN8user_lib4fork17h4831b1e32e249225E>:
   10494:	1141                	add	sp,sp,-16
   10496:	e406                	sd	ra,8(sp)
   10498:	e022                	sd	s0,0(sp)
   1049a:	0800                	add	s0,sp,16
   1049c:	0dc00893          	li	a7,220
   104a0:	4581                	li	a1,0
   104a2:	4601                	li	a2,0
   104a4:	4681                	li	a3,0
   104a6:	4701                	li	a4,0
   104a8:	4781                	li	a5,0
   104aa:	4501                	li	a0,0
   104ac:	00000073          	ecall
   104b0:	60a2                	ld	ra,8(sp)
   104b2:	6402                	ld	s0,0(sp)
   104b4:	0141                	add	sp,sp,16
   104b6:	8082                	ret

00000000000104b8 <_ZN8user_lib6execve17h3e0a48d92ed78303E>:
   104b8:	1141                	add	sp,sp,-16
   104ba:	e406                	sd	ra,8(sp)
   104bc:	e022                	sd	s0,0(sp)
   104be:	0800                	add	s0,sp,16
   104c0:	85b2                	mv	a1,a2
   104c2:	0dd00893          	li	a7,221
   104c6:	863a                	mv	a2,a4
   104c8:	00000073          	ecall
   104cc:	60a2                	ld	ra,8(sp)
   104ce:	6402                	ld	s0,0(sp)
   104d0:	0141                	add	sp,sp,16
   104d2:	8082                	ret

00000000000104d4 <_ZN8user_lib4wait17h8e46acc080b2d5feE>:
   104d4:	1141                	add	sp,sp,-16
   104d6:	e406                	sd	ra,8(sp)
   104d8:	e022                	sd	s0,0(sp)
   104da:	0800                	add	s0,sp,16
   104dc:	85aa                	mv	a1,a0
   104de:	10400893          	li	a7,260
   104e2:	557d                	li	a0,-1
   104e4:	4601                	li	a2,0
   104e6:	00000073          	ecall
   104ea:	60a2                	ld	ra,8(sp)
   104ec:	6402                	ld	s0,0(sp)
   104ee:	0141                	add	sp,sp,16
   104f0:	8082                	ret

00000000000104f2 <__rg_alloc>:
   104f2:	1141                	add	sp,sp,-16
   104f4:	e406                	sd	ra,8(sp)
   104f6:	e022                	sd	s0,0(sp)
   104f8:	0800                	add	s0,sp,16

00000000000104fa <.LBB30_1>:
   104fa:	00038617          	auipc	a2,0x38
   104fe:	e7660613          	add	a2,a2,-394 # 48370 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E>
   10502:	86ae                	mv	a3,a1
   10504:	85aa                	mv	a1,a0
   10506:	8532                	mv	a0,a2
   10508:	8636                	mv	a2,a3
   1050a:	60a2                	ld	ra,8(sp)
   1050c:	6402                	ld	s0,0(sp)
   1050e:	0141                	add	sp,sp,16
   10510:	00000317          	auipc	t1,0x0
   10514:	75430067          	jr	1876(t1) # 10c64 <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h6574ad5d6e524a4aE>

0000000000010518 <__rg_realloc>:
   10518:	7139                	add	sp,sp,-64
   1051a:	fc06                	sd	ra,56(sp)
   1051c:	f822                	sd	s0,48(sp)
   1051e:	f426                	sd	s1,40(sp)
   10520:	f04a                	sd	s2,32(sp)
   10522:	ec4e                	sd	s3,24(sp)
   10524:	e852                	sd	s4,16(sp)
   10526:	e456                	sd	s5,8(sp)
   10528:	0080                	add	s0,sp,64
   1052a:	84b6                	mv	s1,a3
   1052c:	8932                	mv	s2,a2
   1052e:	8a2e                	mv	s4,a1
   10530:	89aa                	mv	s3,a0

0000000000010532 <.LBB32_5>:
   10532:	00038517          	auipc	a0,0x38
   10536:	e3e50513          	add	a0,a0,-450 # 48370 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E>
   1053a:	85b6                	mv	a1,a3
   1053c:	00000097          	auipc	ra,0x0
   10540:	728080e7          	jalr	1832(ra) # 10c64 <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h6574ad5d6e524a4aE>
   10544:	8aaa                	mv	s5,a0
   10546:	c515                	beqz	a0,10572 <.LBB32_6+0x16>
   10548:	0144e363          	bltu	s1,s4,1054e <.LBB32_5+0x1c>
   1054c:	84d2                	mv	s1,s4
   1054e:	8556                	mv	a0,s5
   10550:	85ce                	mv	a1,s3
   10552:	8626                	mv	a2,s1
   10554:	00003097          	auipc	ra,0x3
   10558:	d4a080e7          	jalr	-694(ra) # 1329e <memcpy>

000000000001055c <.LBB32_6>:
   1055c:	00038517          	auipc	a0,0x38
   10560:	e1450513          	add	a0,a0,-492 # 48370 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E>
   10564:	85ce                	mv	a1,s3
   10566:	8652                	mv	a2,s4
   10568:	86ca                	mv	a3,s2
   1056a:	00000097          	auipc	ra,0x0
   1056e:	748080e7          	jalr	1864(ra) # 10cb2 <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hbd168ca56515a3a0E>
   10572:	8556                	mv	a0,s5
   10574:	70e2                	ld	ra,56(sp)
   10576:	7442                	ld	s0,48(sp)
   10578:	74a2                	ld	s1,40(sp)
   1057a:	7902                	ld	s2,32(sp)
   1057c:	69e2                	ld	s3,24(sp)
   1057e:	6a42                	ld	s4,16(sp)
   10580:	6aa2                	ld	s5,8(sp)
   10582:	6121                	add	sp,sp,64
   10584:	8082                	ret

0000000000010586 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE>:
   10586:	7179                	add	sp,sp,-48
   10588:	f406                	sd	ra,40(sp)
   1058a:	f022                	sd	s0,32(sp)
   1058c:	ec26                	sd	s1,24(sp)
   1058e:	e84a                	sd	s2,16(sp)
   10590:	e44e                	sd	s3,8(sp)
   10592:	1800                	add	s0,sp,48
   10594:	892e                	mv	s2,a1
   10596:	89aa                	mv	s3,a0
   10598:	ce19                	beqz	a2,105b6 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x30>
   1059a:	84b2                	mv	s1,a2
   1059c:	6a88                	ld	a0,16(a3)
   1059e:	c10d                	beqz	a0,105c0 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x3a>
   105a0:	668c                	ld	a1,8(a3)
   105a2:	cd99                	beqz	a1,105c0 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x3a>
   105a4:	6288                	ld	a0,0(a3)
   105a6:	8626                	mv	a2,s1
   105a8:	86ca                	mv	a3,s2
   105aa:	00000097          	auipc	ra,0x0
   105ae:	c96080e7          	jalr	-874(ra) # 10240 <__rust_realloc>
   105b2:	e11d                	bnez	a0,105d8 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x52>
   105b4:	a011                	j	105b8 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x32>
   105b6:	4481                	li	s1,0
   105b8:	0129b423          	sd	s2,8(s3)
   105bc:	4585                	li	a1,1
   105be:	a00d                	j	105e0 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x5a>
   105c0:	00090a63          	beqz	s2,105d4 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x4e>
   105c4:	854a                	mv	a0,s2
   105c6:	85a6                	mv	a1,s1
   105c8:	00000097          	auipc	ra,0x0
   105cc:	c70080e7          	jalr	-912(ra) # 10238 <__rust_alloc>
   105d0:	e501                	bnez	a0,105d8 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x52>
   105d2:	b7dd                	j	105b8 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE+0x32>
   105d4:	4901                	li	s2,0
   105d6:	8526                	mv	a0,s1
   105d8:	4581                	li	a1,0
   105da:	00a9b423          	sd	a0,8(s3)
   105de:	84ca                	mv	s1,s2
   105e0:	0099b823          	sd	s1,16(s3)
   105e4:	00b9b023          	sd	a1,0(s3)
   105e8:	70a2                	ld	ra,40(sp)
   105ea:	7402                	ld	s0,32(sp)
   105ec:	64e2                	ld	s1,24(sp)
   105ee:	6942                	ld	s2,16(sp)
   105f0:	69a2                	ld	s3,8(sp)
   105f2:	6145                	add	sp,sp,48
   105f4:	8082                	ret

00000000000105f6 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E>:
   105f6:	715d                	add	sp,sp,-80
   105f8:	e486                	sd	ra,72(sp)
   105fa:	e0a2                	sd	s0,64(sp)
   105fc:	fc26                	sd	s1,56(sp)
   105fe:	f84a                	sd	s2,48(sp)
   10600:	0880                	add	s0,sp,80
   10602:	00158613          	add	a2,a1,1
   10606:	08b66b63          	bltu	a2,a1,1069c <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0xa6>
   1060a:	892a                	mv	s2,a0
   1060c:	6508                	ld	a0,8(a0)
   1060e:	00151493          	sll	s1,a0,0x1
   10612:	00966363          	bltu	a2,s1,10618 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0x22>
   10616:	84b2                	mv	s1,a2
   10618:	4591                	li	a1,4
   1061a:	0095e363          	bltu	a1,s1,10620 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0x2a>
   1061e:	4491                	li	s1,4
   10620:	00449593          	sll	a1,s1,0x4
   10624:	0045d613          	srl	a2,a1,0x4
   10628:	8e25                	xor	a2,a2,s1
   1062a:	00c03633          	snez	a2,a2
   1062e:	56c5                	li	a3,-15
   10630:	8285                	srl	a3,a3,0x1
   10632:	00b6b6b3          	sltu	a3,a3,a1
   10636:	8ed1                	or	a3,a3,a2
   10638:	4601                	li	a2,0
   1063a:	e291                	bnez	a3,1063e <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0x48>
   1063c:	4621                	li	a2,8
   1063e:	c51d                	beqz	a0,1066c <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0x76>
   10640:	00451693          	sll	a3,a0,0x4
   10644:	0046d713          	srl	a4,a3,0x4
   10648:	8d39                	xor	a0,a0,a4
   1064a:	00a03533          	snez	a0,a0
   1064e:	5745                	li	a4,-15
   10650:	8305                	srl	a4,a4,0x1
   10652:	00d73733          	sltu	a4,a4,a3
   10656:	8f49                	or	a4,a4,a0
   10658:	4501                	li	a0,0
   1065a:	e311                	bnez	a4,1065e <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0x68>
   1065c:	4521                	li	a0,8
   1065e:	00093703          	ld	a4,0(s2)
   10662:	fce43423          	sd	a4,-56(s0)
   10666:	fcd43823          	sd	a3,-48(s0)
   1066a:	a011                	j	1066e <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0x78>
   1066c:	4501                	li	a0,0
   1066e:	fca43c23          	sd	a0,-40(s0)
   10672:	fb040513          	add	a0,s0,-80
   10676:	fc840693          	add	a3,s0,-56
   1067a:	00000097          	auipc	ra,0x0
   1067e:	f0c080e7          	jalr	-244(ra) # 10586 <_ZN5alloc7raw_vec11finish_grow17h49b8def637080eddE>
   10682:	fb043583          	ld	a1,-80(s0)
   10686:	fb843503          	ld	a0,-72(s0)
   1068a:	cd91                	beqz	a1,106a6 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0xb0>
   1068c:	fc043583          	ld	a1,-64(s0)
   10690:	567d                	li	a2,-1
   10692:	167e                	sll	a2,a2,0x3f
   10694:	0605                	add	a2,a2,1
   10696:	00c58c63          	beq	a1,a2,106ae <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0xb8>
   1069a:	e185                	bnez	a1,106ba <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hc05acd2cd4ca7bd2E+0xc4>
   1069c:	00000097          	auipc	ra,0x0
   106a0:	672080e7          	jalr	1650(ra) # 10d0e <_ZN5alloc7raw_vec17capacity_overflow17h8c103c8c1cb34845E>
   106a4:	0000                	unimp
   106a6:	00a93023          	sd	a0,0(s2)
   106aa:	00993423          	sd	s1,8(s2)
   106ae:	60a6                	ld	ra,72(sp)
   106b0:	6406                	ld	s0,64(sp)
   106b2:	74e2                	ld	s1,56(sp)
   106b4:	7942                	ld	s2,48(sp)
   106b6:	6161                	add	sp,sp,80
   106b8:	8082                	ret
   106ba:	00000097          	auipc	ra,0x0
   106be:	688080e7          	jalr	1672(ra) # 10d42 <_ZN5alloc5alloc18handle_alloc_error17h48d9534aaf1ab1f8E>
	...

00000000000106c4 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h64d226d320fa66bfE>:
   106c4:	1141                	add	sp,sp,-16
   106c6:	e406                	sd	ra,8(sp)
   106c8:	e022                	sd	s0,0(sp)
   106ca:	0800                	add	s0,sp,16
   106cc:	6108                	ld	a0,0(a0)
   106ce:	60a2                	ld	ra,8(sp)
   106d0:	6402                	ld	s0,0(sp)
   106d2:	0141                	add	sp,sp,16
   106d4:	00001317          	auipc	t1,0x1
   106d8:	f3430067          	jr	-204(t1) # 11608 <_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17h4d3bee89ff077165E>

00000000000106dc <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hd4b1075b6d8c8353E>:
   106dc:	1141                	add	sp,sp,-16
   106de:	e406                	sd	ra,8(sp)
   106e0:	e022                	sd	s0,0(sp)
   106e2:	0800                	add	s0,sp,16
   106e4:	6110                	ld	a2,0(a0)
   106e6:	6514                	ld	a3,8(a0)
   106e8:	872e                	mv	a4,a1
   106ea:	8532                	mv	a0,a2
   106ec:	85b6                	mv	a1,a3
   106ee:	863a                	mv	a2,a4
   106f0:	60a2                	ld	ra,8(sp)
   106f2:	6402                	ld	s0,0(sp)
   106f4:	0141                	add	sp,sp,16
   106f6:	00001317          	auipc	t1,0x1
   106fa:	5fc30067          	jr	1532(t1) # 11cf2 <_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hd677dbeaf8ccd015E>

00000000000106fe <_ZN8user_lib7syscall8sys_exit17h641f41ab1e635210E>:
   106fe:	1141                	add	sp,sp,-16
   10700:	e406                	sd	ra,8(sp)
   10702:	e022                	sd	s0,0(sp)
   10704:	0800                	add	s0,sp,16
   10706:	2501                	sext.w	a0,a0
   10708:	05d00893          	li	a7,93
   1070c:	4581                	li	a1,0
   1070e:	4601                	li	a2,0
   10710:	00000073          	ecall

0000000000010714 <.LBB8_1>:
   10714:	00004517          	auipc	a0,0x4
   10718:	af450513          	add	a0,a0,-1292 # 14208 <anon.c95dffcbf20f28c4475d7f1508a8896b.0.llvm.11064437996114815796>

000000000001071c <.LBB8_2>:
   1071c:	00004617          	auipc	a2,0x4
   10720:	b1460613          	add	a2,a2,-1260 # 14230 <anon.c95dffcbf20f28c4475d7f1508a8896b.2.llvm.11064437996114815796>
   10724:	45dd                	li	a1,23
   10726:	00000097          	auipc	ra,0x0
   1072a:	722080e7          	jalr	1826(ra) # 10e48 <_ZN4core9panicking5panic17h92f54f473578363dE>
	...

0000000000010730 <_ZN22buddy_system_allocator4Heap4init17h0387239a34ee507aE>:
   10730:	1141                	add	sp,sp,-16
   10732:	e406                	sd	ra,8(sp)
   10734:	e022                	sd	s0,0(sp)
   10736:	0800                	add	s0,sp,16
   10738:	962e                	add	a2,a2,a1
   1073a:	059d                	add	a1,a1,7
   1073c:	99e1                	and	a1,a1,-8
   1073e:	ff867e93          	and	t4,a2,-8
   10742:	12beea63          	bltu	t4,a1,10876 <.LBB4_20>
   10746:	4701                	li	a4,0
   10748:	00858613          	add	a2,a1,8
   1074c:	10cee063          	bltu	t4,a2,1084c <.LBB4_18+0xd8>

0000000000010750 <.LBB4_15>:
   10750:	00006617          	auipc	a2,0x6
   10754:	8d060613          	add	a2,a2,-1840 # 16020 <.LCPI4_0>
   10758:	00063803          	ld	a6,0(a2)

000000000001075c <.LBB4_16>:
   1075c:	00006617          	auipc	a2,0x6
   10760:	8cc60613          	add	a2,a2,-1844 # 16028 <.LCPI4_1>
   10764:	00063f03          	ld	t5,0(a2)

0000000000010768 <.LBB4_17>:
   10768:	00006617          	auipc	a2,0x6
   1076c:	8c860613          	add	a2,a2,-1848 # 16030 <.LCPI4_2>
   10770:	00063883          	ld	a7,0(a2)

0000000000010774 <.LBB4_18>:
   10774:	00006617          	auipc	a2,0x6
   10778:	8c460613          	add	a2,a2,-1852 # 16038 <.LCPI4_3>
   1077c:	00063283          	ld	t0,0(a2)
   10780:	03f00313          	li	t1,63
   10784:	4385                	li	t2,1
   10786:	4e7d                	li	t3,31
   10788:	40be8633          	sub	a2,t4,a1
   1078c:	ca29                	beqz	a2,107de <.LBB4_18+0x6a>
   1078e:	00165693          	srl	a3,a2,0x1
   10792:	8e55                	or	a2,a2,a3
   10794:	00265693          	srl	a3,a2,0x2
   10798:	8e55                	or	a2,a2,a3
   1079a:	00465693          	srl	a3,a2,0x4
   1079e:	8e55                	or	a2,a2,a3
   107a0:	00865693          	srl	a3,a2,0x8
   107a4:	8e55                	or	a2,a2,a3
   107a6:	01065693          	srl	a3,a2,0x10
   107aa:	8e55                	or	a2,a2,a3
   107ac:	02065693          	srl	a3,a2,0x20
   107b0:	8e55                	or	a2,a2,a3
   107b2:	fff64613          	not	a2,a2
   107b6:	00165693          	srl	a3,a2,0x1
   107ba:	0106f6b3          	and	a3,a3,a6
   107be:	8e15                	sub	a2,a2,a3
   107c0:	01e676b3          	and	a3,a2,t5
   107c4:	8209                	srl	a2,a2,0x2
   107c6:	01e67633          	and	a2,a2,t5
   107ca:	9636                	add	a2,a2,a3
   107cc:	00465693          	srl	a3,a2,0x4
   107d0:	9636                	add	a2,a2,a3
   107d2:	01167633          	and	a2,a2,a7
   107d6:	02560633          	mul	a2,a2,t0
   107da:	9261                	srl	a2,a2,0x38
   107dc:	a019                	j	107e2 <.LBB4_18+0x6e>
   107de:	04000613          	li	a2,64
   107e2:	40b006b3          	neg	a3,a1
   107e6:	8eed                	and	a3,a3,a1
   107e8:	40c30633          	sub	a2,t1,a2
   107ec:	00c39633          	sll	a2,t2,a2
   107f0:	00d66363          	bltu	a2,a3,107f6 <.LBB4_18+0x82>
   107f4:	8636                	mv	a2,a3
   107f6:	ce05                	beqz	a2,1082e <.LBB4_18+0xba>
   107f8:	fff60693          	add	a3,a2,-1
   107fc:	fff64793          	not	a5,a2
   10800:	8efd                	and	a3,a3,a5
   10802:	0016d793          	srl	a5,a3,0x1
   10806:	0107f7b3          	and	a5,a5,a6
   1080a:	8e9d                	sub	a3,a3,a5
   1080c:	01e6f7b3          	and	a5,a3,t5
   10810:	8289                	srl	a3,a3,0x2
   10812:	01e6f6b3          	and	a3,a3,t5
   10816:	96be                	add	a3,a3,a5
   10818:	0046d793          	srl	a5,a3,0x4
   1081c:	96be                	add	a3,a3,a5
   1081e:	0116f6b3          	and	a3,a3,a7
   10822:	025686b3          	mul	a3,a3,t0
   10826:	92e1                	srl	a3,a3,0x38
   10828:	00de7763          	bgeu	t3,a3,10836 <.LBB4_18+0xc2>
   1082c:	a80d                	j	1085e <.LBB4_19>
   1082e:	04000693          	li	a3,64
   10832:	02de6663          	bltu	t3,a3,1085e <.LBB4_19>
   10836:	068e                	sll	a3,a3,0x3
   10838:	96aa                	add	a3,a3,a0
   1083a:	629c                	ld	a5,0(a3)
   1083c:	e19c                	sd	a5,0(a1)
   1083e:	e28c                	sd	a1,0(a3)
   10840:	95b2                	add	a1,a1,a2
   10842:	00858693          	add	a3,a1,8
   10846:	9732                	add	a4,a4,a2
   10848:	f4def0e3          	bgeu	t4,a3,10788 <.LBB4_18+0x14>
   1084c:	11053583          	ld	a1,272(a0)
   10850:	95ba                	add	a1,a1,a4
   10852:	10b53823          	sd	a1,272(a0)
   10856:	60a2                	ld	ra,8(sp)
   10858:	6402                	ld	s0,0(sp)
   1085a:	0141                	add	sp,sp,16
   1085c:	8082                	ret

000000000001085e <.LBB4_19>:
   1085e:	00004617          	auipc	a2,0x4
   10862:	a9a60613          	add	a2,a2,-1382 # 142f8 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.3>
   10866:	02000593          	li	a1,32
   1086a:	8536                	mv	a0,a3
   1086c:	00000097          	auipc	ra,0x0
   10870:	650080e7          	jalr	1616(ra) # 10ebc <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>
	...

0000000000010876 <.LBB4_20>:
   10876:	00004517          	auipc	a0,0x4
   1087a:	9d250513          	add	a0,a0,-1582 # 14248 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.0>

000000000001087e <.LBB4_21>:
   1087e:	00004617          	auipc	a2,0x4
   10882:	a6260613          	add	a2,a2,-1438 # 142e0 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.2>
   10886:	45f9                	li	a1,30
   10888:	00000097          	auipc	ra,0x0
   1088c:	5c0080e7          	jalr	1472(ra) # 10e48 <_ZN4core9panicking5panic17h92f54f473578363dE>
	...

0000000000010892 <_ZN22buddy_system_allocator4Heap5alloc17hcac490460a84cc76E>:
   10892:	1141                	add	sp,sp,-16
   10894:	e406                	sd	ra,8(sp)
   10896:	e022                	sd	s0,0(sp)
   10898:	0800                	add	s0,sp,16
   1089a:	4285                	li	t0,1
   1089c:	4709                	li	a4,2

000000000001089e <.LBB5_28>:
   1089e:	00005397          	auipc	t2,0x5
   108a2:	7a238393          	add	t2,t2,1954 # 16040 <.LCPI5_0>

00000000000108a6 <.LBB5_29>:
   108a6:	00005317          	auipc	t1,0x5
   108aa:	7a230313          	add	t1,t1,1954 # 16048 <.LCPI5_1>

00000000000108ae <.LBB5_30>:
   108ae:	00005897          	auipc	a7,0x5
   108b2:	7a288893          	add	a7,a7,1954 # 16050 <.LCPI5_2>

00000000000108b6 <.LBB5_31>:
   108b6:	00005817          	auipc	a6,0x5
   108ba:	7a280813          	add	a6,a6,1954 # 16058 <.LCPI5_3>
   108be:	0ee5f963          	bgeu	a1,a4,109b0 <.LBB5_31+0xfa>
   108c2:	46a1                	li	a3,8
   108c4:	14c6fe63          	bgeu	a3,a2,10a20 <.LBB5_31+0x16a>
   108c8:	14567f63          	bgeu	a2,t0,10a26 <.LBB5_31+0x170>
   108cc:	16028063          	beqz	t0,10a2c <.LBB5_31+0x176>
   108d0:	fff28613          	add	a2,t0,-1
   108d4:	fff2c693          	not	a3,t0
   108d8:	0003b703          	ld	a4,0(t2)
   108dc:	8e75                	and	a2,a2,a3
   108de:	00033683          	ld	a3,0(t1)
   108e2:	00165793          	srl	a5,a2,0x1
   108e6:	8f7d                	and	a4,a4,a5
   108e8:	8e19                	sub	a2,a2,a4
   108ea:	00d67733          	and	a4,a2,a3
   108ee:	8209                	srl	a2,a2,0x2
   108f0:	8e75                	and	a2,a2,a3
   108f2:	963a                	add	a2,a2,a4
   108f4:	0008b683          	ld	a3,0(a7)
   108f8:	00083703          	ld	a4,0(a6)
   108fc:	00465793          	srl	a5,a2,0x4
   10900:	963e                	add	a2,a2,a5
   10902:	8e75                	and	a2,a2,a3
   10904:	02e60633          	mul	a2,a2,a4
   10908:	03865893          	srl	a7,a2,0x38
   1090c:	02000613          	li	a2,32
   10910:	8746                	mv	a4,a7
   10912:	01166463          	bltu	a2,a7,1091a <.LBB5_31+0x64>
   10916:	02000713          	li	a4,32
   1091a:	00389813          	sll	a6,a7,0x3
   1091e:	00a80633          	add	a2,a6,a0
   10922:	ff060793          	add	a5,a2,-16
   10926:	86c6                	mv	a3,a7
   10928:	06d70e63          	beq	a4,a3,109a4 <.LBB5_31+0xee>
   1092c:	6b90                	ld	a2,16(a5)
   1092e:	0685                	add	a3,a3,1
   10930:	07a1                	add	a5,a5,8
   10932:	da7d                	beqz	a2,10928 <.LBB5_31+0x72>
   10934:	fff68713          	add	a4,a3,-1
   10938:	04e8f063          	bgeu	a7,a4,10978 <.LBB5_31+0xc2>
   1093c:	00188e93          	add	t4,a7,1
   10940:	02000313          	li	t1,32
   10944:	43fd                	li	t2,31
   10946:	4e05                	li	t3,1
   10948:	fff68713          	add	a4,a3,-1
   1094c:	0e677863          	bgeu	a4,t1,10a3c <.LBB5_31+0x186>
   10950:	00063f03          	ld	t5,0(a2)
   10954:	16f9                	add	a3,a3,-2
   10956:	01e7b423          	sd	t5,8(a5)
   1095a:	0ed3e863          	bltu	t2,a3,10a4a <.LBB5_33>
   1095e:	0007bf03          	ld	t5,0(a5)
   10962:	00de16b3          	sll	a3,t3,a3
   10966:	96b2                	add	a3,a3,a2
   10968:	01e6b023          	sd	t5,0(a3)
   1096c:	e214                	sd	a3,0(a2)
   1096e:	e390                	sd	a2,0(a5)
   10970:	17e1                	add	a5,a5,-8
   10972:	86ba                	mv	a3,a4
   10974:	fceeeae3          	bltu	t4,a4,10948 <.LBB5_31+0x92>
   10978:	02000613          	li	a2,32
   1097c:	0ec8f363          	bgeu	a7,a2,10a62 <.LBB5_34>
   10980:	010506b3          	add	a3,a0,a6
   10984:	6290                	ld	a2,0(a3)
   10986:	ca75                	beqz	a2,10a7a <.LBB5_35>
   10988:	6218                	ld	a4,0(a2)
   1098a:	e298                	sd	a4,0(a3)
   1098c:	10053683          	ld	a3,256(a0)
   10990:	10853703          	ld	a4,264(a0)
   10994:	95b6                	add	a1,a1,a3
   10996:	10b53023          	sd	a1,256(a0)
   1099a:	005705b3          	add	a1,a4,t0
   1099e:	10b53423          	sd	a1,264(a0)
   109a2:	a011                	j	109a6 <.LBB5_31+0xf0>
   109a4:	4601                	li	a2,0
   109a6:	8532                	mv	a0,a2
   109a8:	60a2                	ld	ra,8(sp)
   109aa:	6402                	ld	s0,0(sp)
   109ac:	0141                	add	sp,sp,16
   109ae:	8082                	ret
   109b0:	fff58713          	add	a4,a1,-1
   109b4:	00175693          	srl	a3,a4,0x1
   109b8:	8ed9                	or	a3,a3,a4
   109ba:	0026d713          	srl	a4,a3,0x2
   109be:	8ed9                	or	a3,a3,a4
   109c0:	0046d713          	srl	a4,a3,0x4
   109c4:	8ed9                	or	a3,a3,a4
   109c6:	0086d713          	srl	a4,a3,0x8
   109ca:	8ed9                	or	a3,a3,a4
   109cc:	0106d713          	srl	a4,a3,0x10
   109d0:	8ed9                	or	a3,a3,a4
   109d2:	0206d713          	srl	a4,a3,0x20
   109d6:	8ed9                	or	a3,a3,a4
   109d8:	0003b283          	ld	t0,0(t2)
   109dc:	fff6c693          	not	a3,a3
   109e0:	00033783          	ld	a5,0(t1)
   109e4:	0016d713          	srl	a4,a3,0x1
   109e8:	00577733          	and	a4,a4,t0
   109ec:	8e99                	sub	a3,a3,a4
   109ee:	00f6f733          	and	a4,a3,a5
   109f2:	8289                	srl	a3,a3,0x2
   109f4:	8efd                	and	a3,a3,a5
   109f6:	96ba                	add	a3,a3,a4
   109f8:	0008b283          	ld	t0,0(a7)
   109fc:	00083783          	ld	a5,0(a6)
   10a00:	0046d713          	srl	a4,a3,0x4
   10a04:	96ba                	add	a3,a3,a4
   10a06:	0056f6b3          	and	a3,a3,t0
   10a0a:	02f686b3          	mul	a3,a3,a5
   10a0e:	92e1                	srl	a3,a3,0x38
   10a10:	577d                	li	a4,-1
   10a12:	00d756b3          	srl	a3,a4,a3
   10a16:	00168293          	add	t0,a3,1
   10a1a:	46a1                	li	a3,8
   10a1c:	eac6e6e3          	bltu	a3,a2,108c8 <.LBB5_31+0x12>
   10a20:	4621                	li	a2,8
   10a22:	ea5665e3          	bltu	a2,t0,108cc <.LBB5_31+0x16>
   10a26:	82b2                	mv	t0,a2
   10a28:	ea0294e3          	bnez	t0,108d0 <.LBB5_31+0x1a>
   10a2c:	04000893          	li	a7,64
   10a30:	02000613          	li	a2,32
   10a34:	8746                	mv	a4,a7
   10a36:	ef1670e3          	bgeu	a2,a7,10916 <.LBB5_31+0x60>
   10a3a:	b5c5                	j	1091a <.LBB5_31+0x64>
   10a3c:	fff68513          	add	a0,a3,-1

0000000000010a40 <.LBB5_32>:
   10a40:	00004617          	auipc	a2,0x4
   10a44:	8d060613          	add	a2,a2,-1840 # 14310 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.4>
   10a48:	a031                	j	10a54 <.LBB5_33+0xa>

0000000000010a4a <.LBB5_33>:
   10a4a:	00004617          	auipc	a2,0x4
   10a4e:	8de60613          	add	a2,a2,-1826 # 14328 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.5>
   10a52:	557d                	li	a0,-1
   10a54:	02000593          	li	a1,32
   10a58:	00000097          	auipc	ra,0x0
   10a5c:	464080e7          	jalr	1124(ra) # 10ebc <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>
	...

0000000000010a62 <.LBB5_34>:
   10a62:	00004617          	auipc	a2,0x4
   10a66:	8de60613          	add	a2,a2,-1826 # 14340 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.6>
   10a6a:	02000593          	li	a1,32
   10a6e:	8546                	mv	a0,a7
   10a70:	00000097          	auipc	ra,0x0
   10a74:	44c080e7          	jalr	1100(ra) # 10ebc <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>
	...

0000000000010a7a <.LBB5_35>:
   10a7a:	00004517          	auipc	a0,0x4
   10a7e:	8de50513          	add	a0,a0,-1826 # 14358 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.7>

0000000000010a82 <.LBB5_36>:
   10a82:	00004617          	auipc	a2,0x4
   10a86:	8be60613          	add	a2,a2,-1858 # 14340 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.6>
   10a8a:	02800593          	li	a1,40
   10a8e:	00000097          	auipc	ra,0x0
   10a92:	3a4080e7          	jalr	932(ra) # 10e32 <_ZN4core6option13expect_failed17h35a4d77224f066bbE>
	...

0000000000010a98 <_ZN22buddy_system_allocator4Heap7dealloc17h73addb8809d46fa4E>:
   10a98:	1141                	add	sp,sp,-16
   10a9a:	e406                	sd	ra,8(sp)
   10a9c:	e022                	sd	s0,0(sp)
   10a9e:	0800                	add	s0,sp,16
   10aa0:	4305                	li	t1,1
   10aa2:	4709                	li	a4,2

0000000000010aa4 <.LBB6_22>:
   10aa4:	00005397          	auipc	t2,0x5
   10aa8:	5bc38393          	add	t2,t2,1468 # 16060 <.LCPI6_0>

0000000000010aac <.LBB6_23>:
   10aac:	00005297          	auipc	t0,0x5
   10ab0:	5bc28293          	add	t0,t0,1468 # 16068 <.LCPI6_1>

0000000000010ab4 <.LBB6_24>:
   10ab4:	00005897          	auipc	a7,0x5
   10ab8:	5bc88893          	add	a7,a7,1468 # 16070 <.LCPI6_2>

0000000000010abc <.LBB6_25>:
   10abc:	00005817          	auipc	a6,0x5
   10ac0:	5bc80813          	add	a6,a6,1468 # 16078 <.LCPI6_3>
   10ac4:	0ce67b63          	bgeu	a2,a4,10b9a <.LBB6_25+0xde>
   10ac8:	4721                	li	a4,8
   10aca:	14d77163          	bgeu	a4,a3,10c0c <.LBB6_25+0x150>
   10ace:	1466f263          	bgeu	a3,t1,10c12 <.LBB6_25+0x156>
   10ad2:	14030363          	beqz	t1,10c18 <.LBB6_25+0x15c>
   10ad6:	fff30693          	add	a3,t1,-1
   10ada:	fff34713          	not	a4,t1
   10ade:	0003b383          	ld	t2,0(t2)
   10ae2:	8ef9                	and	a3,a3,a4
   10ae4:	0002b703          	ld	a4,0(t0)
   10ae8:	0016d793          	srl	a5,a3,0x1
   10aec:	0077f7b3          	and	a5,a5,t2
   10af0:	8e9d                	sub	a3,a3,a5
   10af2:	00e6f7b3          	and	a5,a3,a4
   10af6:	8289                	srl	a3,a3,0x2
   10af8:	8ef9                	and	a3,a3,a4
   10afa:	96be                	add	a3,a3,a5
   10afc:	0008b883          	ld	a7,0(a7)
   10b00:	00083783          	ld	a5,0(a6)
   10b04:	0046d713          	srl	a4,a3,0x4
   10b08:	96ba                	add	a3,a3,a4
   10b0a:	0116f6b3          	and	a3,a3,a7
   10b0e:	02f686b3          	mul	a3,a3,a5
   10b12:	0386d393          	srl	t2,a3,0x38
   10b16:	487d                	li	a6,31
   10b18:	10786563          	bltu	a6,t2,10c22 <.LBB6_26>
   10b1c:	00339693          	sll	a3,t2,0x3
   10b20:	96aa                	add	a3,a3,a0
   10b22:	6298                	ld	a4,0(a3)
   10b24:	e198                	sd	a4,0(a1)
   10b26:	e28c                	sd	a1,0(a3)
   10b28:	4885                	li	a7,1
   10b2a:	82ae                	mv	t0,a1
   10b2c:	007896b3          	sll	a3,a7,t2
   10b30:	00339713          	sll	a4,t2,0x3
   10b34:	00e50e33          	add	t3,a0,a4
   10b38:	00d2ceb3          	xor	t4,t0,a3
   10b3c:	86f2                	mv	a3,t3
   10b3e:	cd9d                	beqz	a1,10b7c <.LBB6_25+0xc0>
   10b40:	872e                	mv	a4,a1
   10b42:	87b6                	mv	a5,a3
   10b44:	618c                	ld	a1,0(a1)
   10b46:	86ba                	mv	a3,a4
   10b48:	feee9be3          	bne	t4,a4,10b3e <.LBB6_25+0x82>
   10b4c:	e38c                	sd	a1,0(a5)
   10b4e:	000e3583          	ld	a1,0(t3)
   10b52:	c581                	beqz	a1,10b5a <.LBB6_25+0x9e>
   10b54:	618c                	ld	a1,0(a1)
   10b56:	00be3023          	sd	a1,0(t3)
   10b5a:	005ee363          	bltu	t4,t0,10b60 <.LBB6_25+0xa4>
   10b5e:	8e96                	mv	t4,t0
   10b60:	0d038d63          	beq	t2,a6,10c3a <.LBB6_27>
   10b64:	0385                	add	t2,t2,1
   10b66:	00339593          	sll	a1,t2,0x3
   10b6a:	95aa                	add	a1,a1,a0
   10b6c:	6194                	ld	a3,0(a1)
   10b6e:	00deb023          	sd	a3,0(t4)
   10b72:	01d5b023          	sd	t4,0(a1)
   10b76:	85f6                	mv	a1,t4
   10b78:	82f6                	mv	t0,t4
   10b7a:	bf4d                	j	10b2c <.LBB6_25+0x70>
   10b7c:	10053583          	ld	a1,256(a0)
   10b80:	10853683          	ld	a3,264(a0)
   10b84:	8d91                	sub	a1,a1,a2
   10b86:	10b53023          	sd	a1,256(a0)
   10b8a:	406685b3          	sub	a1,a3,t1
   10b8e:	10b53423          	sd	a1,264(a0)
   10b92:	60a2                	ld	ra,8(sp)
   10b94:	6402                	ld	s0,0(sp)
   10b96:	0141                	add	sp,sp,16
   10b98:	8082                	ret
   10b9a:	fff60713          	add	a4,a2,-1
   10b9e:	00175793          	srl	a5,a4,0x1
   10ba2:	8f5d                	or	a4,a4,a5
   10ba4:	00275793          	srl	a5,a4,0x2
   10ba8:	8f5d                	or	a4,a4,a5
   10baa:	00475793          	srl	a5,a4,0x4
   10bae:	8f5d                	or	a4,a4,a5
   10bb0:	00875793          	srl	a5,a4,0x8
   10bb4:	8f5d                	or	a4,a4,a5
   10bb6:	01075793          	srl	a5,a4,0x10
   10bba:	8f5d                	or	a4,a4,a5
   10bbc:	02075793          	srl	a5,a4,0x20
   10bc0:	8f5d                	or	a4,a4,a5
   10bc2:	0003b303          	ld	t1,0(t2)
   10bc6:	fff74e13          	not	t3,a4
   10bca:	0002b783          	ld	a5,0(t0)
   10bce:	001e5713          	srl	a4,t3,0x1
   10bd2:	00677733          	and	a4,a4,t1
   10bd6:	40ee0733          	sub	a4,t3,a4
   10bda:	00f77333          	and	t1,a4,a5
   10bde:	8309                	srl	a4,a4,0x2
   10be0:	8f7d                	and	a4,a4,a5
   10be2:	971a                	add	a4,a4,t1
   10be4:	0008b303          	ld	t1,0(a7)
   10be8:	00083e03          	ld	t3,0(a6)
   10bec:	00475793          	srl	a5,a4,0x4
   10bf0:	973e                	add	a4,a4,a5
   10bf2:	00677733          	and	a4,a4,t1
   10bf6:	03c70733          	mul	a4,a4,t3
   10bfa:	9361                	srl	a4,a4,0x38
   10bfc:	57fd                	li	a5,-1
   10bfe:	00e7d733          	srl	a4,a5,a4
   10c02:	00170313          	add	t1,a4,1
   10c06:	4721                	li	a4,8
   10c08:	ecd763e3          	bltu	a4,a3,10ace <.LBB6_25+0x12>
   10c0c:	46a1                	li	a3,8
   10c0e:	ec66e2e3          	bltu	a3,t1,10ad2 <.LBB6_25+0x16>
   10c12:	8336                	mv	t1,a3
   10c14:	ec0311e3          	bnez	t1,10ad6 <.LBB6_25+0x1a>
   10c18:	04000393          	li	t2,64
   10c1c:	487d                	li	a6,31
   10c1e:	ee787fe3          	bgeu	a6,t2,10b1c <.LBB6_25+0x60>

0000000000010c22 <.LBB6_26>:
   10c22:	00003617          	auipc	a2,0x3
   10c26:	75e60613          	add	a2,a2,1886 # 14380 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.8>
   10c2a:	02000593          	li	a1,32
   10c2e:	851e                	mv	a0,t2
   10c30:	00000097          	auipc	ra,0x0
   10c34:	28c080e7          	jalr	652(ra) # 10ebc <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>
	...

0000000000010c3a <.LBB6_27>:
   10c3a:	00003617          	auipc	a2,0x3
   10c3e:	75e60613          	add	a2,a2,1886 # 14398 <.Lanon.aadca89046bacb5d28290fdcc9b6f616.9>
   10c42:	02000513          	li	a0,32
   10c46:	02000593          	li	a1,32
   10c4a:	00000097          	auipc	ra,0x0
   10c4e:	272080e7          	jalr	626(ra) # 10ebc <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>
	...

0000000000010c54 <_ZN78_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..ops..deref..Deref$GT$5deref17h07e5175ddf4fbb7aE>:
   10c54:	1141                	add	sp,sp,-16
   10c56:	e406                	sd	ra,8(sp)
   10c58:	e022                	sd	s0,0(sp)
   10c5a:	0800                	add	s0,sp,16
   10c5c:	60a2                	ld	ra,8(sp)
   10c5e:	6402                	ld	s0,0(sp)
   10c60:	0141                	add	sp,sp,16
   10c62:	8082                	ret

0000000000010c64 <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h6574ad5d6e524a4aE>:
   10c64:	1101                	add	sp,sp,-32
   10c66:	ec06                	sd	ra,24(sp)
   10c68:	e822                	sd	s0,16(sp)
   10c6a:	e426                	sd	s1,8(sp)
   10c6c:	e04a                	sd	s2,0(sp)
   10c6e:	1000                	add	s0,sp,32
   10c70:	84aa                	mv	s1,a0
   10c72:	4505                	li	a0,1
   10c74:	00a4b92f          	amoadd.d	s2,a0,(s1)
   10c78:	6488                	ld	a0,8(s1)
   10c7a:	0230000f          	fence	r,rw
   10c7e:	01250963          	beq	a0,s2,10c90 <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h6574ad5d6e524a4aE+0x2c>
   10c82:	0100000f          	fence	w,unknown
   10c86:	6488                	ld	a0,8(s1)
   10c88:	0230000f          	fence	r,rw
   10c8c:	ff251be3          	bne	a0,s2,10c82 <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h6574ad5d6e524a4aE+0x1e>
   10c90:	01048513          	add	a0,s1,16
   10c94:	00000097          	auipc	ra,0x0
   10c98:	bfe080e7          	jalr	-1026(ra) # 10892 <_ZN22buddy_system_allocator4Heap5alloc17hcac490460a84cc76E>
   10c9c:	00190593          	add	a1,s2,1
   10ca0:	0310000f          	fence	rw,w
   10ca4:	e48c                	sd	a1,8(s1)
   10ca6:	60e2                	ld	ra,24(sp)
   10ca8:	6442                	ld	s0,16(sp)
   10caa:	64a2                	ld	s1,8(sp)
   10cac:	6902                	ld	s2,0(sp)
   10cae:	6105                	add	sp,sp,32
   10cb0:	8082                	ret

0000000000010cb2 <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hbd168ca56515a3a0E>:
   10cb2:	1101                	add	sp,sp,-32
   10cb4:	ec06                	sd	ra,24(sp)
   10cb6:	e822                	sd	s0,16(sp)
   10cb8:	e426                	sd	s1,8(sp)
   10cba:	e04a                	sd	s2,0(sp)
   10cbc:	1000                	add	s0,sp,32
   10cbe:	84aa                	mv	s1,a0
   10cc0:	4505                	li	a0,1
   10cc2:	00a4b92f          	amoadd.d	s2,a0,(s1)
   10cc6:	6488                	ld	a0,8(s1)
   10cc8:	0230000f          	fence	r,rw
   10ccc:	01250963          	beq	a0,s2,10cde <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hbd168ca56515a3a0E+0x2c>
   10cd0:	0100000f          	fence	w,unknown
   10cd4:	6488                	ld	a0,8(s1)
   10cd6:	0230000f          	fence	r,rw
   10cda:	ff251be3          	bne	a0,s2,10cd0 <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hbd168ca56515a3a0E+0x1e>
   10cde:	01048513          	add	a0,s1,16
   10ce2:	00000097          	auipc	ra,0x0
   10ce6:	db6080e7          	jalr	-586(ra) # 10a98 <_ZN22buddy_system_allocator4Heap7dealloc17h73addb8809d46fa4E>
   10cea:	00190513          	add	a0,s2,1
   10cee:	0310000f          	fence	rw,w
   10cf2:	e488                	sd	a0,8(s1)
   10cf4:	60e2                	ld	ra,24(sp)
   10cf6:	6442                	ld	s0,16(sp)
   10cf8:	64a2                	ld	s1,8(sp)
   10cfa:	6902                	ld	s2,0(sp)
   10cfc:	6105                	add	sp,sp,32
   10cfe:	8082                	ret

0000000000010d00 <_ZN4core3ops8function6FnOnce9call_once17h6ee4dec5eed49e53E>:
   10d00:	1141                	add	sp,sp,-16
   10d02:	e406                	sd	ra,8(sp)
   10d04:	00000097          	auipc	ra,0x0
   10d08:	04c080e7          	jalr	76(ra) # 10d50 <_ZN5alloc5alloc18handle_alloc_error8rt_error17h3e76e7b1bd103949E>
	...

0000000000010d0e <_ZN5alloc7raw_vec17capacity_overflow17h8c103c8c1cb34845E>:
   10d0e:	7139                	add	sp,sp,-64
   10d10:	fc06                	sd	ra,56(sp)

0000000000010d12 <.LBB18_1>:
   10d12:	00003517          	auipc	a0,0x3
   10d16:	6ce50513          	add	a0,a0,1742 # 143e0 <.Lanon.bc962af17d66e1a6bfbd1e5dd004f443.6>
   10d1a:	e42a                	sd	a0,8(sp)
   10d1c:	4505                	li	a0,1
   10d1e:	e82a                	sd	a0,16(sp)
   10d20:	ec02                	sd	zero,24(sp)

0000000000010d22 <.LBB18_2>:
   10d22:	00003517          	auipc	a0,0x3
   10d26:	68e50513          	add	a0,a0,1678 # 143b0 <.Lanon.bc962af17d66e1a6bfbd1e5dd004f443.4>
   10d2a:	f42a                	sd	a0,40(sp)
   10d2c:	f802                	sd	zero,48(sp)

0000000000010d2e <.LBB18_3>:
   10d2e:	00003597          	auipc	a1,0x3
   10d32:	6c258593          	add	a1,a1,1730 # 143f0 <.Lanon.bc962af17d66e1a6bfbd1e5dd004f443.7>
   10d36:	0028                	add	a0,sp,8
   10d38:	00000097          	auipc	ra,0x0
   10d3c:	1c4080e7          	jalr	452(ra) # 10efc <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

0000000000010d42 <_ZN5alloc5alloc18handle_alloc_error17h48d9534aaf1ab1f8E>:
   10d42:	1141                	add	sp,sp,-16
   10d44:	e406                	sd	ra,8(sp)
   10d46:	00000097          	auipc	ra,0x0
   10d4a:	fba080e7          	jalr	-70(ra) # 10d00 <_ZN4core3ops8function6FnOnce9call_once17h6ee4dec5eed49e53E>
	...

0000000000010d50 <_ZN5alloc5alloc18handle_alloc_error8rt_error17h3e76e7b1bd103949E>:
   10d50:	1141                	add	sp,sp,-16
   10d52:	e406                	sd	ra,8(sp)
   10d54:	fffff097          	auipc	ra,0xfffff
   10d58:	4f4080e7          	jalr	1268(ra) # 10248 <__rust_alloc_error_handler>
	...

0000000000010d5e <__rg_oom>:
   10d5e:	1141                	add	sp,sp,-16
   10d60:	e406                	sd	ra,8(sp)
   10d62:	fffff097          	auipc	ra,0xfffff
   10d66:	6c4080e7          	jalr	1732(ra) # 10426 <rust_oom>
	...

0000000000010d6c <_ZN4core3ops8function6FnOnce9call_once17h0008a32bd325903dE>:
   10d6c:	1141                	add	sp,sp,-16
   10d6e:	e406                	sd	ra,8(sp)
   10d70:	00001097          	auipc	ra,0x1
   10d74:	172080e7          	jalr	370(ra) # 11ee2 <_ZN4core5slice5index29slice_start_index_len_fail_rt17h66247b7e841f83e5E>
	...

0000000000010d7a <_ZN4core3ops8function6FnOnce9call_once17h0ccd98de653a7264E>:
   10d7a:	1141                	add	sp,sp,-16
   10d7c:	e406                	sd	ra,8(sp)
   10d7e:	00001097          	auipc	ra,0x1
   10d82:	20c080e7          	jalr	524(ra) # 11f8a <_ZN4core5slice5index25slice_index_order_fail_rt17h814668a4a9208686E>
	...

0000000000010d88 <_ZN4core3ops8function6FnOnce9call_once17h6b85840bc58c33c1E>:
   10d88:	6108                	ld	a0,0(a0)
   10d8a:	a001                	j	10d8a <_ZN4core3ops8function6FnOnce9call_once17h6b85840bc58c33c1E+0x2>

0000000000010d8c <_ZN4core3ops8function6FnOnce9call_once17had1f8e39903f1947E>:
   10d8c:	1141                	add	sp,sp,-16
   10d8e:	e406                	sd	ra,8(sp)
   10d90:	00001097          	auipc	ra,0x1
   10d94:	1a6080e7          	jalr	422(ra) # 11f36 <_ZN4core5slice5index27slice_end_index_len_fail_rt17h3a149a007ccdb3bbE>
	...

0000000000010d9a <_ZN4core3ops8function6FnOnce9call_once17hbd59230e70bde5e6E>:
   10d9a:	1141                	add	sp,sp,-16
   10d9c:	e406                	sd	ra,8(sp)
   10d9e:	00001097          	auipc	ra,0x1
   10da2:	5f4080e7          	jalr	1524(ra) # 12392 <_ZN4core3str19slice_error_fail_rt17hcb246852ed3ab8e1E>
	...

0000000000010da8 <_ZN4core3ptr102drop_in_place$LT$$RF$core..iter..adapters..copied..Copied$LT$core..slice..iter..Iter$LT$u8$GT$$GT$$GT$17hea562d0102c22270E>:
   10da8:	8082                	ret

0000000000010daa <_ZN4core10intrinsics17const_eval_select17h4d2f7b41c60bf971E>:
   10daa:	1141                	add	sp,sp,-16
   10dac:	e406                	sd	ra,8(sp)
   10dae:	6118                	ld	a4,0(a0)
   10db0:	650c                	ld	a1,8(a0)
   10db2:	6910                	ld	a2,16(a0)
   10db4:	6d14                	ld	a3,24(a0)
   10db6:	853a                	mv	a0,a4
   10db8:	00000097          	auipc	ra,0x0
   10dbc:	fe2080e7          	jalr	-30(ra) # 10d9a <_ZN4core3ops8function6FnOnce9call_once17hbd59230e70bde5e6E>
	...

0000000000010dc2 <_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h59f5c39c772cdb32E>:
   10dc2:	715d                	add	sp,sp,-80
   10dc4:	e486                	sd	ra,72(sp)
   10dc6:	e0a2                	sd	s0,64(sp)
   10dc8:	fc26                	sd	s1,56(sp)
   10dca:	842e                	mv	s0,a1
   10dcc:	84aa                	mv	s1,a0
   10dce:	00002097          	auipc	ra,0x2
   10dd2:	c90080e7          	jalr	-880(ra) # 12a5e <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E>
   10dd6:	e51d                	bnez	a0,10e04 <.LBB84_5+0x18>
   10dd8:	7008                	ld	a0,32(s0)
   10dda:	740c                	ld	a1,40(s0)

0000000000010ddc <.LBB84_4>:
   10ddc:	00004617          	auipc	a2,0x4
   10de0:	80c60613          	add	a2,a2,-2036 # 145e8 <.Lanon.442aba94db1f841cd37d39ada1516238.140>
   10de4:	e432                	sd	a2,8(sp)
   10de6:	4605                	li	a2,1
   10de8:	e832                	sd	a2,16(sp)
   10dea:	ec02                	sd	zero,24(sp)

0000000000010dec <.LBB84_5>:
   10dec:	00003617          	auipc	a2,0x3
   10df0:	78c60613          	add	a2,a2,1932 # 14578 <.Lanon.442aba94db1f841cd37d39ada1516238.83>
   10df4:	f432                	sd	a2,40(sp)
   10df6:	f802                	sd	zero,48(sp)
   10df8:	0030                	add	a2,sp,8
   10dfa:	00001097          	auipc	ra,0x1
   10dfe:	840080e7          	jalr	-1984(ra) # 1163a <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   10e02:	c519                	beqz	a0,10e10 <.LBB84_5+0x24>
   10e04:	4505                	li	a0,1
   10e06:	60a6                	ld	ra,72(sp)
   10e08:	6406                	ld	s0,64(sp)
   10e0a:	74e2                	ld	s1,56(sp)
   10e0c:	6161                	add	sp,sp,80
   10e0e:	8082                	ret
   10e10:	00848513          	add	a0,s1,8
   10e14:	85a2                	mv	a1,s0
   10e16:	60a6                	ld	ra,72(sp)
   10e18:	6406                	ld	s0,64(sp)
   10e1a:	74e2                	ld	s1,56(sp)
   10e1c:	6161                	add	sp,sp,80
   10e1e:	00002317          	auipc	t1,0x2
   10e22:	c4030067          	jr	-960(t1) # 12a5e <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E>

0000000000010e26 <_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h34db90cd42bdf307E>:
   10e26:	00005517          	auipc	a0,0x5
   10e2a:	36250513          	add	a0,a0,866 # 16188 <.LCPI85_0>
   10e2e:	6108                	ld	a0,0(a0)
   10e30:	8082                	ret

0000000000010e32 <_ZN4core6option13expect_failed17h35a4d77224f066bbE>:
   10e32:	1141                	add	sp,sp,-16
   10e34:	e406                	sd	ra,8(sp)
   10e36:	00000097          	auipc	ra,0x0
   10e3a:	03e080e7          	jalr	62(ra) # 10e74 <_ZN4core9panicking9panic_str17hf10af3bd6a6f7e35E>
	...

0000000000010e40 <_ZN4core5panic10panic_info9PanicInfo7message17h4abe68e22d422758E>:
   10e40:	6908                	ld	a0,16(a0)
   10e42:	8082                	ret

0000000000010e44 <_ZN4core5panic10panic_info9PanicInfo8location17h873d58c3c1958ff8E>:
   10e44:	6d08                	ld	a0,24(a0)
   10e46:	8082                	ret

0000000000010e48 <_ZN4core9panicking5panic17h92f54f473578363dE>:
   10e48:	715d                	add	sp,sp,-80
   10e4a:	e486                	sd	ra,72(sp)
   10e4c:	fc2a                	sd	a0,56(sp)
   10e4e:	e0ae                	sd	a1,64(sp)
   10e50:	1828                	add	a0,sp,56
   10e52:	e42a                	sd	a0,8(sp)
   10e54:	4505                	li	a0,1
   10e56:	e82a                	sd	a0,16(sp)
   10e58:	ec02                	sd	zero,24(sp)

0000000000010e5a <.LBB150_1>:
   10e5a:	00003517          	auipc	a0,0x3
   10e5e:	71e50513          	add	a0,a0,1822 # 14578 <.Lanon.442aba94db1f841cd37d39ada1516238.83>
   10e62:	f42a                	sd	a0,40(sp)
   10e64:	f802                	sd	zero,48(sp)
   10e66:	0028                	add	a0,sp,8
   10e68:	85b2                	mv	a1,a2
   10e6a:	00000097          	auipc	ra,0x0
   10e6e:	092080e7          	jalr	146(ra) # 10efc <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

0000000000010e74 <_ZN4core9panicking9panic_str17hf10af3bd6a6f7e35E>:
   10e74:	1101                	add	sp,sp,-32
   10e76:	ec06                	sd	ra,24(sp)
   10e78:	e42a                	sd	a0,8(sp)
   10e7a:	e82e                	sd	a1,16(sp)
   10e7c:	0028                	add	a0,sp,8
   10e7e:	85b2                	mv	a1,a2
   10e80:	00000097          	auipc	ra,0x0
   10e84:	00a080e7          	jalr	10(ra) # 10e8a <_ZN4core9panicking13panic_display17h2406e68106876c5fE>
	...

0000000000010e8a <_ZN4core9panicking13panic_display17h2406e68106876c5fE>:
   10e8a:	715d                	add	sp,sp,-80
   10e8c:	e486                	sd	ra,72(sp)
   10e8e:	fc2a                	sd	a0,56(sp)

0000000000010e90 <.LBB152_1>:
   10e90:	00002517          	auipc	a0,0x2
   10e94:	00050513          	mv	a0,a0
   10e98:	e0aa                	sd	a0,64(sp)

0000000000010e9a <.LBB152_2>:
   10e9a:	00003517          	auipc	a0,0x3
   10e9e:	7ce50513          	add	a0,a0,1998 # 14668 <.Lanon.442aba94db1f841cd37d39ada1516238.203>
   10ea2:	e42a                	sd	a0,8(sp)
   10ea4:	4505                	li	a0,1
   10ea6:	e82a                	sd	a0,16(sp)
   10ea8:	ec02                	sd	zero,24(sp)
   10eaa:	1830                	add	a2,sp,56
   10eac:	f432                	sd	a2,40(sp)
   10eae:	f82a                	sd	a0,48(sp)
   10eb0:	0028                	add	a0,sp,8
   10eb2:	00000097          	auipc	ra,0x0
   10eb6:	04a080e7          	jalr	74(ra) # 10efc <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

0000000000010ebc <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>:
   10ebc:	7159                	add	sp,sp,-112
   10ebe:	f486                	sd	ra,104(sp)
   10ec0:	e42a                	sd	a0,8(sp)
   10ec2:	e82e                	sd	a1,16(sp)
   10ec4:	0808                	add	a0,sp,16
   10ec6:	e4aa                	sd	a0,72(sp)

0000000000010ec8 <.LBB153_1>:
   10ec8:	00002517          	auipc	a0,0x2
   10ecc:	e2250513          	add	a0,a0,-478 # 12cea <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h68be92e0167cbc01E>
   10ed0:	e8aa                	sd	a0,80(sp)
   10ed2:	002c                	add	a1,sp,8
   10ed4:	ecae                	sd	a1,88(sp)
   10ed6:	f0aa                	sd	a0,96(sp)

0000000000010ed8 <.LBB153_2>:
   10ed8:	00003517          	auipc	a0,0x3
   10edc:	75850513          	add	a0,a0,1880 # 14630 <.Lanon.442aba94db1f841cd37d39ada1516238.178>
   10ee0:	ec2a                	sd	a0,24(sp)
   10ee2:	4509                	li	a0,2
   10ee4:	f02a                	sd	a0,32(sp)
   10ee6:	f402                	sd	zero,40(sp)
   10ee8:	00ac                	add	a1,sp,72
   10eea:	fc2e                	sd	a1,56(sp)
   10eec:	e0aa                	sd	a0,64(sp)
   10eee:	0828                	add	a0,sp,24
   10ef0:	85b2                	mv	a1,a2
   10ef2:	00000097          	auipc	ra,0x0
   10ef6:	00a080e7          	jalr	10(ra) # 10efc <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

0000000000010efc <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>:
   10efc:	7179                	add	sp,sp,-48
   10efe:	f406                	sd	ra,40(sp)

0000000000010f00 <.LBB155_1>:
   10f00:	00003617          	auipc	a2,0x3
   10f04:	67860613          	add	a2,a2,1656 # 14578 <.Lanon.442aba94db1f841cd37d39ada1516238.83>
   10f08:	e032                	sd	a2,0(sp)

0000000000010f0a <.LBB155_2>:
   10f0a:	00003617          	auipc	a2,0x3
   10f0e:	76e60613          	add	a2,a2,1902 # 14678 <.Lanon.442aba94db1f841cd37d39ada1516238.208>
   10f12:	e432                	sd	a2,8(sp)
   10f14:	e82a                	sd	a0,16(sp)
   10f16:	ec2e                	sd	a1,24(sp)
   10f18:	4505                	li	a0,1
   10f1a:	02a10023          	sb	a0,32(sp)
   10f1e:	850a                	mv	a0,sp
   10f20:	00002097          	auipc	ra,0x2
   10f24:	21e080e7          	jalr	542(ra) # 1313e <rust_begin_unwind>
	...

0000000000010f2a <_ZN4core6result13unwrap_failed17h3c2e5884ed497eadE>:
   10f2a:	7119                	add	sp,sp,-128
   10f2c:	fc86                	sd	ra,120(sp)
   10f2e:	e42a                	sd	a0,8(sp)
   10f30:	e82e                	sd	a1,16(sp)
   10f32:	ec32                	sd	a2,24(sp)
   10f34:	f036                	sd	a3,32(sp)
   10f36:	0028                	add	a0,sp,8
   10f38:	ecaa                	sd	a0,88(sp)

0000000000010f3a <.LBB161_1>:
   10f3a:	00002517          	auipc	a0,0x2
   10f3e:	f5650513          	add	a0,a0,-170 # 12e90 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdd6e30ce7f47bbb8E>
   10f42:	f0aa                	sd	a0,96(sp)
   10f44:	0828                	add	a0,sp,24
   10f46:	f4aa                	sd	a0,104(sp)

0000000000010f48 <.LBB161_2>:
   10f48:	00002517          	auipc	a0,0x2
   10f4c:	f4050513          	add	a0,a0,-192 # 12e88 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf12cc52608b51daeE>
   10f50:	f8aa                	sd	a0,112(sp)

0000000000010f52 <.LBB161_3>:
   10f52:	00003517          	auipc	a0,0x3
   10f56:	76e50513          	add	a0,a0,1902 # 146c0 <.Lanon.442aba94db1f841cd37d39ada1516238.223>
   10f5a:	f42a                	sd	a0,40(sp)
   10f5c:	4509                	li	a0,2
   10f5e:	f82a                	sd	a0,48(sp)
   10f60:	fc02                	sd	zero,56(sp)
   10f62:	08ac                	add	a1,sp,88
   10f64:	e4ae                	sd	a1,72(sp)
   10f66:	e8aa                	sd	a0,80(sp)
   10f68:	1028                	add	a0,sp,40
   10f6a:	85ba                	mv	a1,a4
   10f6c:	00000097          	auipc	ra,0x0
   10f70:	f90080e7          	jalr	-112(ra) # 10efc <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

0000000000010f76 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h3f75591b6059cd6cE>:
   10f76:	7159                	add	sp,sp,-112
   10f78:	f486                	sd	ra,104(sp)
   10f7a:	f0a2                	sd	s0,96(sp)
   10f7c:	eca6                	sd	s1,88(sp)
   10f7e:	e8ca                	sd	s2,80(sp)
   10f80:	e4ce                	sd	s3,72(sp)
   10f82:	e0d2                	sd	s4,64(sp)
   10f84:	fc56                	sd	s5,56(sp)
   10f86:	f85a                	sd	s6,48(sp)
   10f88:	f45e                	sd	s7,40(sp)
   10f8a:	f062                	sd	s8,32(sp)
   10f8c:	ec66                	sd	s9,24(sp)
   10f8e:	e86a                	sd	s10,16(sp)
   10f90:	e46e                	sd	s11,8(sp)
   10f92:	1a060263          	beqz	a2,11136 <.LBB163_49+0x142>
   10f96:	8d32                	mv	s10,a2
   10f98:	89ae                	mv	s3,a1

0000000000010f9a <.LBB163_46>:
   10f9a:	00005597          	auipc	a1,0x5
   10f9e:	1fe58593          	add	a1,a1,510 # 16198 <.LCPI163_0>
   10fa2:	0005ba03          	ld	s4,0(a1)

0000000000010fa6 <.LBB163_47>:
   10fa6:	00005597          	auipc	a1,0x5
   10faa:	1fa58593          	add	a1,a1,506 # 161a0 <.LCPI163_1>
   10fae:	0005ba83          	ld	s5,0(a1)

0000000000010fb2 <.LBB163_48>:
   10fb2:	00005597          	auipc	a1,0x5
   10fb6:	1f658593          	add	a1,a1,502 # 161a8 <.LCPI163_2>
   10fba:	0005bb03          	ld	s6,0(a1)
   10fbe:	01053b83          	ld	s7,16(a0)
   10fc2:	00053903          	ld	s2,0(a0)
   10fc6:	00853c03          	ld	s8,8(a0)
   10fca:	4cc1                	li	s9,16
   10fcc:	4da9                	li	s11,10
   10fce:	a831                	j	10fea <.LBB163_48+0x38>
   10fd0:	00898533          	add	a0,s3,s0
   10fd4:	00050503          	lb	a0,0(a0)
   10fd8:	fbf00593          	li	a1,-65
   10fdc:	16a5df63          	bge	a1,a0,1115a <.LBB163_49+0x166>
   10fe0:	408d0d33          	sub	s10,s10,s0
   10fe4:	99a2                	add	s3,s3,s0
   10fe6:	140d0863          	beqz	s10,11136 <.LBB163_49+0x142>
   10fea:	000bc503          	lbu	a0,0(s7)
   10fee:	cd01                	beqz	a0,11006 <.LBB163_49+0x12>
   10ff0:	018c3683          	ld	a3,24(s8)

0000000000010ff4 <.LBB163_49>:
   10ff4:	00003597          	auipc	a1,0x3
   10ff8:	65c58593          	add	a1,a1,1628 # 14650 <.Lanon.442aba94db1f841cd37d39ada1516238.178+0x20>
   10ffc:	4611                	li	a2,4
   10ffe:	854a                	mv	a0,s2
   11000:	9682                	jalr	a3
   11002:	12051c63          	bnez	a0,1113a <.LBB163_49+0x146>
   11006:	4681                	li	a3,0
   11008:	85ea                	mv	a1,s10
   1100a:	a029                	j	11014 <.LBB163_49+0x20>
   1100c:	40dd05b3          	sub	a1,s10,a3
   11010:	0edd6963          	bltu	s10,a3,11102 <.LBB163_49+0x10e>
   11014:	00d98633          	add	a2,s3,a3
   11018:	0195fe63          	bgeu	a1,s9,11034 <.LBB163_49+0x40>
   1101c:	c1fd                	beqz	a1,11102 <.LBB163_49+0x10e>
   1101e:	4701                	li	a4,0
   11020:	00e60533          	add	a0,a2,a4
   11024:	00054503          	lbu	a0,0(a0)
   11028:	09b50f63          	beq	a0,s11,110c6 <.LBB163_49+0xd2>
   1102c:	0705                	add	a4,a4,1
   1102e:	fee599e3          	bne	a1,a4,11020 <.LBB163_49+0x2c>
   11032:	a8c1                	j	11102 <.LBB163_49+0x10e>
   11034:	00760513          	add	a0,a2,7
   11038:	9961                	and	a0,a0,-8
   1103a:	40c50733          	sub	a4,a0,a2
   1103e:	c705                	beqz	a4,11066 <.LBB163_49+0x72>
   11040:	852e                	mv	a0,a1
   11042:	00e5e363          	bltu	a1,a4,11048 <.LBB163_49+0x54>
   11046:	853a                	mv	a0,a4
   11048:	4701                	li	a4,0
   1104a:	00e607b3          	add	a5,a2,a4
   1104e:	0007c783          	lbu	a5,0(a5)
   11052:	07b78a63          	beq	a5,s11,110c6 <.LBB163_49+0xd2>
   11056:	0705                	add	a4,a4,1
   11058:	fee519e3          	bne	a0,a4,1104a <.LBB163_49+0x56>
   1105c:	ff058713          	add	a4,a1,-16
   11060:	00a77663          	bgeu	a4,a0,1106c <.LBB163_49+0x78>
   11064:	a83d                	j	110a2 <.LBB163_49+0xae>
   11066:	4501                	li	a0,0
   11068:	ff058713          	add	a4,a1,-16
   1106c:	00a607b3          	add	a5,a2,a0
   11070:	6380                	ld	s0,0(a5)
   11072:	fff44493          	not	s1,s0
   11076:	01644433          	xor	s0,s0,s6
   1107a:	9452                	add	s0,s0,s4
   1107c:	0154f4b3          	and	s1,s1,s5
   11080:	8ce1                	and	s1,s1,s0
   11082:	ec91                	bnez	s1,1109e <.LBB163_49+0xaa>
   11084:	679c                	ld	a5,8(a5)
   11086:	0167c4b3          	xor	s1,a5,s6
   1108a:	fff7c793          	not	a5,a5
   1108e:	94d2                	add	s1,s1,s4
   11090:	0157f7b3          	and	a5,a5,s5
   11094:	8fe5                	and	a5,a5,s1
   11096:	e781                	bnez	a5,1109e <.LBB163_49+0xaa>
   11098:	0541                	add	a0,a0,16
   1109a:	fca779e3          	bgeu	a4,a0,1106c <.LBB163_49+0x78>
   1109e:	0ca5e763          	bltu	a1,a0,1116c <.LBB163_49+0x178>
   110a2:	06b50063          	beq	a0,a1,11102 <.LBB163_49+0x10e>
   110a6:	4701                	li	a4,0
   110a8:	962a                	add	a2,a2,a0
   110aa:	40b505b3          	sub	a1,a0,a1
   110ae:	00e607b3          	add	a5,a2,a4
   110b2:	0007c783          	lbu	a5,0(a5)
   110b6:	01b78763          	beq	a5,s11,110c4 <.LBB163_49+0xd0>
   110ba:	0705                	add	a4,a4,1
   110bc:	00e587b3          	add	a5,a1,a4
   110c0:	f7fd                	bnez	a5,110ae <.LBB163_49+0xba>
   110c2:	a081                	j	11102 <.LBB163_49+0x10e>
   110c4:	972a                	add	a4,a4,a0
   110c6:	00d70533          	add	a0,a4,a3
   110ca:	00150693          	add	a3,a0,1
   110ce:	00a6b5b3          	sltu	a1,a3,a0
   110d2:	00dd3633          	sltu	a2,s10,a3
   110d6:	8dd1                	or	a1,a1,a2
   110d8:	f995                	bnez	a1,1100c <.LBB163_49+0x18>
   110da:	954e                	add	a0,a0,s3
   110dc:	00054503          	lbu	a0,0(a0)
   110e0:	f3b516e3          	bne	a0,s11,1100c <.LBB163_49+0x18>
   110e4:	4505                	li	a0,1
   110e6:	00ab8023          	sb	a0,0(s7)
   110ea:	03a6fb63          	bgeu	a3,s10,11120 <.LBB163_49+0x12c>
   110ee:	00d98533          	add	a0,s3,a3
   110f2:	00050503          	lb	a0,0(a0)
   110f6:	fbf00593          	li	a1,-65
   110fa:	02a5d663          	bge	a1,a0,11126 <.LBB163_49+0x132>
   110fe:	8436                	mv	s0,a3
   11100:	a021                	j	11108 <.LBB163_49+0x114>
   11102:	000b8023          	sb	zero,0(s7)
   11106:	846a                	mv	s0,s10
   11108:	018c3683          	ld	a3,24(s8)
   1110c:	854a                	mv	a0,s2
   1110e:	85ce                	mv	a1,s3
   11110:	8622                	mv	a2,s0
   11112:	9682                	jalr	a3
   11114:	e11d                	bnez	a0,1113a <.LBB163_49+0x146>
   11116:	eba46de3          	bltu	s0,s10,10fd0 <.LBB163_48+0x1e>
   1111a:	ec8d03e3          	beq	s10,s0,10fe0 <.LBB163_48+0x2e>
   1111e:	a835                	j	1115a <.LBB163_49+0x166>
   11120:	846a                	mv	s0,s10
   11122:	fedd03e3          	beq	s10,a3,11108 <.LBB163_49+0x114>
   11126:	854e                	mv	a0,s3
   11128:	85ea                	mv	a1,s10
   1112a:	4601                	li	a2,0
   1112c:	00001097          	auipc	ra,0x1
   11130:	24e080e7          	jalr	590(ra) # 1237a <_ZN4core3str16slice_error_fail17h0f23970489177861E>
   11134:	0000                	unimp
   11136:	4501                	li	a0,0
   11138:	a011                	j	1113c <.LBB163_49+0x148>
   1113a:	4505                	li	a0,1
   1113c:	70a6                	ld	ra,104(sp)
   1113e:	7406                	ld	s0,96(sp)
   11140:	64e6                	ld	s1,88(sp)
   11142:	6946                	ld	s2,80(sp)
   11144:	69a6                	ld	s3,72(sp)
   11146:	6a06                	ld	s4,64(sp)
   11148:	7ae2                	ld	s5,56(sp)
   1114a:	7b42                	ld	s6,48(sp)
   1114c:	7ba2                	ld	s7,40(sp)
   1114e:	7c02                	ld	s8,32(sp)
   11150:	6ce2                	ld	s9,24(sp)
   11152:	6d42                	ld	s10,16(sp)
   11154:	6da2                	ld	s11,8(sp)
   11156:	6165                	add	sp,sp,112
   11158:	8082                	ret
   1115a:	854e                	mv	a0,s3
   1115c:	85ea                	mv	a1,s10
   1115e:	8622                	mv	a2,s0
   11160:	86ea                	mv	a3,s10
   11162:	00001097          	auipc	ra,0x1
   11166:	218080e7          	jalr	536(ra) # 1237a <_ZN4core3str16slice_error_fail17h0f23970489177861E>
   1116a:	0000                	unimp
   1116c:	00001097          	auipc	ra,0x1
   11170:	d68080e7          	jalr	-664(ra) # 11ed4 <_ZN4core5slice5index26slice_start_index_len_fail17hc36fc5bb321621f5E>
	...

0000000000011176 <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E>:
   11176:	7171                	add	sp,sp,-176
   11178:	f506                	sd	ra,168(sp)
   1117a:	f122                	sd	s0,160(sp)
   1117c:	ed26                	sd	s1,152(sp)
   1117e:	e94a                	sd	s2,144(sp)
   11180:	e54e                	sd	s3,136(sp)
   11182:	e152                	sd	s4,128(sp)
   11184:	fcd6                	sd	s5,120(sp)
   11186:	f8da                	sd	s6,112(sp)
   11188:	f4de                	sd	s7,104(sp)
   1118a:	842a                	mv	s0,a0
   1118c:	00854503          	lbu	a0,8(a0)
   11190:	4b85                	li	s7,1
   11192:	4485                	li	s1,1
   11194:	c10d                	beqz	a0,111b6 <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x40>
   11196:	00940423          	sb	s1,8(s0)
   1119a:	017404a3          	sb	s7,9(s0)
   1119e:	8522                	mv	a0,s0
   111a0:	70aa                	ld	ra,168(sp)
   111a2:	740a                	ld	s0,160(sp)
   111a4:	64ea                	ld	s1,152(sp)
   111a6:	694a                	ld	s2,144(sp)
   111a8:	69aa                	ld	s3,136(sp)
   111aa:	6a0a                	ld	s4,128(sp)
   111ac:	7ae6                	ld	s5,120(sp)
   111ae:	7b46                	ld	s6,112(sp)
   111b0:	7ba6                	ld	s7,104(sp)
   111b2:	614d                	add	sp,sp,176
   111b4:	8082                	ret
   111b6:	89ba                	mv	s3,a4
   111b8:	8936                	mv	s2,a3
   111ba:	8a32                	mv	s4,a2
   111bc:	8aae                	mv	s5,a1
   111be:	00043b03          	ld	s6,0(s0)
   111c2:	030b6503          	lwu	a0,48(s6)
   111c6:	00944583          	lbu	a1,9(s0)
   111ca:	00457613          	and	a2,a0,4
   111ce:	ea09                	bnez	a2,111e0 <.LBB164_18+0xa>
   111d0:	0015b613          	seqz	a2,a1
   111d4:	c1e1                	beqz	a1,11294 <.LBB164_23>

00000000000111d6 <.LBB164_18>:
   111d6:	00003597          	auipc	a1,0x3
   111da:	53f58593          	add	a1,a1,1343 # 14715 <.Lanon.442aba94db1f841cd37d39ada1516238.229>
   111de:	a87d                	j	1129c <.LBB164_23+0x8>
   111e0:	e185                	bnez	a1,11200 <.LBB164_19+0x14>
   111e2:	028b3583          	ld	a1,40(s6)
   111e6:	020b3503          	ld	a0,32(s6)
   111ea:	6d94                	ld	a3,24(a1)

00000000000111ec <.LBB164_19>:
   111ec:	00003597          	auipc	a1,0x3
   111f0:	52458593          	add	a1,a1,1316 # 14710 <.Lanon.442aba94db1f841cd37d39ada1516238.227>
   111f4:	460d                	li	a2,3
   111f6:	9682                	jalr	a3
   111f8:	4485                	li	s1,1
   111fa:	fd51                	bnez	a0,11196 <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x20>
   111fc:	030b2503          	lw	a0,48(s6)
   11200:	4485                	li	s1,1
   11202:	029103a3          	sb	s1,39(sp)
   11206:	020b3583          	ld	a1,32(s6)
   1120a:	028b3603          	ld	a2,40(s6)
   1120e:	e42e                	sd	a1,8(sp)
   11210:	e832                	sd	a2,16(sp)
   11212:	02710593          	add	a1,sp,39
   11216:	ec2e                	sd	a1,24(sp)
   11218:	034b2583          	lw	a1,52(s6)
   1121c:	038b0603          	lb	a2,56(s6)
   11220:	000b3683          	ld	a3,0(s6)
   11224:	008b3703          	ld	a4,8(s6)
   11228:	010b3783          	ld	a5,16(s6)
   1122c:	018b3803          	ld	a6,24(s6)
   11230:	ccaa                	sw	a0,88(sp)
   11232:	ceae                	sw	a1,92(sp)
   11234:	06c10023          	sb	a2,96(sp)
   11238:	f436                	sd	a3,40(sp)
   1123a:	f83a                	sd	a4,48(sp)
   1123c:	fc3e                	sd	a5,56(sp)
   1123e:	e0c2                	sd	a6,64(sp)
   11240:	0028                	add	a0,sp,8
   11242:	e4aa                	sd	a0,72(sp)

0000000000011244 <.LBB164_20>:
   11244:	00003517          	auipc	a0,0x3
   11248:	49c50513          	add	a0,a0,1180 # 146e0 <.Lanon.442aba94db1f841cd37d39ada1516238.224>
   1124c:	e8aa                	sd	a0,80(sp)
   1124e:	0028                	add	a0,sp,8
   11250:	85d6                	mv	a1,s5
   11252:	8652                	mv	a2,s4
   11254:	00000097          	auipc	ra,0x0
   11258:	d22080e7          	jalr	-734(ra) # 10f76 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h3f75591b6059cd6cE>
   1125c:	fd0d                	bnez	a0,11196 <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x20>

000000000001125e <.LBB164_21>:
   1125e:	00003597          	auipc	a1,0x3
   11262:	45b58593          	add	a1,a1,1115 # 146b9 <.Lanon.442aba94db1f841cd37d39ada1516238.222>
   11266:	0028                	add	a0,sp,8
   11268:	4609                	li	a2,2
   1126a:	00000097          	auipc	ra,0x0
   1126e:	d0c080e7          	jalr	-756(ra) # 10f76 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h3f75591b6059cd6cE>
   11272:	f115                	bnez	a0,11196 <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x20>
   11274:	0189b603          	ld	a2,24(s3)
   11278:	102c                	add	a1,sp,40
   1127a:	854a                	mv	a0,s2
   1127c:	9602                	jalr	a2
   1127e:	fd01                	bnez	a0,11196 <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x20>
   11280:	65c6                	ld	a1,80(sp)
   11282:	6526                	ld	a0,72(sp)
   11284:	6d94                	ld	a3,24(a1)

0000000000011286 <.LBB164_22>:
   11286:	00003597          	auipc	a1,0x3
   1128a:	48d58593          	add	a1,a1,1165 # 14713 <.Lanon.442aba94db1f841cd37d39ada1516238.228>
   1128e:	4609                	li	a2,2
   11290:	9682                	jalr	a3
   11292:	a8b1                	j	112ee <.LBB164_24+0x1c>

0000000000011294 <.LBB164_23>:
   11294:	00003597          	auipc	a1,0x3
   11298:	48358593          	add	a1,a1,1155 # 14717 <.Lanon.442aba94db1f841cd37d39ada1516238.230>
   1129c:	028b3683          	ld	a3,40(s6)
   112a0:	020b3503          	ld	a0,32(s6)
   112a4:	6e94                	ld	a3,24(a3)
   112a6:	00266613          	or	a2,a2,2
   112aa:	9682                	jalr	a3
   112ac:	4485                	li	s1,1
   112ae:	ee0514e3          	bnez	a0,11196 <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x20>
   112b2:	028b3583          	ld	a1,40(s6)
   112b6:	020b3503          	ld	a0,32(s6)
   112ba:	6d94                	ld	a3,24(a1)
   112bc:	85d6                	mv	a1,s5
   112be:	8652                	mv	a2,s4
   112c0:	9682                	jalr	a3
   112c2:	4485                	li	s1,1
   112c4:	ec0519e3          	bnez	a0,11196 <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x20>
   112c8:	028b3583          	ld	a1,40(s6)
   112cc:	020b3503          	ld	a0,32(s6)
   112d0:	6d94                	ld	a3,24(a1)

00000000000112d2 <.LBB164_24>:
   112d2:	00003597          	auipc	a1,0x3
   112d6:	3e758593          	add	a1,a1,999 # 146b9 <.Lanon.442aba94db1f841cd37d39ada1516238.222>
   112da:	4609                	li	a2,2
   112dc:	9682                	jalr	a3
   112de:	4485                	li	s1,1
   112e0:	ea051be3          	bnez	a0,11196 <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x20>
   112e4:	0189b603          	ld	a2,24(s3)
   112e8:	854a                	mv	a0,s2
   112ea:	85da                	mv	a1,s6
   112ec:	9602                	jalr	a2
   112ee:	84aa                	mv	s1,a0
   112f0:	b55d                	j	11196 <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E+0x20>

00000000000112f2 <_ZN4core3fmt8builders10DebugTuple5field17hb4c5d4885bb0d25dE>:
   112f2:	7135                	add	sp,sp,-160
   112f4:	ed06                	sd	ra,152(sp)
   112f6:	e922                	sd	s0,144(sp)
   112f8:	e526                	sd	s1,136(sp)
   112fa:	e14a                	sd	s2,128(sp)
   112fc:	fcce                	sd	s3,120(sp)
   112fe:	f8d2                	sd	s4,112(sp)
   11300:	f4d6                	sd	s5,104(sp)
   11302:	842a                	mv	s0,a0
   11304:	01054503          	lbu	a0,16(a0)
   11308:	c509                	beqz	a0,11312 <_ZN4core3fmt8builders10DebugTuple5field17hb4c5d4885bb0d25dE+0x20>
   1130a:	00843a83          	ld	s5,8(s0)
   1130e:	4485                	li	s1,1
   11310:	a0e5                	j	113f8 <.LBB167_20+0xe>
   11312:	89b2                	mv	s3,a2
   11314:	892e                	mv	s2,a1
   11316:	00043a03          	ld	s4,0(s0)
   1131a:	030a6503          	lwu	a0,48(s4)
   1131e:	00843a83          	ld	s5,8(s0)
   11322:	00457593          	and	a1,a0,4
   11326:	e991                	bnez	a1,1133a <.LBB167_16+0xa>
   11328:	001ab613          	seqz	a2,s5
   1132c:	020a8863          	beqz	s5,1135c <.LBB167_18>

0000000000011330 <.LBB167_16>:
   11330:	00003597          	auipc	a1,0x3
   11334:	3e558593          	add	a1,a1,997 # 14715 <.Lanon.442aba94db1f841cd37d39ada1516238.229>
   11338:	a035                	j	11364 <.LBB167_18+0x8>
   1133a:	040a9863          	bnez	s5,1138a <.LBB167_18+0x2e>
   1133e:	028a3583          	ld	a1,40(s4)
   11342:	020a3503          	ld	a0,32(s4)
   11346:	6d94                	ld	a3,24(a1)

0000000000011348 <.LBB167_17>:
   11348:	00003597          	auipc	a1,0x3
   1134c:	3d558593          	add	a1,a1,981 # 1471d <.Lanon.442aba94db1f841cd37d39ada1516238.236>
   11350:	4609                	li	a2,2
   11352:	9682                	jalr	a3
   11354:	c90d                	beqz	a0,11386 <.LBB167_18+0x2a>
   11356:	4a81                	li	s5,0
   11358:	4485                	li	s1,1
   1135a:	a879                	j	113f8 <.LBB167_20+0xe>

000000000001135c <.LBB167_18>:
   1135c:	00003597          	auipc	a1,0x3
   11360:	3c358593          	add	a1,a1,963 # 1471f <.Lanon.442aba94db1f841cd37d39ada1516238.237>
   11364:	028a3683          	ld	a3,40(s4)
   11368:	020a3503          	ld	a0,32(s4)
   1136c:	6e94                	ld	a3,24(a3)
   1136e:	4709                	li	a4,2
   11370:	40c70633          	sub	a2,a4,a2
   11374:	9682                	jalr	a3
   11376:	4485                	li	s1,1
   11378:	e141                	bnez	a0,113f8 <.LBB167_20+0xe>
   1137a:	0189b603          	ld	a2,24(s3)
   1137e:	854a                	mv	a0,s2
   11380:	85d2                	mv	a1,s4
   11382:	9602                	jalr	a2
   11384:	a88d                	j	113f6 <.LBB167_20+0xc>
   11386:	030a2503          	lw	a0,48(s4)
   1138a:	4485                	li	s1,1
   1138c:	029103a3          	sb	s1,39(sp)
   11390:	020a3583          	ld	a1,32(s4)
   11394:	028a3603          	ld	a2,40(s4)
   11398:	e42e                	sd	a1,8(sp)
   1139a:	e832                	sd	a2,16(sp)
   1139c:	02710593          	add	a1,sp,39
   113a0:	ec2e                	sd	a1,24(sp)
   113a2:	034a2583          	lw	a1,52(s4)
   113a6:	038a0603          	lb	a2,56(s4)
   113aa:	000a3683          	ld	a3,0(s4)
   113ae:	008a3703          	ld	a4,8(s4)
   113b2:	010a3783          	ld	a5,16(s4)
   113b6:	018a3803          	ld	a6,24(s4)
   113ba:	ccaa                	sw	a0,88(sp)
   113bc:	ceae                	sw	a1,92(sp)
   113be:	06c10023          	sb	a2,96(sp)
   113c2:	f436                	sd	a3,40(sp)
   113c4:	f83a                	sd	a4,48(sp)
   113c6:	fc3e                	sd	a5,56(sp)
   113c8:	e0c2                	sd	a6,64(sp)
   113ca:	0028                	add	a0,sp,8
   113cc:	0189b603          	ld	a2,24(s3)
   113d0:	e4aa                	sd	a0,72(sp)

00000000000113d2 <.LBB167_19>:
   113d2:	00003517          	auipc	a0,0x3
   113d6:	30e50513          	add	a0,a0,782 # 146e0 <.Lanon.442aba94db1f841cd37d39ada1516238.224>
   113da:	e8aa                	sd	a0,80(sp)
   113dc:	102c                	add	a1,sp,40
   113de:	854a                	mv	a0,s2
   113e0:	9602                	jalr	a2
   113e2:	e919                	bnez	a0,113f8 <.LBB167_20+0xe>
   113e4:	65c6                	ld	a1,80(sp)
   113e6:	6526                	ld	a0,72(sp)
   113e8:	6d94                	ld	a3,24(a1)

00000000000113ea <.LBB167_20>:
   113ea:	00003597          	auipc	a1,0x3
   113ee:	32958593          	add	a1,a1,809 # 14713 <.Lanon.442aba94db1f841cd37d39ada1516238.228>
   113f2:	4609                	li	a2,2
   113f4:	9682                	jalr	a3
   113f6:	84aa                	mv	s1,a0
   113f8:	00940823          	sb	s1,16(s0)
   113fc:	001a8513          	add	a0,s5,1
   11400:	e408                	sd	a0,8(s0)
   11402:	8522                	mv	a0,s0
   11404:	60ea                	ld	ra,152(sp)
   11406:	644a                	ld	s0,144(sp)
   11408:	64aa                	ld	s1,136(sp)
   1140a:	690a                	ld	s2,128(sp)
   1140c:	79e6                	ld	s3,120(sp)
   1140e:	7a46                	ld	s4,112(sp)
   11410:	7aa6                	ld	s5,104(sp)
   11412:	610d                	add	sp,sp,160
   11414:	8082                	ret

0000000000011416 <_ZN4core3fmt5Write10write_char17hebf5064c644471f8E>:
   11416:	1141                	add	sp,sp,-16
   11418:	e406                	sd	ra,8(sp)
   1141a:	0005861b          	sext.w	a2,a1
   1141e:	08000693          	li	a3,128
   11422:	c202                	sw	zero,4(sp)
   11424:	00d67663          	bgeu	a2,a3,11430 <_ZN4core3fmt5Write10write_char17hebf5064c644471f8E+0x1a>
   11428:	00b10223          	sb	a1,4(sp)
   1142c:	4605                	li	a2,1
   1142e:	a849                	j	114c0 <_ZN4core3fmt5Write10write_char17hebf5064c644471f8E+0xaa>
   11430:	00b5d61b          	srlw	a2,a1,0xb
   11434:	ee19                	bnez	a2,11452 <_ZN4core3fmt5Write10write_char17hebf5064c644471f8E+0x3c>
   11436:	0065d613          	srl	a2,a1,0x6
   1143a:	0c066613          	or	a2,a2,192
   1143e:	00c10223          	sb	a2,4(sp)
   11442:	03f5f593          	and	a1,a1,63
   11446:	0805e593          	or	a1,a1,128
   1144a:	00b102a3          	sb	a1,5(sp)
   1144e:	4609                	li	a2,2
   11450:	a885                	j	114c0 <_ZN4core3fmt5Write10write_char17hebf5064c644471f8E+0xaa>
   11452:	0105d61b          	srlw	a2,a1,0x10
   11456:	e61d                	bnez	a2,11484 <_ZN4core3fmt5Write10write_char17hebf5064c644471f8E+0x6e>
   11458:	00c5d61b          	srlw	a2,a1,0xc
   1145c:	0e066613          	or	a2,a2,224
   11460:	00c10223          	sb	a2,4(sp)
   11464:	0065d61b          	srlw	a2,a1,0x6
   11468:	03f67613          	and	a2,a2,63
   1146c:	08066613          	or	a2,a2,128
   11470:	00c102a3          	sb	a2,5(sp)
   11474:	03f5f593          	and	a1,a1,63
   11478:	0805e593          	or	a1,a1,128
   1147c:	00b10323          	sb	a1,6(sp)
   11480:	460d                	li	a2,3
   11482:	a83d                	j	114c0 <_ZN4core3fmt5Write10write_char17hebf5064c644471f8E+0xaa>
   11484:	0125d61b          	srlw	a2,a1,0x12
   11488:	8a1d                	and	a2,a2,7
   1148a:	0f066613          	or	a2,a2,240
   1148e:	00c10223          	sb	a2,4(sp)
   11492:	00c5d61b          	srlw	a2,a1,0xc
   11496:	03f67613          	and	a2,a2,63
   1149a:	08066613          	or	a2,a2,128
   1149e:	00c102a3          	sb	a2,5(sp)
   114a2:	0065d61b          	srlw	a2,a1,0x6
   114a6:	03f67613          	and	a2,a2,63
   114aa:	08066613          	or	a2,a2,128
   114ae:	00c10323          	sb	a2,6(sp)
   114b2:	03f5f593          	and	a1,a1,63
   114b6:	0805e593          	or	a1,a1,128
   114ba:	00b103a3          	sb	a1,7(sp)
   114be:	4611                	li	a2,4
   114c0:	004c                	add	a1,sp,4
   114c2:	00000097          	auipc	ra,0x0
   114c6:	ab4080e7          	jalr	-1356(ra) # 10f76 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h3f75591b6059cd6cE>
   114ca:	60a2                	ld	ra,8(sp)
   114cc:	0141                	add	sp,sp,16
   114ce:	8082                	ret

00000000000114d0 <_ZN4core3fmt5Write9write_fmt17h4daaf3148a56cafcE>:
   114d0:	7139                	add	sp,sp,-64
   114d2:	fc06                	sd	ra,56(sp)
   114d4:	7590                	ld	a2,40(a1)
   114d6:	7194                	ld	a3,32(a1)
   114d8:	e02a                	sd	a0,0(sp)
   114da:	f832                	sd	a2,48(sp)
   114dc:	f436                	sd	a3,40(sp)
   114de:	6d88                	ld	a0,24(a1)
   114e0:	6990                	ld	a2,16(a1)
   114e2:	6594                	ld	a3,8(a1)
   114e4:	618c                	ld	a1,0(a1)
   114e6:	f02a                	sd	a0,32(sp)
   114e8:	ec32                	sd	a2,24(sp)
   114ea:	e836                	sd	a3,16(sp)
   114ec:	e42e                	sd	a1,8(sp)

00000000000114ee <.LBB190_1>:
   114ee:	00003597          	auipc	a1,0x3
   114f2:	32a58593          	add	a1,a1,810 # 14818 <.Lanon.442aba94db1f841cd37d39ada1516238.262>
   114f6:	850a                	mv	a0,sp
   114f8:	0030                	add	a2,sp,8
   114fa:	00000097          	auipc	ra,0x0
   114fe:	140080e7          	jalr	320(ra) # 1163a <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   11502:	70e2                	ld	ra,56(sp)
   11504:	6121                	add	sp,sp,64
   11506:	8082                	ret

0000000000011508 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h438723c400931e56E>:
   11508:	6108                	ld	a0,0(a0)
   1150a:	00000317          	auipc	t1,0x0
   1150e:	a6c30067          	jr	-1428(t1) # 10f76 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h3f75591b6059cd6cE>

0000000000011512 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9e9bbdc7252b47dcE>:
   11512:	1141                	add	sp,sp,-16
   11514:	e406                	sd	ra,8(sp)
   11516:	6108                	ld	a0,0(a0)
   11518:	0005861b          	sext.w	a2,a1
   1151c:	08000693          	li	a3,128
   11520:	c202                	sw	zero,4(sp)
   11522:	00d67663          	bgeu	a2,a3,1152e <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9e9bbdc7252b47dcE+0x1c>
   11526:	00b10223          	sb	a1,4(sp)
   1152a:	4605                	li	a2,1
   1152c:	a849                	j	115be <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9e9bbdc7252b47dcE+0xac>
   1152e:	00b5d61b          	srlw	a2,a1,0xb
   11532:	ee19                	bnez	a2,11550 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9e9bbdc7252b47dcE+0x3e>
   11534:	0065d613          	srl	a2,a1,0x6
   11538:	0c066613          	or	a2,a2,192
   1153c:	00c10223          	sb	a2,4(sp)
   11540:	03f5f593          	and	a1,a1,63
   11544:	0805e593          	or	a1,a1,128
   11548:	00b102a3          	sb	a1,5(sp)
   1154c:	4609                	li	a2,2
   1154e:	a885                	j	115be <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9e9bbdc7252b47dcE+0xac>
   11550:	0105d61b          	srlw	a2,a1,0x10
   11554:	e61d                	bnez	a2,11582 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9e9bbdc7252b47dcE+0x70>
   11556:	00c5d61b          	srlw	a2,a1,0xc
   1155a:	0e066613          	or	a2,a2,224
   1155e:	00c10223          	sb	a2,4(sp)
   11562:	0065d61b          	srlw	a2,a1,0x6
   11566:	03f67613          	and	a2,a2,63
   1156a:	08066613          	or	a2,a2,128
   1156e:	00c102a3          	sb	a2,5(sp)
   11572:	03f5f593          	and	a1,a1,63
   11576:	0805e593          	or	a1,a1,128
   1157a:	00b10323          	sb	a1,6(sp)
   1157e:	460d                	li	a2,3
   11580:	a83d                	j	115be <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9e9bbdc7252b47dcE+0xac>
   11582:	0125d61b          	srlw	a2,a1,0x12
   11586:	8a1d                	and	a2,a2,7
   11588:	0f066613          	or	a2,a2,240
   1158c:	00c10223          	sb	a2,4(sp)
   11590:	00c5d61b          	srlw	a2,a1,0xc
   11594:	03f67613          	and	a2,a2,63
   11598:	08066613          	or	a2,a2,128
   1159c:	00c102a3          	sb	a2,5(sp)
   115a0:	0065d61b          	srlw	a2,a1,0x6
   115a4:	03f67613          	and	a2,a2,63
   115a8:	08066613          	or	a2,a2,128
   115ac:	00c10323          	sb	a2,6(sp)
   115b0:	03f5f593          	and	a1,a1,63
   115b4:	0805e593          	or	a1,a1,128
   115b8:	00b103a3          	sb	a1,7(sp)
   115bc:	4611                	li	a2,4
   115be:	004c                	add	a1,sp,4
   115c0:	00000097          	auipc	ra,0x0
   115c4:	9b6080e7          	jalr	-1610(ra) # 10f76 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h3f75591b6059cd6cE>
   115c8:	60a2                	ld	ra,8(sp)
   115ca:	0141                	add	sp,sp,16
   115cc:	8082                	ret

00000000000115ce <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17he338dd4ffa451fd9E>:
   115ce:	7139                	add	sp,sp,-64
   115d0:	fc06                	sd	ra,56(sp)
   115d2:	6108                	ld	a0,0(a0)
   115d4:	7590                	ld	a2,40(a1)
   115d6:	7194                	ld	a3,32(a1)
   115d8:	e02a                	sd	a0,0(sp)
   115da:	f832                	sd	a2,48(sp)
   115dc:	f436                	sd	a3,40(sp)
   115de:	6d88                	ld	a0,24(a1)
   115e0:	6990                	ld	a2,16(a1)
   115e2:	6594                	ld	a3,8(a1)
   115e4:	618c                	ld	a1,0(a1)
   115e6:	f02a                	sd	a0,32(sp)
   115e8:	ec32                	sd	a2,24(sp)
   115ea:	e836                	sd	a3,16(sp)
   115ec:	e42e                	sd	a1,8(sp)

00000000000115ee <.LBB193_1>:
   115ee:	00003597          	auipc	a1,0x3
   115f2:	22a58593          	add	a1,a1,554 # 14818 <.Lanon.442aba94db1f841cd37d39ada1516238.262>
   115f6:	850a                	mv	a0,sp
   115f8:	0030                	add	a2,sp,8
   115fa:	00000097          	auipc	ra,0x0
   115fe:	040080e7          	jalr	64(ra) # 1163a <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   11602:	70e2                	ld	ra,56(sp)
   11604:	6121                	add	sp,sp,64
   11606:	8082                	ret

0000000000011608 <_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17h4d3bee89ff077165E>:
   11608:	7139                	add	sp,sp,-64
   1160a:	fc06                	sd	ra,56(sp)
   1160c:	7510                	ld	a2,40(a0)
   1160e:	7118                	ld	a4,32(a0)
   11610:	6d1c                	ld	a5,24(a0)
   11612:	f832                	sd	a2,48(sp)
   11614:	7194                	ld	a3,32(a1)
   11616:	f43a                	sd	a4,40(sp)
   11618:	f03e                	sd	a5,32(sp)
   1161a:	6910                	ld	a2,16(a0)
   1161c:	6518                	ld	a4,8(a0)
   1161e:	6108                	ld	a0,0(a0)
   11620:	758c                	ld	a1,40(a1)
   11622:	ec32                	sd	a2,24(sp)
   11624:	e83a                	sd	a4,16(sp)
   11626:	e42a                	sd	a0,8(sp)
   11628:	0030                	add	a2,sp,8
   1162a:	8536                	mv	a0,a3
   1162c:	00000097          	auipc	ra,0x0
   11630:	00e080e7          	jalr	14(ra) # 1163a <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   11634:	70e2                	ld	ra,56(sp)
   11636:	6121                	add	sp,sp,64
   11638:	8082                	ret

000000000001163a <_ZN4core3fmt5write17ha708f69ea5ad27e5E>:
   1163a:	7119                	add	sp,sp,-128
   1163c:	fc86                	sd	ra,120(sp)
   1163e:	f8a2                	sd	s0,112(sp)
   11640:	f4a6                	sd	s1,104(sp)
   11642:	f0ca                	sd	s2,96(sp)
   11644:	ecce                	sd	s3,88(sp)
   11646:	e8d2                	sd	s4,80(sp)
   11648:	e4d6                	sd	s5,72(sp)
   1164a:	e0da                	sd	s6,64(sp)
   1164c:	89b2                	mv	s3,a2
   1164e:	4605                	li	a2,1
   11650:	1616                	sll	a2,a2,0x25
   11652:	f832                	sd	a2,48(sp)
   11654:	460d                	li	a2,3
   11656:	02c10c23          	sb	a2,56(sp)
   1165a:	0109b603          	ld	a2,16(s3)
   1165e:	e002                	sd	zero,0(sp)
   11660:	e802                	sd	zero,16(sp)
   11662:	f02a                	sd	a0,32(sp)
   11664:	f42e                	sd	a1,40(sp)
   11666:	ca69                	beqz	a2,11738 <.LBB199_34+0xa6>
   11668:	0189b503          	ld	a0,24(s3)
   1166c:	10050c63          	beqz	a0,11784 <.LBB199_34+0xf2>
   11670:	0009b583          	ld	a1,0(s3)
   11674:	fff50693          	add	a3,a0,-1
   11678:	068e                	sll	a3,a3,0x3
   1167a:	828d                	srl	a3,a3,0x3
   1167c:	00168913          	add	s2,a3,1
   11680:	00858493          	add	s1,a1,8
   11684:	03800593          	li	a1,56
   11688:	02b50a33          	mul	s4,a0,a1
   1168c:	03060413          	add	s0,a2,48
   11690:	4a85                	li	s5,1

0000000000011692 <.LBB199_34>:
   11692:	fffffb17          	auipc	s6,0xfffff
   11696:	6f6b0b13          	add	s6,s6,1782 # 10d88 <_ZN4core3ops8function6FnOnce9call_once17h6b85840bc58c33c1E>
   1169a:	6090                	ld	a2,0(s1)
   1169c:	ca09                	beqz	a2,116ae <.LBB199_34+0x1c>
   1169e:	76a2                	ld	a3,40(sp)
   116a0:	7502                	ld	a0,32(sp)
   116a2:	ff84b583          	ld	a1,-8(s1)
   116a6:	6e94                	ld	a3,24(a3)
   116a8:	9682                	jalr	a3
   116aa:	10051963          	bnez	a0,117bc <.LBB199_34+0x12a>
   116ae:	ff842503          	lw	a0,-8(s0)
   116b2:	da2a                	sw	a0,52(sp)
   116b4:	00040503          	lb	a0,0(s0)
   116b8:	02a10c23          	sb	a0,56(sp)
   116bc:	ffc42583          	lw	a1,-4(s0)
   116c0:	0209b503          	ld	a0,32(s3)
   116c4:	d82e                	sw	a1,48(sp)
   116c6:	fe843683          	ld	a3,-24(s0)
   116ca:	ff043583          	ld	a1,-16(s0)
   116ce:	ce89                	beqz	a3,116e8 <.LBB199_34+0x56>
   116d0:	4601                	li	a2,0
   116d2:	01569c63          	bne	a3,s5,116ea <.LBB199_34+0x58>
   116d6:	0592                	sll	a1,a1,0x4
   116d8:	95aa                	add	a1,a1,a0
   116da:	6590                	ld	a2,8(a1)
   116dc:	01660463          	beq	a2,s6,116e4 <.LBB199_34+0x52>
   116e0:	4601                	li	a2,0
   116e2:	a021                	j	116ea <.LBB199_34+0x58>
   116e4:	618c                	ld	a1,0(a1)
   116e6:	618c                	ld	a1,0(a1)
   116e8:	4605                	li	a2,1
   116ea:	e032                	sd	a2,0(sp)
   116ec:	e42e                	sd	a1,8(sp)
   116ee:	fd843683          	ld	a3,-40(s0)
   116f2:	fe043583          	ld	a1,-32(s0)
   116f6:	ce89                	beqz	a3,11710 <.LBB199_34+0x7e>
   116f8:	4601                	li	a2,0
   116fa:	01569c63          	bne	a3,s5,11712 <.LBB199_34+0x80>
   116fe:	0592                	sll	a1,a1,0x4
   11700:	95aa                	add	a1,a1,a0
   11702:	6590                	ld	a2,8(a1)
   11704:	01660463          	beq	a2,s6,1170c <.LBB199_34+0x7a>
   11708:	4601                	li	a2,0
   1170a:	a021                	j	11712 <.LBB199_34+0x80>
   1170c:	618c                	ld	a1,0(a1)
   1170e:	618c                	ld	a1,0(a1)
   11710:	4605                	li	a2,1
   11712:	e832                	sd	a2,16(sp)
   11714:	ec2e                	sd	a1,24(sp)
   11716:	fd043583          	ld	a1,-48(s0)
   1171a:	0592                	sll	a1,a1,0x4
   1171c:	952e                	add	a0,a0,a1
   1171e:	6510                	ld	a2,8(a0)
   11720:	6108                	ld	a0,0(a0)
   11722:	858a                	mv	a1,sp
   11724:	9602                	jalr	a2
   11726:	e959                	bnez	a0,117bc <.LBB199_34+0x12a>
   11728:	04c1                	add	s1,s1,16
   1172a:	fc8a0a13          	add	s4,s4,-56
   1172e:	03840413          	add	s0,s0,56
   11732:	f60a14e3          	bnez	s4,1169a <.LBB199_34+0x8>
   11736:	a881                	j	11786 <.LBB199_34+0xf4>
   11738:	0289b503          	ld	a0,40(s3)
   1173c:	cd29                	beqz	a0,11796 <.LBB199_34+0x104>
   1173e:	0209b583          	ld	a1,32(s3)
   11742:	0009b603          	ld	a2,0(s3)
   11746:	157d                	add	a0,a0,-1
   11748:	0512                	sll	a0,a0,0x4
   1174a:	8111                	srl	a0,a0,0x4
   1174c:	00150913          	add	s2,a0,1
   11750:	00860413          	add	s0,a2,8
   11754:	00858493          	add	s1,a1,8
   11758:	8a4a                	mv	s4,s2
   1175a:	6010                	ld	a2,0(s0)
   1175c:	ca01                	beqz	a2,1176c <.LBB199_34+0xda>
   1175e:	76a2                	ld	a3,40(sp)
   11760:	7502                	ld	a0,32(sp)
   11762:	ff843583          	ld	a1,-8(s0)
   11766:	6e94                	ld	a3,24(a3)
   11768:	9682                	jalr	a3
   1176a:	e929                	bnez	a0,117bc <.LBB199_34+0x12a>
   1176c:	6090                	ld	a2,0(s1)
   1176e:	ff84b503          	ld	a0,-8(s1)
   11772:	858a                	mv	a1,sp
   11774:	9602                	jalr	a2
   11776:	e139                	bnez	a0,117bc <.LBB199_34+0x12a>
   11778:	1a7d                	add	s4,s4,-1
   1177a:	0441                	add	s0,s0,16
   1177c:	04c1                	add	s1,s1,16
   1177e:	fc0a1ee3          	bnez	s4,1175a <.LBB199_34+0xc8>
   11782:	a011                	j	11786 <.LBB199_34+0xf4>
   11784:	4901                	li	s2,0
   11786:	0089b503          	ld	a0,8(s3)
   1178a:	00a96b63          	bltu	s2,a0,117a0 <.LBB199_34+0x10e>
   1178e:	4601                	li	a2,0
   11790:	00a96f63          	bltu	s2,a0,117ae <.LBB199_34+0x11c>
   11794:	a035                	j	117c0 <.LBB199_34+0x12e>
   11796:	4901                	li	s2,0
   11798:	0089b503          	ld	a0,8(s3)
   1179c:	fea979e3          	bgeu	s2,a0,1178e <.LBB199_34+0xfc>
   117a0:	0009b583          	ld	a1,0(s3)
   117a4:	00491613          	sll	a2,s2,0x4
   117a8:	962e                	add	a2,a2,a1
   117aa:	00a97b63          	bgeu	s2,a0,117c0 <.LBB199_34+0x12e>
   117ae:	76a2                	ld	a3,40(sp)
   117b0:	7502                	ld	a0,32(sp)
   117b2:	620c                	ld	a1,0(a2)
   117b4:	6610                	ld	a2,8(a2)
   117b6:	6e94                	ld	a3,24(a3)
   117b8:	9682                	jalr	a3
   117ba:	c119                	beqz	a0,117c0 <.LBB199_34+0x12e>
   117bc:	4505                	li	a0,1
   117be:	a011                	j	117c2 <.LBB199_34+0x130>
   117c0:	4501                	li	a0,0
   117c2:	70e6                	ld	ra,120(sp)
   117c4:	7446                	ld	s0,112(sp)
   117c6:	74a6                	ld	s1,104(sp)
   117c8:	7906                	ld	s2,96(sp)
   117ca:	69e6                	ld	s3,88(sp)
   117cc:	6a46                	ld	s4,80(sp)
   117ce:	6aa6                	ld	s5,72(sp)
   117d0:	6b06                	ld	s6,64(sp)
   117d2:	6109                	add	sp,sp,128
   117d4:	8082                	ret

00000000000117d6 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E>:
   117d6:	711d                	add	sp,sp,-96
   117d8:	ec86                	sd	ra,88(sp)
   117da:	e8a2                	sd	s0,80(sp)
   117dc:	e4a6                	sd	s1,72(sp)
   117de:	e0ca                	sd	s2,64(sp)
   117e0:	fc4e                	sd	s3,56(sp)
   117e2:	f852                	sd	s4,48(sp)
   117e4:	f456                	sd	s5,40(sp)
   117e6:	f05a                	sd	s6,32(sp)
   117e8:	ec5e                	sd	s7,24(sp)
   117ea:	e862                	sd	s8,16(sp)
   117ec:	e466                	sd	s9,8(sp)
   117ee:	e06a                	sd	s10,0(sp)
   117f0:	89be                	mv	s3,a5
   117f2:	893a                	mv	s2,a4
   117f4:	8b36                	mv	s6,a3
   117f6:	8ab2                	mv	s5,a2
   117f8:	8c2a                	mv	s8,a0
   117fa:	c5b9                	beqz	a1,11848 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x72>
   117fc:	030c6503          	lwu	a0,48(s8)
   11800:	00157593          	and	a1,a0,1
   11804:	00110a37          	lui	s4,0x110
   11808:	c199                	beqz	a1,1180e <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x38>
   1180a:	02b00a13          	li	s4,43
   1180e:	01358433          	add	s0,a1,s3
   11812:	8911                	and	a0,a0,4
   11814:	c131                	beqz	a0,11858 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x82>
   11816:	02000513          	li	a0,32
   1181a:	04ab7463          	bgeu	s6,a0,11862 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x8c>
   1181e:	4501                	li	a0,0
   11820:	000b0e63          	beqz	s6,1183c <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x66>
   11824:	fbf00593          	li	a1,-65
   11828:	865a                	mv	a2,s6
   1182a:	86d6                	mv	a3,s5
   1182c:	00068703          	lb	a4,0(a3)
   11830:	0685                	add	a3,a3,1
   11832:	00e5a733          	slt	a4,a1,a4
   11836:	167d                	add	a2,a2,-1
   11838:	953a                	add	a0,a0,a4
   1183a:	fa6d                	bnez	a2,1182c <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x56>
   1183c:	942a                	add	s0,s0,a0
   1183e:	8bd6                	mv	s7,s5
   11840:	000c3503          	ld	a0,0(s8)
   11844:	e915                	bnez	a0,11878 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0xa2>
   11846:	a095                	j	118aa <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0xd4>
   11848:	030c2503          	lw	a0,48(s8)
   1184c:	00198413          	add	s0,s3,1
   11850:	02d00a13          	li	s4,45
   11854:	8911                	and	a0,a0,4
   11856:	f161                	bnez	a0,11816 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x40>
   11858:	4b81                	li	s7,0
   1185a:	000c3503          	ld	a0,0(s8)
   1185e:	ed09                	bnez	a0,11878 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0xa2>
   11860:	a0a9                	j	118aa <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0xd4>
   11862:	8556                	mv	a0,s5
   11864:	85da                	mv	a1,s6
   11866:	00001097          	auipc	ra,0x1
   1186a:	97e080e7          	jalr	-1666(ra) # 121e4 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E>
   1186e:	942a                	add	s0,s0,a0
   11870:	8bd6                	mv	s7,s5
   11872:	000c3503          	ld	a0,0(s8)
   11876:	c915                	beqz	a0,118aa <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0xd4>
   11878:	008c3483          	ld	s1,8(s8)
   1187c:	02947763          	bgeu	s0,s1,118aa <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0xd4>
   11880:	030c4503          	lbu	a0,48(s8)
   11884:	8921                	and	a0,a0,8
   11886:	e135                	bnez	a0,118ea <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x114>
   11888:	038c4583          	lbu	a1,56(s8)
   1188c:	460d                	li	a2,3
   1188e:	4505                	li	a0,1
   11890:	00c58363          	beq	a1,a2,11896 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0xc0>
   11894:	852e                	mv	a0,a1
   11896:	00357593          	and	a1,a0,3
   1189a:	40848533          	sub	a0,s1,s0
   1189e:	c9d9                	beqz	a1,11934 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x15e>
   118a0:	4605                	li	a2,1
   118a2:	08c59c63          	bne	a1,a2,1193a <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x164>
   118a6:	4c81                	li	s9,0
   118a8:	a871                	j	11944 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x16e>
   118aa:	8562                	mv	a0,s8
   118ac:	85d2                	mv	a1,s4
   118ae:	865e                	mv	a2,s7
   118b0:	86da                	mv	a3,s6
   118b2:	00000097          	auipc	ra,0x0
   118b6:	1a0080e7          	jalr	416(ra) # 11a52 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h7777cb7311890bc3E>
   118ba:	4a85                	li	s5,1
   118bc:	12051363          	bnez	a0,119e2 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   118c0:	028c3583          	ld	a1,40(s8)
   118c4:	020c3503          	ld	a0,32(s8)
   118c8:	6d9c                	ld	a5,24(a1)
   118ca:	85ca                	mv	a1,s2
   118cc:	864e                	mv	a2,s3
   118ce:	60e6                	ld	ra,88(sp)
   118d0:	6446                	ld	s0,80(sp)
   118d2:	64a6                	ld	s1,72(sp)
   118d4:	6906                	ld	s2,64(sp)
   118d6:	79e2                	ld	s3,56(sp)
   118d8:	7a42                	ld	s4,48(sp)
   118da:	7aa2                	ld	s5,40(sp)
   118dc:	7b02                	ld	s6,32(sp)
   118de:	6be2                	ld	s7,24(sp)
   118e0:	6c42                	ld	s8,16(sp)
   118e2:	6ca2                	ld	s9,8(sp)
   118e4:	6d02                	ld	s10,0(sp)
   118e6:	6125                	add	sp,sp,96
   118e8:	8782                	jr	a5
   118ea:	034c2c83          	lw	s9,52(s8)
   118ee:	03000513          	li	a0,48
   118f2:	038c4d03          	lbu	s10,56(s8)
   118f6:	02ac2a23          	sw	a0,52(s8)
   118fa:	4a85                	li	s5,1
   118fc:	035c0c23          	sb	s5,56(s8)
   11900:	8562                	mv	a0,s8
   11902:	85d2                	mv	a1,s4
   11904:	865e                	mv	a2,s7
   11906:	86da                	mv	a3,s6
   11908:	00000097          	auipc	ra,0x0
   1190c:	14a080e7          	jalr	330(ra) # 11a52 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h7777cb7311890bc3E>
   11910:	e969                	bnez	a0,119e2 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   11912:	038c4583          	lbu	a1,56(s8)
   11916:	460d                	li	a2,3
   11918:	4505                	li	a0,1
   1191a:	00c58363          	beq	a1,a2,11920 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x14a>
   1191e:	852e                	mv	a0,a1
   11920:	00357593          	and	a1,a0,3
   11924:	40848533          	sub	a0,s1,s0
   11928:	c5c9                	beqz	a1,119b2 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x1dc>
   1192a:	4605                	li	a2,1
   1192c:	08c59663          	bne	a1,a2,119b8 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x1e2>
   11930:	4a01                	li	s4,0
   11932:	a841                	j	119c2 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x1ec>
   11934:	8caa                	mv	s9,a0
   11936:	4501                	li	a0,0
   11938:	a031                	j	11944 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x16e>
   1193a:	00150593          	add	a1,a0,1
   1193e:	8105                	srl	a0,a0,0x1
   11940:	0015dc93          	srl	s9,a1,0x1
   11944:	020c3a83          	ld	s5,32(s8)
   11948:	028c3483          	ld	s1,40(s8)
   1194c:	034c2d03          	lw	s10,52(s8)
   11950:	00150413          	add	s0,a0,1
   11954:	147d                	add	s0,s0,-1
   11956:	c419                	beqz	s0,11964 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x18e>
   11958:	7090                	ld	a2,32(s1)
   1195a:	8556                	mv	a0,s5
   1195c:	85ea                	mv	a1,s10
   1195e:	9602                	jalr	a2
   11960:	d975                	beqz	a0,11954 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x17e>
   11962:	a8bd                	j	119e0 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20a>
   11964:	00110537          	lui	a0,0x110
   11968:	4a85                	li	s5,1
   1196a:	06ad0c63          	beq	s10,a0,119e2 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   1196e:	8562                	mv	a0,s8
   11970:	85d2                	mv	a1,s4
   11972:	865e                	mv	a2,s7
   11974:	86da                	mv	a3,s6
   11976:	00000097          	auipc	ra,0x0
   1197a:	0dc080e7          	jalr	220(ra) # 11a52 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h7777cb7311890bc3E>
   1197e:	e135                	bnez	a0,119e2 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   11980:	028c3583          	ld	a1,40(s8)
   11984:	020c3503          	ld	a0,32(s8)
   11988:	6d94                	ld	a3,24(a1)
   1198a:	85ca                	mv	a1,s2
   1198c:	864e                	mv	a2,s3
   1198e:	9682                	jalr	a3
   11990:	e929                	bnez	a0,119e2 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   11992:	020c3903          	ld	s2,32(s8)
   11996:	028c3483          	ld	s1,40(s8)
   1199a:	4401                	li	s0,0
   1199c:	0a8c8763          	beq	s9,s0,11a4a <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x274>
   119a0:	7090                	ld	a2,32(s1)
   119a2:	0405                	add	s0,s0,1
   119a4:	854a                	mv	a0,s2
   119a6:	85ea                	mv	a1,s10
   119a8:	9602                	jalr	a2
   119aa:	d96d                	beqz	a0,1199c <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x1c6>
   119ac:	fff40513          	add	a0,s0,-1
   119b0:	a871                	j	11a4c <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x276>
   119b2:	8a2a                	mv	s4,a0
   119b4:	4501                	li	a0,0
   119b6:	a031                	j	119c2 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x1ec>
   119b8:	00150593          	add	a1,a0,1 # 110001 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xc7c91>
   119bc:	8105                	srl	a0,a0,0x1
   119be:	0015da13          	srl	s4,a1,0x1
   119c2:	020c3a83          	ld	s5,32(s8)
   119c6:	028c3483          	ld	s1,40(s8)
   119ca:	034c2b03          	lw	s6,52(s8)
   119ce:	00150413          	add	s0,a0,1
   119d2:	147d                	add	s0,s0,-1
   119d4:	c415                	beqz	s0,11a00 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x22a>
   119d6:	7090                	ld	a2,32(s1)
   119d8:	8556                	mv	a0,s5
   119da:	85da                	mv	a1,s6
   119dc:	9602                	jalr	a2
   119de:	d975                	beqz	a0,119d2 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x1fc>
   119e0:	4a85                	li	s5,1
   119e2:	8556                	mv	a0,s5
   119e4:	60e6                	ld	ra,88(sp)
   119e6:	6446                	ld	s0,80(sp)
   119e8:	64a6                	ld	s1,72(sp)
   119ea:	6906                	ld	s2,64(sp)
   119ec:	79e2                	ld	s3,56(sp)
   119ee:	7a42                	ld	s4,48(sp)
   119f0:	7aa2                	ld	s5,40(sp)
   119f2:	7b02                	ld	s6,32(sp)
   119f4:	6be2                	ld	s7,24(sp)
   119f6:	6c42                	ld	s8,16(sp)
   119f8:	6ca2                	ld	s9,8(sp)
   119fa:	6d02                	ld	s10,0(sp)
   119fc:	6125                	add	sp,sp,96
   119fe:	8082                	ret
   11a00:	00110537          	lui	a0,0x110
   11a04:	4a85                	li	s5,1
   11a06:	fcab0ee3          	beq	s6,a0,119e2 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   11a0a:	028c3583          	ld	a1,40(s8)
   11a0e:	020c3503          	ld	a0,32(s8)
   11a12:	6d94                	ld	a3,24(a1)
   11a14:	85ca                	mv	a1,s2
   11a16:	864e                	mv	a2,s3
   11a18:	9682                	jalr	a3
   11a1a:	f561                	bnez	a0,119e2 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   11a1c:	020c3903          	ld	s2,32(s8)
   11a20:	028c3483          	ld	s1,40(s8)
   11a24:	4401                	li	s0,0
   11a26:	008a0c63          	beq	s4,s0,11a3e <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x268>
   11a2a:	7090                	ld	a2,32(s1)
   11a2c:	0405                	add	s0,s0,1
   11a2e:	854a                	mv	a0,s2
   11a30:	85da                	mv	a1,s6
   11a32:	9602                	jalr	a2
   11a34:	d96d                	beqz	a0,11a26 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x250>
   11a36:	fff40513          	add	a0,s0,-1
   11a3a:	fb4564e3          	bltu	a0,s4,119e2 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   11a3e:	4a81                	li	s5,0
   11a40:	039c2a23          	sw	s9,52(s8)
   11a44:	03ac0c23          	sb	s10,56(s8)
   11a48:	bf69                	j	119e2 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>
   11a4a:	8566                	mv	a0,s9
   11a4c:	01953ab3          	sltu	s5,a0,s9
   11a50:	bf49                	j	119e2 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E+0x20c>

0000000000011a52 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h7777cb7311890bc3E>:
   11a52:	1101                	add	sp,sp,-32
   11a54:	ec06                	sd	ra,24(sp)
   11a56:	e822                	sd	s0,16(sp)
   11a58:	e426                	sd	s1,8(sp)
   11a5a:	e04a                	sd	s2,0(sp)
   11a5c:	0005871b          	sext.w	a4,a1
   11a60:	001107b7          	lui	a5,0x110
   11a64:	8936                	mv	s2,a3
   11a66:	84b2                	mv	s1,a2
   11a68:	842a                	mv	s0,a0
   11a6a:	00f70963          	beq	a4,a5,11a7c <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h7777cb7311890bc3E+0x2a>
   11a6e:	7410                	ld	a2,40(s0)
   11a70:	7008                	ld	a0,32(s0)
   11a72:	7210                	ld	a2,32(a2)
   11a74:	9602                	jalr	a2
   11a76:	85aa                	mv	a1,a0
   11a78:	4505                	li	a0,1
   11a7a:	ed91                	bnez	a1,11a96 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h7777cb7311890bc3E+0x44>
   11a7c:	cc81                	beqz	s1,11a94 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h7777cb7311890bc3E+0x42>
   11a7e:	740c                	ld	a1,40(s0)
   11a80:	7008                	ld	a0,32(s0)
   11a82:	6d9c                	ld	a5,24(a1)
   11a84:	85a6                	mv	a1,s1
   11a86:	864a                	mv	a2,s2
   11a88:	60e2                	ld	ra,24(sp)
   11a8a:	6442                	ld	s0,16(sp)
   11a8c:	64a2                	ld	s1,8(sp)
   11a8e:	6902                	ld	s2,0(sp)
   11a90:	6105                	add	sp,sp,32
   11a92:	8782                	jr	a5
   11a94:	4501                	li	a0,0
   11a96:	60e2                	ld	ra,24(sp)
   11a98:	6442                	ld	s0,16(sp)
   11a9a:	64a2                	ld	s1,8(sp)
   11a9c:	6902                	ld	s2,0(sp)
   11a9e:	6105                	add	sp,sp,32
   11aa0:	8082                	ret

0000000000011aa2 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E>:
   11aa2:	715d                	add	sp,sp,-80
   11aa4:	e486                	sd	ra,72(sp)
   11aa6:	e0a2                	sd	s0,64(sp)
   11aa8:	fc26                	sd	s1,56(sp)
   11aaa:	f84a                	sd	s2,48(sp)
   11aac:	f44e                	sd	s3,40(sp)
   11aae:	f052                	sd	s4,32(sp)
   11ab0:	ec56                	sd	s5,24(sp)
   11ab2:	e85a                	sd	s6,16(sp)
   11ab4:	e45e                	sd	s7,8(sp)
   11ab6:	84aa                	mv	s1,a0
   11ab8:	00053303          	ld	t1,0(a0) # 110000 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xc7c90>
   11abc:	6914                	ld	a3,16(a0)
   11abe:	fff30713          	add	a4,t1,-1
   11ac2:	00e03733          	snez	a4,a4
   11ac6:	fff68793          	add	a5,a3,-1
   11aca:	00f037b3          	snez	a5,a5
   11ace:	8f7d                	and	a4,a4,a5
   11ad0:	89b2                	mv	s3,a2
   11ad2:	892e                	mv	s2,a1
   11ad4:	16071b63          	bnez	a4,11c4a <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1a8>
   11ad8:	4585                	li	a1,1
   11ada:	10b69763          	bne	a3,a1,11be8 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   11ade:	6c94                	ld	a3,24(s1)
   11ae0:	01390633          	add	a2,s2,s3
   11ae4:	4581                	li	a1,0
   11ae6:	cea5                	beqz	a3,11b5e <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0xbc>
   11ae8:	0e000293          	li	t0,224
   11aec:	0f000893          	li	a7,240
   11af0:	00110837          	lui	a6,0x110
   11af4:	844a                	mv	s0,s2
   11af6:	a811                	j	11b0a <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x68>
   11af8:	00140793          	add	a5,s0,1
   11afc:	40858533          	sub	a0,a1,s0
   11b00:	16fd                	add	a3,a3,-1
   11b02:	00f505b3          	add	a1,a0,a5
   11b06:	843e                	mv	s0,a5
   11b08:	cea1                	beqz	a3,11b60 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0xbe>
   11b0a:	0cc40f63          	beq	s0,a2,11be8 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   11b0e:	00040783          	lb	a5,0(s0)
   11b12:	fe07d3e3          	bgez	a5,11af8 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x56>
   11b16:	0ff7f793          	zext.b	a5,a5
   11b1a:	0257ec63          	bltu	a5,t0,11b52 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0xb0>
   11b1e:	0317ed63          	bltu	a5,a7,11b58 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0xb6>
   11b22:	00144703          	lbu	a4,1(s0)
   11b26:	00244503          	lbu	a0,2(s0)
   11b2a:	03f77713          	and	a4,a4,63
   11b2e:	03f57513          	and	a0,a0,63
   11b32:	00344383          	lbu	t2,3(s0)
   11b36:	17f6                	sll	a5,a5,0x3d
   11b38:	93ad                	srl	a5,a5,0x2b
   11b3a:	0732                	sll	a4,a4,0xc
   11b3c:	051a                	sll	a0,a0,0x6
   11b3e:	8d59                	or	a0,a0,a4
   11b40:	03f3f713          	and	a4,t2,63
   11b44:	8d59                	or	a0,a0,a4
   11b46:	8d5d                	or	a0,a0,a5
   11b48:	0b050063          	beq	a0,a6,11be8 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   11b4c:	00440793          	add	a5,s0,4
   11b50:	b775                	j	11afc <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x5a>
   11b52:	00240793          	add	a5,s0,2
   11b56:	b75d                	j	11afc <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x5a>
   11b58:	00340793          	add	a5,s0,3
   11b5c:	b745                	j	11afc <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x5a>
   11b5e:	87ca                	mv	a5,s2
   11b60:	08c78463          	beq	a5,a2,11be8 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   11b64:	00078603          	lb	a2,0(a5) # 110000 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xc7c90>
   11b68:	04065363          	bgez	a2,11bae <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x10c>
   11b6c:	0ff67613          	zext.b	a2,a2
   11b70:	0e000513          	li	a0,224
   11b74:	02a66d63          	bltu	a2,a0,11bae <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x10c>
   11b78:	0f000513          	li	a0,240
   11b7c:	02a66963          	bltu	a2,a0,11bae <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x10c>
   11b80:	0017c503          	lbu	a0,1(a5)
   11b84:	0027c683          	lbu	a3,2(a5)
   11b88:	03f57513          	and	a0,a0,63
   11b8c:	03f6f693          	and	a3,a3,63
   11b90:	0037c703          	lbu	a4,3(a5)
   11b94:	1676                	sll	a2,a2,0x3d
   11b96:	922d                	srl	a2,a2,0x2b
   11b98:	0532                	sll	a0,a0,0xc
   11b9a:	069a                	sll	a3,a3,0x6
   11b9c:	8d55                	or	a0,a0,a3
   11b9e:	03f77693          	and	a3,a4,63
   11ba2:	8d55                	or	a0,a0,a3
   11ba4:	8d51                	or	a0,a0,a2
   11ba6:	00110637          	lui	a2,0x110
   11baa:	02c50f63          	beq	a0,a2,11be8 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   11bae:	c185                	beqz	a1,11bce <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x12c>
   11bb0:	0335f463          	bgeu	a1,s3,11bd8 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x136>
   11bb4:	00b90533          	add	a0,s2,a1
   11bb8:	00050503          	lb	a0,0(a0)
   11bbc:	fc000613          	li	a2,-64
   11bc0:	10c54763          	blt	a0,a2,11cce <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x22c>
   11bc4:	862e                	mv	a2,a1
   11bc6:	85b2                	mv	a1,a2
   11bc8:	864a                	mv	a2,s2
   11bca:	ce19                	beqz	a2,11be8 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   11bcc:	a821                	j	11be4 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x142>
   11bce:	4601                	li	a2,0
   11bd0:	85b2                	mv	a1,a2
   11bd2:	864a                	mv	a2,s2
   11bd4:	ca11                	beqz	a2,11be8 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   11bd6:	a039                	j	11be4 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x142>
   11bd8:	864e                	mv	a2,s3
   11bda:	0f359a63          	bne	a1,s3,11cce <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x22c>
   11bde:	85b2                	mv	a1,a2
   11be0:	864a                	mv	a2,s2
   11be2:	c219                	beqz	a2,11be8 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   11be4:	89ae                	mv	s3,a1
   11be6:	8932                	mv	s2,a2
   11be8:	06030163          	beqz	t1,11c4a <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1a8>
   11bec:	6480                	ld	s0,8(s1)
   11bee:	02000513          	li	a0,32
   11bf2:	04a9f463          	bgeu	s3,a0,11c3a <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x198>
   11bf6:	4501                	li	a0,0
   11bf8:	00098e63          	beqz	s3,11c14 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x172>
   11bfc:	fbf00593          	li	a1,-65
   11c00:	864e                	mv	a2,s3
   11c02:	86ca                	mv	a3,s2
   11c04:	00068703          	lb	a4,0(a3)
   11c08:	0685                	add	a3,a3,1
   11c0a:	00e5a733          	slt	a4,a1,a4
   11c0e:	167d                	add	a2,a2,-1 # 10ffff <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xc7c8f>
   11c10:	953a                	add	a0,a0,a4
   11c12:	fa6d                	bnez	a2,11c04 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x162>
   11c14:	02857b63          	bgeu	a0,s0,11c4a <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1a8>
   11c18:	0384c583          	lbu	a1,56(s1)
   11c1c:	468d                	li	a3,3
   11c1e:	4601                	li	a2,0
   11c20:	00d58363          	beq	a1,a3,11c26 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x184>
   11c24:	862e                	mv	a2,a1
   11c26:	00367593          	and	a1,a2,3
   11c2a:	40a40533          	sub	a0,s0,a0
   11c2e:	cd95                	beqz	a1,11c6a <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1c8>
   11c30:	4605                	li	a2,1
   11c32:	02c59f63          	bne	a1,a2,11c70 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1ce>
   11c36:	4a81                	li	s5,0
   11c38:	a089                	j	11c7a <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1d8>
   11c3a:	854a                	mv	a0,s2
   11c3c:	85ce                	mv	a1,s3
   11c3e:	00000097          	auipc	ra,0x0
   11c42:	5a6080e7          	jalr	1446(ra) # 121e4 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E>
   11c46:	fc8569e3          	bltu	a0,s0,11c18 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x176>
   11c4a:	748c                	ld	a1,40(s1)
   11c4c:	7088                	ld	a0,32(s1)
   11c4e:	6d9c                	ld	a5,24(a1)
   11c50:	85ca                	mv	a1,s2
   11c52:	864e                	mv	a2,s3
   11c54:	60a6                	ld	ra,72(sp)
   11c56:	6406                	ld	s0,64(sp)
   11c58:	74e2                	ld	s1,56(sp)
   11c5a:	7942                	ld	s2,48(sp)
   11c5c:	79a2                	ld	s3,40(sp)
   11c5e:	7a02                	ld	s4,32(sp)
   11c60:	6ae2                	ld	s5,24(sp)
   11c62:	6b42                	ld	s6,16(sp)
   11c64:	6ba2                	ld	s7,8(sp)
   11c66:	6161                	add	sp,sp,80
   11c68:	8782                	jr	a5
   11c6a:	8aaa                	mv	s5,a0
   11c6c:	4501                	li	a0,0
   11c6e:	a031                	j	11c7a <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1d8>
   11c70:	00150593          	add	a1,a0,1
   11c74:	8105                	srl	a0,a0,0x1
   11c76:	0015da93          	srl	s5,a1,0x1
   11c7a:	0204bb03          	ld	s6,32(s1)
   11c7e:	0284bb83          	ld	s7,40(s1)
   11c82:	58c4                	lw	s1,52(s1)
   11c84:	00150413          	add	s0,a0,1
   11c88:	147d                	add	s0,s0,-1
   11c8a:	c809                	beqz	s0,11c9c <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1fa>
   11c8c:	020bb603          	ld	a2,32(s7)
   11c90:	855a                	mv	a0,s6
   11c92:	85a6                	mv	a1,s1
   11c94:	9602                	jalr	a2
   11c96:	d96d                	beqz	a0,11c88 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x1e6>
   11c98:	4a05                	li	s4,1
   11c9a:	a081                	j	11cda <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x238>
   11c9c:	00110537          	lui	a0,0x110
   11ca0:	4a05                	li	s4,1
   11ca2:	02a48c63          	beq	s1,a0,11cda <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x238>
   11ca6:	018bb683          	ld	a3,24(s7)
   11caa:	855a                	mv	a0,s6
   11cac:	85ca                	mv	a1,s2
   11cae:	864e                	mv	a2,s3
   11cb0:	9682                	jalr	a3
   11cb2:	e505                	bnez	a0,11cda <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x238>
   11cb4:	4401                	li	s0,0
   11cb6:	008a8f63          	beq	s5,s0,11cd4 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x232>
   11cba:	020bb603          	ld	a2,32(s7)
   11cbe:	0405                	add	s0,s0,1
   11cc0:	855a                	mv	a0,s6
   11cc2:	85a6                	mv	a1,s1
   11cc4:	9602                	jalr	a2
   11cc6:	d965                	beqz	a0,11cb6 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x214>
   11cc8:	fff40513          	add	a0,s0,-1
   11ccc:	a029                	j	11cd6 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x234>
   11cce:	4601                	li	a2,0
   11cd0:	de01                	beqz	a2,11be8 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x146>
   11cd2:	bf09                	j	11be4 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E+0x142>
   11cd4:	8556                	mv	a0,s5
   11cd6:	01553a33          	sltu	s4,a0,s5
   11cda:	8552                	mv	a0,s4
   11cdc:	60a6                	ld	ra,72(sp)
   11cde:	6406                	ld	s0,64(sp)
   11ce0:	74e2                	ld	s1,56(sp)
   11ce2:	7942                	ld	s2,48(sp)
   11ce4:	79a2                	ld	s3,40(sp)
   11ce6:	7a02                	ld	s4,32(sp)
   11ce8:	6ae2                	ld	s5,24(sp)
   11cea:	6b42                	ld	s6,16(sp)
   11cec:	6ba2                	ld	s7,8(sp)
   11cee:	6161                	add	sp,sp,80
   11cf0:	8082                	ret

0000000000011cf2 <_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hd677dbeaf8ccd015E>:
   11cf2:	86ae                	mv	a3,a1
   11cf4:	85aa                	mv	a1,a0
   11cf6:	8532                	mv	a0,a2
   11cf8:	8636                	mv	a2,a3
   11cfa:	00000317          	auipc	t1,0x0
   11cfe:	da830067          	jr	-600(t1) # 11aa2 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E>

0000000000011d02 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hdf624fa96493adceE>:
   11d02:	711d                	add	sp,sp,-96
   11d04:	ec86                	sd	ra,88(sp)
   11d06:	e8a2                	sd	s0,80(sp)
   11d08:	e4a6                	sd	s1,72(sp)
   11d0a:	e0ca                	sd	s2,64(sp)
   11d0c:	fc4e                	sd	s3,56(sp)
   11d0e:	f852                	sd	s4,48(sp)
   11d10:	f456                	sd	s5,40(sp)
   11d12:	f05a                	sd	s6,32(sp)
   11d14:	ec5e                	sd	s7,24(sp)
   11d16:	e862                	sd	s8,16(sp)
   11d18:	e466                	sd	s9,8(sp)
   11d1a:	7590                	ld	a2,40(a1)
   11d1c:	0205ba83          	ld	s5,32(a1)
   11d20:	721c                	ld	a5,32(a2)
   11d22:	84aa                	mv	s1,a0
   11d24:	02700593          	li	a1,39
   11d28:	02700413          	li	s0,39
   11d2c:	8556                	mv	a0,s5
   11d2e:	e03e                	sd	a5,0(sp)
   11d30:	9782                	jalr	a5
   11d32:	cd19                	beqz	a0,11d50 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hdf624fa96493adceE+0x4e>
   11d34:	4505                	li	a0,1
   11d36:	60e6                	ld	ra,88(sp)
   11d38:	6446                	ld	s0,80(sp)
   11d3a:	64a6                	ld	s1,72(sp)
   11d3c:	6906                	ld	s2,64(sp)
   11d3e:	79e2                	ld	s3,56(sp)
   11d40:	7a42                	ld	s4,48(sp)
   11d42:	7aa2                	ld	s5,40(sp)
   11d44:	7b02                	ld	s6,32(sp)
   11d46:	6be2                	ld	s7,24(sp)
   11d48:	6c42                	ld	s8,16(sp)
   11d4a:	6ca2                	ld	s9,8(sp)
   11d4c:	6125                	add	sp,sp,96
   11d4e:	8082                	ret
   11d50:	0004e903          	lwu	s2,0(s1)
   11d54:	02b91513          	sll	a0,s2,0x2b
   11d58:	912d                	srl	a0,a0,0x2b
   11d5a:	4489                	li	s1,2
   11d5c:	02a46063          	bltu	s0,a0,11d7c <.LBB242_4+0x6>
   11d60:	00391513          	sll	a0,s2,0x3

0000000000011d64 <.LBB242_31>:
   11d64:	00002597          	auipc	a1,0x2
   11d68:	6a458593          	add	a1,a1,1700 # 14408 <.LJTI242_0>
   11d6c:	952e                	add	a0,a0,a1
   11d6e:	6108                	ld	a0,0(a0)
   11d70:	03000993          	li	s3,48
   11d74:	8502                	jr	a0

0000000000011d76 <.LBB242_4>:
   11d76:	07400993          	li	s3,116
   11d7a:	a065                	j	11e22 <.LBB242_14>
   11d7c:	05c00993          	li	s3,92
   11d80:	03390363          	beq	s2,s3,11da6 <.LBB242_9+0x4>

0000000000011d84 <.LBB242_6>:
   11d84:	854a                	mv	a0,s2
   11d86:	00001097          	auipc	ra,0x1
   11d8a:	1ca080e7          	jalr	458(ra) # 12f50 <_ZN4core7unicode12unicode_data15grapheme_extend6lookup17h412ef2d588f4e502E>
   11d8e:	e11d                	bnez	a0,11db4 <.LBB242_12+0x6>
   11d90:	854a                	mv	a0,s2
   11d92:	00001097          	auipc	ra,0x1
   11d96:	9e2080e7          	jalr	-1566(ra) # 12774 <_ZN4core7unicode9printable12is_printable17h190ab7f3e4a90aabE>
   11d9a:	cd09                	beqz	a0,11db4 <.LBB242_12+0x6>
   11d9c:	4485                	li	s1,1
   11d9e:	89ca                	mv	s3,s2
   11da0:	a049                	j	11e22 <.LBB242_14>

0000000000011da2 <.LBB242_9>:
   11da2:	06e00993          	li	s3,110
   11da6:	a8b5                	j	11e22 <.LBB242_14>

0000000000011da8 <.LBB242_11>:
   11da8:	07200993          	li	s3,114
   11dac:	a89d                	j	11e22 <.LBB242_14>

0000000000011dae <.LBB242_12>:
   11dae:	02700993          	li	s3,39
   11db2:	a885                	j	11e22 <.LBB242_14>
   11db4:	00196513          	or	a0,s2,1
   11db8:	00155593          	srl	a1,a0,0x1
   11dbc:	8d4d                	or	a0,a0,a1
   11dbe:	00255593          	srl	a1,a0,0x2
   11dc2:	8d4d                	or	a0,a0,a1
   11dc4:	00455593          	srl	a1,a0,0x4
   11dc8:	8d4d                	or	a0,a0,a1
   11dca:	00855593          	srl	a1,a0,0x8
   11dce:	8d4d                	or	a0,a0,a1
   11dd0:	0105559b          	srlw	a1,a0,0x10
   11dd4:	8d4d                	or	a0,a0,a1
   11dd6:	fff54513          	not	a0,a0
   11dda:	00155593          	srl	a1,a0,0x1
   11dde:	55555637          	lui	a2,0x55555
   11de2:	5556061b          	addw	a2,a2,1365 # 55555555 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0x5550d1e5>
   11de6:	8df1                	and	a1,a1,a2
   11de8:	9d0d                	subw	a0,a0,a1
   11dea:	333335b7          	lui	a1,0x33333
   11dee:	3335859b          	addw	a1,a1,819 # 33333333 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0x332eafc3>
   11df2:	00b57633          	and	a2,a0,a1
   11df6:	8109                	srl	a0,a0,0x2
   11df8:	8d6d                	and	a0,a0,a1
   11dfa:	9532                	add	a0,a0,a2
   11dfc:	00455593          	srl	a1,a0,0x4
   11e00:	952e                	add	a0,a0,a1
   11e02:	0f0f15b7          	lui	a1,0xf0f1
   11e06:	f0f5859b          	addw	a1,a1,-241 # f0f0f0f <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xf0a8b9f>
   11e0a:	8d6d                	and	a0,a0,a1
   11e0c:	010105b7          	lui	a1,0x1010
   11e10:	1015859b          	addw	a1,a1,257 # 1010101 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xfc7d91>
   11e14:	02b5053b          	mulw	a0,a0,a1
   11e18:	01a5551b          	srlw	a0,a0,0x1a
   11e1c:	00754a13          	xor	s4,a0,7
   11e20:	448d                	li	s1,3

0000000000011e22 <.LBB242_14>:
   11e22:	4415                	li	s0,5
   11e24:	4c85                	li	s9,1
   11e26:	4b09                	li	s6,2

0000000000011e28 <.LBB242_32>:
   11e28:	00002b97          	auipc	s7,0x2
   11e2c:	720b8b93          	add	s7,s7,1824 # 14548 <.LJTI242_1>
   11e30:	4c29                	li	s8,10
   11e32:	a039                	j	11e40 <.LBB242_16+0xa>
   11e34:	4485                	li	s1,1

0000000000011e36 <.LBB242_16>:
   11e36:	8556                	mv	a0,s5
   11e38:	6782                	ld	a5,0(sp)
   11e3a:	9782                	jalr	a5
   11e3c:	ee051ce3          	bnez	a0,11d34 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hdf624fa96493adceE+0x32>
   11e40:	049cd763          	bge	s9,s1,11e8e <.LBB242_20+0x2e>
   11e44:	05c00593          	li	a1,92
   11e48:	ff6486e3          	beq	s1,s6,11e34 <.LBB242_32+0xc>
   11e4c:	0ff47513          	zext.b	a0,s0
   11e50:	050e                	sll	a0,a0,0x3
   11e52:	955e                	add	a0,a0,s7
   11e54:	6108                	ld	a0,0(a0)
   11e56:	448d                	li	s1,3
   11e58:	07d00593          	li	a1,125
   11e5c:	4401                	li	s0,0
   11e5e:	8502                	jr	a0

0000000000011e60 <.LBB242_20>:
   11e60:	002a151b          	sllw	a0,s4,0x2
   11e64:	00a9553b          	srlw	a0,s2,a0
   11e68:	893d                	and	a0,a0,15
   11e6a:	03000593          	li	a1,48
   11e6e:	01856463          	bltu	a0,s8,11e76 <.LBB242_20+0x16>
   11e72:	05700593          	li	a1,87
   11e76:	4601                	li	a2,0
   11e78:	000a0463          	beqz	s4,11e80 <.LBB242_20+0x20>
   11e7c:	fffa0613          	add	a2,s4,-1 # 10ffff <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xc7c8f>
   11e80:	95aa                	add	a1,a1,a0
   11e82:	001a3513          	seqz	a0,s4
   11e86:	40ab0433          	sub	s0,s6,a0
   11e8a:	8a32                	mv	s4,a2
   11e8c:	b76d                	j	11e36 <.LBB242_16>
   11e8e:	03949263          	bne	s1,s9,11eb2 <.LBB242_30>
   11e92:	4481                	li	s1,0
   11e94:	85ce                	mv	a1,s3
   11e96:	b745                	j	11e36 <.LBB242_16>

0000000000011e98 <.LBB242_27>:
   11e98:	4409                	li	s0,2
   11e9a:	07b00593          	li	a1,123
   11e9e:	bf61                	j	11e36 <.LBB242_16>

0000000000011ea0 <.LBB242_28>:
   11ea0:	440d                	li	s0,3
   11ea2:	07500593          	li	a1,117
   11ea6:	448d                	li	s1,3
   11ea8:	b779                	j	11e36 <.LBB242_16>

0000000000011eaa <.LBB242_29>:
   11eaa:	4411                	li	s0,4
   11eac:	05c00593          	li	a1,92
   11eb0:	b759                	j	11e36 <.LBB242_16>

0000000000011eb2 <.LBB242_30>:
   11eb2:	02700593          	li	a1,39
   11eb6:	8556                	mv	a0,s5
   11eb8:	6782                	ld	a5,0(sp)
   11eba:	60e6                	ld	ra,88(sp)
   11ebc:	6446                	ld	s0,80(sp)
   11ebe:	64a6                	ld	s1,72(sp)
   11ec0:	6906                	ld	s2,64(sp)
   11ec2:	79e2                	ld	s3,56(sp)
   11ec4:	7a42                	ld	s4,48(sp)
   11ec6:	7aa2                	ld	s5,40(sp)
   11ec8:	7b02                	ld	s6,32(sp)
   11eca:	6be2                	ld	s7,24(sp)
   11ecc:	6c42                	ld	s8,16(sp)
   11ece:	6ca2                	ld	s9,8(sp)
   11ed0:	6125                	add	sp,sp,96
   11ed2:	8782                	jr	a5

0000000000011ed4 <_ZN4core5slice5index26slice_start_index_len_fail17hc36fc5bb321621f5E>:
   11ed4:	1141                	add	sp,sp,-16
   11ed6:	e406                	sd	ra,8(sp)
   11ed8:	fffff097          	auipc	ra,0xfffff
   11edc:	e94080e7          	jalr	-364(ra) # 10d6c <_ZN4core3ops8function6FnOnce9call_once17h0008a32bd325903dE>
	...

0000000000011ee2 <_ZN4core5slice5index29slice_start_index_len_fail_rt17h66247b7e841f83e5E>:
   11ee2:	7159                	add	sp,sp,-112
   11ee4:	f486                	sd	ra,104(sp)
   11ee6:	e42a                	sd	a0,8(sp)
   11ee8:	e82e                	sd	a1,16(sp)
   11eea:	0028                	add	a0,sp,8
   11eec:	e4aa                	sd	a0,72(sp)

0000000000011eee <.LBB259_1>:
   11eee:	00001517          	auipc	a0,0x1
   11ef2:	dfc50513          	add	a0,a0,-516 # 12cea <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h68be92e0167cbc01E>
   11ef6:	e8aa                	sd	a0,80(sp)
   11ef8:	080c                	add	a1,sp,16
   11efa:	ecae                	sd	a1,88(sp)
   11efc:	f0aa                	sd	a0,96(sp)

0000000000011efe <.LBB259_2>:
   11efe:	00003517          	auipc	a0,0x3
   11f02:	98250513          	add	a0,a0,-1662 # 14880 <.Lanon.442aba94db1f841cd37d39ada1516238.276>
   11f06:	ec2a                	sd	a0,24(sp)
   11f08:	4509                	li	a0,2
   11f0a:	f02a                	sd	a0,32(sp)
   11f0c:	f402                	sd	zero,40(sp)
   11f0e:	00ac                	add	a1,sp,72
   11f10:	fc2e                	sd	a1,56(sp)
   11f12:	e0aa                	sd	a0,64(sp)

0000000000011f14 <.LBB259_3>:
   11f14:	00003597          	auipc	a1,0x3
   11f18:	9ac58593          	add	a1,a1,-1620 # 148c0 <.Lanon.442aba94db1f841cd37d39ada1516238.278>
   11f1c:	0828                	add	a0,sp,24
   11f1e:	fffff097          	auipc	ra,0xfffff
   11f22:	fde080e7          	jalr	-34(ra) # 10efc <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

0000000000011f28 <_ZN4core5slice5index24slice_end_index_len_fail17h5d1e1d044f43082eE>:
   11f28:	1141                	add	sp,sp,-16
   11f2a:	e406                	sd	ra,8(sp)
   11f2c:	fffff097          	auipc	ra,0xfffff
   11f30:	e60080e7          	jalr	-416(ra) # 10d8c <_ZN4core3ops8function6FnOnce9call_once17had1f8e39903f1947E>
	...

0000000000011f36 <_ZN4core5slice5index27slice_end_index_len_fail_rt17h3a149a007ccdb3bbE>:
   11f36:	7159                	add	sp,sp,-112
   11f38:	f486                	sd	ra,104(sp)
   11f3a:	e42a                	sd	a0,8(sp)
   11f3c:	e82e                	sd	a1,16(sp)
   11f3e:	0028                	add	a0,sp,8
   11f40:	e4aa                	sd	a0,72(sp)

0000000000011f42 <.LBB262_1>:
   11f42:	00001517          	auipc	a0,0x1
   11f46:	da850513          	add	a0,a0,-600 # 12cea <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h68be92e0167cbc01E>
   11f4a:	e8aa                	sd	a0,80(sp)
   11f4c:	080c                	add	a1,sp,16
   11f4e:	ecae                	sd	a1,88(sp)
   11f50:	f0aa                	sd	a0,96(sp)

0000000000011f52 <.LBB262_2>:
   11f52:	00003517          	auipc	a0,0x3
   11f56:	98650513          	add	a0,a0,-1658 # 148d8 <.Lanon.442aba94db1f841cd37d39ada1516238.283>
   11f5a:	ec2a                	sd	a0,24(sp)
   11f5c:	4509                	li	a0,2
   11f5e:	f02a                	sd	a0,32(sp)
   11f60:	f402                	sd	zero,40(sp)
   11f62:	00ac                	add	a1,sp,72
   11f64:	fc2e                	sd	a1,56(sp)
   11f66:	e0aa                	sd	a0,64(sp)

0000000000011f68 <.LBB262_3>:
   11f68:	00003597          	auipc	a1,0x3
   11f6c:	99058593          	add	a1,a1,-1648 # 148f8 <.Lanon.442aba94db1f841cd37d39ada1516238.284>
   11f70:	0828                	add	a0,sp,24
   11f72:	fffff097          	auipc	ra,0xfffff
   11f76:	f8a080e7          	jalr	-118(ra) # 10efc <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

0000000000011f7c <_ZN4core5slice5index22slice_index_order_fail17h5b8db1271a95aea8E>:
   11f7c:	1141                	add	sp,sp,-16
   11f7e:	e406                	sd	ra,8(sp)
   11f80:	fffff097          	auipc	ra,0xfffff
   11f84:	dfa080e7          	jalr	-518(ra) # 10d7a <_ZN4core3ops8function6FnOnce9call_once17h0ccd98de653a7264E>
	...

0000000000011f8a <_ZN4core5slice5index25slice_index_order_fail_rt17h814668a4a9208686E>:
   11f8a:	7159                	add	sp,sp,-112
   11f8c:	f486                	sd	ra,104(sp)
   11f8e:	e42a                	sd	a0,8(sp)
   11f90:	e82e                	sd	a1,16(sp)
   11f92:	0028                	add	a0,sp,8
   11f94:	e4aa                	sd	a0,72(sp)

0000000000011f96 <.LBB265_1>:
   11f96:	00001517          	auipc	a0,0x1
   11f9a:	d5450513          	add	a0,a0,-684 # 12cea <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h68be92e0167cbc01E>
   11f9e:	e8aa                	sd	a0,80(sp)
   11fa0:	080c                	add	a1,sp,16
   11fa2:	ecae                	sd	a1,88(sp)
   11fa4:	f0aa                	sd	a0,96(sp)

0000000000011fa6 <.LBB265_2>:
   11fa6:	00003517          	auipc	a0,0x3
   11faa:	99250513          	add	a0,a0,-1646 # 14938 <.Lanon.442aba94db1f841cd37d39ada1516238.290>
   11fae:	ec2a                	sd	a0,24(sp)
   11fb0:	4509                	li	a0,2
   11fb2:	f02a                	sd	a0,32(sp)
   11fb4:	f402                	sd	zero,40(sp)
   11fb6:	00ac                	add	a1,sp,72
   11fb8:	fc2e                	sd	a1,56(sp)
   11fba:	e0aa                	sd	a0,64(sp)

0000000000011fbc <.LBB265_3>:
   11fbc:	00003597          	auipc	a1,0x3
   11fc0:	99c58593          	add	a1,a1,-1636 # 14958 <.Lanon.442aba94db1f841cd37d39ada1516238.291>
   11fc4:	0828                	add	a0,sp,24
   11fc6:	fffff097          	auipc	ra,0xfffff
   11fca:	f36080e7          	jalr	-202(ra) # 10efc <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

0000000000011fd0 <_ZN4core3str8converts9from_utf817hbe620603d93abf90E>:
   11fd0:	715d                	add	sp,sp,-80
   11fd2:	e4a2                	sd	s0,72(sp)
   11fd4:	e0a6                	sd	s1,64(sp)
   11fd6:	fc4a                	sd	s2,56(sp)
   11fd8:	f84e                	sd	s3,48(sp)
   11fda:	f452                	sd	s4,40(sp)
   11fdc:	f056                	sd	s5,32(sp)
   11fde:	ec5a                	sd	s6,24(sp)
   11fe0:	e85e                	sd	s7,16(sp)
   11fe2:	e462                	sd	s8,8(sp)
   11fe4:	e066                	sd	s9,0(sp)
   11fe6:	ff160693          	add	a3,a2,-15
   11fea:	4c81                	li	s9,0
   11fec:	00d66363          	bltu	a2,a3,11ff2 <_ZN4core3str8converts9from_utf817hbe620603d93abf90E+0x22>
   11ff0:	8cb6                	mv	s9,a3
   11ff2:	1a060a63          	beqz	a2,121a6 <.LBB274_50+0x19a>
   11ff6:	4681                	li	a3,0
   11ff8:	00758713          	add	a4,a1,7
   11ffc:	9b61                	and	a4,a4,-8
   11ffe:	40b70833          	sub	a6,a4,a1

0000000000012002 <.LBB274_49>:
   12002:	00004717          	auipc	a4,0x4
   12006:	26670713          	add	a4,a4,614 # 16268 <.LCPI274_0>
   1200a:	631c                	ld	a5,0(a4)

000000000001200c <.LBB274_50>:
   1200c:	00003897          	auipc	a7,0x3
   12010:	96488893          	add	a7,a7,-1692 # 14970 <.Lanon.442aba94db1f841cd37d39ada1516238.311>
   12014:	4291                	li	t0,4
   12016:	0f000313          	li	t1,240
   1201a:	03000393          	li	t2,48
   1201e:	fbf00e13          	li	t3,-65
   12022:	0f400e93          	li	t4,244
   12026:	f8f00f13          	li	t5,-113
   1202a:	4f8d                	li	t6,3
   1202c:	0e000a93          	li	s5,224
   12030:	fa000913          	li	s2,-96
   12034:	0ed00a13          	li	s4,237
   12038:	49b1                	li	s3,12
   1203a:	4b09                	li	s6,2
   1203c:	4b85                	li	s7,1
   1203e:	a021                	j	12046 <.LBB274_50+0x3a>
   12040:	0685                	add	a3,a3,1
   12042:	16c6f263          	bgeu	a3,a2,121a6 <.LBB274_50+0x19a>
   12046:	00d58733          	add	a4,a1,a3
   1204a:	00074403          	lbu	s0,0(a4)
   1204e:	03841713          	sll	a4,s0,0x38
   12052:	43875493          	sra	s1,a4,0x38
   12056:	0404c663          	bltz	s1,120a2 <.LBB274_50+0x96>
   1205a:	00180713          	add	a4,a6,1 # 110001 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xc7c91>
   1205e:	00e03733          	snez	a4,a4
   12062:	40d8043b          	subw	s0,a6,a3
   12066:	881d                	and	s0,s0,7
   12068:	00143413          	seqz	s0,s0
   1206c:	8f61                	and	a4,a4,s0
   1206e:	db69                	beqz	a4,12040 <.LBB274_50+0x34>
   12070:	0196fd63          	bgeu	a3,s9,1208a <.LBB274_50+0x7e>
   12074:	00d58733          	add	a4,a1,a3
   12078:	6300                	ld	s0,0(a4)
   1207a:	8c7d                	and	s0,s0,a5
   1207c:	e419                	bnez	s0,1208a <.LBB274_50+0x7e>
   1207e:	6718                	ld	a4,8(a4)
   12080:	8f7d                	and	a4,a4,a5
   12082:	e701                	bnez	a4,1208a <.LBB274_50+0x7e>
   12084:	06c1                	add	a3,a3,16
   12086:	ff96e7e3          	bltu	a3,s9,12074 <.LBB274_50+0x68>
   1208a:	10c6fc63          	bgeu	a3,a2,121a2 <.LBB274_50+0x196>
   1208e:	00d58733          	add	a4,a1,a3
   12092:	00070703          	lb	a4,0(a4)
   12096:	10074663          	bltz	a4,121a2 <.LBB274_50+0x196>
   1209a:	0685                	add	a3,a3,1
   1209c:	fed619e3          	bne	a2,a3,1208e <.LBB274_50+0x82>
   120a0:	a219                	j	121a6 <.LBB274_50+0x19a>
   120a2:	01140733          	add	a4,s0,a7
   120a6:	00074703          	lbu	a4,0(a4)
   120aa:	02570263          	beq	a4,t0,120ce <.LBB274_50+0xc2>
   120ae:	03f70e63          	beq	a4,t6,120ea <.LBB274_50+0xde>
   120b2:	11671d63          	bne	a4,s6,121cc <.LBB274_50+0x1c0>
   120b6:	00168413          	add	s0,a3,1
   120ba:	10c47663          	bgeu	s0,a2,121c6 <.LBB274_50+0x1ba>
   120be:	00858733          	add	a4,a1,s0
   120c2:	00070703          	lb	a4,0(a4)
   120c6:	4485                	li	s1,1
   120c8:	0cee5b63          	bge	t3,a4,1219e <.LBB274_50+0x192>
   120cc:	a229                	j	121d6 <.LBB274_50+0x1ca>
   120ce:	00168713          	add	a4,a3,1
   120d2:	0ec77c63          	bgeu	a4,a2,121ca <.LBB274_50+0x1be>
   120d6:	972e                	add	a4,a4,a1
   120d8:	00070c03          	lb	s8,0(a4)
   120dc:	02640563          	beq	s0,t1,12106 <.LBB274_50+0xfa>
   120e0:	03d41a63          	bne	s0,t4,12114 <.LBB274_50+0x108>
   120e4:	058f5563          	bge	t5,s8,1212e <.LBB274_50+0x122>
   120e8:	a0d5                	j	121cc <.LBB274_50+0x1c0>
   120ea:	00168713          	add	a4,a3,1
   120ee:	0cc77e63          	bgeu	a4,a2,121ca <.LBB274_50+0x1be>
   120f2:	972e                	add	a4,a4,a1
   120f4:	00070c03          	lb	s8,0(a4)
   120f8:	05540f63          	beq	s0,s5,12156 <.LBB274_50+0x14a>
   120fc:	07441263          	bne	s0,s4,12160 <.LBB274_50+0x154>
   12100:	092c4563          	blt	s8,s2,1218a <.LBB274_50+0x17e>
   12104:	a0e1                	j	121cc <.LBB274_50+0x1c0>
   12106:	070c071b          	addw	a4,s8,112
   1210a:	0ff77713          	zext.b	a4,a4
   1210e:	02776063          	bltu	a4,t2,1212e <.LBB274_50+0x122>
   12112:	a86d                	j	121cc <.LBB274_50+0x1c0>
   12114:	00f4871b          	addw	a4,s1,15
   12118:	0ff77713          	zext.b	a4,a4
   1211c:	00373713          	sltiu	a4,a4,3
   12120:	000c2413          	slti	s0,s8,0
   12124:	8f61                	and	a4,a4,s0
   12126:	fc0c3413          	sltiu	s0,s8,-64
   1212a:	8f61                	and	a4,a4,s0
   1212c:	c345                	beqz	a4,121cc <.LBB274_50+0x1c0>
   1212e:	00268713          	add	a4,a3,2
   12132:	08c77a63          	bgeu	a4,a2,121c6 <.LBB274_50+0x1ba>
   12136:	972e                	add	a4,a4,a1
   12138:	00070703          	lb	a4,0(a4)
   1213c:	08ee4a63          	blt	t3,a4,121d0 <.LBB274_50+0x1c4>
   12140:	00368413          	add	s0,a3,3
   12144:	08c47163          	bgeu	s0,a2,121c6 <.LBB274_50+0x1ba>
   12148:	00858733          	add	a4,a1,s0
   1214c:	00070703          	lb	a4,0(a4)
   12150:	04ee5763          	bge	t3,a4,1219e <.LBB274_50+0x192>
   12154:	a041                	j	121d4 <.LBB274_50+0x1c8>
   12156:	fe0c7713          	and	a4,s8,-32
   1215a:	03270863          	beq	a4,s2,1218a <.LBB274_50+0x17e>
   1215e:	a0bd                	j	121cc <.LBB274_50+0x1c0>
   12160:	01f4871b          	addw	a4,s1,31
   12164:	0ff77713          	zext.b	a4,a4
   12168:	01377563          	bgeu	a4,s3,12172 <.LBB274_50+0x166>
   1216c:	018e5f63          	bge	t3,s8,1218a <.LBB274_50+0x17e>
   12170:	a8b1                	j	121cc <.LBB274_50+0x1c0>
   12172:	ffe4f713          	and	a4,s1,-2
   12176:	0749                	add	a4,a4,18
   12178:	00173713          	seqz	a4,a4
   1217c:	000c2413          	slti	s0,s8,0
   12180:	8f61                	and	a4,a4,s0
   12182:	fc0c3413          	sltiu	s0,s8,-64
   12186:	8f61                	and	a4,a4,s0
   12188:	c331                	beqz	a4,121cc <.LBB274_50+0x1c0>
   1218a:	00268413          	add	s0,a3,2
   1218e:	02c47c63          	bgeu	s0,a2,121c6 <.LBB274_50+0x1ba>
   12192:	00858733          	add	a4,a1,s0
   12196:	00070703          	lb	a4,0(a4)
   1219a:	02ee4b63          	blt	t3,a4,121d0 <.LBB274_50+0x1c4>
   1219e:	00140693          	add	a3,s0,1
   121a2:	eac6e2e3          	bltu	a3,a2,12046 <.LBB274_50+0x3a>
   121a6:	4681                	li	a3,0
   121a8:	e50c                	sd	a1,8(a0)
   121aa:	e910                	sd	a2,16(a0)
   121ac:	e114                	sd	a3,0(a0)
   121ae:	6426                	ld	s0,72(sp)
   121b0:	6486                	ld	s1,64(sp)
   121b2:	7962                	ld	s2,56(sp)
   121b4:	79c2                	ld	s3,48(sp)
   121b6:	7a22                	ld	s4,40(sp)
   121b8:	7a82                	ld	s5,32(sp)
   121ba:	6b62                	ld	s6,24(sp)
   121bc:	6bc2                	ld	s7,16(sp)
   121be:	6c22                	ld	s8,8(sp)
   121c0:	6c82                	ld	s9,0(sp)
   121c2:	6161                	add	sp,sp,80
   121c4:	8082                	ret
   121c6:	4b81                	li	s7,0
   121c8:	a039                	j	121d6 <.LBB274_50+0x1ca>
   121ca:	4b81                	li	s7,0
   121cc:	4485                	li	s1,1
   121ce:	a021                	j	121d6 <.LBB274_50+0x1ca>
   121d0:	4489                	li	s1,2
   121d2:	a011                	j	121d6 <.LBB274_50+0x1ca>
   121d4:	448d                	li	s1,3
   121d6:	e514                	sd	a3,8(a0)
   121d8:	01750823          	sb	s7,16(a0)
   121dc:	009508a3          	sb	s1,17(a0)
   121e0:	4685                	li	a3,1
   121e2:	b7e9                	j	121ac <.LBB274_50+0x1a0>

00000000000121e4 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E>:
   121e4:	862a                	mv	a2,a0
   121e6:	051d                	add	a0,a0,7
   121e8:	9961                	and	a0,a0,-8
   121ea:	40c508b3          	sub	a7,a0,a2
   121ee:	0115eb63          	bltu	a1,a7,12204 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x20>
   121f2:	41158833          	sub	a6,a1,a7
   121f6:	00883693          	sltiu	a3,a6,8
   121fa:	4721                	li	a4,8
   121fc:	01173733          	sltu	a4,a4,a7
   12200:	8ed9                	or	a3,a3,a4
   12202:	ce91                	beqz	a3,1221e <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x3a>
   12204:	4501                	li	a0,0
   12206:	c999                	beqz	a1,1221c <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x38>
   12208:	fbf00693          	li	a3,-65
   1220c:	00060703          	lb	a4,0(a2)
   12210:	0605                	add	a2,a2,1
   12212:	00e6a733          	slt	a4,a3,a4
   12216:	15fd                	add	a1,a1,-1
   12218:	953a                	add	a0,a0,a4
   1221a:	f9ed                	bnez	a1,1220c <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x28>
   1221c:	8082                	ret
   1221e:	00787693          	and	a3,a6,7
   12222:	4701                	li	a4,0
   12224:	00088f63          	beqz	a7,12242 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x5e>
   12228:	40a60533          	sub	a0,a2,a0
   1222c:	fbf00293          	li	t0,-65
   12230:	87b2                	mv	a5,a2
   12232:	00078583          	lb	a1,0(a5)
   12236:	0785                	add	a5,a5,1
   12238:	00b2a5b3          	slt	a1,t0,a1
   1223c:	0505                	add	a0,a0,1
   1223e:	972e                	add	a4,a4,a1
   12240:	f96d                	bnez	a0,12232 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x4e>
   12242:	011602b3          	add	t0,a2,a7
   12246:	4581                	li	a1,0
   12248:	ce99                	beqz	a3,12266 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x82>
   1224a:	ff887513          	and	a0,a6,-8
   1224e:	00a28633          	add	a2,t0,a0
   12252:	fbf00513          	li	a0,-65
   12256:	00060783          	lb	a5,0(a2)
   1225a:	0605                	add	a2,a2,1
   1225c:	00f527b3          	slt	a5,a0,a5
   12260:	16fd                	add	a3,a3,-1
   12262:	95be                	add	a1,a1,a5
   12264:	faed                	bnez	a3,12256 <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x72>
   12266:	00385613          	srl	a2,a6,0x3

000000000001226a <.LBB276_25>:
   1226a:	00004517          	auipc	a0,0x4
   1226e:	00e50513          	add	a0,a0,14 # 16278 <.LCPI276_0>
   12272:	00053e83          	ld	t4,0(a0)

0000000000012276 <.LBB276_26>:
   12276:	00004517          	auipc	a0,0x4
   1227a:	00a50513          	add	a0,a0,10 # 16280 <.LCPI276_1>
   1227e:	00053883          	ld	a7,0(a0)
   12282:	10001537          	lui	a0,0x10001
   12286:	0512                	sll	a0,a0,0x4
   12288:	0505                	add	a0,a0,1 # 10001001 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xffb8c91>
   1228a:	0542                	sll	a0,a0,0x10
   1228c:	00150813          	add	a6,a0,1
   12290:	00e58533          	add	a0,a1,a4
   12294:	a025                	j	122bc <.LBB276_26+0x46>
   12296:	00339593          	sll	a1,t2,0x3
   1229a:	92ae                	add	t0,t0,a1
   1229c:	40730633          	sub	a2,t1,t2
   122a0:	0033f393          	and	t2,t2,3
   122a4:	0117f5b3          	and	a1,a5,a7
   122a8:	83a1                	srl	a5,a5,0x8
   122aa:	0117f7b3          	and	a5,a5,a7
   122ae:	95be                	add	a1,a1,a5
   122b0:	030585b3          	mul	a1,a1,a6
   122b4:	91c1                	srl	a1,a1,0x30
   122b6:	952e                	add	a0,a0,a1
   122b8:	06039f63          	bnez	t2,12336 <.LBB276_26+0xc0>
   122bc:	d225                	beqz	a2,1221c <_ZN4core3str5count14do_count_chars17hd57045ad0c285889E+0x38>
   122be:	8332                	mv	t1,a2
   122c0:	0c000593          	li	a1,192
   122c4:	83b2                	mv	t2,a2
   122c6:	00b66463          	bltu	a2,a1,122ce <.LBB276_26+0x58>
   122ca:	0c000393          	li	t2,192
   122ce:	0fc3f593          	and	a1,t2,252
   122d2:	058e                	sll	a1,a1,0x3
   122d4:	00b28e33          	add	t3,t0,a1
   122d8:	4781                	li	a5,0
   122da:	ddd5                	beqz	a1,12296 <.LBB276_26+0x20>
   122dc:	8596                	mv	a1,t0
   122de:	6190                	ld	a2,0(a1)
   122e0:	fff64713          	not	a4,a2
   122e4:	831d                	srl	a4,a4,0x7
   122e6:	8219                	srl	a2,a2,0x6
   122e8:	6594                	ld	a3,8(a1)
   122ea:	8e59                	or	a2,a2,a4
   122ec:	01d67633          	and	a2,a2,t4
   122f0:	963e                	add	a2,a2,a5
   122f2:	fff6c713          	not	a4,a3
   122f6:	831d                	srl	a4,a4,0x7
   122f8:	8299                	srl	a3,a3,0x6
   122fa:	699c                	ld	a5,16(a1)
   122fc:	8ed9                	or	a3,a3,a4
   122fe:	01d6f6b3          	and	a3,a3,t4
   12302:	9636                	add	a2,a2,a3
   12304:	fff7c693          	not	a3,a5
   12308:	829d                	srl	a3,a3,0x7
   1230a:	0067d713          	srl	a4,a5,0x6
   1230e:	6d9c                	ld	a5,24(a1)
   12310:	8ed9                	or	a3,a3,a4
   12312:	01d6f6b3          	and	a3,a3,t4
   12316:	9636                	add	a2,a2,a3
   12318:	fff7c693          	not	a3,a5
   1231c:	829d                	srl	a3,a3,0x7
   1231e:	0067d713          	srl	a4,a5,0x6
   12322:	8ed9                	or	a3,a3,a4
   12324:	01d6f6b3          	and	a3,a3,t4
   12328:	02058593          	add	a1,a1,32
   1232c:	00c687b3          	add	a5,a3,a2
   12330:	fabe17e3          	bne	t3,a1,122de <.LBB276_26+0x68>
   12334:	b78d                	j	12296 <.LBB276_26+0x20>
   12336:	0c000593          	li	a1,192
   1233a:	00b36463          	bltu	t1,a1,12342 <.LBB276_26+0xcc>
   1233e:	0c000313          	li	t1,192
   12342:	4581                	li	a1,0
   12344:	00337613          	and	a2,t1,3
   12348:	060e                	sll	a2,a2,0x3
   1234a:	000e3683          	ld	a3,0(t3)
   1234e:	0e21                	add	t3,t3,8
   12350:	fff6c713          	not	a4,a3
   12354:	831d                	srl	a4,a4,0x7
   12356:	8299                	srl	a3,a3,0x6
   12358:	8ed9                	or	a3,a3,a4
   1235a:	01d6f6b3          	and	a3,a3,t4
   1235e:	1661                	add	a2,a2,-8
   12360:	95b6                	add	a1,a1,a3
   12362:	f665                	bnez	a2,1234a <.LBB276_26+0xd4>
   12364:	0115f633          	and	a2,a1,a7
   12368:	81a1                	srl	a1,a1,0x8
   1236a:	0115f5b3          	and	a1,a1,a7
   1236e:	95b2                	add	a1,a1,a2
   12370:	030585b3          	mul	a1,a1,a6
   12374:	91c1                	srl	a1,a1,0x30
   12376:	952e                	add	a0,a0,a1
   12378:	8082                	ret

000000000001237a <_ZN4core3str16slice_error_fail17h0f23970489177861E>:
   1237a:	7179                	add	sp,sp,-48
   1237c:	f406                	sd	ra,40(sp)
   1237e:	e42a                	sd	a0,8(sp)
   12380:	e82e                	sd	a1,16(sp)
   12382:	ec32                	sd	a2,24(sp)
   12384:	f036                	sd	a3,32(sp)
   12386:	0028                	add	a0,sp,8
   12388:	fffff097          	auipc	ra,0xfffff
   1238c:	a22080e7          	jalr	-1502(ra) # 10daa <_ZN4core10intrinsics17const_eval_select17h4d2f7b41c60bf971E>
	...

0000000000012392 <_ZN4core3str19slice_error_fail_rt17hcb246852ed3ab8e1E>:
   12392:	7115                	add	sp,sp,-224
   12394:	ed86                	sd	ra,216(sp)
   12396:	e432                	sd	a2,8(sp)
   12398:	10100713          	li	a4,257
   1239c:	e836                	sd	a3,16(sp)
   1239e:	04e5eb63          	bltu	a1,a4,123f4 <.LBB293_48+0x12>
   123a2:	10050783          	lb	a5,256(a0)
   123a6:	fbf00813          	li	a6,-65
   123aa:	470d                	li	a4,3
   123ac:	00f84d63          	blt	a6,a5,123c6 <_ZN4core3str19slice_error_fail_rt17hcb246852ed3ab8e1E+0x34>
   123b0:	0ff50783          	lb	a5,255(a0)
   123b4:	4709                	li	a4,2
   123b6:	00f84863          	blt	a6,a5,123c6 <_ZN4core3str19slice_error_fail_rt17hcb246852ed3ab8e1E+0x34>
   123ba:	0fe50703          	lb	a4,254(a0)
   123be:	fbf00793          	li	a5,-65
   123c2:	00e7a733          	slt	a4,a5,a4
   123c6:	0fd70713          	add	a4,a4,253
   123ca:	02b77363          	bgeu	a4,a1,123f0 <.LBB293_48+0xe>
   123ce:	00e507b3          	add	a5,a0,a4
   123d2:	00078803          	lb	a6,0(a5)
   123d6:	fbf00793          	li	a5,-65
   123da:	0f07d163          	bge	a5,a6,124bc <.LBB293_57+0x14>
   123de:	ec2a                	sd	a0,24(sp)
   123e0:	f03a                	sd	a4,32(sp)

00000000000123e2 <.LBB293_48>:
   123e2:	00002717          	auipc	a4,0x2
   123e6:	6a970713          	add	a4,a4,1705 # 14a8b <.Lanon.442aba94db1f841cd37d39ada1516238.351>
   123ea:	f43a                	sd	a4,40(sp)
   123ec:	4815                	li	a6,5
   123ee:	a819                	j	12404 <.LBB293_49+0xa>
   123f0:	0cb71663          	bne	a4,a1,124bc <.LBB293_57+0x14>
   123f4:	ec2a                	sd	a0,24(sp)
   123f6:	f02e                	sd	a1,32(sp)
   123f8:	4801                	li	a6,0

00000000000123fa <.LBB293_49>:
   123fa:	00002797          	auipc	a5,0x2
   123fe:	17e78793          	add	a5,a5,382 # 14578 <.Lanon.442aba94db1f841cd37d39ada1516238.83>
   12402:	f43e                	sd	a5,40(sp)
   12404:	00c5b7b3          	sltu	a5,a1,a2
   12408:	00d5b733          	sltu	a4,a1,a3
   1240c:	8f5d                	or	a4,a4,a5
   1240e:	f842                	sd	a6,48(sp)
   12410:	cb39                	beqz	a4,12466 <.LBB293_53+0x14>
   12412:	00c5e363          	bltu	a1,a2,12418 <.LBB293_49+0x1e>
   12416:	8636                	mv	a2,a3
   12418:	e4b2                	sd	a2,72(sp)
   1241a:	00a8                	add	a0,sp,72
   1241c:	e52a                	sd	a0,136(sp)

000000000001241e <.LBB293_50>:
   1241e:	00001517          	auipc	a0,0x1
   12422:	8cc50513          	add	a0,a0,-1844 # 12cea <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h68be92e0167cbc01E>
   12426:	e92a                	sd	a0,144(sp)
   12428:	0828                	add	a0,sp,24
   1242a:	ed2a                	sd	a0,152(sp)

000000000001242c <.LBB293_51>:
   1242c:	00001517          	auipc	a0,0x1
   12430:	a6450513          	add	a0,a0,-1436 # 12e90 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdd6e30ce7f47bbb8E>
   12434:	f12a                	sd	a0,160(sp)
   12436:	102c                	add	a1,sp,40
   12438:	f52e                	sd	a1,168(sp)
   1243a:	f92a                	sd	a0,176(sp)

000000000001243c <.LBB293_52>:
   1243c:	00002517          	auipc	a0,0x2
   12440:	67c50513          	add	a0,a0,1660 # 14ab8 <.Lanon.442aba94db1f841cd37d39ada1516238.354>
   12444:	ecaa                	sd	a0,88(sp)
   12446:	450d                	li	a0,3
   12448:	f0aa                	sd	a0,96(sp)
   1244a:	f482                	sd	zero,104(sp)
   1244c:	012c                	add	a1,sp,136
   1244e:	fcae                	sd	a1,120(sp)
   12450:	e12a                	sd	a0,128(sp)

0000000000012452 <.LBB293_53>:
   12452:	00002597          	auipc	a1,0x2
   12456:	69658593          	add	a1,a1,1686 # 14ae8 <.Lanon.442aba94db1f841cd37d39ada1516238.355>
   1245a:	08a8                	add	a0,sp,88
   1245c:	fffff097          	auipc	ra,0xfffff
   12460:	aa0080e7          	jalr	-1376(ra) # 10efc <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
   12464:	0000                	unimp
   12466:	06c6f263          	bgeu	a3,a2,124ca <.LBB293_57+0x22>
   1246a:	0028                	add	a0,sp,8
   1246c:	e52a                	sd	a0,136(sp)

000000000001246e <.LBB293_54>:
   1246e:	00001517          	auipc	a0,0x1
   12472:	87c50513          	add	a0,a0,-1924 # 12cea <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h68be92e0167cbc01E>
   12476:	e92a                	sd	a0,144(sp)
   12478:	080c                	add	a1,sp,16
   1247a:	ed2e                	sd	a1,152(sp)
   1247c:	f12a                	sd	a0,160(sp)
   1247e:	0828                	add	a0,sp,24
   12480:	f52a                	sd	a0,168(sp)

0000000000012482 <.LBB293_55>:
   12482:	00001517          	auipc	a0,0x1
   12486:	a0e50513          	add	a0,a0,-1522 # 12e90 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdd6e30ce7f47bbb8E>
   1248a:	f92a                	sd	a0,176(sp)
   1248c:	102c                	add	a1,sp,40
   1248e:	fd2e                	sd	a1,184(sp)
   12490:	e1aa                	sd	a0,192(sp)

0000000000012492 <.LBB293_56>:
   12492:	00002517          	auipc	a0,0x2
   12496:	67e50513          	add	a0,a0,1662 # 14b10 <.Lanon.442aba94db1f841cd37d39ada1516238.359>
   1249a:	ecaa                	sd	a0,88(sp)
   1249c:	4511                	li	a0,4
   1249e:	f0aa                	sd	a0,96(sp)
   124a0:	f482                	sd	zero,104(sp)
   124a2:	012c                	add	a1,sp,136
   124a4:	fcae                	sd	a1,120(sp)
   124a6:	e12a                	sd	a0,128(sp)

00000000000124a8 <.LBB293_57>:
   124a8:	00002597          	auipc	a1,0x2
   124ac:	6a858593          	add	a1,a1,1704 # 14b50 <.Lanon.442aba94db1f841cd37d39ada1516238.360>
   124b0:	08a8                	add	a0,sp,88
   124b2:	fffff097          	auipc	ra,0xfffff
   124b6:	a4a080e7          	jalr	-1462(ra) # 10efc <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
   124ba:	0000                	unimp
   124bc:	4601                	li	a2,0
   124be:	86ba                	mv	a3,a4
   124c0:	00000097          	auipc	ra,0x0
   124c4:	eba080e7          	jalr	-326(ra) # 1237a <_ZN4core3str16slice_error_fail17h0f23970489177861E>
   124c8:	0000                	unimp
   124ca:	e61d                	bnez	a2,124f8 <.LBB293_57+0x50>
   124cc:	8636                	mv	a2,a3
   124ce:	fc32                	sd	a2,56(sp)
   124d0:	86ae                	mv	a3,a1
   124d2:	04b67e63          	bgeu	a2,a1,1252e <.LBB293_57+0x86>
   124d6:	ffd60713          	add	a4,a2,-3
   124da:	4801                	li	a6,0
   124dc:	00e66363          	bltu	a2,a4,124e2 <.LBB293_57+0x3a>
   124e0:	883a                	mv	a6,a4
   124e2:	00160713          	add	a4,a2,1
   124e6:	03077463          	bgeu	a4,a6,1250e <.LBB293_57+0x66>
   124ea:	8542                	mv	a0,a6
   124ec:	85ba                	mv	a1,a4
   124ee:	00000097          	auipc	ra,0x0
   124f2:	a8e080e7          	jalr	-1394(ra) # 11f7c <_ZN4core5slice5index22slice_index_order_fail17h5b8db1271a95aea8E>
   124f6:	0000                	unimp
   124f8:	04b67d63          	bgeu	a2,a1,12552 <.LBB293_57+0xaa>
   124fc:	00c50733          	add	a4,a0,a2
   12500:	00070703          	lb	a4,0(a4)
   12504:	fc000793          	li	a5,-64
   12508:	fcf752e3          	bge	a4,a5,124cc <.LBB293_57+0x24>
   1250c:	b7c9                	j	124ce <.LBB293_57+0x26>
   1250e:	010507b3          	add	a5,a0,a6
   12512:	972a                	add	a4,a4,a0
   12514:	8f1d                	sub	a4,a4,a5
   12516:	962a                	add	a2,a2,a0
   12518:	fc000793          	li	a5,-64
   1251c:	c719                	beqz	a4,1252a <.LBB293_57+0x82>
   1251e:	00060683          	lb	a3,0(a2)
   12522:	177d                	add	a4,a4,-1
   12524:	167d                	add	a2,a2,-1
   12526:	fef6cbe3          	blt	a3,a5,1251c <.LBB293_57+0x74>
   1252a:	010706b3          	add	a3,a4,a6
   1252e:	c69d                	beqz	a3,1255c <.LBB293_57+0xb4>
   12530:	02b6f463          	bgeu	a3,a1,12558 <.LBB293_57+0xb0>
   12534:	00d50633          	add	a2,a0,a3
   12538:	00060603          	lb	a2,0(a2)
   1253c:	fbf00713          	li	a4,-65
   12540:	00c74e63          	blt	a4,a2,1255c <.LBB293_57+0xb4>
   12544:	8636                	mv	a2,a3
   12546:	86ae                	mv	a3,a1
   12548:	00000097          	auipc	ra,0x0
   1254c:	e32080e7          	jalr	-462(ra) # 1237a <_ZN4core3str16slice_error_fail17h0f23970489177861E>
   12550:	0000                	unimp
   12552:	f6b60de3          	beq	a2,a1,124cc <.LBB293_57+0x24>
   12556:	bfa5                	j	124ce <.LBB293_57+0x26>
   12558:	feb696e3          	bne	a3,a1,12544 <.LBB293_57+0x9c>
   1255c:	02b69163          	bne	a3,a1,1257e <.LBB293_59+0x16>

0000000000012560 <.LBB293_58>:
   12560:	00002517          	auipc	a0,0x2
   12564:	01850513          	add	a0,a0,24 # 14578 <.Lanon.442aba94db1f841cd37d39ada1516238.83>

0000000000012568 <.LBB293_59>:
   12568:	00002617          	auipc	a2,0x2
   1256c:	60060613          	add	a2,a2,1536 # 14b68 <.Lanon.442aba94db1f841cd37d39ada1516238.361>
   12570:	02b00593          	li	a1,43
   12574:	fffff097          	auipc	ra,0xfffff
   12578:	8d4080e7          	jalr	-1836(ra) # 10e48 <_ZN4core9panicking5panic17h92f54f473578363dE>
   1257c:	0000                	unimp
   1257e:	9536                	add	a0,a0,a3
   12580:	00050603          	lb	a2,0(a0)
   12584:	0ff67593          	zext.b	a1,a2
   12588:	00064563          	bltz	a2,12592 <.LBB293_59+0x2a>
   1258c:	c2ae                	sw	a1,68(sp)
   1258e:	4585                	li	a1,1
   12590:	a89d                	j	12606 <.LBB293_59+0x9e>
   12592:	00154703          	lbu	a4,1(a0)
   12596:	01f5f613          	and	a2,a1,31
   1259a:	0df00793          	li	a5,223
   1259e:	03f77713          	and	a4,a4,63
   125a2:	02b7fc63          	bgeu	a5,a1,125da <.LBB293_59+0x72>
   125a6:	00254783          	lbu	a5,2(a0)
   125aa:	071a                	sll	a4,a4,0x6
   125ac:	03f7f793          	and	a5,a5,63
   125b0:	0f000813          	li	a6,240
   125b4:	8f5d                	or	a4,a4,a5
   125b6:	0305e663          	bltu	a1,a6,125e2 <.LBB293_59+0x7a>
   125ba:	00354503          	lbu	a0,3(a0)
   125be:	03d61593          	sll	a1,a2,0x3d
   125c2:	91ad                	srl	a1,a1,0x2b
   125c4:	00671613          	sll	a2,a4,0x6
   125c8:	03f57513          	and	a0,a0,63
   125cc:	8d51                	or	a0,a0,a2
   125ce:	8d4d                	or	a0,a0,a1
   125d0:	001105b7          	lui	a1,0x110
   125d4:	f8b506e3          	beq	a0,a1,12560 <.LBB293_58>
   125d8:	a801                	j	125e8 <.LBB293_59+0x80>
   125da:	00661513          	sll	a0,a2,0x6
   125de:	8d59                	or	a0,a0,a4
   125e0:	a021                	j	125e8 <.LBB293_59+0x80>
   125e2:	00c61513          	sll	a0,a2,0xc
   125e6:	8d59                	or	a0,a0,a4
   125e8:	c2aa                	sw	a0,68(sp)
   125ea:	08000613          	li	a2,128
   125ee:	4585                	li	a1,1
   125f0:	00c56b63          	bltu	a0,a2,12606 <.LBB293_59+0x9e>
   125f4:	00b55613          	srl	a2,a0,0xb
   125f8:	4589                	li	a1,2
   125fa:	c611                	beqz	a2,12606 <.LBB293_59+0x9e>
   125fc:	8141                	srl	a0,a0,0x10
   125fe:	00153513          	seqz	a0,a0
   12602:	4591                	li	a1,4
   12604:	8d89                	sub	a1,a1,a0
   12606:	00d58533          	add	a0,a1,a3
   1260a:	e4b6                	sd	a3,72(sp)
   1260c:	e8aa                	sd	a0,80(sp)
   1260e:	1828                	add	a0,sp,56
   12610:	e52a                	sd	a0,136(sp)

0000000000012612 <.LBB293_60>:
   12612:	00000517          	auipc	a0,0x0
   12616:	6d850513          	add	a0,a0,1752 # 12cea <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h68be92e0167cbc01E>
   1261a:	e92a                	sd	a0,144(sp)
   1261c:	00c8                	add	a0,sp,68
   1261e:	ed2a                	sd	a0,152(sp)

0000000000012620 <.LBB293_61>:
   12620:	fffff517          	auipc	a0,0xfffff
   12624:	6e250513          	add	a0,a0,1762 # 11d02 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hdf624fa96493adceE>
   12628:	f12a                	sd	a0,160(sp)
   1262a:	00a8                	add	a0,sp,72
   1262c:	f52a                	sd	a0,168(sp)

000000000001262e <.LBB293_62>:
   1262e:	ffffe517          	auipc	a0,0xffffe
   12632:	79450513          	add	a0,a0,1940 # 10dc2 <_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h59f5c39c772cdb32E>
   12636:	f92a                	sd	a0,176(sp)
   12638:	0828                	add	a0,sp,24
   1263a:	fd2a                	sd	a0,184(sp)

000000000001263c <.LBB293_63>:
   1263c:	00001517          	auipc	a0,0x1
   12640:	85450513          	add	a0,a0,-1964 # 12e90 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdd6e30ce7f47bbb8E>
   12644:	e1aa                	sd	a0,192(sp)
   12646:	102c                	add	a1,sp,40
   12648:	e5ae                	sd	a1,200(sp)
   1264a:	e9aa                	sd	a0,208(sp)

000000000001264c <.LBB293_64>:
   1264c:	00002517          	auipc	a0,0x2
   12650:	56450513          	add	a0,a0,1380 # 14bb0 <.Lanon.442aba94db1f841cd37d39ada1516238.365>
   12654:	ecaa                	sd	a0,88(sp)
   12656:	4515                	li	a0,5
   12658:	f0aa                	sd	a0,96(sp)
   1265a:	f482                	sd	zero,104(sp)
   1265c:	012c                	add	a1,sp,136
   1265e:	fcae                	sd	a1,120(sp)
   12660:	e12a                	sd	a0,128(sp)

0000000000012662 <.LBB293_65>:
   12662:	00002597          	auipc	a1,0x2
   12666:	59e58593          	add	a1,a1,1438 # 14c00 <.Lanon.442aba94db1f841cd37d39ada1516238.366>
   1266a:	08a8                	add	a0,sp,88
   1266c:	fffff097          	auipc	ra,0xfffff
   12670:	890080e7          	jalr	-1904(ra) # 10efc <_ZN4core9panicking9panic_fmt17h19eb6297ffd5ff01E>
	...

0000000000012676 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E>:
   12676:	1141                	add	sp,sp,-16
   12678:	e406                	sd	ra,8(sp)
   1267a:	03051293          	sll	t0,a0,0x30
   1267e:	ce29                	beqz	a2,126d8 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x62>
   12680:	88aa                	mv	a7,a0
   12682:	4501                	li	a0,0
   12684:	0382d313          	srl	t1,t0,0x38
   12688:	0606                	sll	a2,a2,0x1
   1268a:	00c583b3          	add	t2,a1,a2
   1268e:	0ff8fe13          	zext.b	t3,a7
   12692:	a811                	j	126a6 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x30>
   12694:	01d33533          	sltu	a0,t1,t4
   12698:	00b3c633          	xor	a2,t2,a1
   1269c:	00163613          	seqz	a2,a2
   126a0:	8e49                	or	a2,a2,a0
   126a2:	8546                	mv	a0,a7
   126a4:	ea15                	bnez	a2,126d8 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x62>
   126a6:	0005ce83          	lbu	t4,0(a1)
   126aa:	0015c603          	lbu	a2,1(a1)
   126ae:	0589                	add	a1,a1,2
   126b0:	00c508b3          	add	a7,a0,a2
   126b4:	fe6e90e3          	bne	t4,t1,12694 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x1e>
   126b8:	0aa8e163          	bltu	a7,a0,1275a <.LBB312_24+0x16>
   126bc:	0b176563          	bltu	a4,a7,12766 <.LBB312_24+0x22>
   126c0:	9536                	add	a0,a0,a3
   126c2:	ca01                	beqz	a2,126d2 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x5c>
   126c4:	00054e83          	lbu	t4,0(a0)
   126c8:	0505                	add	a0,a0,1
   126ca:	167d                	add	a2,a2,-1
   126cc:	ffce9be3          	bne	t4,t3,126c2 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x4c>
   126d0:	a8a1                	j	12728 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0xb2>
   126d2:	8546                	mv	a0,a7
   126d4:	fcb399e3          	bne	t2,a1,126a6 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x30>
   126d8:	04080d63          	beqz	a6,12732 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0xbc>
   126dc:	010785b3          	add	a1,a5,a6
   126e0:	0302d613          	srl	a2,t0,0x30
   126e4:	4505                	li	a0,1
   126e6:	00078683          	lb	a3,0(a5)
   126ea:	00178713          	add	a4,a5,1
   126ee:	0006c963          	bltz	a3,12700 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x8a>
   126f2:	0ff6f693          	zext.b	a3,a3
   126f6:	87ba                	mv	a5,a4
   126f8:	9e15                	subw	a2,a2,a3
   126fa:	00065f63          	bgez	a2,12718 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0xa2>
   126fe:	a00d                	j	12720 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0xaa>
   12700:	02b70e63          	beq	a4,a1,1273c <.LBB312_23>
   12704:	0017c703          	lbu	a4,1(a5)
   12708:	0789                	add	a5,a5,2
   1270a:	07f6f693          	and	a3,a3,127
   1270e:	06a2                	sll	a3,a3,0x8
   12710:	8ed9                	or	a3,a3,a4
   12712:	9e15                	subw	a2,a2,a3
   12714:	00064663          	bltz	a2,12720 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0xaa>
   12718:	00154513          	xor	a0,a0,1
   1271c:	fcb795e3          	bne	a5,a1,126e6 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E+0x70>
   12720:	8905                	and	a0,a0,1
   12722:	60a2                	ld	ra,8(sp)
   12724:	0141                	add	sp,sp,16
   12726:	8082                	ret
   12728:	4501                	li	a0,0
   1272a:	8905                	and	a0,a0,1
   1272c:	60a2                	ld	ra,8(sp)
   1272e:	0141                	add	sp,sp,16
   12730:	8082                	ret
   12732:	4505                	li	a0,1
   12734:	8905                	and	a0,a0,1
   12736:	60a2                	ld	ra,8(sp)
   12738:	0141                	add	sp,sp,16
   1273a:	8082                	ret

000000000001273c <.LBB312_23>:
   1273c:	00002517          	auipc	a0,0x2
   12740:	e3c50513          	add	a0,a0,-452 # 14578 <.Lanon.442aba94db1f841cd37d39ada1516238.83>

0000000000012744 <.LBB312_24>:
   12744:	00002617          	auipc	a2,0x2
   12748:	4fc60613          	add	a2,a2,1276 # 14c40 <.Lanon.442aba94db1f841cd37d39ada1516238.388>
   1274c:	02b00593          	li	a1,43
   12750:	ffffe097          	auipc	ra,0xffffe
   12754:	6f8080e7          	jalr	1784(ra) # 10e48 <_ZN4core9panicking5panic17h92f54f473578363dE>
   12758:	0000                	unimp
   1275a:	85c6                	mv	a1,a7
   1275c:	00000097          	auipc	ra,0x0
   12760:	820080e7          	jalr	-2016(ra) # 11f7c <_ZN4core5slice5index22slice_index_order_fail17h5b8db1271a95aea8E>
   12764:	0000                	unimp
   12766:	8546                	mv	a0,a7
   12768:	85ba                	mv	a1,a4
   1276a:	fffff097          	auipc	ra,0xfffff
   1276e:	7be080e7          	jalr	1982(ra) # 11f28 <_ZN4core5slice5index24slice_end_index_len_fail17h5d1e1d044f43082eE>
	...

0000000000012774 <_ZN4core7unicode9printable12is_printable17h190ab7f3e4a90aabE>:
   12774:	0005061b          	sext.w	a2,a0
   12778:	02000593          	li	a1,32
   1277c:	00b67463          	bgeu	a2,a1,12784 <_ZN4core7unicode9printable12is_printable17h190ab7f3e4a90aabE+0x10>
   12780:	4501                	li	a0,0
   12782:	8082                	ret
   12784:	07f00693          	li	a3,127
   12788:	4585                	li	a1,1
   1278a:	00d67463          	bgeu	a2,a3,12792 <_ZN4core7unicode9printable12is_printable17h190ab7f3e4a90aabE+0x1e>
   1278e:	852e                	mv	a0,a1
   12790:	8082                	ret
   12792:	0105559b          	srlw	a1,a0,0x10
   12796:	e59d                	bnez	a1,127c4 <.LBB313_12+0x1c>

0000000000012798 <.LBB313_10>:
   12798:	00002597          	auipc	a1,0x2
   1279c:	4c058593          	add	a1,a1,1216 # 14c58 <.Lanon.442aba94db1f841cd37d39ada1516238.389>

00000000000127a0 <.LBB313_11>:
   127a0:	00002697          	auipc	a3,0x2
   127a4:	50868693          	add	a3,a3,1288 # 14ca8 <.Lanon.442aba94db1f841cd37d39ada1516238.390>

00000000000127a8 <.LBB313_12>:
   127a8:	00002797          	auipc	a5,0x2
   127ac:	62078793          	add	a5,a5,1568 # 14dc8 <.Lanon.442aba94db1f841cd37d39ada1516238.391>
   127b0:	02800613          	li	a2,40
   127b4:	12000713          	li	a4,288
   127b8:	12f00813          	li	a6,303
   127bc:	00000317          	auipc	t1,0x0
   127c0:	eba30067          	jr	-326(t1) # 12676 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E>
   127c4:	0115559b          	srlw	a1,a0,0x11
   127c8:	e59d                	bnez	a1,127f6 <.LBB313_15+0x1c>

00000000000127ca <.LBB313_13>:
   127ca:	00002597          	auipc	a1,0x2
   127ce:	72d58593          	add	a1,a1,1837 # 14ef7 <.Lanon.442aba94db1f841cd37d39ada1516238.392>

00000000000127d2 <.LBB313_14>:
   127d2:	00002697          	auipc	a3,0x2
   127d6:	77968693          	add	a3,a3,1913 # 14f4b <.Lanon.442aba94db1f841cd37d39ada1516238.393>

00000000000127da <.LBB313_15>:
   127da:	00003797          	auipc	a5,0x3
   127de:	83178793          	add	a5,a5,-1999 # 1500b <.Lanon.442aba94db1f841cd37d39ada1516238.394>
   127e2:	02a00613          	li	a2,42
   127e6:	0c000713          	li	a4,192
   127ea:	1b600813          	li	a6,438
   127ee:	00000317          	auipc	t1,0x0
   127f2:	e8830067          	jr	-376(t1) # 12676 <_ZN4core7unicode9printable5check17h43f13e2ae9b9e566E>
   127f6:	0055559b          	srlw	a1,a0,0x5
   127fa:	6605                	lui	a2,0x1
   127fc:	5376069b          	addw	a3,a2,1335 # 1537 <_start-0xeac9>
   12800:	8db5                	xor	a1,a1,a3
   12802:	0015b593          	seqz	a1,a1
   12806:	fffd56b7          	lui	a3,0xfffd5
   1280a:	8c76869b          	addw	a3,a3,-1849 # fffffffffffd48c7 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xfffffffffff8c557>
   1280e:	9ea9                	addw	a3,a3,a0
   12810:	0076b693          	sltiu	a3,a3,7
   12814:	8dd5                	or	a1,a1,a3
   12816:	0015569b          	srlw	a3,a0,0x1
   1281a:	6759                	lui	a4,0x16
   1281c:	c0f7071b          	addw	a4,a4,-1009 # 15c0f <.Lanon.86a3613c128665d32fc75176e6ae67c2.14+0x56f>
   12820:	8eb9                	xor	a3,a3,a4
   12822:	0016b693          	seqz	a3,a3
   12826:	8dd5                	or	a1,a1,a3
   12828:	fffd36b7          	lui	a3,0xfffd3
   1282c:	15e6869b          	addw	a3,a3,350 # fffffffffffd315e <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xfffffffffff8adee>
   12830:	9ea9                	addw	a3,a3,a0
   12832:	00e6b693          	sltiu	a3,a3,14
   12836:	8dd5                	or	a1,a1,a3
   12838:	fffd16b7          	lui	a3,0xfffd1
   1283c:	41f6869b          	addw	a3,a3,1055 # fffffffffffd141f <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xfffffffffff890af>
   12840:	9ea9                	addw	a3,a3,a0
   12842:	c1f6061b          	addw	a2,a2,-993
   12846:	00c6b633          	sltu	a2,a3,a2
   1284a:	8dd1                	or	a1,a1,a2
   1284c:	fffd0637          	lui	a2,0xfffd0
   12850:	5e26069b          	addw	a3,a2,1506 # fffffffffffd05e2 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xfffffffffff88272>
   12854:	9ea9                	addw	a3,a3,a0
   12856:	5e26b693          	sltiu	a3,a3,1506
   1285a:	8dd5                	or	a1,a1,a3
   1285c:	fffcf6b7          	lui	a3,0xfffcf
   12860:	cb56869b          	addw	a3,a3,-843 # fffffffffffcecb5 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xfffffffffff86945>
   12864:	9ea9                	addw	a3,a3,a0
   12866:	000af737          	lui	a4,0xaf
   1286a:	db57071b          	addw	a4,a4,-587 # aedb5 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0x66a45>
   1286e:	00e6b6b3          	sltu	a3,a3,a4
   12872:	8dd5                	or	a1,a1,a3
   12874:	0015f693          	and	a3,a1,1
   12878:	4581                	li	a1,0
   1287a:	fa91                	bnez	a3,1278e <_ZN4core7unicode9printable12is_printable17h190ab7f3e4a90aabE+0x1a>
   1287c:	ffef05b7          	lui	a1,0xffef0
   12880:	9d2d                	addw	a0,a0,a1
   12882:	1f06059b          	addw	a1,a2,496
   12886:	00b535b3          	sltu	a1,a0,a1
   1288a:	852e                	mv	a0,a1
   1288c:	8082                	ret

000000000001288e <_ZN69_$LT$core..num..nonzero..NonZeroUsize$u20$as$u20$core..fmt..Debug$GT$3fmt17hcd3cf5bdc6888583E>:
   1288e:	1141                	add	sp,sp,-16
   12890:	e406                	sd	ra,8(sp)
   12892:	6108                	ld	a0,0(a0)
   12894:	e02a                	sd	a0,0(sp)
   12896:	850a                	mv	a0,sp
   12898:	00000097          	auipc	ra,0x0
   1289c:	1c6080e7          	jalr	454(ra) # 12a5e <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E>
   128a0:	60a2                	ld	ra,8(sp)
   128a2:	0141                	add	sp,sp,16
   128a4:	8082                	ret

00000000000128a6 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E>:
   128a6:	7175                	add	sp,sp,-144
   128a8:	e506                	sd	ra,136(sp)
   128aa:	882e                	mv	a6,a1
   128ac:	0305e583          	lwu	a1,48(a1) # ffffffffffef0030 <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0xffffffffffea7cc0>
   128b0:	0105f613          	and	a2,a1,16
   128b4:	ee11                	bnez	a2,128d0 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x2a>
   128b6:	0205f593          	and	a1,a1,32
   128ba:	e9a1                	bnez	a1,1290a <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x64>
   128bc:	00054503          	lbu	a0,0(a0)
   128c0:	4585                	li	a1,1
   128c2:	8642                	mv	a2,a6
   128c4:	60aa                	ld	ra,136(sp)
   128c6:	6149                	add	sp,sp,144
   128c8:	00000317          	auipc	t1,0x0
   128cc:	26c30067          	jr	620(t1) # 12b34 <_ZN4core3fmt3num3imp7fmt_u6417h5e0a68673bc51c93E>
   128d0:	4581                	li	a1,0
   128d2:	00054683          	lbu	a3,0(a0)
   128d6:	00810893          	add	a7,sp,8
   128da:	42a9                	li	t0,10
   128dc:	433d                	li	t1,15
   128de:	a829                	j	128f8 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x52>
   128e0:	00b88633          	add	a2,a7,a1
   128e4:	0ff6f713          	zext.b	a4,a3
   128e8:	00475693          	srl	a3,a4,0x4
   128ec:	9d3d                	addw	a0,a0,a5
   128ee:	06a60fa3          	sb	a0,127(a2)
   128f2:	15fd                	add	a1,a1,-1
   128f4:	04e37863          	bgeu	t1,a4,12944 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x9e>
   128f8:	00f6f793          	and	a5,a3,15
   128fc:	03000513          	li	a0,48
   12900:	fe57e0e3          	bltu	a5,t0,128e0 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x3a>
   12904:	05700513          	li	a0,87
   12908:	bfe1                	j	128e0 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x3a>
   1290a:	4581                	li	a1,0
   1290c:	00054683          	lbu	a3,0(a0)
   12910:	00810893          	add	a7,sp,8
   12914:	42a9                	li	t0,10
   12916:	433d                	li	t1,15
   12918:	a829                	j	12932 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x8c>
   1291a:	00b88633          	add	a2,a7,a1
   1291e:	0ff6f713          	zext.b	a4,a3
   12922:	00475693          	srl	a3,a4,0x4
   12926:	9d3d                	addw	a0,a0,a5
   12928:	06a60fa3          	sb	a0,127(a2)
   1292c:	15fd                	add	a1,a1,-1
   1292e:	00e37b63          	bgeu	t1,a4,12944 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x9e>
   12932:	00f6f793          	and	a5,a3,15
   12936:	03000513          	li	a0,48
   1293a:	fe57e0e3          	bltu	a5,t0,1291a <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x74>
   1293e:	03700513          	li	a0,55
   12942:	bfe1                	j	1291a <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E+0x74>
   12944:	08058513          	add	a0,a1,128
   12948:	08100613          	li	a2,129
   1294c:	02c57663          	bgeu	a0,a2,12978 <.LBB538_14+0x1c>
   12950:	40b007b3          	neg	a5,a1
   12954:	0028                	add	a0,sp,8
   12956:	952e                	add	a0,a0,a1
   12958:	08050713          	add	a4,a0,128

000000000001295c <.LBB538_14>:
   1295c:	00002617          	auipc	a2,0x2
   12960:	dec60613          	add	a2,a2,-532 # 14748 <.Lanon.442aba94db1f841cd37d39ada1516238.257>
   12964:	4585                	li	a1,1
   12966:	4689                	li	a3,2
   12968:	8542                	mv	a0,a6
   1296a:	fffff097          	auipc	ra,0xfffff
   1296e:	e6c080e7          	jalr	-404(ra) # 117d6 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E>
   12972:	60aa                	ld	ra,136(sp)
   12974:	6149                	add	sp,sp,144
   12976:	8082                	ret
   12978:	08000593          	li	a1,128
   1297c:	fffff097          	auipc	ra,0xfffff
   12980:	558080e7          	jalr	1368(ra) # 11ed4 <_ZN4core5slice5index26slice_start_index_len_fail17hc36fc5bb321621f5E>
	...

0000000000012986 <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E>:
   12986:	7175                	add	sp,sp,-144
   12988:	e506                	sd	ra,136(sp)
   1298a:	882e                	mv	a6,a1
   1298c:	0305e583          	lwu	a1,48(a1)
   12990:	0105f613          	and	a2,a1,16
   12994:	ee11                	bnez	a2,129b0 <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x2a>
   12996:	0205f593          	and	a1,a1,32
   1299a:	e5b1                	bnez	a1,129e6 <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x60>
   1299c:	00056503          	lwu	a0,0(a0)
   129a0:	4585                	li	a1,1
   129a2:	8642                	mv	a2,a6
   129a4:	60aa                	ld	ra,136(sp)
   129a6:	6149                	add	sp,sp,144
   129a8:	00000317          	auipc	t1,0x0
   129ac:	18c30067          	jr	396(t1) # 12b34 <_ZN4core3fmt3num3imp7fmt_u6417h5e0a68673bc51c93E>
   129b0:	4581                	li	a1,0
   129b2:	4118                	lw	a4,0(a0)
   129b4:	00810893          	add	a7,sp,8
   129b8:	42a9                	li	t0,10
   129ba:	433d                	li	t1,15
   129bc:	a819                	j	129d2 <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x4c>
   129be:	00b886b3          	add	a3,a7,a1
   129c2:	0045571b          	srlw	a4,a0,0x4
   129c6:	9e3d                	addw	a2,a2,a5
   129c8:	06c68fa3          	sb	a2,127(a3)
   129cc:	15fd                	add	a1,a1,-1
   129ce:	04a37763          	bgeu	t1,a0,12a1c <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x96>
   129d2:	853a                	mv	a0,a4
   129d4:	00f77793          	and	a5,a4,15
   129d8:	03000613          	li	a2,48
   129dc:	fe57e1e3          	bltu	a5,t0,129be <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x38>
   129e0:	05700613          	li	a2,87
   129e4:	bfe9                	j	129be <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x38>
   129e6:	4581                	li	a1,0
   129e8:	4118                	lw	a4,0(a0)
   129ea:	00810893          	add	a7,sp,8
   129ee:	42a9                	li	t0,10
   129f0:	433d                	li	t1,15
   129f2:	a819                	j	12a08 <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x82>
   129f4:	00b886b3          	add	a3,a7,a1
   129f8:	0045571b          	srlw	a4,a0,0x4
   129fc:	9e3d                	addw	a2,a2,a5
   129fe:	06c68fa3          	sb	a2,127(a3)
   12a02:	15fd                	add	a1,a1,-1
   12a04:	00a37c63          	bgeu	t1,a0,12a1c <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x96>
   12a08:	853a                	mv	a0,a4
   12a0a:	00f77793          	and	a5,a4,15
   12a0e:	03000613          	li	a2,48
   12a12:	fe57e1e3          	bltu	a5,t0,129f4 <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x6e>
   12a16:	03700613          	li	a2,55
   12a1a:	bfe9                	j	129f4 <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E+0x6e>
   12a1c:	08058513          	add	a0,a1,128
   12a20:	08100613          	li	a2,129
   12a24:	02c57663          	bgeu	a0,a2,12a50 <.LBB540_14+0x1c>
   12a28:	40b007b3          	neg	a5,a1
   12a2c:	0028                	add	a0,sp,8
   12a2e:	952e                	add	a0,a0,a1
   12a30:	08050713          	add	a4,a0,128

0000000000012a34 <.LBB540_14>:
   12a34:	00002617          	auipc	a2,0x2
   12a38:	d1460613          	add	a2,a2,-748 # 14748 <.Lanon.442aba94db1f841cd37d39ada1516238.257>
   12a3c:	4585                	li	a1,1
   12a3e:	4689                	li	a3,2
   12a40:	8542                	mv	a0,a6
   12a42:	fffff097          	auipc	ra,0xfffff
   12a46:	d94080e7          	jalr	-620(ra) # 117d6 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E>
   12a4a:	60aa                	ld	ra,136(sp)
   12a4c:	6149                	add	sp,sp,144
   12a4e:	8082                	ret
   12a50:	08000593          	li	a1,128
   12a54:	fffff097          	auipc	ra,0xfffff
   12a58:	480080e7          	jalr	1152(ra) # 11ed4 <_ZN4core5slice5index26slice_start_index_len_fail17hc36fc5bb321621f5E>
	...

0000000000012a5e <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E>:
   12a5e:	7175                	add	sp,sp,-144
   12a60:	e506                	sd	ra,136(sp)
   12a62:	882e                	mv	a6,a1
   12a64:	0305e583          	lwu	a1,48(a1)
   12a68:	0105f613          	and	a2,a1,16
   12a6c:	ee09                	bnez	a2,12a86 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x28>
   12a6e:	0205f593          	and	a1,a1,32
   12a72:	e5a9                	bnez	a1,12abc <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x5e>
   12a74:	6108                	ld	a0,0(a0)
   12a76:	4585                	li	a1,1
   12a78:	8642                	mv	a2,a6
   12a7a:	60aa                	ld	ra,136(sp)
   12a7c:	6149                	add	sp,sp,144
   12a7e:	00000317          	auipc	t1,0x0
   12a82:	0b630067          	jr	182(t1) # 12b34 <_ZN4core3fmt3num3imp7fmt_u6417h5e0a68673bc51c93E>
   12a86:	4581                	li	a1,0
   12a88:	6118                	ld	a4,0(a0)
   12a8a:	00810893          	add	a7,sp,8
   12a8e:	42a9                	li	t0,10
   12a90:	433d                	li	t1,15
   12a92:	a819                	j	12aa8 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x4a>
   12a94:	00b886b3          	add	a3,a7,a1
   12a98:	00455713          	srl	a4,a0,0x4
   12a9c:	9e3d                	addw	a2,a2,a5
   12a9e:	06c68fa3          	sb	a2,127(a3)
   12aa2:	15fd                	add	a1,a1,-1
   12aa4:	04a37763          	bgeu	t1,a0,12af2 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x94>
   12aa8:	853a                	mv	a0,a4
   12aaa:	00f77793          	and	a5,a4,15
   12aae:	03000613          	li	a2,48
   12ab2:	fe57e1e3          	bltu	a5,t0,12a94 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x36>
   12ab6:	05700613          	li	a2,87
   12aba:	bfe9                	j	12a94 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x36>
   12abc:	4581                	li	a1,0
   12abe:	6118                	ld	a4,0(a0)
   12ac0:	00810893          	add	a7,sp,8
   12ac4:	42a9                	li	t0,10
   12ac6:	433d                	li	t1,15
   12ac8:	a819                	j	12ade <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x80>
   12aca:	00b886b3          	add	a3,a7,a1
   12ace:	00455713          	srl	a4,a0,0x4
   12ad2:	9e3d                	addw	a2,a2,a5
   12ad4:	06c68fa3          	sb	a2,127(a3)
   12ad8:	15fd                	add	a1,a1,-1
   12ada:	00a37c63          	bgeu	t1,a0,12af2 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x94>
   12ade:	853a                	mv	a0,a4
   12ae0:	00f77793          	and	a5,a4,15
   12ae4:	03000613          	li	a2,48
   12ae8:	fe57e1e3          	bltu	a5,t0,12aca <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x6c>
   12aec:	03700613          	li	a2,55
   12af0:	bfe9                	j	12aca <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E+0x6c>
   12af2:	08058513          	add	a0,a1,128
   12af6:	08100613          	li	a2,129
   12afa:	02c57663          	bgeu	a0,a2,12b26 <.LBB542_14+0x1c>
   12afe:	40b007b3          	neg	a5,a1
   12b02:	0028                	add	a0,sp,8
   12b04:	952e                	add	a0,a0,a1
   12b06:	08050713          	add	a4,a0,128

0000000000012b0a <.LBB542_14>:
   12b0a:	00002617          	auipc	a2,0x2
   12b0e:	c3e60613          	add	a2,a2,-962 # 14748 <.Lanon.442aba94db1f841cd37d39ada1516238.257>
   12b12:	4585                	li	a1,1
   12b14:	4689                	li	a3,2
   12b16:	8542                	mv	a0,a6
   12b18:	fffff097          	auipc	ra,0xfffff
   12b1c:	cbe080e7          	jalr	-834(ra) # 117d6 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E>
   12b20:	60aa                	ld	ra,136(sp)
   12b22:	6149                	add	sp,sp,144
   12b24:	8082                	ret
   12b26:	08000593          	li	a1,128
   12b2a:	fffff097          	auipc	ra,0xfffff
   12b2e:	3aa080e7          	jalr	938(ra) # 11ed4 <_ZN4core5slice5index26slice_start_index_len_fail17hc36fc5bb321621f5E>
	...

0000000000012b34 <_ZN4core3fmt3num3imp7fmt_u6417h5e0a68673bc51c93E>:
   12b34:	7139                	add	sp,sp,-64
   12b36:	fc06                	sd	ra,56(sp)
   12b38:	f822                	sd	s0,48(sp)
   12b3a:	f426                	sd	s1,40(sp)
   12b3c:	8832                	mv	a6,a2
   12b3e:	00455693          	srl	a3,a0,0x4
   12b42:	02700713          	li	a4,39
   12b46:	27100793          	li	a5,625

0000000000012b4a <.LBB543_10>:
   12b4a:	00002e17          	auipc	t3,0x2
   12b4e:	c00e0e13          	add	t3,t3,-1024 # 1474a <.Lanon.442aba94db1f841cd37d39ada1516238.259>
   12b52:	02f6f363          	bgeu	a3,a5,12b78 <.LBB543_10+0x2e>
   12b56:	06300693          	li	a3,99
   12b5a:	0aa6e963          	bltu	a3,a0,12c0c <.LBB543_11+0x92>
   12b5e:	4629                	li	a2,10
   12b60:	0ec57763          	bgeu	a0,a2,12c4e <.LBB543_11+0xd4>
   12b64:	fff70693          	add	a3,a4,-1
   12b68:	00110613          	add	a2,sp,1
   12b6c:	9636                	add	a2,a2,a3
   12b6e:	0305051b          	addw	a0,a0,48
   12b72:	00a60023          	sb	a0,0(a2)
   12b76:	a8dd                	j	12c6c <.LBB543_11+0xf2>
   12b78:	4701                	li	a4,0

0000000000012b7a <.LBB543_11>:
   12b7a:	00003697          	auipc	a3,0x3
   12b7e:	76e68693          	add	a3,a3,1902 # 162e8 <.LCPI543_0>
   12b82:	0006b883          	ld	a7,0(a3)
   12b86:	6689                	lui	a3,0x2
   12b88:	7106839b          	addw	t2,a3,1808 # 2710 <_start-0xd8f0>
   12b8c:	6685                	lui	a3,0x1
   12b8e:	47b68e9b          	addw	t4,a3,1147 # 147b <_start-0xeb85>
   12b92:	06400293          	li	t0,100
   12b96:	00110313          	add	t1,sp,1
   12b9a:	05f5e6b7          	lui	a3,0x5f5e
   12b9e:	0ff68f1b          	addw	t5,a3,255 # 5f5e0ff <_ZN8user_lib4HEAP17hf8a3a2ebe9f03884E+0x5f15d8f>
   12ba2:	862a                	mv	a2,a0
   12ba4:	03153533          	mulhu	a0,a0,a7
   12ba8:	812d                	srl	a0,a0,0xb
   12baa:	027507bb          	mulw	a5,a0,t2
   12bae:	40f607bb          	subw	a5,a2,a5
   12bb2:	03079693          	sll	a3,a5,0x30
   12bb6:	92c9                	srl	a3,a3,0x32
   12bb8:	03d686b3          	mul	a3,a3,t4
   12bbc:	82c5                	srl	a3,a3,0x11
   12bbe:	00169f93          	sll	t6,a3,0x1
   12bc2:	025686bb          	mulw	a3,a3,t0
   12bc6:	40d786bb          	subw	a3,a5,a3
   12bca:	16c6                	sll	a3,a3,0x31
   12bcc:	0306d413          	srl	s0,a3,0x30
   12bd0:	01cf87b3          	add	a5,t6,t3
   12bd4:	00e306b3          	add	a3,t1,a4
   12bd8:	0007cf83          	lbu	t6,0(a5)
   12bdc:	00178783          	lb	a5,1(a5)
   12be0:	9472                	add	s0,s0,t3
   12be2:	00140483          	lb	s1,1(s0)
   12be6:	00044403          	lbu	s0,0(s0)
   12bea:	02f68223          	sb	a5,36(a3)
   12bee:	03f681a3          	sb	t6,35(a3)
   12bf2:	02968323          	sb	s1,38(a3)
   12bf6:	028682a3          	sb	s0,37(a3)
   12bfa:	1771                	add	a4,a4,-4
   12bfc:	facf63e3          	bltu	t5,a2,12ba2 <.LBB543_11+0x28>
   12c00:	02770713          	add	a4,a4,39
   12c04:	06300693          	li	a3,99
   12c08:	f4a6fbe3          	bgeu	a3,a0,12b5e <.LBB543_10+0x14>
   12c0c:	03051613          	sll	a2,a0,0x30
   12c10:	9249                	srl	a2,a2,0x32
   12c12:	6685                	lui	a3,0x1
   12c14:	47b6869b          	addw	a3,a3,1147 # 147b <_start-0xeb85>
   12c18:	02d60633          	mul	a2,a2,a3
   12c1c:	8245                	srl	a2,a2,0x11
   12c1e:	06400693          	li	a3,100
   12c22:	02d606bb          	mulw	a3,a2,a3
   12c26:	9d15                	subw	a0,a0,a3
   12c28:	1546                	sll	a0,a0,0x31
   12c2a:	9141                	srl	a0,a0,0x30
   12c2c:	1779                	add	a4,a4,-2
   12c2e:	9572                	add	a0,a0,t3
   12c30:	00150683          	lb	a3,1(a0)
   12c34:	00054503          	lbu	a0,0(a0)
   12c38:	00110793          	add	a5,sp,1
   12c3c:	97ba                	add	a5,a5,a4
   12c3e:	00d780a3          	sb	a3,1(a5)
   12c42:	00a78023          	sb	a0,0(a5)
   12c46:	8532                	mv	a0,a2
   12c48:	4629                	li	a2,10
   12c4a:	f0c56de3          	bltu	a0,a2,12b64 <.LBB543_10+0x1a>
   12c4e:	0506                	sll	a0,a0,0x1
   12c50:	ffe70693          	add	a3,a4,-2
   12c54:	9572                	add	a0,a0,t3
   12c56:	00150603          	lb	a2,1(a0)
   12c5a:	00054503          	lbu	a0,0(a0)
   12c5e:	00110713          	add	a4,sp,1
   12c62:	9736                	add	a4,a4,a3
   12c64:	00c700a3          	sb	a2,1(a4)
   12c68:	00a70023          	sb	a0,0(a4)
   12c6c:	00110513          	add	a0,sp,1
   12c70:	00d50733          	add	a4,a0,a3
   12c74:	02700513          	li	a0,39
   12c78:	40d507b3          	sub	a5,a0,a3

0000000000012c7c <.LBB543_12>:
   12c7c:	00002617          	auipc	a2,0x2
   12c80:	8fc60613          	add	a2,a2,-1796 # 14578 <.Lanon.442aba94db1f841cd37d39ada1516238.83>
   12c84:	8542                	mv	a0,a6
   12c86:	4681                	li	a3,0
   12c88:	fffff097          	auipc	ra,0xfffff
   12c8c:	b4e080e7          	jalr	-1202(ra) # 117d6 <_ZN4core3fmt9Formatter12pad_integral17hcdac551b7d93b770E>
   12c90:	70e2                	ld	ra,56(sp)
   12c92:	7442                	ld	s0,48(sp)
   12c94:	74a2                	ld	s1,40(sp)
   12c96:	6121                	add	sp,sp,64
   12c98:	8082                	ret

0000000000012c9a <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h629811fb06cc9421E>:
   12c9a:	00056503          	lwu	a0,0(a0)
   12c9e:	862e                	mv	a2,a1
   12ca0:	0005069b          	sext.w	a3,a0
   12ca4:	55fd                	li	a1,-1
   12ca6:	00d5a5b3          	slt	a1,a1,a3
   12caa:	0006d463          	bgez	a3,12cb2 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h629811fb06cc9421E+0x18>
   12cae:	40d00533          	neg	a0,a3
   12cb2:	00000317          	auipc	t1,0x0
   12cb6:	e8230067          	jr	-382(t1) # 12b34 <_ZN4core3fmt3num3imp7fmt_u6417h5e0a68673bc51c93E>

0000000000012cba <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h2cede4cfd0806b52E>:
   12cba:	00056503          	lwu	a0,0(a0)
   12cbe:	862e                	mv	a2,a1
   12cc0:	4585                	li	a1,1
   12cc2:	00000317          	auipc	t1,0x0
   12cc6:	e7230067          	jr	-398(t1) # 12b34 <_ZN4core3fmt3num3imp7fmt_u6417h5e0a68673bc51c93E>

0000000000012cca <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i64$GT$3fmt17h23de1c985db3efdcE>:
   12cca:	6110                	ld	a2,0(a0)
   12ccc:	43f65513          	sra	a0,a2,0x3f
   12cd0:	00a606b3          	add	a3,a2,a0
   12cd4:	8d35                	xor	a0,a0,a3
   12cd6:	fff64613          	not	a2,a2
   12cda:	927d                	srl	a2,a2,0x3f
   12cdc:	86ae                	mv	a3,a1
   12cde:	85b2                	mv	a1,a2
   12ce0:	8636                	mv	a2,a3
   12ce2:	00000317          	auipc	t1,0x0
   12ce6:	e5230067          	jr	-430(t1) # 12b34 <_ZN4core3fmt3num3imp7fmt_u6417h5e0a68673bc51c93E>

0000000000012cea <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h68be92e0167cbc01E>:
   12cea:	6108                	ld	a0,0(a0)
   12cec:	862e                	mv	a2,a1
   12cee:	4585                	li	a1,1
   12cf0:	00000317          	auipc	t1,0x0
   12cf4:	e4430067          	jr	-444(t1) # 12b34 <_ZN4core3fmt3num3imp7fmt_u6417h5e0a68673bc51c93E>

0000000000012cf8 <_ZN53_$LT$core..fmt..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17ha7da3f83b0a295bbE>:
   12cf8:	7590                	ld	a2,40(a1)
   12cfa:	7188                	ld	a0,32(a1)
   12cfc:	6e1c                	ld	a5,24(a2)

0000000000012cfe <.LBB575_1>:
   12cfe:	00002597          	auipc	a1,0x2
   12d02:	50a58593          	add	a1,a1,1290 # 15208 <.Lanon.442aba94db1f841cd37d39ada1516238.584>
   12d06:	4615                	li	a2,5
   12d08:	8782                	jr	a5

0000000000012d0a <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h0c2533d9de598038E>:
   12d0a:	6108                	ld	a0,0(a0)
   12d0c:	00000317          	auipc	t1,0x0
   12d10:	d5230067          	jr	-686(t1) # 12a5e <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hca120a018cc732a8E>

0000000000012d14 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h2e9275016a084c2bE>:
   12d14:	7139                	add	sp,sp,-64
   12d16:	fc06                	sd	ra,56(sp)
   12d18:	f822                	sd	s0,48(sp)
   12d1a:	f426                	sd	s1,40(sp)
   12d1c:	6108                	ld	a0,0(a0)
   12d1e:	00054603          	lbu	a2,0(a0)
   12d22:	842e                	mv	s0,a1
   12d24:	ce49                	beqz	a2,12dbe <.LBB588_13+0x1e>
   12d26:	740c                	ld	a1,40(s0)
   12d28:	0505                	add	a0,a0,1
   12d2a:	e42a                	sd	a0,8(sp)
   12d2c:	7008                	ld	a0,32(s0)
   12d2e:	6d94                	ld	a3,24(a1)

0000000000012d30 <.LBB588_10>:
   12d30:	00002597          	auipc	a1,0x2
   12d34:	92858593          	add	a1,a1,-1752 # 14658 <.Lanon.442aba94db1f841cd37d39ada1516238.178+0x28>
   12d38:	4611                	li	a2,4
   12d3a:	9682                	jalr	a3
   12d3c:	e822                	sd	s0,16(sp)
   12d3e:	02a10023          	sb	a0,32(sp)
   12d42:	ec02                	sd	zero,24(sp)
   12d44:	020100a3          	sb	zero,33(sp)

0000000000012d48 <.LBB588_11>:
   12d48:	00002617          	auipc	a2,0x2
   12d4c:	9e060613          	add	a2,a2,-1568 # 14728 <.Lanon.442aba94db1f841cd37d39ada1516238.243>
   12d50:	0808                	add	a0,sp,16
   12d52:	002c                	add	a1,sp,8
   12d54:	ffffe097          	auipc	ra,0xffffe
   12d58:	59e080e7          	jalr	1438(ra) # 112f2 <_ZN4core3fmt8builders10DebugTuple5field17hb4c5d4885bb0d25dE>
   12d5c:	6562                	ld	a0,24(sp)
   12d5e:	02014583          	lbu	a1,32(sp)
   12d62:	c539                	beqz	a0,12db0 <.LBB588_13+0x10>
   12d64:	4405                	li	s0,1
   12d66:	e5a1                	bnez	a1,12dae <.LBB588_13+0xe>
   12d68:	02114583          	lbu	a1,33(sp)
   12d6c:	157d                	add	a0,a0,-1
   12d6e:	00153513          	seqz	a0,a0
   12d72:	64c2                	ld	s1,16(sp)
   12d74:	00b035b3          	snez	a1,a1
   12d78:	8d6d                	and	a0,a0,a1
   12d7a:	c105                	beqz	a0,12d9a <.LBB588_12+0x10>
   12d7c:	0304c503          	lbu	a0,48(s1)
   12d80:	8911                	and	a0,a0,4
   12d82:	ed01                	bnez	a0,12d9a <.LBB588_12+0x10>
   12d84:	748c                	ld	a1,40(s1)
   12d86:	7088                	ld	a0,32(s1)
   12d88:	6d94                	ld	a3,24(a1)

0000000000012d8a <.LBB588_12>:
   12d8a:	00002597          	auipc	a1,0x2
   12d8e:	99658593          	add	a1,a1,-1642 # 14720 <.Lanon.442aba94db1f841cd37d39ada1516238.238>
   12d92:	4605                	li	a2,1
   12d94:	4405                	li	s0,1
   12d96:	9682                	jalr	a3
   12d98:	e919                	bnez	a0,12dae <.LBB588_13+0xe>
   12d9a:	748c                	ld	a1,40(s1)
   12d9c:	7088                	ld	a0,32(s1)
   12d9e:	6d94                	ld	a3,24(a1)

0000000000012da0 <.LBB588_13>:
   12da0:	00002597          	auipc	a1,0x2
   12da4:	80a58593          	add	a1,a1,-2038 # 145aa <.Lanon.442aba94db1f841cd37d39ada1516238.137>
   12da8:	4605                	li	a2,1
   12daa:	9682                	jalr	a3
   12dac:	842a                	mv	s0,a0
   12dae:	85a2                	mv	a1,s0
   12db0:	00b03533          	snez	a0,a1
   12db4:	70e2                	ld	ra,56(sp)
   12db6:	7442                	ld	s0,48(sp)
   12db8:	74a2                	ld	s1,40(sp)
   12dba:	6121                	add	sp,sp,64
   12dbc:	8082                	ret
   12dbe:	740c                	ld	a1,40(s0)
   12dc0:	7008                	ld	a0,32(s0)
   12dc2:	6d9c                	ld	a5,24(a1)

0000000000012dc4 <.LBB588_14>:
   12dc4:	00002597          	auipc	a1,0x2
   12dc8:	89c58593          	add	a1,a1,-1892 # 14660 <.Lanon.442aba94db1f841cd37d39ada1516238.178+0x30>
   12dcc:	4611                	li	a2,4
   12dce:	70e2                	ld	ra,56(sp)
   12dd0:	7442                	ld	s0,48(sp)
   12dd2:	74a2                	ld	s1,40(sp)
   12dd4:	6121                	add	sp,sp,64
   12dd6:	8782                	jr	a5

0000000000012dd8 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h3c4bf84e4a8f4da0E>:
   12dd8:	6108                	ld	a0,0(a0)
   12dda:	00000317          	auipc	t1,0x0
   12dde:	acc30067          	jr	-1332(t1) # 128a6 <_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h2131a93360b3db16E>

0000000000012de2 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hdf0bbe7209f97e5eE>:
   12de2:	7159                	add	sp,sp,-112
   12de4:	f486                	sd	ra,104(sp)
   12de6:	6108                	ld	a0,0(a0)
   12de8:	6108                	ld	a0,0(a0)
   12dea:	f42a                	sd	a0,40(sp)
   12dec:	fff50613          	add	a2,a0,-1
   12df0:	fff54513          	not	a0,a0
   12df4:	8d71                	and	a0,a0,a2

0000000000012df6 <.LBB647_1>:
   12df6:	00003617          	auipc	a2,0x3
   12dfa:	55a60613          	add	a2,a2,1370 # 16350 <.LCPI647_0>
   12dfe:	6210                	ld	a2,0(a2)

0000000000012e00 <.LBB647_2>:
   12e00:	00003697          	auipc	a3,0x3
   12e04:	55868693          	add	a3,a3,1368 # 16358 <.LCPI647_1>
   12e08:	6294                	ld	a3,0(a3)
   12e0a:	00155713          	srl	a4,a0,0x1
   12e0e:	8e79                	and	a2,a2,a4
   12e10:	8d11                	sub	a0,a0,a2
   12e12:	00d57633          	and	a2,a0,a3
   12e16:	8109                	srl	a0,a0,0x2
   12e18:	8d75                	and	a0,a0,a3
   12e1a:	9532                	add	a0,a0,a2

0000000000012e1c <.LBB647_3>:
   12e1c:	00003617          	auipc	a2,0x3
   12e20:	54460613          	add	a2,a2,1348 # 16360 <.LCPI647_2>
   12e24:	6210                	ld	a2,0(a2)

0000000000012e26 <.LBB647_4>:
   12e26:	00003697          	auipc	a3,0x3
   12e2a:	54268693          	add	a3,a3,1346 # 16368 <.LCPI647_3>
   12e2e:	6294                	ld	a3,0(a3)
   12e30:	00455713          	srl	a4,a0,0x4
   12e34:	953a                	add	a0,a0,a4
   12e36:	8d71                	and	a0,a0,a2
   12e38:	02d50533          	mul	a0,a0,a3
   12e3c:	9161                	srl	a0,a0,0x38
   12e3e:	da2a                	sw	a0,52(sp)
   12e40:	1028                	add	a0,sp,40
   12e42:	e42a                	sd	a0,8(sp)

0000000000012e44 <.LBB647_5>:
   12e44:	00000517          	auipc	a0,0x0
   12e48:	a4a50513          	add	a0,a0,-1462 # 1288e <_ZN69_$LT$core..num..nonzero..NonZeroUsize$u20$as$u20$core..fmt..Debug$GT$3fmt17hcd3cf5bdc6888583E>
   12e4c:	e82a                	sd	a0,16(sp)
   12e4e:	1848                	add	a0,sp,52
   12e50:	ec2a                	sd	a0,24(sp)

0000000000012e52 <.LBB647_6>:
   12e52:	00000517          	auipc	a0,0x0
   12e56:	b3450513          	add	a0,a0,-1228 # 12986 <_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9819dc7829e5fed7E>
   12e5a:	f02a                	sd	a0,32(sp)
   12e5c:	7188                	ld	a0,32(a1)
   12e5e:	758c                	ld	a1,40(a1)

0000000000012e60 <.LBB647_7>:
   12e60:	00001617          	auipc	a2,0x1
   12e64:	75060613          	add	a2,a2,1872 # 145b0 <.Lanon.442aba94db1f841cd37d39ada1516238.138>
   12e68:	fc32                	sd	a2,56(sp)
   12e6a:	460d                	li	a2,3
   12e6c:	e0b2                	sd	a2,64(sp)
   12e6e:	e482                	sd	zero,72(sp)
   12e70:	0030                	add	a2,sp,8
   12e72:	ecb2                	sd	a2,88(sp)
   12e74:	4609                	li	a2,2
   12e76:	f0b2                	sd	a2,96(sp)
   12e78:	1830                	add	a2,sp,56
   12e7a:	ffffe097          	auipc	ra,0xffffe
   12e7e:	7c0080e7          	jalr	1984(ra) # 1163a <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   12e82:	70a6                	ld	ra,104(sp)
   12e84:	6165                	add	sp,sp,112
   12e86:	8082                	ret

0000000000012e88 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf12cc52608b51daeE>:
   12e88:	6510                	ld	a2,8(a0)
   12e8a:	6108                	ld	a0,0(a0)
   12e8c:	6e1c                	ld	a5,24(a2)
   12e8e:	8782                	jr	a5

0000000000012e90 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdd6e30ce7f47bbb8E>:
   12e90:	6114                	ld	a3,0(a0)
   12e92:	6510                	ld	a2,8(a0)
   12e94:	852e                	mv	a0,a1
   12e96:	85b6                	mv	a1,a3
   12e98:	fffff317          	auipc	t1,0xfffff
   12e9c:	c0a30067          	jr	-1014(t1) # 11aa2 <_ZN4core3fmt9Formatter3pad17h774baf72f2ffc077E>

0000000000012ea0 <_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h0090dc94d9494495E>:
   12ea0:	7179                	add	sp,sp,-48
   12ea2:	f406                	sd	ra,40(sp)
   12ea4:	f022                	sd	s0,32(sp)
   12ea6:	842e                	mv	s0,a1
   12ea8:	e02a                	sd	a0,0(sp)
   12eaa:	758c                	ld	a1,40(a1)
   12eac:	0521                	add	a0,a0,8
   12eae:	e42a                	sd	a0,8(sp)
   12eb0:	7008                	ld	a0,32(s0)
   12eb2:	6d94                	ld	a3,24(a1)

0000000000012eb4 <.LBB669_8>:
   12eb4:	00002597          	auipc	a1,0x2
   12eb8:	35958593          	add	a1,a1,857 # 1520d <.Lanon.442aba94db1f841cd37d39ada1516238.608>
   12ebc:	4625                	li	a2,9
   12ebe:	9682                	jalr	a3
   12ec0:	e822                	sd	s0,16(sp)
   12ec2:	00a10c23          	sb	a0,24(sp)
   12ec6:	00010ca3          	sb	zero,25(sp)

0000000000012eca <.LBB669_9>:
   12eca:	00002597          	auipc	a1,0x2
   12ece:	34c58593          	add	a1,a1,844 # 15216 <.Lanon.442aba94db1f841cd37d39ada1516238.609>

0000000000012ed2 <.LBB669_10>:
   12ed2:	00001717          	auipc	a4,0x1
   12ed6:	7c670713          	add	a4,a4,1990 # 14698 <.Lanon.442aba94db1f841cd37d39ada1516238.210>
   12eda:	0808                	add	a0,sp,16
   12edc:	462d                	li	a2,11
   12ede:	868a                	mv	a3,sp
   12ee0:	ffffe097          	auipc	ra,0xffffe
   12ee4:	296080e7          	jalr	662(ra) # 11176 <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E>

0000000000012ee8 <.LBB669_11>:
   12ee8:	00002597          	auipc	a1,0x2
   12eec:	33958593          	add	a1,a1,825 # 15221 <.Lanon.442aba94db1f841cd37d39ada1516238.610>

0000000000012ef0 <.LBB669_12>:
   12ef0:	00002717          	auipc	a4,0x2
   12ef4:	34070713          	add	a4,a4,832 # 15230 <.Lanon.442aba94db1f841cd37d39ada1516238.611>
   12ef8:	0808                	add	a0,sp,16
   12efa:	4625                	li	a2,9
   12efc:	0034                	add	a3,sp,8
   12efe:	ffffe097          	auipc	ra,0xffffe
   12f02:	278080e7          	jalr	632(ra) # 11176 <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E>
   12f06:	01914503          	lbu	a0,25(sp)
   12f0a:	01814583          	lbu	a1,24(sp)
   12f0e:	c91d                	beqz	a0,12f44 <.LBB669_14+0xe>
   12f10:	4505                	li	a0,1
   12f12:	e985                	bnez	a1,12f42 <.LBB669_14+0xc>
   12f14:	6542                	ld	a0,16(sp)
   12f16:	03054583          	lbu	a1,48(a0)
   12f1a:	8991                	and	a1,a1,4
   12f1c:	e991                	bnez	a1,12f30 <.LBB669_13+0xc>
   12f1e:	750c                	ld	a1,40(a0)
   12f20:	7108                	ld	a0,32(a0)
   12f22:	6d94                	ld	a3,24(a1)

0000000000012f24 <.LBB669_13>:
   12f24:	00001597          	auipc	a1,0x1
   12f28:	7f758593          	add	a1,a1,2039 # 1471b <.Lanon.442aba94db1f841cd37d39ada1516238.235>
   12f2c:	4609                	li	a2,2
   12f2e:	a809                	j	12f40 <.LBB669_14+0xa>
   12f30:	750c                	ld	a1,40(a0)
   12f32:	7108                	ld	a0,32(a0)
   12f34:	6d94                	ld	a3,24(a1)

0000000000012f36 <.LBB669_14>:
   12f36:	00001597          	auipc	a1,0x1
   12f3a:	7e458593          	add	a1,a1,2020 # 1471a <.Lanon.442aba94db1f841cd37d39ada1516238.232>
   12f3e:	4605                	li	a2,1
   12f40:	9682                	jalr	a3
   12f42:	85aa                	mv	a1,a0
   12f44:	00b03533          	snez	a0,a1
   12f48:	70a2                	ld	ra,40(sp)
   12f4a:	7402                	ld	s0,32(sp)
   12f4c:	6145                	add	sp,sp,48
   12f4e:	8082                	ret

0000000000012f50 <_ZN4core7unicode12unicode_data15grapheme_extend6lookup17h412ef2d588f4e502E>:
   12f50:	1141                	add	sp,sp,-16
   12f52:	e406                	sd	ra,8(sp)
   12f54:	4701                	li	a4,0
   12f56:	00b5131b          	sllw	t1,a0,0xb
   12f5a:	02000793          	li	a5,32

0000000000012f5e <.LBB699_26>:
   12f5e:	00002297          	auipc	t0,0x2
   12f62:	2f228293          	add	t0,t0,754 # 15250 <_ZN4core7unicode12unicode_data15grapheme_extend17SHORT_OFFSET_RUNS17h0644dacd678a09b8E>
   12f66:	4885                	li	a7,1
   12f68:	0ff00813          	li	a6,255
   12f6c:	02000693          	li	a3,32
   12f70:	a031                	j	12f7c <.LBB699_26+0x1e>
   12f72:	86be                	mv	a3,a5
   12f74:	40e687b3          	sub	a5,a3,a4
   12f78:	02d77b63          	bgeu	a4,a3,12fae <.LBB699_26+0x50>
   12f7c:	0017d593          	srl	a1,a5,0x1
   12f80:	00e587b3          	add	a5,a1,a4
   12f84:	00279593          	sll	a1,a5,0x2
   12f88:	9596                	add	a1,a1,t0
   12f8a:	418c                	lw	a1,0(a1)
   12f8c:	00b5961b          	sllw	a2,a1,0xb
   12f90:	55fd                	li	a1,-1
   12f92:	00666663          	bltu	a2,t1,12f9e <.LBB699_26+0x40>
   12f96:	006645b3          	xor	a1,a2,t1
   12f9a:	00b035b3          	snez	a1,a1
   12f9e:	fd158ae3          	beq	a1,a7,12f72 <.LBB699_26+0x14>
   12fa2:	00178713          	add	a4,a5,1
   12fa6:	0ff5f593          	zext.b	a1,a1
   12faa:	fd0585e3          	beq	a1,a6,12f74 <.LBB699_26+0x16>
   12fae:	45fd                	li	a1,31
   12fb0:	0ae5e763          	bltu	a1,a4,1305e <.LBB699_29>
   12fb4:	00271793          	sll	a5,a4,0x2
   12fb8:	2c300613          	li	a2,707
   12fbc:	00b70863          	beq	a4,a1,12fcc <.LBB699_26+0x6e>
   12fc0:	00f285b3          	add	a1,t0,a5
   12fc4:	0045e583          	lwu	a1,4(a1)
   12fc8:	0155d613          	srl	a2,a1,0x15
   12fcc:	fff70693          	add	a3,a4,-1
   12fd0:	00d77463          	bgeu	a4,a3,12fd8 <.LBB699_26+0x7a>
   12fd4:	4701                	li	a4,0
   12fd6:	a829                	j	12ff0 <.LBB699_26+0x92>
   12fd8:	02000593          	li	a1,32
   12fdc:	08b6fd63          	bgeu	a3,a1,13076 <.LBB699_30>
   12fe0:	00269593          	sll	a1,a3,0x2
   12fe4:	9596                	add	a1,a1,t0
   12fe6:	0005e583          	lwu	a1,0(a1)
   12fea:	15ae                	sll	a1,a1,0x2b
   12fec:	02b5d713          	srl	a4,a1,0x2b
   12ff0:	005785b3          	add	a1,a5,t0
   12ff4:	0005e583          	lwu	a1,0(a1)
   12ff8:	81d5                	srl	a1,a1,0x15
   12ffa:	fff5c693          	not	a3,a1
   12ffe:	96b2                	add	a3,a3,a2
   13000:	ce95                	beqz	a3,1303c <.LBB699_27+0x22>
   13002:	2c300793          	li	a5,707
   13006:	882e                	mv	a6,a1
   13008:	00b7e463          	bltu	a5,a1,13010 <.LBB699_26+0xb2>
   1300c:	2c300813          	li	a6,707
   13010:	4781                	li	a5,0
   13012:	40e5073b          	subw	a4,a0,a4
   13016:	fff60513          	add	a0,a2,-1

000000000001301a <.LBB699_27>:
   1301a:	00002617          	auipc	a2,0x2
   1301e:	2b660613          	add	a2,a2,694 # 152d0 <_ZN4core7unicode12unicode_data15grapheme_extend7OFFSETS17h3c7ea72c87f82ab8E>
   13022:	02b80263          	beq	a6,a1,13046 <.LBB699_28>
   13026:	00c586b3          	add	a3,a1,a2
   1302a:	0006c683          	lbu	a3,0(a3)
   1302e:	9fb5                	addw	a5,a5,a3
   13030:	00f76663          	bltu	a4,a5,1303c <.LBB699_27+0x22>
   13034:	0585                	add	a1,a1,1
   13036:	feb516e3          	bne	a0,a1,13022 <.LBB699_27+0x8>
   1303a:	85aa                	mv	a1,a0
   1303c:	0015f513          	and	a0,a1,1
   13040:	60a2                	ld	ra,8(sp)
   13042:	0141                	add	sp,sp,16
   13044:	8082                	ret

0000000000013046 <.LBB699_28>:
   13046:	00002617          	auipc	a2,0x2
   1304a:	55260613          	add	a2,a2,1362 # 15598 <.Lanon.442aba94db1f841cd37d39ada1516238.677>
   1304e:	2c300593          	li	a1,707
   13052:	8542                	mv	a0,a6
   13054:	ffffe097          	auipc	ra,0xffffe
   13058:	e68080e7          	jalr	-408(ra) # 10ebc <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>
	...

000000000001305e <.LBB699_29>:
   1305e:	00002617          	auipc	a2,0x2
   13062:	53a60613          	add	a2,a2,1338 # 15598 <.Lanon.442aba94db1f841cd37d39ada1516238.677>
   13066:	02000593          	li	a1,32
   1306a:	853a                	mv	a0,a4
   1306c:	ffffe097          	auipc	ra,0xffffe
   13070:	e50080e7          	jalr	-432(ra) # 10ebc <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>
	...

0000000000013076 <.LBB699_30>:
   13076:	00002617          	auipc	a2,0x2
   1307a:	17a60613          	add	a2,a2,378 # 151f0 <.Lanon.442aba94db1f841cd37d39ada1516238.396>
   1307e:	02000593          	li	a1,32
   13082:	8536                	mv	a0,a3
   13084:	ffffe097          	auipc	ra,0xffffe
   13088:	e38080e7          	jalr	-456(ra) # 10ebc <_ZN4core9panicking18panic_bounds_check17ha97449e8e354fe83E>
	...

000000000001308e <_ZN64_$LT$core..alloc..layout..Layout$u20$as$u20$core..fmt..Debug$GT$3fmt17h86d7136df2fe6134E>:
   1308e:	7179                	add	sp,sp,-48
   13090:	f406                	sd	ra,40(sp)
   13092:	f022                	sd	s0,32(sp)
   13094:	842e                	mv	s0,a1
   13096:	e02a                	sd	a0,0(sp)
   13098:	758c                	ld	a1,40(a1)
   1309a:	0521                	add	a0,a0,8
   1309c:	e42a                	sd	a0,8(sp)
   1309e:	7008                	ld	a0,32(s0)
   130a0:	6d94                	ld	a3,24(a1)

00000000000130a2 <.LBB709_8>:
   130a2:	00002597          	auipc	a1,0x2
   130a6:	50e58593          	add	a1,a1,1294 # 155b0 <.Lanon.442aba94db1f841cd37d39ada1516238.695>
   130aa:	4619                	li	a2,6
   130ac:	9682                	jalr	a3
   130ae:	e822                	sd	s0,16(sp)
   130b0:	00a10c23          	sb	a0,24(sp)
   130b4:	00010ca3          	sb	zero,25(sp)

00000000000130b8 <.LBB709_9>:
   130b8:	00001597          	auipc	a1,0x1
   130bc:	59c58593          	add	a1,a1,1436 # 14654 <.Lanon.442aba94db1f841cd37d39ada1516238.178+0x24>

00000000000130c0 <.LBB709_10>:
   130c0:	00001717          	auipc	a4,0x1
   130c4:	5d870713          	add	a4,a4,1496 # 14698 <.Lanon.442aba94db1f841cd37d39ada1516238.210>
   130c8:	0808                	add	a0,sp,16
   130ca:	4611                	li	a2,4
   130cc:	868a                	mv	a3,sp
   130ce:	ffffe097          	auipc	ra,0xffffe
   130d2:	0a8080e7          	jalr	168(ra) # 11176 <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E>

00000000000130d6 <.LBB709_11>:
   130d6:	00002597          	auipc	a1,0x2
   130da:	4e058593          	add	a1,a1,1248 # 155b6 <.Lanon.442aba94db1f841cd37d39ada1516238.697>

00000000000130de <.LBB709_12>:
   130de:	00002717          	auipc	a4,0x2
   130e2:	4e270713          	add	a4,a4,1250 # 155c0 <.Lanon.442aba94db1f841cd37d39ada1516238.698>
   130e6:	0808                	add	a0,sp,16
   130e8:	4615                	li	a2,5
   130ea:	0034                	add	a3,sp,8
   130ec:	ffffe097          	auipc	ra,0xffffe
   130f0:	08a080e7          	jalr	138(ra) # 11176 <_ZN4core3fmt8builders11DebugStruct5field17hf5b706c894532b15E>
   130f4:	01914503          	lbu	a0,25(sp)
   130f8:	01814583          	lbu	a1,24(sp)
   130fc:	c91d                	beqz	a0,13132 <.LBB709_14+0xe>
   130fe:	4505                	li	a0,1
   13100:	e985                	bnez	a1,13130 <.LBB709_14+0xc>
   13102:	6542                	ld	a0,16(sp)
   13104:	03054583          	lbu	a1,48(a0)
   13108:	8991                	and	a1,a1,4
   1310a:	e991                	bnez	a1,1311e <.LBB709_13+0xc>
   1310c:	750c                	ld	a1,40(a0)
   1310e:	7108                	ld	a0,32(a0)
   13110:	6d94                	ld	a3,24(a1)

0000000000013112 <.LBB709_13>:
   13112:	00001597          	auipc	a1,0x1
   13116:	60958593          	add	a1,a1,1545 # 1471b <.Lanon.442aba94db1f841cd37d39ada1516238.235>
   1311a:	4609                	li	a2,2
   1311c:	a809                	j	1312e <.LBB709_14+0xa>
   1311e:	750c                	ld	a1,40(a0)
   13120:	7108                	ld	a0,32(a0)
   13122:	6d94                	ld	a3,24(a1)

0000000000013124 <.LBB709_14>:
   13124:	00001597          	auipc	a1,0x1
   13128:	5f658593          	add	a1,a1,1526 # 1471a <.Lanon.442aba94db1f841cd37d39ada1516238.232>
   1312c:	4605                	li	a2,1
   1312e:	9682                	jalr	a3
   13130:	85aa                	mv	a1,a0
   13132:	00b03533          	snez	a0,a1
   13136:	70a2                	ld	ra,40(sp)
   13138:	7402                	ld	s0,32(sp)
   1313a:	6145                	add	sp,sp,48
   1313c:	8082                	ret

000000000001313e <rust_begin_unwind>:
   1313e:	7171                	add	sp,sp,-176
   13140:	f506                	sd	ra,168(sp)
   13142:	f122                	sd	s0,160(sp)
   13144:	ed26                	sd	s1,152(sp)
   13146:	1900                	add	s0,sp,176
   13148:	84aa                	mv	s1,a0
   1314a:	ffffe097          	auipc	ra,0xffffe
   1314e:	cf6080e7          	jalr	-778(ra) # 10e40 <_ZN4core5panic10panic_info9PanicInfo7message17h4abe68e22d422758E>
   13152:	12050763          	beqz	a0,13280 <.LBB0_18>
   13156:	f4a43c23          	sd	a0,-168(s0)
   1315a:	8526                	mv	a0,s1
   1315c:	ffffe097          	auipc	ra,0xffffe
   13160:	ce8080e7          	jalr	-792(ra) # 10e44 <_ZN4core5panic10panic_info9PanicInfo8location17h873d58c3c1958ff8E>
   13164:	cd41                	beqz	a0,131fc <.LBB0_11+0x1c>
   13166:	610c                	ld	a1,0(a0)
   13168:	6510                	ld	a2,8(a0)
   1316a:	f8b43823          	sd	a1,-112(s0)
   1316e:	f8c43c23          	sd	a2,-104(s0)
   13172:	4908                	lw	a0,16(a0)
   13174:	faa42223          	sw	a0,-92(s0)
   13178:	f9040513          	add	a0,s0,-112
   1317c:	f6a43023          	sd	a0,-160(s0)

0000000000013180 <.LBB0_7>:
   13180:	ffffd517          	auipc	a0,0xffffd
   13184:	55c50513          	add	a0,a0,1372 # 106dc <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hd4b1075b6d8c8353E>
   13188:	f6a43423          	sd	a0,-152(s0)
   1318c:	fa440513          	add	a0,s0,-92
   13190:	f6a43823          	sd	a0,-144(s0)

0000000000013194 <.LBB0_8>:
   13194:	00000517          	auipc	a0,0x0
   13198:	b2650513          	add	a0,a0,-1242 # 12cba <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h2cede4cfd0806b52E>
   1319c:	f6a43c23          	sd	a0,-136(s0)
   131a0:	f5840513          	add	a0,s0,-168
   131a4:	f8a43023          	sd	a0,-128(s0)

00000000000131a8 <.LBB0_9>:
   131a8:	ffffd517          	auipc	a0,0xffffd
   131ac:	51c50513          	add	a0,a0,1308 # 106c4 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h64d226d320fa66bfE>
   131b0:	f8a43423          	sd	a0,-120(s0)
   131b4:	fe040513          	add	a0,s0,-32
   131b8:	faa43423          	sd	a0,-88(s0)

00000000000131bc <.LBB0_10>:
   131bc:	00002517          	auipc	a0,0x2
   131c0:	46450513          	add	a0,a0,1124 # 15620 <.Lanon.86a3613c128665d32fc75176e6ae67c2.11>
   131c4:	faa43823          	sd	a0,-80(s0)
   131c8:	4511                	li	a0,4
   131ca:	faa43c23          	sd	a0,-72(s0)
   131ce:	fc043023          	sd	zero,-64(s0)
   131d2:	f6040513          	add	a0,s0,-160
   131d6:	fca43823          	sd	a0,-48(s0)
   131da:	450d                	li	a0,3
   131dc:	fca43c23          	sd	a0,-40(s0)

00000000000131e0 <.LBB0_11>:
   131e0:	00001597          	auipc	a1,0x1
   131e4:	ea058593          	add	a1,a1,-352 # 14080 <anon.cab5b07038618639c4e6406ab92cac85.0.llvm.17959331584031496199>
   131e8:	fa840513          	add	a0,s0,-88
   131ec:	fb040613          	add	a2,s0,-80
   131f0:	ffffe097          	auipc	ra,0xffffe
   131f4:	44a080e7          	jalr	1098(ra) # 1163a <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   131f8:	ed39                	bnez	a0,13256 <.LBB0_15>
   131fa:	a001                	j	131fa <.LBB0_11+0x1a>
   131fc:	f5840513          	add	a0,s0,-168
   13200:	f6a43023          	sd	a0,-160(s0)

0000000000013204 <.LBB0_12>:
   13204:	ffffd517          	auipc	a0,0xffffd
   13208:	4c050513          	add	a0,a0,1216 # 106c4 <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h64d226d320fa66bfE>
   1320c:	f6a43423          	sd	a0,-152(s0)
   13210:	fe040513          	add	a0,s0,-32
   13214:	f8a43823          	sd	a0,-112(s0)

0000000000013218 <.LBB0_13>:
   13218:	00002517          	auipc	a0,0x2
   1321c:	3d850513          	add	a0,a0,984 # 155f0 <.Lanon.86a3613c128665d32fc75176e6ae67c2.7>
   13220:	faa43823          	sd	a0,-80(s0)
   13224:	4509                	li	a0,2
   13226:	faa43c23          	sd	a0,-72(s0)
   1322a:	fc043023          	sd	zero,-64(s0)
   1322e:	f6040513          	add	a0,s0,-160
   13232:	fca43823          	sd	a0,-48(s0)
   13236:	4505                	li	a0,1
   13238:	fca43c23          	sd	a0,-40(s0)

000000000001323c <.LBB0_14>:
   1323c:	00001597          	auipc	a1,0x1
   13240:	e4458593          	add	a1,a1,-444 # 14080 <anon.cab5b07038618639c4e6406ab92cac85.0.llvm.17959331584031496199>
   13244:	f9040513          	add	a0,s0,-112
   13248:	fb040613          	add	a2,s0,-80
   1324c:	ffffe097          	auipc	ra,0xffffe
   13250:	3ee080e7          	jalr	1006(ra) # 1163a <_ZN4core3fmt5write17ha708f69ea5ad27e5E>
   13254:	d15d                	beqz	a0,131fa <.LBB0_11+0x1a>

0000000000013256 <.LBB0_15>:
   13256:	00001517          	auipc	a0,0x1
   1325a:	e5a50513          	add	a0,a0,-422 # 140b0 <anon.cab5b07038618639c4e6406ab92cac85.1.llvm.17959331584031496199>

000000000001325e <.LBB0_16>:
   1325e:	00001697          	auipc	a3,0x1
   13262:	e8268693          	add	a3,a3,-382 # 140e0 <anon.cab5b07038618639c4e6406ab92cac85.2.llvm.17959331584031496199>

0000000000013266 <.LBB0_17>:
   13266:	00001717          	auipc	a4,0x1
   1326a:	eaa70713          	add	a4,a4,-342 # 14110 <anon.cab5b07038618639c4e6406ab92cac85.4.llvm.17959331584031496199>
   1326e:	02b00593          	li	a1,43
   13272:	fe040613          	add	a2,s0,-32
   13276:	ffffe097          	auipc	ra,0xffffe
   1327a:	cb4080e7          	jalr	-844(ra) # 10f2a <_ZN4core6result13unwrap_failed17h3c2e5884ed497eadE>
	...

0000000000013280 <.LBB0_18>:
   13280:	00002517          	auipc	a0,0x2
   13284:	3e050513          	add	a0,a0,992 # 15660 <.Lanon.86a3613c128665d32fc75176e6ae67c2.12>

0000000000013288 <.LBB0_19>:
   13288:	00002617          	auipc	a2,0x2
   1328c:	41860613          	add	a2,a2,1048 # 156a0 <.Lanon.86a3613c128665d32fc75176e6ae67c2.14>
   13290:	02b00593          	li	a1,43
   13294:	ffffe097          	auipc	ra,0xffffe
   13298:	bb4080e7          	jalr	-1100(ra) # 10e48 <_ZN4core9panicking5panic17h92f54f473578363dE>
	...

000000000001329e <memcpy>:
   1329e:	00000317          	auipc	t1,0x0
   132a2:	00830067          	jr	8(t1) # 132a6 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE>

00000000000132a6 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE>:
   132a6:	46bd                	li	a3,15
   132a8:	06c6fa63          	bgeu	a3,a2,1331c <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x76>
   132ac:	40a006bb          	negw	a3,a0
   132b0:	0076f813          	and	a6,a3,7
   132b4:	010503b3          	add	t2,a0,a6
   132b8:	00080c63          	beqz	a6,132d0 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x2a>
   132bc:	87aa                	mv	a5,a0
   132be:	86ae                	mv	a3,a1
   132c0:	00068703          	lb	a4,0(a3)
   132c4:	00e78023          	sb	a4,0(a5)
   132c8:	0785                	add	a5,a5,1
   132ca:	0685                	add	a3,a3,1
   132cc:	fe77eae3          	bltu	a5,t2,132c0 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x1a>
   132d0:	010588b3          	add	a7,a1,a6
   132d4:	41060833          	sub	a6,a2,a6
   132d8:	ff887293          	and	t0,a6,-8
   132dc:	0078f593          	and	a1,a7,7
   132e0:	005386b3          	add	a3,t2,t0
   132e4:	cd9d                	beqz	a1,13322 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x7c>
   132e6:	04505863          	blez	t0,13336 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x90>
   132ea:	00359313          	sll	t1,a1,0x3
   132ee:	ff88f713          	and	a4,a7,-8
   132f2:	6310                	ld	a2,0(a4)
   132f4:	406005bb          	negw	a1,t1
   132f8:	0385fe13          	and	t3,a1,56
   132fc:	00870793          	add	a5,a4,8
   13300:	6398                	ld	a4,0(a5)
   13302:	00665633          	srl	a2,a2,t1
   13306:	01c715b3          	sll	a1,a4,t3
   1330a:	8dd1                	or	a1,a1,a2
   1330c:	00b3b023          	sd	a1,0(t2)
   13310:	03a1                	add	t2,t2,8
   13312:	07a1                	add	a5,a5,8
   13314:	863a                	mv	a2,a4
   13316:	fed3e5e3          	bltu	t2,a3,13300 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x5a>
   1331a:	a831                	j	13336 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x90>
   1331c:	86aa                	mv	a3,a0
   1331e:	e20d                	bnez	a2,13340 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x9a>
   13320:	a80d                	j	13352 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0xac>
   13322:	00505a63          	blez	t0,13336 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x90>
   13326:	85c6                	mv	a1,a7
   13328:	6190                	ld	a2,0(a1)
   1332a:	00c3b023          	sd	a2,0(t2)
   1332e:	03a1                	add	t2,t2,8
   13330:	05a1                	add	a1,a1,8
   13332:	fed3ebe3          	bltu	t2,a3,13328 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x82>
   13336:	005885b3          	add	a1,a7,t0
   1333a:	00787613          	and	a2,a6,7
   1333e:	ca11                	beqz	a2,13352 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0xac>
   13340:	9636                	add	a2,a2,a3
   13342:	00058703          	lb	a4,0(a1)
   13346:	00e68023          	sb	a4,0(a3)
   1334a:	0685                	add	a3,a3,1
   1334c:	0585                	add	a1,a1,1
   1334e:	fec6eae3          	bltu	a3,a2,13342 <_ZN17compiler_builtins3mem6memcpy17h1682501fa558d2baE+0x9c>
   13352:	8082                	ret
