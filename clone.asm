
clone:     file format elf64-littleriscv


Disassembly of section .text:

0000000000001000 <_start>:
    1000:	850a                	mv	a0,sp
    1002:	a0f5                	j	10ee <__start_main>

0000000000001004 <child_func>:
    1004:	1141                	add	sp,sp,-16
    1006:	00001517          	auipc	a0,0x1
    100a:	efa50513          	add	a0,a0,-262 # 1f00 <__clone+0x2a>
    100e:	e406                	sd	ra,8(sp)
    1010:	358000ef          	jal	1368 <printf>
    1014:	60a2                	ld	ra,8(sp)
    1016:	4501                	li	a0,0
    1018:	0141                	add	sp,sp,16
    101a:	8082                	ret

000000000000101c <test_clone>:
    101c:	1101                	add	sp,sp,-32
    101e:	00001517          	auipc	a0,0x1
    1022:	f0250513          	add	a0,a0,-254 # 1f20 <__clone+0x4a>
    1026:	ec06                	sd	ra,24(sp)
    1028:	e822                	sd	s0,16(sp)
    102a:	31c000ef          	jal	1346 <puts>
    102e:	00003517          	auipc	a0,0x3
    1032:	fb250513          	add	a0,a0,-78 # 3fe0 <__func__.0>
    1036:	310000ef          	jal	1346 <puts>
    103a:	00001517          	auipc	a0,0x1
    103e:	efe50513          	add	a0,a0,-258 # 1f38 <__clone+0x62>
    1042:	304000ef          	jal	1346 <puts>
    1046:	4745                	li	a4,17
    1048:	40000693          	li	a3,1024
    104c:	00001617          	auipc	a2,0x1
    1050:	f8c60613          	add	a2,a2,-116 # 1fd8 <stack>
    1054:	4581                	li	a1,0
    1056:	00000517          	auipc	a0,0x0
    105a:	fae50513          	add	a0,a0,-82 # 1004 <child_func>
    105e:	47f000ef          	jal	1cdc <clone>
    1062:	00003417          	auipc	s0,0x3
    1066:	f7640413          	add	s0,s0,-138 # 3fd8 <child_pid>
    106a:	c008                	sw	a0,0(s0)
    106c:	57fd                	li	a5,-1
    106e:	04f50863          	beq	a0,a5,10be <test_clone+0xa2>
    1072:	e90d                	bnez	a0,10a4 <test_clone+0x88>
    1074:	47b000ef          	jal	1cee <exit>
    1078:	00001517          	auipc	a0,0x1
    107c:	f3050513          	add	a0,a0,-208 # 1fa8 <__clone+0xd2>
    1080:	2c6000ef          	jal	1346 <puts>
    1084:	00003517          	auipc	a0,0x3
    1088:	f5c50513          	add	a0,a0,-164 # 3fe0 <__func__.0>
    108c:	2ba000ef          	jal	1346 <puts>
    1090:	00001517          	auipc	a0,0x1
    1094:	ea850513          	add	a0,a0,-344 # 1f38 <__clone+0x62>
    1098:	2ae000ef          	jal	1346 <puts>
    109c:	60e2                	ld	ra,24(sp)
    109e:	6442                	ld	s0,16(sp)
    10a0:	6105                	add	sp,sp,32
    10a2:	8082                	ret
    10a4:	0068                	add	a0,sp,12
    10a6:	50d000ef          	jal	1db2 <wait>
    10aa:	401c                	lw	a5,0(s0)
    10ac:	02f50163          	beq	a0,a5,10ce <test_clone+0xb2>
    10b0:	00001517          	auipc	a0,0x1
    10b4:	ee050513          	add	a0,a0,-288 # 1f90 <__clone+0xba>
    10b8:	2b0000ef          	jal	1368 <printf>
    10bc:	bf75                	j	1078 <test_clone+0x5c>
    10be:	00001517          	auipc	a0,0x1
    10c2:	e8a50513          	add	a0,a0,-374 # 1f48 <__clone+0x72>
    10c6:	51c000ef          	jal	15e2 <panic>
    10ca:	4008                	lw	a0,0(s0)
    10cc:	b75d                	j	1072 <test_clone+0x56>
    10ce:	85aa                	mv	a1,a0
    10d0:	00001517          	auipc	a0,0x1
    10d4:	e9850513          	add	a0,a0,-360 # 1f68 <__clone+0x92>
    10d8:	290000ef          	jal	1368 <printf>
    10dc:	bf71                	j	1078 <test_clone+0x5c>

00000000000010de <main>:
    10de:	1141                	add	sp,sp,-16
    10e0:	e406                	sd	ra,8(sp)
    10e2:	f3bff0ef          	jal	101c <test_clone>
    10e6:	60a2                	ld	ra,8(sp)
    10e8:	4501                	li	a0,0
    10ea:	0141                	add	sp,sp,16
    10ec:	8082                	ret

00000000000010ee <__start_main>:
    10ee:	85aa                	mv	a1,a0
    10f0:	4108                	lw	a0,0(a0)
    10f2:	1141                	add	sp,sp,-16
    10f4:	05a1                	add	a1,a1,8
    10f6:	e406                	sd	ra,8(sp)
    10f8:	fe7ff0ef          	jal	10de <main>
    10fc:	3f3000ef          	jal	1cee <exit>
    1100:	60a2                	ld	ra,8(sp)
    1102:	4501                	li	a0,0
    1104:	0141                	add	sp,sp,16
    1106:	8082                	ret

0000000000001108 <printint.constprop.0>:
    1108:	7179                	add	sp,sp,-48
    110a:	f406                	sd	ra,40(sp)
    110c:	12054863          	bltz	a0,123c <printint.constprop.0+0x134>
    1110:	02b577bb          	remuw	a5,a0,a1
    1114:	00003697          	auipc	a3,0x3
    1118:	edc68693          	add	a3,a3,-292 # 3ff0 <digits>
    111c:	00010c23          	sb	zero,24(sp)
    1120:	0005871b          	sext.w	a4,a1
    1124:	1782                	sll	a5,a5,0x20
    1126:	9381                	srl	a5,a5,0x20
    1128:	97b6                	add	a5,a5,a3
    112a:	0007c783          	lbu	a5,0(a5)
    112e:	02b5583b          	divuw	a6,a0,a1
    1132:	00f10ba3          	sb	a5,23(sp)
    1136:	1ab56663          	bltu	a0,a1,12e2 <printint.constprop.0+0x1da>
    113a:	02e8763b          	remuw	a2,a6,a4
    113e:	1602                	sll	a2,a2,0x20
    1140:	9201                	srl	a2,a2,0x20
    1142:	9636                	add	a2,a2,a3
    1144:	00064603          	lbu	a2,0(a2)
    1148:	02e855bb          	divuw	a1,a6,a4
    114c:	00c10b23          	sb	a2,22(sp)
    1150:	12e86c63          	bltu	a6,a4,1288 <printint.constprop.0+0x180>
    1154:	02e5f63b          	remuw	a2,a1,a4
    1158:	1602                	sll	a2,a2,0x20
    115a:	9201                	srl	a2,a2,0x20
    115c:	9636                	add	a2,a2,a3
    115e:	00064603          	lbu	a2,0(a2)
    1162:	02e5d83b          	divuw	a6,a1,a4
    1166:	00c10aa3          	sb	a2,21(sp)
    116a:	12e5e863          	bltu	a1,a4,129a <printint.constprop.0+0x192>
    116e:	02e8763b          	remuw	a2,a6,a4
    1172:	1602                	sll	a2,a2,0x20
    1174:	9201                	srl	a2,a2,0x20
    1176:	9636                	add	a2,a2,a3
    1178:	00064603          	lbu	a2,0(a2)
    117c:	02e855bb          	divuw	a1,a6,a4
    1180:	00c10a23          	sb	a2,20(sp)
    1184:	12e86463          	bltu	a6,a4,12ac <printint.constprop.0+0x1a4>
    1188:	02e5f63b          	remuw	a2,a1,a4
    118c:	1602                	sll	a2,a2,0x20
    118e:	9201                	srl	a2,a2,0x20
    1190:	9636                	add	a2,a2,a3
    1192:	00064603          	lbu	a2,0(a2)
    1196:	02e5d83b          	divuw	a6,a1,a4
    119a:	00c109a3          	sb	a2,19(sp)
    119e:	12e5e063          	bltu	a1,a4,12be <printint.constprop.0+0x1b6>
    11a2:	02e8763b          	remuw	a2,a6,a4
    11a6:	1602                	sll	a2,a2,0x20
    11a8:	9201                	srl	a2,a2,0x20
    11aa:	9636                	add	a2,a2,a3
    11ac:	00064603          	lbu	a2,0(a2)
    11b0:	02e855bb          	divuw	a1,a6,a4
    11b4:	00c10923          	sb	a2,18(sp)
    11b8:	0ae86f63          	bltu	a6,a4,1276 <printint.constprop.0+0x16e>
    11bc:	02e5f63b          	remuw	a2,a1,a4
    11c0:	1602                	sll	a2,a2,0x20
    11c2:	9201                	srl	a2,a2,0x20
    11c4:	9636                	add	a2,a2,a3
    11c6:	00064603          	lbu	a2,0(a2)
    11ca:	02e5d83b          	divuw	a6,a1,a4
    11ce:	00c108a3          	sb	a2,17(sp)
    11d2:	0ee5ef63          	bltu	a1,a4,12d0 <printint.constprop.0+0x1c8>
    11d6:	02e8763b          	remuw	a2,a6,a4
    11da:	1602                	sll	a2,a2,0x20
    11dc:	9201                	srl	a2,a2,0x20
    11de:	9636                	add	a2,a2,a3
    11e0:	00064603          	lbu	a2,0(a2)
    11e4:	02e855bb          	divuw	a1,a6,a4
    11e8:	00c10823          	sb	a2,16(sp)
    11ec:	0ee86d63          	bltu	a6,a4,12e6 <printint.constprop.0+0x1de>
    11f0:	02e5f63b          	remuw	a2,a1,a4
    11f4:	1602                	sll	a2,a2,0x20
    11f6:	9201                	srl	a2,a2,0x20
    11f8:	9636                	add	a2,a2,a3
    11fa:	00064603          	lbu	a2,0(a2)
    11fe:	02e5d7bb          	divuw	a5,a1,a4
    1202:	00c107a3          	sb	a2,15(sp)
    1206:	0ee5e963          	bltu	a1,a4,12f8 <printint.constprop.0+0x1f0>
    120a:	1782                	sll	a5,a5,0x20
    120c:	9381                	srl	a5,a5,0x20
    120e:	96be                	add	a3,a3,a5
    1210:	0006c783          	lbu	a5,0(a3)
    1214:	4599                	li	a1,6
    1216:	00f10723          	sb	a5,14(sp)
    121a:	00055763          	bgez	a0,1228 <printint.constprop.0+0x120>
    121e:	02d00793          	li	a5,45
    1222:	00f106a3          	sb	a5,13(sp)
    1226:	4595                	li	a1,5
    1228:	003c                	add	a5,sp,8
    122a:	4641                	li	a2,16
    122c:	9e0d                	subw	a2,a2,a1
    122e:	4505                	li	a0,1
    1230:	95be                	add	a1,a1,a5
    1232:	26d000ef          	jal	1c9e <write>
    1236:	70a2                	ld	ra,40(sp)
    1238:	6145                	add	sp,sp,48
    123a:	8082                	ret
    123c:	40a0063b          	negw	a2,a0
    1240:	02b677bb          	remuw	a5,a2,a1
    1244:	00003697          	auipc	a3,0x3
    1248:	dac68693          	add	a3,a3,-596 # 3ff0 <digits>
    124c:	00010c23          	sb	zero,24(sp)
    1250:	0005871b          	sext.w	a4,a1
    1254:	1782                	sll	a5,a5,0x20
    1256:	9381                	srl	a5,a5,0x20
    1258:	97b6                	add	a5,a5,a3
    125a:	0007c783          	lbu	a5,0(a5)
    125e:	02b6583b          	divuw	a6,a2,a1
    1262:	00f10ba3          	sb	a5,23(sp)
    1266:	ecb67ae3          	bgeu	a2,a1,113a <printint.constprop.0+0x32>
    126a:	02d00793          	li	a5,45
    126e:	00f10b23          	sb	a5,22(sp)
    1272:	45b9                	li	a1,14
    1274:	bf55                	j	1228 <printint.constprop.0+0x120>
    1276:	45a9                	li	a1,10
    1278:	fa0558e3          	bgez	a0,1228 <printint.constprop.0+0x120>
    127c:	02d00793          	li	a5,45
    1280:	00f108a3          	sb	a5,17(sp)
    1284:	45a5                	li	a1,9
    1286:	b74d                	j	1228 <printint.constprop.0+0x120>
    1288:	45b9                	li	a1,14
    128a:	f8055fe3          	bgez	a0,1228 <printint.constprop.0+0x120>
    128e:	02d00793          	li	a5,45
    1292:	00f10aa3          	sb	a5,21(sp)
    1296:	45b5                	li	a1,13
    1298:	bf41                	j	1228 <printint.constprop.0+0x120>
    129a:	45b5                	li	a1,13
    129c:	f80556e3          	bgez	a0,1228 <printint.constprop.0+0x120>
    12a0:	02d00793          	li	a5,45
    12a4:	00f10a23          	sb	a5,20(sp)
    12a8:	45b1                	li	a1,12
    12aa:	bfbd                	j	1228 <printint.constprop.0+0x120>
    12ac:	45b1                	li	a1,12
    12ae:	f6055de3          	bgez	a0,1228 <printint.constprop.0+0x120>
    12b2:	02d00793          	li	a5,45
    12b6:	00f109a3          	sb	a5,19(sp)
    12ba:	45ad                	li	a1,11
    12bc:	b7b5                	j	1228 <printint.constprop.0+0x120>
    12be:	45ad                	li	a1,11
    12c0:	f60554e3          	bgez	a0,1228 <printint.constprop.0+0x120>
    12c4:	02d00793          	li	a5,45
    12c8:	00f10923          	sb	a5,18(sp)
    12cc:	45a9                	li	a1,10
    12ce:	bfa9                	j	1228 <printint.constprop.0+0x120>
    12d0:	45a5                	li	a1,9
    12d2:	f4055be3          	bgez	a0,1228 <printint.constprop.0+0x120>
    12d6:	02d00793          	li	a5,45
    12da:	00f10823          	sb	a5,16(sp)
    12de:	45a1                	li	a1,8
    12e0:	b7a1                	j	1228 <printint.constprop.0+0x120>
    12e2:	45bd                	li	a1,15
    12e4:	b791                	j	1228 <printint.constprop.0+0x120>
    12e6:	45a1                	li	a1,8
    12e8:	f40550e3          	bgez	a0,1228 <printint.constprop.0+0x120>
    12ec:	02d00793          	li	a5,45
    12f0:	00f107a3          	sb	a5,15(sp)
    12f4:	459d                	li	a1,7
    12f6:	bf0d                	j	1228 <printint.constprop.0+0x120>
    12f8:	459d                	li	a1,7
    12fa:	f20557e3          	bgez	a0,1228 <printint.constprop.0+0x120>
    12fe:	02d00793          	li	a5,45
    1302:	00f10723          	sb	a5,14(sp)
    1306:	4599                	li	a1,6
    1308:	b705                	j	1228 <printint.constprop.0+0x120>

000000000000130a <getchar>:
    130a:	1101                	add	sp,sp,-32
    130c:	00f10593          	add	a1,sp,15
    1310:	4605                	li	a2,1
    1312:	4501                	li	a0,0
    1314:	ec06                	sd	ra,24(sp)
    1316:	000107a3          	sb	zero,15(sp)
    131a:	17b000ef          	jal	1c94 <read>
    131e:	60e2                	ld	ra,24(sp)
    1320:	00f14503          	lbu	a0,15(sp)
    1324:	6105                	add	sp,sp,32
    1326:	8082                	ret

0000000000001328 <putchar>:
    1328:	1101                	add	sp,sp,-32
    132a:	87aa                	mv	a5,a0
    132c:	00f10593          	add	a1,sp,15
    1330:	4605                	li	a2,1
    1332:	4505                	li	a0,1
    1334:	ec06                	sd	ra,24(sp)
    1336:	00f107a3          	sb	a5,15(sp)
    133a:	165000ef          	jal	1c9e <write>
    133e:	60e2                	ld	ra,24(sp)
    1340:	2501                	sext.w	a0,a0
    1342:	6105                	add	sp,sp,32
    1344:	8082                	ret

0000000000001346 <puts>:
    1346:	1141                	add	sp,sp,-16
    1348:	e406                	sd	ra,8(sp)
    134a:	e022                	sd	s0,0(sp)
    134c:	842a                	mv	s0,a0
    134e:	574000ef          	jal	18c2 <strlen>
    1352:	862a                	mv	a2,a0
    1354:	85a2                	mv	a1,s0
    1356:	4505                	li	a0,1
    1358:	147000ef          	jal	1c9e <write>
    135c:	60a2                	ld	ra,8(sp)
    135e:	6402                	ld	s0,0(sp)
    1360:	957d                	sra	a0,a0,0x3f
    1362:	2501                	sext.w	a0,a0
    1364:	0141                	add	sp,sp,16
    1366:	8082                	ret

0000000000001368 <printf>:
    1368:	7171                	add	sp,sp,-176
    136a:	f85a                	sd	s6,48(sp)
    136c:	ed3e                	sd	a5,152(sp)
    136e:	7b61                	lui	s6,0xffff8
    1370:	18bc                	add	a5,sp,120
    1372:	e8ca                	sd	s2,80(sp)
    1374:	e4ce                	sd	s3,72(sp)
    1376:	e0d2                	sd	s4,64(sp)
    1378:	fc56                	sd	s5,56(sp)
    137a:	f486                	sd	ra,104(sp)
    137c:	f0a2                	sd	s0,96(sp)
    137e:	eca6                	sd	s1,88(sp)
    1380:	fcae                	sd	a1,120(sp)
    1382:	e132                	sd	a2,128(sp)
    1384:	e536                	sd	a3,136(sp)
    1386:	e93a                	sd	a4,144(sp)
    1388:	f142                	sd	a6,160(sp)
    138a:	f546                	sd	a7,168(sp)
    138c:	e03e                	sd	a5,0(sp)
    138e:	02500913          	li	s2,37
    1392:	07300a13          	li	s4,115
    1396:	07800a93          	li	s5,120
    139a:	830b4b13          	xor	s6,s6,-2000
    139e:	00003997          	auipc	s3,0x3
    13a2:	c5298993          	add	s3,s3,-942 # 3ff0 <digits>
    13a6:	00054783          	lbu	a5,0(a0)
    13aa:	16078a63          	beqz	a5,151e <printf+0x1b6>
    13ae:	862a                	mv	a2,a0
    13b0:	19278d63          	beq	a5,s2,154a <printf+0x1e2>
    13b4:	00164783          	lbu	a5,1(a2)
    13b8:	0605                	add	a2,a2,1
    13ba:	fbfd                	bnez	a5,13b0 <printf+0x48>
    13bc:	84b2                	mv	s1,a2
    13be:	40a6043b          	subw	s0,a2,a0
    13c2:	85aa                	mv	a1,a0
    13c4:	8622                	mv	a2,s0
    13c6:	4505                	li	a0,1
    13c8:	0d7000ef          	jal	1c9e <write>
    13cc:	1a041463          	bnez	s0,1574 <printf+0x20c>
    13d0:	0014c783          	lbu	a5,1(s1)
    13d4:	14078563          	beqz	a5,151e <printf+0x1b6>
    13d8:	1b478063          	beq	a5,s4,1578 <printf+0x210>
    13dc:	14fa6b63          	bltu	s4,a5,1532 <printf+0x1ca>
    13e0:	06400713          	li	a4,100
    13e4:	1ee78063          	beq	a5,a4,15c4 <printf+0x25c>
    13e8:	07000713          	li	a4,112
    13ec:	1ae79963          	bne	a5,a4,159e <printf+0x236>
    13f0:	6702                	ld	a4,0(sp)
    13f2:	01611423          	sh	s6,8(sp)
    13f6:	4649                	li	a2,18
    13f8:	631c                	ld	a5,0(a4)
    13fa:	0721                	add	a4,a4,8
    13fc:	e03a                	sd	a4,0(sp)
    13fe:	00479293          	sll	t0,a5,0x4
    1402:	00879f93          	sll	t6,a5,0x8
    1406:	00c79f13          	sll	t5,a5,0xc
    140a:	01079e93          	sll	t4,a5,0x10
    140e:	01479e13          	sll	t3,a5,0x14
    1412:	01879313          	sll	t1,a5,0x18
    1416:	01c79893          	sll	a7,a5,0x1c
    141a:	02479813          	sll	a6,a5,0x24
    141e:	02879513          	sll	a0,a5,0x28
    1422:	02c79593          	sll	a1,a5,0x2c
    1426:	03079693          	sll	a3,a5,0x30
    142a:	03479713          	sll	a4,a5,0x34
    142e:	03c7d413          	srl	s0,a5,0x3c
    1432:	01c7d39b          	srlw	t2,a5,0x1c
    1436:	03c2d293          	srl	t0,t0,0x3c
    143a:	03cfdf93          	srl	t6,t6,0x3c
    143e:	03cf5f13          	srl	t5,t5,0x3c
    1442:	03cede93          	srl	t4,t4,0x3c
    1446:	03ce5e13          	srl	t3,t3,0x3c
    144a:	03c35313          	srl	t1,t1,0x3c
    144e:	03c8d893          	srl	a7,a7,0x3c
    1452:	03c85813          	srl	a6,a6,0x3c
    1456:	9171                	srl	a0,a0,0x3c
    1458:	91f1                	srl	a1,a1,0x3c
    145a:	92f1                	srl	a3,a3,0x3c
    145c:	9371                	srl	a4,a4,0x3c
    145e:	96ce                	add	a3,a3,s3
    1460:	974e                	add	a4,a4,s3
    1462:	944e                	add	s0,s0,s3
    1464:	92ce                	add	t0,t0,s3
    1466:	9fce                	add	t6,t6,s3
    1468:	9f4e                	add	t5,t5,s3
    146a:	9ece                	add	t4,t4,s3
    146c:	9e4e                	add	t3,t3,s3
    146e:	934e                	add	t1,t1,s3
    1470:	98ce                	add	a7,a7,s3
    1472:	93ce                	add	t2,t2,s3
    1474:	984e                	add	a6,a6,s3
    1476:	954e                	add	a0,a0,s3
    1478:	95ce                	add	a1,a1,s3
    147a:	0006c083          	lbu	ra,0(a3)
    147e:	0002c283          	lbu	t0,0(t0)
    1482:	00074683          	lbu	a3,0(a4)
    1486:	000fcf83          	lbu	t6,0(t6)
    148a:	000f4f03          	lbu	t5,0(t5)
    148e:	000ece83          	lbu	t4,0(t4)
    1492:	000e4e03          	lbu	t3,0(t3)
    1496:	00034303          	lbu	t1,0(t1)
    149a:	0008c883          	lbu	a7,0(a7)
    149e:	0003c383          	lbu	t2,0(t2)
    14a2:	00084803          	lbu	a6,0(a6)
    14a6:	00054503          	lbu	a0,0(a0)
    14aa:	0005c583          	lbu	a1,0(a1)
    14ae:	00044403          	lbu	s0,0(s0)
    14b2:	03879713          	sll	a4,a5,0x38
    14b6:	9371                	srl	a4,a4,0x3c
    14b8:	8bbd                	and	a5,a5,15
    14ba:	974e                	add	a4,a4,s3
    14bc:	97ce                	add	a5,a5,s3
    14be:	005105a3          	sb	t0,11(sp)
    14c2:	01f10623          	sb	t6,12(sp)
    14c6:	01e106a3          	sb	t5,13(sp)
    14ca:	01d10723          	sb	t4,14(sp)
    14ce:	01c107a3          	sb	t3,15(sp)
    14d2:	00610823          	sb	t1,16(sp)
    14d6:	011108a3          	sb	a7,17(sp)
    14da:	00710923          	sb	t2,18(sp)
    14de:	010109a3          	sb	a6,19(sp)
    14e2:	00a10a23          	sb	a0,20(sp)
    14e6:	00b10aa3          	sb	a1,21(sp)
    14ea:	00110b23          	sb	ra,22(sp)
    14ee:	00d10ba3          	sb	a3,23(sp)
    14f2:	00810523          	sb	s0,10(sp)
    14f6:	00074703          	lbu	a4,0(a4)
    14fa:	0007c783          	lbu	a5,0(a5)
    14fe:	002c                	add	a1,sp,8
    1500:	4505                	li	a0,1
    1502:	00e10c23          	sb	a4,24(sp)
    1506:	00f10ca3          	sb	a5,25(sp)
    150a:	00010d23          	sb	zero,26(sp)
    150e:	790000ef          	jal	1c9e <write>
    1512:	00248513          	add	a0,s1,2
    1516:	00054783          	lbu	a5,0(a0)
    151a:	e8079ae3          	bnez	a5,13ae <printf+0x46>
    151e:	70a6                	ld	ra,104(sp)
    1520:	7406                	ld	s0,96(sp)
    1522:	64e6                	ld	s1,88(sp)
    1524:	6946                	ld	s2,80(sp)
    1526:	69a6                	ld	s3,72(sp)
    1528:	6a06                	ld	s4,64(sp)
    152a:	7ae2                	ld	s5,56(sp)
    152c:	7b42                	ld	s6,48(sp)
    152e:	614d                	add	sp,sp,176
    1530:	8082                	ret
    1532:	07579663          	bne	a5,s5,159e <printf+0x236>
    1536:	6782                	ld	a5,0(sp)
    1538:	45c1                	li	a1,16
    153a:	4388                	lw	a0,0(a5)
    153c:	07a1                	add	a5,a5,8
    153e:	e03e                	sd	a5,0(sp)
    1540:	bc9ff0ef          	jal	1108 <printint.constprop.0>
    1544:	00248513          	add	a0,s1,2
    1548:	b7f9                	j	1516 <printf+0x1ae>
    154a:	84b2                	mv	s1,a2
    154c:	a039                	j	155a <printf+0x1f2>
    154e:	0024c783          	lbu	a5,2(s1)
    1552:	0605                	add	a2,a2,1
    1554:	0489                	add	s1,s1,2
    1556:	e72794e3          	bne	a5,s2,13be <printf+0x56>
    155a:	0014c783          	lbu	a5,1(s1)
    155e:	ff2788e3          	beq	a5,s2,154e <printf+0x1e6>
    1562:	40a6043b          	subw	s0,a2,a0
    1566:	85aa                	mv	a1,a0
    1568:	8622                	mv	a2,s0
    156a:	4505                	li	a0,1
    156c:	732000ef          	jal	1c9e <write>
    1570:	e60400e3          	beqz	s0,13d0 <printf+0x68>
    1574:	8526                	mv	a0,s1
    1576:	bd05                	j	13a6 <printf+0x3e>
    1578:	6782                	ld	a5,0(sp)
    157a:	6380                	ld	s0,0(a5)
    157c:	07a1                	add	a5,a5,8
    157e:	e03e                	sd	a5,0(sp)
    1580:	cc21                	beqz	s0,15d8 <printf+0x270>
    1582:	0c800593          	li	a1,200
    1586:	8522                	mv	a0,s0
    1588:	424000ef          	jal	19ac <strnlen>
    158c:	0005061b          	sext.w	a2,a0
    1590:	85a2                	mv	a1,s0
    1592:	4505                	li	a0,1
    1594:	70a000ef          	jal	1c9e <write>
    1598:	00248513          	add	a0,s1,2
    159c:	bfad                	j	1516 <printf+0x1ae>
    159e:	4605                	li	a2,1
    15a0:	002c                	add	a1,sp,8
    15a2:	4505                	li	a0,1
    15a4:	01210423          	sb	s2,8(sp)
    15a8:	6f6000ef          	jal	1c9e <write>
    15ac:	0014c783          	lbu	a5,1(s1)
    15b0:	4605                	li	a2,1
    15b2:	002c                	add	a1,sp,8
    15b4:	4505                	li	a0,1
    15b6:	00f10423          	sb	a5,8(sp)
    15ba:	6e4000ef          	jal	1c9e <write>
    15be:	00248513          	add	a0,s1,2
    15c2:	bf91                	j	1516 <printf+0x1ae>
    15c4:	6782                	ld	a5,0(sp)
    15c6:	45a9                	li	a1,10
    15c8:	4388                	lw	a0,0(a5)
    15ca:	07a1                	add	a5,a5,8
    15cc:	e03e                	sd	a5,0(sp)
    15ce:	b3bff0ef          	jal	1108 <printint.constprop.0>
    15d2:	00248513          	add	a0,s1,2
    15d6:	b781                	j	1516 <printf+0x1ae>
    15d8:	00001417          	auipc	s0,0x1
    15dc:	9e040413          	add	s0,s0,-1568 # 1fb8 <__clone+0xe2>
    15e0:	b74d                	j	1582 <printf+0x21a>

00000000000015e2 <panic>:
    15e2:	1141                	add	sp,sp,-16
    15e4:	e406                	sd	ra,8(sp)
    15e6:	d61ff0ef          	jal	1346 <puts>
    15ea:	60a2                	ld	ra,8(sp)
    15ec:	f9c00513          	li	a0,-100
    15f0:	0141                	add	sp,sp,16
    15f2:	adf5                	j	1cee <exit>

00000000000015f4 <isspace>:
    15f4:	02000793          	li	a5,32
    15f8:	00f50663          	beq	a0,a5,1604 <isspace+0x10>
    15fc:	355d                	addw	a0,a0,-9
    15fe:	00553513          	sltiu	a0,a0,5
    1602:	8082                	ret
    1604:	4505                	li	a0,1
    1606:	8082                	ret

0000000000001608 <isdigit>:
    1608:	fd05051b          	addw	a0,a0,-48
    160c:	00a53513          	sltiu	a0,a0,10
    1610:	8082                	ret

0000000000001612 <atoi>:
    1612:	02000693          	li	a3,32
    1616:	4591                	li	a1,4
    1618:	00054783          	lbu	a5,0(a0)
    161c:	ff77871b          	addw	a4,a5,-9
    1620:	04d78c63          	beq	a5,a3,1678 <atoi+0x66>
    1624:	0007861b          	sext.w	a2,a5
    1628:	04e5f863          	bgeu	a1,a4,1678 <atoi+0x66>
    162c:	02b00713          	li	a4,43
    1630:	04e78963          	beq	a5,a4,1682 <atoi+0x70>
    1634:	02d00713          	li	a4,45
    1638:	06e78263          	beq	a5,a4,169c <atoi+0x8a>
    163c:	fd06069b          	addw	a3,a2,-48
    1640:	47a5                	li	a5,9
    1642:	872a                	mv	a4,a0
    1644:	4301                	li	t1,0
    1646:	04d7e963          	bltu	a5,a3,1698 <atoi+0x86>
    164a:	4501                	li	a0,0
    164c:	48a5                	li	a7,9
    164e:	00174683          	lbu	a3,1(a4)
    1652:	0025179b          	sllw	a5,a0,0x2
    1656:	9fa9                	addw	a5,a5,a0
    1658:	fd06059b          	addw	a1,a2,-48
    165c:	0017979b          	sllw	a5,a5,0x1
    1660:	fd06881b          	addw	a6,a3,-48
    1664:	0705                	add	a4,a4,1
    1666:	40b7853b          	subw	a0,a5,a1
    166a:	0006861b          	sext.w	a2,a3
    166e:	ff08f0e3          	bgeu	a7,a6,164e <atoi+0x3c>
    1672:	00030563          	beqz	t1,167c <atoi+0x6a>
    1676:	8082                	ret
    1678:	0505                	add	a0,a0,1
    167a:	bf79                	j	1618 <atoi+0x6>
    167c:	40f5853b          	subw	a0,a1,a5
    1680:	8082                	ret
    1682:	00154603          	lbu	a2,1(a0)
    1686:	47a5                	li	a5,9
    1688:	00150713          	add	a4,a0,1
    168c:	fd06069b          	addw	a3,a2,-48
    1690:	4301                	li	t1,0
    1692:	2601                	sext.w	a2,a2
    1694:	fad7fbe3          	bgeu	a5,a3,164a <atoi+0x38>
    1698:	4501                	li	a0,0
    169a:	8082                	ret
    169c:	00154603          	lbu	a2,1(a0)
    16a0:	47a5                	li	a5,9
    16a2:	00150713          	add	a4,a0,1
    16a6:	fd06069b          	addw	a3,a2,-48
    16aa:	2601                	sext.w	a2,a2
    16ac:	fed7e6e3          	bltu	a5,a3,1698 <atoi+0x86>
    16b0:	4305                	li	t1,1
    16b2:	bf61                	j	164a <atoi+0x38>

00000000000016b4 <memset>:
    16b4:	18060163          	beqz	a2,1836 <memset+0x182>
    16b8:	40a006b3          	neg	a3,a0
    16bc:	0076f793          	and	a5,a3,7
    16c0:	00778813          	add	a6,a5,7
    16c4:	48ad                	li	a7,11
    16c6:	0ff5f713          	zext.b	a4,a1
    16ca:	fff60593          	add	a1,a2,-1
    16ce:	17186563          	bltu	a6,a7,1838 <memset+0x184>
    16d2:	1705ed63          	bltu	a1,a6,184c <memset+0x198>
    16d6:	16078363          	beqz	a5,183c <memset+0x188>
    16da:	00e50023          	sb	a4,0(a0)
    16de:	0066f593          	and	a1,a3,6
    16e2:	16058063          	beqz	a1,1842 <memset+0x18e>
    16e6:	00e500a3          	sb	a4,1(a0)
    16ea:	4589                	li	a1,2
    16ec:	16f5f363          	bgeu	a1,a5,1852 <memset+0x19e>
    16f0:	00e50123          	sb	a4,2(a0)
    16f4:	8a91                	and	a3,a3,4
    16f6:	00350593          	add	a1,a0,3
    16fa:	4e0d                	li	t3,3
    16fc:	ce9d                	beqz	a3,173a <memset+0x86>
    16fe:	00e501a3          	sb	a4,3(a0)
    1702:	4691                	li	a3,4
    1704:	00450593          	add	a1,a0,4
    1708:	4e11                	li	t3,4
    170a:	02f6f863          	bgeu	a3,a5,173a <memset+0x86>
    170e:	00e50223          	sb	a4,4(a0)
    1712:	4695                	li	a3,5
    1714:	00550593          	add	a1,a0,5
    1718:	4e15                	li	t3,5
    171a:	02d78063          	beq	a5,a3,173a <memset+0x86>
    171e:	fff50693          	add	a3,a0,-1
    1722:	00e502a3          	sb	a4,5(a0)
    1726:	8a9d                	and	a3,a3,7
    1728:	00650593          	add	a1,a0,6
    172c:	4e19                	li	t3,6
    172e:	e691                	bnez	a3,173a <memset+0x86>
    1730:	00750593          	add	a1,a0,7
    1734:	00e50323          	sb	a4,6(a0)
    1738:	4e1d                	li	t3,7
    173a:	00871693          	sll	a3,a4,0x8
    173e:	01071813          	sll	a6,a4,0x10
    1742:	8ed9                	or	a3,a3,a4
    1744:	01871893          	sll	a7,a4,0x18
    1748:	0106e6b3          	or	a3,a3,a6
    174c:	0116e6b3          	or	a3,a3,a7
    1750:	02071813          	sll	a6,a4,0x20
    1754:	02871313          	sll	t1,a4,0x28
    1758:	0106e6b3          	or	a3,a3,a6
    175c:	40f608b3          	sub	a7,a2,a5
    1760:	03071813          	sll	a6,a4,0x30
    1764:	0066e6b3          	or	a3,a3,t1
    1768:	0106e6b3          	or	a3,a3,a6
    176c:	03871313          	sll	t1,a4,0x38
    1770:	97aa                	add	a5,a5,a0
    1772:	ff88f813          	and	a6,a7,-8
    1776:	0066e6b3          	or	a3,a3,t1
    177a:	983e                	add	a6,a6,a5
    177c:	e394                	sd	a3,0(a5)
    177e:	07a1                	add	a5,a5,8
    1780:	ff079ee3          	bne	a5,a6,177c <memset+0xc8>
    1784:	ff88f793          	and	a5,a7,-8
    1788:	0078f893          	and	a7,a7,7
    178c:	00f586b3          	add	a3,a1,a5
    1790:	01c787bb          	addw	a5,a5,t3
    1794:	0a088b63          	beqz	a7,184a <memset+0x196>
    1798:	00e68023          	sb	a4,0(a3)
    179c:	0017859b          	addw	a1,a5,1
    17a0:	08c5fb63          	bgeu	a1,a2,1836 <memset+0x182>
    17a4:	00e680a3          	sb	a4,1(a3)
    17a8:	0027859b          	addw	a1,a5,2
    17ac:	08c5f563          	bgeu	a1,a2,1836 <memset+0x182>
    17b0:	00e68123          	sb	a4,2(a3)
    17b4:	0037859b          	addw	a1,a5,3
    17b8:	06c5ff63          	bgeu	a1,a2,1836 <memset+0x182>
    17bc:	00e681a3          	sb	a4,3(a3)
    17c0:	0047859b          	addw	a1,a5,4
    17c4:	06c5f963          	bgeu	a1,a2,1836 <memset+0x182>
    17c8:	00e68223          	sb	a4,4(a3)
    17cc:	0057859b          	addw	a1,a5,5
    17d0:	06c5f363          	bgeu	a1,a2,1836 <memset+0x182>
    17d4:	00e682a3          	sb	a4,5(a3)
    17d8:	0067859b          	addw	a1,a5,6
    17dc:	04c5fd63          	bgeu	a1,a2,1836 <memset+0x182>
    17e0:	00e68323          	sb	a4,6(a3)
    17e4:	0077859b          	addw	a1,a5,7
    17e8:	04c5f763          	bgeu	a1,a2,1836 <memset+0x182>
    17ec:	00e683a3          	sb	a4,7(a3)
    17f0:	0087859b          	addw	a1,a5,8
    17f4:	04c5f163          	bgeu	a1,a2,1836 <memset+0x182>
    17f8:	00e68423          	sb	a4,8(a3)
    17fc:	0097859b          	addw	a1,a5,9
    1800:	02c5fb63          	bgeu	a1,a2,1836 <memset+0x182>
    1804:	00e684a3          	sb	a4,9(a3)
    1808:	00a7859b          	addw	a1,a5,10
    180c:	02c5f563          	bgeu	a1,a2,1836 <memset+0x182>
    1810:	00e68523          	sb	a4,10(a3)
    1814:	00b7859b          	addw	a1,a5,11
    1818:	00c5ff63          	bgeu	a1,a2,1836 <memset+0x182>
    181c:	00e685a3          	sb	a4,11(a3)
    1820:	00c7859b          	addw	a1,a5,12
    1824:	00c5f963          	bgeu	a1,a2,1836 <memset+0x182>
    1828:	00e68623          	sb	a4,12(a3)
    182c:	27b5                	addw	a5,a5,13
    182e:	00c7f463          	bgeu	a5,a2,1836 <memset+0x182>
    1832:	00e686a3          	sb	a4,13(a3)
    1836:	8082                	ret
    1838:	482d                	li	a6,11
    183a:	bd61                	j	16d2 <memset+0x1e>
    183c:	85aa                	mv	a1,a0
    183e:	4e01                	li	t3,0
    1840:	bded                	j	173a <memset+0x86>
    1842:	00150593          	add	a1,a0,1
    1846:	4e05                	li	t3,1
    1848:	bdcd                	j	173a <memset+0x86>
    184a:	8082                	ret
    184c:	86aa                	mv	a3,a0
    184e:	4781                	li	a5,0
    1850:	b7a1                	j	1798 <memset+0xe4>
    1852:	00250593          	add	a1,a0,2
    1856:	4e09                	li	t3,2
    1858:	b5cd                	j	173a <memset+0x86>

000000000000185a <strcmp>:
    185a:	00054783          	lbu	a5,0(a0)
    185e:	0005c703          	lbu	a4,0(a1)
    1862:	00e79863          	bne	a5,a4,1872 <strcmp+0x18>
    1866:	0505                	add	a0,a0,1
    1868:	0585                	add	a1,a1,1
    186a:	fbe5                	bnez	a5,185a <strcmp>
    186c:	4501                	li	a0,0
    186e:	9d19                	subw	a0,a0,a4
    1870:	8082                	ret
    1872:	0007851b          	sext.w	a0,a5
    1876:	bfe5                	j	186e <strcmp+0x14>

0000000000001878 <strncmp>:
    1878:	ca15                	beqz	a2,18ac <strncmp+0x34>
    187a:	00054783          	lbu	a5,0(a0)
    187e:	167d                	add	a2,a2,-1
    1880:	00c506b3          	add	a3,a0,a2
    1884:	eb99                	bnez	a5,189a <strncmp+0x22>
    1886:	a815                	j	18ba <strncmp+0x42>
    1888:	00a68e63          	beq	a3,a0,18a4 <strncmp+0x2c>
    188c:	0505                	add	a0,a0,1
    188e:	00f71b63          	bne	a4,a5,18a4 <strncmp+0x2c>
    1892:	00054783          	lbu	a5,0(a0)
    1896:	cf89                	beqz	a5,18b0 <strncmp+0x38>
    1898:	85b2                	mv	a1,a2
    189a:	0005c703          	lbu	a4,0(a1)
    189e:	00158613          	add	a2,a1,1
    18a2:	f37d                	bnez	a4,1888 <strncmp+0x10>
    18a4:	0007851b          	sext.w	a0,a5
    18a8:	9d19                	subw	a0,a0,a4
    18aa:	8082                	ret
    18ac:	4501                	li	a0,0
    18ae:	8082                	ret
    18b0:	0015c703          	lbu	a4,1(a1)
    18b4:	4501                	li	a0,0
    18b6:	9d19                	subw	a0,a0,a4
    18b8:	8082                	ret
    18ba:	0005c703          	lbu	a4,0(a1)
    18be:	4501                	li	a0,0
    18c0:	b7e5                	j	18a8 <strncmp+0x30>

00000000000018c2 <strlen>:
    18c2:	00757793          	and	a5,a0,7
    18c6:	cf89                	beqz	a5,18e0 <strlen+0x1e>
    18c8:	87aa                	mv	a5,a0
    18ca:	a029                	j	18d4 <strlen+0x12>
    18cc:	0785                	add	a5,a5,1
    18ce:	0077f713          	and	a4,a5,7
    18d2:	cb01                	beqz	a4,18e2 <strlen+0x20>
    18d4:	0007c703          	lbu	a4,0(a5)
    18d8:	fb75                	bnez	a4,18cc <strlen+0xa>
    18da:	40a78533          	sub	a0,a5,a0
    18de:	8082                	ret
    18e0:	87aa                	mv	a5,a0
    18e2:	6394                	ld	a3,0(a5)
    18e4:	00000597          	auipc	a1,0x0
    18e8:	6dc5b583          	ld	a1,1756(a1) # 1fc0 <__clone+0xea>
    18ec:	00000617          	auipc	a2,0x0
    18f0:	6dc63603          	ld	a2,1756(a2) # 1fc8 <__clone+0xf2>
    18f4:	a019                	j	18fa <strlen+0x38>
    18f6:	6794                	ld	a3,8(a5)
    18f8:	07a1                	add	a5,a5,8
    18fa:	00b68733          	add	a4,a3,a1
    18fe:	fff6c693          	not	a3,a3
    1902:	8f75                	and	a4,a4,a3
    1904:	8f71                	and	a4,a4,a2
    1906:	db65                	beqz	a4,18f6 <strlen+0x34>
    1908:	0007c703          	lbu	a4,0(a5)
    190c:	d779                	beqz	a4,18da <strlen+0x18>
    190e:	0017c703          	lbu	a4,1(a5)
    1912:	0785                	add	a5,a5,1
    1914:	d379                	beqz	a4,18da <strlen+0x18>
    1916:	0017c703          	lbu	a4,1(a5)
    191a:	0785                	add	a5,a5,1
    191c:	fb6d                	bnez	a4,190e <strlen+0x4c>
    191e:	bf75                	j	18da <strlen+0x18>

0000000000001920 <memchr>:
    1920:	00757713          	and	a4,a0,7
    1924:	87aa                	mv	a5,a0
    1926:	0ff5f593          	zext.b	a1,a1
    192a:	cb19                	beqz	a4,1940 <memchr+0x20>
    192c:	ce25                	beqz	a2,19a4 <memchr+0x84>
    192e:	0007c703          	lbu	a4,0(a5)
    1932:	00b70763          	beq	a4,a1,1940 <memchr+0x20>
    1936:	0785                	add	a5,a5,1
    1938:	0077f713          	and	a4,a5,7
    193c:	167d                	add	a2,a2,-1
    193e:	f77d                	bnez	a4,192c <memchr+0xc>
    1940:	4501                	li	a0,0
    1942:	c235                	beqz	a2,19a6 <memchr+0x86>
    1944:	0007c703          	lbu	a4,0(a5)
    1948:	06b70063          	beq	a4,a1,19a8 <memchr+0x88>
    194c:	00000517          	auipc	a0,0x0
    1950:	68453503          	ld	a0,1668(a0) # 1fd0 <__clone+0xfa>
    1954:	471d                	li	a4,7
    1956:	02a58533          	mul	a0,a1,a0
    195a:	04c77763          	bgeu	a4,a2,19a8 <memchr+0x88>
    195e:	00000897          	auipc	a7,0x0
    1962:	6628b883          	ld	a7,1634(a7) # 1fc0 <__clone+0xea>
    1966:	00000817          	auipc	a6,0x0
    196a:	66283803          	ld	a6,1634(a6) # 1fc8 <__clone+0xf2>
    196e:	431d                	li	t1,7
    1970:	a029                	j	197a <memchr+0x5a>
    1972:	1661                	add	a2,a2,-8
    1974:	07a1                	add	a5,a5,8
    1976:	00c37c63          	bgeu	t1,a2,198e <memchr+0x6e>
    197a:	6398                	ld	a4,0(a5)
    197c:	8f29                	xor	a4,a4,a0
    197e:	011706b3          	add	a3,a4,a7
    1982:	fff74713          	not	a4,a4
    1986:	8f75                	and	a4,a4,a3
    1988:	01077733          	and	a4,a4,a6
    198c:	d37d                	beqz	a4,1972 <memchr+0x52>
    198e:	853e                	mv	a0,a5
    1990:	e601                	bnez	a2,1998 <memchr+0x78>
    1992:	a809                	j	19a4 <memchr+0x84>
    1994:	0505                	add	a0,a0,1
    1996:	c619                	beqz	a2,19a4 <memchr+0x84>
    1998:	00054783          	lbu	a5,0(a0)
    199c:	167d                	add	a2,a2,-1
    199e:	feb79be3          	bne	a5,a1,1994 <memchr+0x74>
    19a2:	8082                	ret
    19a4:	4501                	li	a0,0
    19a6:	8082                	ret
    19a8:	853e                	mv	a0,a5
    19aa:	b7fd                	j	1998 <memchr+0x78>

00000000000019ac <strnlen>:
    19ac:	1101                	add	sp,sp,-32
    19ae:	e822                	sd	s0,16(sp)
    19b0:	862e                	mv	a2,a1
    19b2:	842e                	mv	s0,a1
    19b4:	4581                	li	a1,0
    19b6:	e426                	sd	s1,8(sp)
    19b8:	ec06                	sd	ra,24(sp)
    19ba:	84aa                	mv	s1,a0
    19bc:	f65ff0ef          	jal	1920 <memchr>
    19c0:	c519                	beqz	a0,19ce <strnlen+0x22>
    19c2:	60e2                	ld	ra,24(sp)
    19c4:	6442                	ld	s0,16(sp)
    19c6:	8d05                	sub	a0,a0,s1
    19c8:	64a2                	ld	s1,8(sp)
    19ca:	6105                	add	sp,sp,32
    19cc:	8082                	ret
    19ce:	60e2                	ld	ra,24(sp)
    19d0:	8522                	mv	a0,s0
    19d2:	6442                	ld	s0,16(sp)
    19d4:	64a2                	ld	s1,8(sp)
    19d6:	6105                	add	sp,sp,32
    19d8:	8082                	ret

00000000000019da <strcpy>:
    19da:	00a5c7b3          	xor	a5,a1,a0
    19de:	8b9d                	and	a5,a5,7
    19e0:	eb95                	bnez	a5,1a14 <strcpy+0x3a>
    19e2:	0075f793          	and	a5,a1,7
    19e6:	e7b1                	bnez	a5,1a32 <strcpy+0x58>
    19e8:	6198                	ld	a4,0(a1)
    19ea:	00000617          	auipc	a2,0x0
    19ee:	5d663603          	ld	a2,1494(a2) # 1fc0 <__clone+0xea>
    19f2:	00000817          	auipc	a6,0x0
    19f6:	5d683803          	ld	a6,1494(a6) # 1fc8 <__clone+0xf2>
    19fa:	a029                	j	1a04 <strcpy+0x2a>
    19fc:	05a1                	add	a1,a1,8
    19fe:	e118                	sd	a4,0(a0)
    1a00:	6198                	ld	a4,0(a1)
    1a02:	0521                	add	a0,a0,8
    1a04:	00c707b3          	add	a5,a4,a2
    1a08:	fff74693          	not	a3,a4
    1a0c:	8ff5                	and	a5,a5,a3
    1a0e:	0107f7b3          	and	a5,a5,a6
    1a12:	d7ed                	beqz	a5,19fc <strcpy+0x22>
    1a14:	0005c783          	lbu	a5,0(a1)
    1a18:	00f50023          	sb	a5,0(a0)
    1a1c:	c785                	beqz	a5,1a44 <strcpy+0x6a>
    1a1e:	0015c783          	lbu	a5,1(a1)
    1a22:	0505                	add	a0,a0,1
    1a24:	0585                	add	a1,a1,1
    1a26:	00f50023          	sb	a5,0(a0)
    1a2a:	fbf5                	bnez	a5,1a1e <strcpy+0x44>
    1a2c:	8082                	ret
    1a2e:	0505                	add	a0,a0,1
    1a30:	df45                	beqz	a4,19e8 <strcpy+0xe>
    1a32:	0005c783          	lbu	a5,0(a1)
    1a36:	0585                	add	a1,a1,1
    1a38:	0075f713          	and	a4,a1,7
    1a3c:	00f50023          	sb	a5,0(a0)
    1a40:	f7fd                	bnez	a5,1a2e <strcpy+0x54>
    1a42:	8082                	ret
    1a44:	8082                	ret

0000000000001a46 <strncpy>:
    1a46:	00a5c7b3          	xor	a5,a1,a0
    1a4a:	8b9d                	and	a5,a5,7
    1a4c:	e3b5                	bnez	a5,1ab0 <strncpy+0x6a>
    1a4e:	0075f793          	and	a5,a1,7
    1a52:	cf99                	beqz	a5,1a70 <strncpy+0x2a>
    1a54:	ea09                	bnez	a2,1a66 <strncpy+0x20>
    1a56:	a421                	j	1c5e <strncpy+0x218>
    1a58:	0585                	add	a1,a1,1
    1a5a:	0075f793          	and	a5,a1,7
    1a5e:	167d                	add	a2,a2,-1
    1a60:	0505                	add	a0,a0,1
    1a62:	c799                	beqz	a5,1a70 <strncpy+0x2a>
    1a64:	c225                	beqz	a2,1ac4 <strncpy+0x7e>
    1a66:	0005c783          	lbu	a5,0(a1)
    1a6a:	00f50023          	sb	a5,0(a0)
    1a6e:	f7ed                	bnez	a5,1a58 <strncpy+0x12>
    1a70:	ca31                	beqz	a2,1ac4 <strncpy+0x7e>
    1a72:	0005c783          	lbu	a5,0(a1)
    1a76:	cba1                	beqz	a5,1ac6 <strncpy+0x80>
    1a78:	479d                	li	a5,7
    1a7a:	02c7fc63          	bgeu	a5,a2,1ab2 <strncpy+0x6c>
    1a7e:	00000897          	auipc	a7,0x0
    1a82:	5428b883          	ld	a7,1346(a7) # 1fc0 <__clone+0xea>
    1a86:	00000817          	auipc	a6,0x0
    1a8a:	54283803          	ld	a6,1346(a6) # 1fc8 <__clone+0xf2>
    1a8e:	431d                	li	t1,7
    1a90:	a039                	j	1a9e <strncpy+0x58>
    1a92:	e118                	sd	a4,0(a0)
    1a94:	1661                	add	a2,a2,-8
    1a96:	05a1                	add	a1,a1,8
    1a98:	0521                	add	a0,a0,8
    1a9a:	00c37b63          	bgeu	t1,a2,1ab0 <strncpy+0x6a>
    1a9e:	6198                	ld	a4,0(a1)
    1aa0:	011707b3          	add	a5,a4,a7
    1aa4:	fff74693          	not	a3,a4
    1aa8:	8ff5                	and	a5,a5,a3
    1aaa:	0107f7b3          	and	a5,a5,a6
    1aae:	d3f5                	beqz	a5,1a92 <strncpy+0x4c>
    1ab0:	ca11                	beqz	a2,1ac4 <strncpy+0x7e>
    1ab2:	0005c783          	lbu	a5,0(a1)
    1ab6:	0585                	add	a1,a1,1
    1ab8:	00f50023          	sb	a5,0(a0)
    1abc:	c789                	beqz	a5,1ac6 <strncpy+0x80>
    1abe:	167d                	add	a2,a2,-1
    1ac0:	0505                	add	a0,a0,1
    1ac2:	fa65                	bnez	a2,1ab2 <strncpy+0x6c>
    1ac4:	8082                	ret
    1ac6:	4805                	li	a6,1
    1ac8:	14061b63          	bnez	a2,1c1e <strncpy+0x1d8>
    1acc:	40a00733          	neg	a4,a0
    1ad0:	00777793          	and	a5,a4,7
    1ad4:	4581                	li	a1,0
    1ad6:	12061c63          	bnez	a2,1c0e <strncpy+0x1c8>
    1ada:	00778693          	add	a3,a5,7
    1ade:	48ad                	li	a7,11
    1ae0:	1316e563          	bltu	a3,a7,1c0a <strncpy+0x1c4>
    1ae4:	16d5e263          	bltu	a1,a3,1c48 <strncpy+0x202>
    1ae8:	14078c63          	beqz	a5,1c40 <strncpy+0x1fa>
    1aec:	00050023          	sb	zero,0(a0)
    1af0:	00677693          	and	a3,a4,6
    1af4:	14068263          	beqz	a3,1c38 <strncpy+0x1f2>
    1af8:	000500a3          	sb	zero,1(a0)
    1afc:	4689                	li	a3,2
    1afe:	14f6f863          	bgeu	a3,a5,1c4e <strncpy+0x208>
    1b02:	00050123          	sb	zero,2(a0)
    1b06:	8b11                	and	a4,a4,4
    1b08:	12070463          	beqz	a4,1c30 <strncpy+0x1ea>
    1b0c:	000501a3          	sb	zero,3(a0)
    1b10:	4711                	li	a4,4
    1b12:	00450693          	add	a3,a0,4
    1b16:	02f77563          	bgeu	a4,a5,1b40 <strncpy+0xfa>
    1b1a:	00050223          	sb	zero,4(a0)
    1b1e:	4715                	li	a4,5
    1b20:	00550693          	add	a3,a0,5
    1b24:	00e78e63          	beq	a5,a4,1b40 <strncpy+0xfa>
    1b28:	fff50713          	add	a4,a0,-1
    1b2c:	000502a3          	sb	zero,5(a0)
    1b30:	8b1d                	and	a4,a4,7
    1b32:	12071263          	bnez	a4,1c56 <strncpy+0x210>
    1b36:	00750693          	add	a3,a0,7
    1b3a:	00050323          	sb	zero,6(a0)
    1b3e:	471d                	li	a4,7
    1b40:	40f80833          	sub	a6,a6,a5
    1b44:	ff887593          	and	a1,a6,-8
    1b48:	97aa                	add	a5,a5,a0
    1b4a:	95be                	add	a1,a1,a5
    1b4c:	0007b023          	sd	zero,0(a5)
    1b50:	07a1                	add	a5,a5,8
    1b52:	feb79de3          	bne	a5,a1,1b4c <strncpy+0x106>
    1b56:	ff887593          	and	a1,a6,-8
    1b5a:	00787813          	and	a6,a6,7
    1b5e:	00e587bb          	addw	a5,a1,a4
    1b62:	00b68733          	add	a4,a3,a1
    1b66:	0e080063          	beqz	a6,1c46 <strncpy+0x200>
    1b6a:	00070023          	sb	zero,0(a4)
    1b6e:	0017869b          	addw	a3,a5,1
    1b72:	f4c6f9e3          	bgeu	a3,a2,1ac4 <strncpy+0x7e>
    1b76:	000700a3          	sb	zero,1(a4)
    1b7a:	0027869b          	addw	a3,a5,2
    1b7e:	f4c6f3e3          	bgeu	a3,a2,1ac4 <strncpy+0x7e>
    1b82:	00070123          	sb	zero,2(a4)
    1b86:	0037869b          	addw	a3,a5,3
    1b8a:	f2c6fde3          	bgeu	a3,a2,1ac4 <strncpy+0x7e>
    1b8e:	000701a3          	sb	zero,3(a4)
    1b92:	0047869b          	addw	a3,a5,4
    1b96:	f2c6f7e3          	bgeu	a3,a2,1ac4 <strncpy+0x7e>
    1b9a:	00070223          	sb	zero,4(a4)
    1b9e:	0057869b          	addw	a3,a5,5
    1ba2:	f2c6f1e3          	bgeu	a3,a2,1ac4 <strncpy+0x7e>
    1ba6:	000702a3          	sb	zero,5(a4)
    1baa:	0067869b          	addw	a3,a5,6
    1bae:	f0c6fbe3          	bgeu	a3,a2,1ac4 <strncpy+0x7e>
    1bb2:	00070323          	sb	zero,6(a4)
    1bb6:	0077869b          	addw	a3,a5,7
    1bba:	f0c6f5e3          	bgeu	a3,a2,1ac4 <strncpy+0x7e>
    1bbe:	000703a3          	sb	zero,7(a4)
    1bc2:	0087869b          	addw	a3,a5,8
    1bc6:	eec6ffe3          	bgeu	a3,a2,1ac4 <strncpy+0x7e>
    1bca:	00070423          	sb	zero,8(a4)
    1bce:	0097869b          	addw	a3,a5,9
    1bd2:	eec6f9e3          	bgeu	a3,a2,1ac4 <strncpy+0x7e>
    1bd6:	000704a3          	sb	zero,9(a4)
    1bda:	00a7869b          	addw	a3,a5,10
    1bde:	eec6f3e3          	bgeu	a3,a2,1ac4 <strncpy+0x7e>
    1be2:	00070523          	sb	zero,10(a4)
    1be6:	00b7869b          	addw	a3,a5,11
    1bea:	ecc6fde3          	bgeu	a3,a2,1ac4 <strncpy+0x7e>
    1bee:	000705a3          	sb	zero,11(a4)
    1bf2:	00c7869b          	addw	a3,a5,12
    1bf6:	ecc6f7e3          	bgeu	a3,a2,1ac4 <strncpy+0x7e>
    1bfa:	00070623          	sb	zero,12(a4)
    1bfe:	27b5                	addw	a5,a5,13
    1c00:	ecc7f2e3          	bgeu	a5,a2,1ac4 <strncpy+0x7e>
    1c04:	000706a3          	sb	zero,13(a4)
    1c08:	8082                	ret
    1c0a:	46ad                	li	a3,11
    1c0c:	bde1                	j	1ae4 <strncpy+0x9e>
    1c0e:	00778693          	add	a3,a5,7
    1c12:	48ad                	li	a7,11
    1c14:	fff60593          	add	a1,a2,-1
    1c18:	ed16f6e3          	bgeu	a3,a7,1ae4 <strncpy+0x9e>
    1c1c:	b7fd                	j	1c0a <strncpy+0x1c4>
    1c1e:	40a00733          	neg	a4,a0
    1c22:	8832                	mv	a6,a2
    1c24:	00777793          	and	a5,a4,7
    1c28:	4581                	li	a1,0
    1c2a:	ea0608e3          	beqz	a2,1ada <strncpy+0x94>
    1c2e:	b7c5                	j	1c0e <strncpy+0x1c8>
    1c30:	00350693          	add	a3,a0,3
    1c34:	470d                	li	a4,3
    1c36:	b729                	j	1b40 <strncpy+0xfa>
    1c38:	00150693          	add	a3,a0,1
    1c3c:	4705                	li	a4,1
    1c3e:	b709                	j	1b40 <strncpy+0xfa>
    1c40:	86aa                	mv	a3,a0
    1c42:	4701                	li	a4,0
    1c44:	bdf5                	j	1b40 <strncpy+0xfa>
    1c46:	8082                	ret
    1c48:	872a                	mv	a4,a0
    1c4a:	4781                	li	a5,0
    1c4c:	bf39                	j	1b6a <strncpy+0x124>
    1c4e:	00250693          	add	a3,a0,2
    1c52:	4709                	li	a4,2
    1c54:	b5f5                	j	1b40 <strncpy+0xfa>
    1c56:	00650693          	add	a3,a0,6
    1c5a:	4719                	li	a4,6
    1c5c:	b5d5                	j	1b40 <strncpy+0xfa>
    1c5e:	8082                	ret

0000000000001c60 <open>:
    1c60:	87aa                	mv	a5,a0
    1c62:	862e                	mv	a2,a1
    1c64:	03800893          	li	a7,56
    1c68:	f9c00513          	li	a0,-100
    1c6c:	85be                	mv	a1,a5
    1c6e:	4689                	li	a3,2
    1c70:	00000073          	ecall
    1c74:	2501                	sext.w	a0,a0
    1c76:	8082                	ret

0000000000001c78 <openat>:
    1c78:	03800893          	li	a7,56
    1c7c:	18000693          	li	a3,384
    1c80:	00000073          	ecall
    1c84:	2501                	sext.w	a0,a0
    1c86:	8082                	ret

0000000000001c88 <close>:
    1c88:	03900893          	li	a7,57
    1c8c:	00000073          	ecall
    1c90:	2501                	sext.w	a0,a0
    1c92:	8082                	ret

0000000000001c94 <read>:
    1c94:	03f00893          	li	a7,63
    1c98:	00000073          	ecall
    1c9c:	8082                	ret

0000000000001c9e <write>:
    1c9e:	04000893          	li	a7,64
    1ca2:	00000073          	ecall
    1ca6:	8082                	ret

0000000000001ca8 <getpid>:
    1ca8:	0ac00893          	li	a7,172
    1cac:	00000073          	ecall
    1cb0:	2501                	sext.w	a0,a0
    1cb2:	8082                	ret

0000000000001cb4 <getppid>:
    1cb4:	0ad00893          	li	a7,173
    1cb8:	00000073          	ecall
    1cbc:	2501                	sext.w	a0,a0
    1cbe:	8082                	ret

0000000000001cc0 <sched_yield>:
    1cc0:	07c00893          	li	a7,124
    1cc4:	00000073          	ecall
    1cc8:	2501                	sext.w	a0,a0
    1cca:	8082                	ret

0000000000001ccc <fork>:
    1ccc:	0dc00893          	li	a7,220
    1cd0:	4545                	li	a0,17
    1cd2:	4581                	li	a1,0
    1cd4:	00000073          	ecall
    1cd8:	2501                	sext.w	a0,a0
    1cda:	8082                	ret

0000000000001cdc <clone>:
    1cdc:	85b2                	mv	a1,a2
    1cde:	863a                	mv	a2,a4
    1ce0:	c191                	beqz	a1,1ce4 <clone+0x8>
    1ce2:	95b6                	add	a1,a1,a3
    1ce4:	4781                	li	a5,0
    1ce6:	4701                	li	a4,0
    1ce8:	4681                	li	a3,0
    1cea:	2601                	sext.w	a2,a2
    1cec:	a2ed                	j	1ed6 <__clone>

0000000000001cee <exit>:
    1cee:	05d00893          	li	a7,93
    1cf2:	00000073          	ecall
    1cf6:	8082                	ret

0000000000001cf8 <waitpid>:
    1cf8:	10400893          	li	a7,260
    1cfc:	4681                	li	a3,0
    1cfe:	00000073          	ecall
    1d02:	2501                	sext.w	a0,a0
    1d04:	8082                	ret

0000000000001d06 <exec>:
    1d06:	0dd00893          	li	a7,221
    1d0a:	00000073          	ecall
    1d0e:	2501                	sext.w	a0,a0
    1d10:	8082                	ret

0000000000001d12 <execve>:
    1d12:	0dd00893          	li	a7,221
    1d16:	00000073          	ecall
    1d1a:	2501                	sext.w	a0,a0
    1d1c:	8082                	ret

0000000000001d1e <times>:
    1d1e:	09900893          	li	a7,153
    1d22:	00000073          	ecall
    1d26:	2501                	sext.w	a0,a0
    1d28:	8082                	ret

0000000000001d2a <get_time>:
    1d2a:	1141                	add	sp,sp,-16
    1d2c:	0a900893          	li	a7,169
    1d30:	850a                	mv	a0,sp
    1d32:	4581                	li	a1,0
    1d34:	00000073          	ecall
    1d38:	2501                	sext.w	a0,a0
    1d3a:	ed09                	bnez	a0,1d54 <get_time+0x2a>
    1d3c:	67a2                	ld	a5,8(sp)
    1d3e:	3e800713          	li	a4,1000
    1d42:	00015503          	lhu	a0,0(sp)
    1d46:	02e7d7b3          	divu	a5,a5,a4
    1d4a:	02e50533          	mul	a0,a0,a4
    1d4e:	953e                	add	a0,a0,a5
    1d50:	0141                	add	sp,sp,16
    1d52:	8082                	ret
    1d54:	557d                	li	a0,-1
    1d56:	bfed                	j	1d50 <get_time+0x26>

0000000000001d58 <sys_get_time>:
    1d58:	0a900893          	li	a7,169
    1d5c:	00000073          	ecall
    1d60:	2501                	sext.w	a0,a0
    1d62:	8082                	ret

0000000000001d64 <time>:
    1d64:	42600893          	li	a7,1062
    1d68:	00000073          	ecall
    1d6c:	2501                	sext.w	a0,a0
    1d6e:	8082                	ret

0000000000001d70 <sleep>:
    1d70:	1141                	add	sp,sp,-16
    1d72:	e02a                	sd	a0,0(sp)
    1d74:	850a                	mv	a0,sp
    1d76:	e402                	sd	zero,8(sp)
    1d78:	06500893          	li	a7,101
    1d7c:	85aa                	mv	a1,a0
    1d7e:	00000073          	ecall
    1d82:	e501                	bnez	a0,1d8a <sleep+0x1a>
    1d84:	4501                	li	a0,0
    1d86:	0141                	add	sp,sp,16
    1d88:	8082                	ret
    1d8a:	4502                	lw	a0,0(sp)
    1d8c:	0141                	add	sp,sp,16
    1d8e:	8082                	ret

0000000000001d90 <set_priority>:
    1d90:	08c00893          	li	a7,140
    1d94:	00000073          	ecall
    1d98:	2501                	sext.w	a0,a0
    1d9a:	8082                	ret

0000000000001d9c <mmap>:
    1d9c:	0de00893          	li	a7,222
    1da0:	00000073          	ecall
    1da4:	8082                	ret

0000000000001da6 <munmap>:
    1da6:	0d700893          	li	a7,215
    1daa:	00000073          	ecall
    1dae:	2501                	sext.w	a0,a0
    1db0:	8082                	ret

0000000000001db2 <wait>:
    1db2:	85aa                	mv	a1,a0
    1db4:	10400893          	li	a7,260
    1db8:	557d                	li	a0,-1
    1dba:	4601                	li	a2,0
    1dbc:	4681                	li	a3,0
    1dbe:	00000073          	ecall
    1dc2:	2501                	sext.w	a0,a0
    1dc4:	8082                	ret

0000000000001dc6 <spawn>:
    1dc6:	19000893          	li	a7,400
    1dca:	00000073          	ecall
    1dce:	2501                	sext.w	a0,a0
    1dd0:	8082                	ret

0000000000001dd2 <mailread>:
    1dd2:	19100893          	li	a7,401
    1dd6:	00000073          	ecall
    1dda:	2501                	sext.w	a0,a0
    1ddc:	8082                	ret

0000000000001dde <mailwrite>:
    1dde:	19200893          	li	a7,402
    1de2:	00000073          	ecall
    1de6:	2501                	sext.w	a0,a0
    1de8:	8082                	ret

0000000000001dea <fstat>:
    1dea:	05000893          	li	a7,80
    1dee:	00000073          	ecall
    1df2:	2501                	sext.w	a0,a0
    1df4:	8082                	ret

0000000000001df6 <sys_linkat>:
    1df6:	1702                	sll	a4,a4,0x20
    1df8:	02500893          	li	a7,37
    1dfc:	9301                	srl	a4,a4,0x20
    1dfe:	00000073          	ecall
    1e02:	2501                	sext.w	a0,a0
    1e04:	8082                	ret

0000000000001e06 <sys_unlinkat>:
    1e06:	1602                	sll	a2,a2,0x20
    1e08:	02300893          	li	a7,35
    1e0c:	9201                	srl	a2,a2,0x20
    1e0e:	00000073          	ecall
    1e12:	2501                	sext.w	a0,a0
    1e14:	8082                	ret

0000000000001e16 <link>:
    1e16:	87aa                	mv	a5,a0
    1e18:	86ae                	mv	a3,a1
    1e1a:	02500893          	li	a7,37
    1e1e:	f9c00513          	li	a0,-100
    1e22:	85be                	mv	a1,a5
    1e24:	f9c00613          	li	a2,-100
    1e28:	4701                	li	a4,0
    1e2a:	00000073          	ecall
    1e2e:	2501                	sext.w	a0,a0
    1e30:	8082                	ret

0000000000001e32 <unlink>:
    1e32:	85aa                	mv	a1,a0
    1e34:	02300893          	li	a7,35
    1e38:	f9c00513          	li	a0,-100
    1e3c:	4601                	li	a2,0
    1e3e:	00000073          	ecall
    1e42:	2501                	sext.w	a0,a0
    1e44:	8082                	ret

0000000000001e46 <uname>:
    1e46:	0a000893          	li	a7,160
    1e4a:	00000073          	ecall
    1e4e:	2501                	sext.w	a0,a0
    1e50:	8082                	ret

0000000000001e52 <brk>:
    1e52:	0d600893          	li	a7,214
    1e56:	00000073          	ecall
    1e5a:	2501                	sext.w	a0,a0
    1e5c:	8082                	ret

0000000000001e5e <getcwd>:
    1e5e:	48c5                	li	a7,17
    1e60:	00000073          	ecall
    1e64:	8082                	ret

0000000000001e66 <chdir>:
    1e66:	03100893          	li	a7,49
    1e6a:	00000073          	ecall
    1e6e:	2501                	sext.w	a0,a0
    1e70:	8082                	ret

0000000000001e72 <mkdir>:
    1e72:	862e                	mv	a2,a1
    1e74:	87aa                	mv	a5,a0
    1e76:	1602                	sll	a2,a2,0x20
    1e78:	02200893          	li	a7,34
    1e7c:	f9c00513          	li	a0,-100
    1e80:	85be                	mv	a1,a5
    1e82:	9201                	srl	a2,a2,0x20
    1e84:	00000073          	ecall
    1e88:	2501                	sext.w	a0,a0
    1e8a:	8082                	ret

0000000000001e8c <getdents>:
    1e8c:	03d00893          	li	a7,61
    1e90:	00000073          	ecall
    1e94:	2501                	sext.w	a0,a0
    1e96:	8082                	ret

0000000000001e98 <pipe>:
    1e98:	03b00893          	li	a7,59
    1e9c:	4581                	li	a1,0
    1e9e:	00000073          	ecall
    1ea2:	2501                	sext.w	a0,a0
    1ea4:	8082                	ret

0000000000001ea6 <dup>:
    1ea6:	48dd                	li	a7,23
    1ea8:	00000073          	ecall
    1eac:	2501                	sext.w	a0,a0
    1eae:	8082                	ret

0000000000001eb0 <dup2>:
    1eb0:	48e1                	li	a7,24
    1eb2:	4601                	li	a2,0
    1eb4:	00000073          	ecall
    1eb8:	2501                	sext.w	a0,a0
    1eba:	8082                	ret

0000000000001ebc <mount>:
    1ebc:	02800893          	li	a7,40
    1ec0:	00000073          	ecall
    1ec4:	2501                	sext.w	a0,a0
    1ec6:	8082                	ret

0000000000001ec8 <umount>:
    1ec8:	02700893          	li	a7,39
    1ecc:	4581                	li	a1,0
    1ece:	00000073          	ecall
    1ed2:	2501                	sext.w	a0,a0
    1ed4:	8082                	ret

0000000000001ed6 <__clone>:
    1ed6:	15c1                	add	a1,a1,-16
    1ed8:	e188                	sd	a0,0(a1)
    1eda:	e594                	sd	a3,8(a1)
    1edc:	8532                	mv	a0,a2
    1ede:	863a                	mv	a2,a4
    1ee0:	86be                	mv	a3,a5
    1ee2:	8742                	mv	a4,a6
    1ee4:	0dc00893          	li	a7,220
    1ee8:	00000073          	ecall
    1eec:	c111                	beqz	a0,1ef0 <__clone+0x1a>
    1eee:	8082                	ret
    1ef0:	6582                	ld	a1,0(sp)
    1ef2:	6522                	ld	a0,8(sp)
    1ef4:	9582                	jalr	a1
    1ef6:	05d00893          	li	a7,93
    1efa:	00000073          	ecall
