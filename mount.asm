
mount:     file format elf64-littleriscv


Disassembly of section .text:

0000000000001000 <_start>:
    1000:	850a                	mv	a0,sp
    1002:	aa31                	j	111e <__start_main>

0000000000001004 <test_mount>:
    1004:	1141                	add	sp,sp,-16
    1006:	00001517          	auipc	a0,0x1
    100a:	f2a50513          	add	a0,a0,-214 # 1f30 <__clone+0x2a>
    100e:	e406                	sd	ra,8(sp)
    1010:	e022                	sd	s0,0(sp)
    1012:	364000ef          	jal	1376 <puts>
    1016:	00001517          	auipc	a0,0x1
    101a:	07a50513          	add	a0,a0,122 # 2090 <__func__.0>
    101e:	358000ef          	jal	1376 <puts>
    1022:	00001517          	auipc	a0,0x1
    1026:	f2650513          	add	a0,a0,-218 # 1f48 <__clone+0x42>
    102a:	34c000ef          	jal	1376 <puts>
    102e:	00001617          	auipc	a2,0x1
    1032:	fe260613          	add	a2,a2,-30 # 2010 <mntpoint>
    1036:	00001597          	auipc	a1,0x1
    103a:	01a58593          	add	a1,a1,26 # 2050 <device>
    103e:	00001517          	auipc	a0,0x1
    1042:	f1a50513          	add	a0,a0,-230 # 1f58 <__clone+0x52>
    1046:	352000ef          	jal	1398 <printf>
    104a:	00001597          	auipc	a1,0x1
    104e:	fc658593          	add	a1,a1,-58 # 2010 <mntpoint>
    1052:	4701                	li	a4,0
    1054:	4681                	li	a3,0
    1056:	00001617          	auipc	a2,0x1
    105a:	f1a60613          	add	a2,a2,-230 # 1f70 <__clone+0x6a>
    105e:	00001517          	auipc	a0,0x1
    1062:	ff250513          	add	a0,a0,-14 # 2050 <device>
    1066:	687000ef          	jal	1eec <mount>
    106a:	842a                	mv	s0,a0
    106c:	85aa                	mv	a1,a0
    106e:	00001517          	auipc	a0,0x1
    1072:	f0a50513          	add	a0,a0,-246 # 1f78 <__clone+0x72>
    1076:	322000ef          	jal	1398 <printf>
    107a:	e821                	bnez	s0,10ca <test_mount+0xc6>
    107c:	00001517          	auipc	a0,0x1
    1080:	f1450513          	add	a0,a0,-236 # 1f90 <__clone+0x8a>
    1084:	314000ef          	jal	1398 <printf>
    1088:	00001517          	auipc	a0,0x1
    108c:	f8850513          	add	a0,a0,-120 # 2010 <mntpoint>
    1090:	669000ef          	jal	1ef8 <umount>
    1094:	85aa                	mv	a1,a0
    1096:	00001517          	auipc	a0,0x1
    109a:	f1250513          	add	a0,a0,-238 # 1fa8 <__clone+0xa2>
    109e:	2fa000ef          	jal	1398 <printf>
    10a2:	00001517          	auipc	a0,0x1
    10a6:	f3e50513          	add	a0,a0,-194 # 1fe0 <__clone+0xda>
    10aa:	2cc000ef          	jal	1376 <puts>
    10ae:	00001517          	auipc	a0,0x1
    10b2:	fe250513          	add	a0,a0,-30 # 2090 <__func__.0>
    10b6:	2c0000ef          	jal	1376 <puts>
    10ba:	6402                	ld	s0,0(sp)
    10bc:	60a2                	ld	ra,8(sp)
    10be:	00001517          	auipc	a0,0x1
    10c2:	e8a50513          	add	a0,a0,-374 # 1f48 <__clone+0x42>
    10c6:	0141                	add	sp,sp,16
    10c8:	a47d                	j	1376 <puts>
    10ca:	00001517          	auipc	a0,0x1
    10ce:	ef650513          	add	a0,a0,-266 # 1fc0 <__clone+0xba>
    10d2:	540000ef          	jal	1612 <panic>
    10d6:	b7f1                	j	10a2 <test_mount+0x9e>

00000000000010d8 <main>:
    10d8:	1101                	add	sp,sp,-32
    10da:	ec06                	sd	ra,24(sp)
    10dc:	e822                	sd	s0,16(sp)
    10de:	e426                	sd	s1,8(sp)
    10e0:	4785                	li	a5,1
    10e2:	00a7ca63          	blt	a5,a0,10f6 <main+0x1e>
    10e6:	f1fff0ef          	jal	1004 <test_mount>
    10ea:	60e2                	ld	ra,24(sp)
    10ec:	6442                	ld	s0,16(sp)
    10ee:	64a2                	ld	s1,8(sp)
    10f0:	4501                	li	a0,0
    10f2:	6105                	add	sp,sp,32
    10f4:	8082                	ret
    10f6:	84ae                	mv	s1,a1
    10f8:	658c                	ld	a1,8(a1)
    10fa:	842a                	mv	s0,a0
    10fc:	00001517          	auipc	a0,0x1
    1100:	f5450513          	add	a0,a0,-172 # 2050 <device>
    1104:	107000ef          	jal	1a0a <strcpy>
    1108:	4789                	li	a5,2
    110a:	fcf40ee3          	beq	s0,a5,10e6 <main+0xe>
    110e:	688c                	ld	a1,16(s1)
    1110:	00001517          	auipc	a0,0x1
    1114:	f0050513          	add	a0,a0,-256 # 2010 <mntpoint>
    1118:	0f3000ef          	jal	1a0a <strcpy>
    111c:	b7e9                	j	10e6 <main+0xe>

000000000000111e <__start_main>:
    111e:	85aa                	mv	a1,a0
    1120:	4108                	lw	a0,0(a0)
    1122:	1141                	add	sp,sp,-16
    1124:	05a1                	add	a1,a1,8
    1126:	e406                	sd	ra,8(sp)
    1128:	fb1ff0ef          	jal	10d8 <main>
    112c:	3f3000ef          	jal	1d1e <exit>
    1130:	60a2                	ld	ra,8(sp)
    1132:	4501                	li	a0,0
    1134:	0141                	add	sp,sp,16
    1136:	8082                	ret

0000000000001138 <printint.constprop.0>:
    1138:	7179                	add	sp,sp,-48
    113a:	f406                	sd	ra,40(sp)
    113c:	12054863          	bltz	a0,126c <printint.constprop.0+0x134>
    1140:	02b577bb          	remuw	a5,a0,a1
    1144:	00001697          	auipc	a3,0x1
    1148:	f5c68693          	add	a3,a3,-164 # 20a0 <digits>
    114c:	00010c23          	sb	zero,24(sp)
    1150:	0005871b          	sext.w	a4,a1
    1154:	1782                	sll	a5,a5,0x20
    1156:	9381                	srl	a5,a5,0x20
    1158:	97b6                	add	a5,a5,a3
    115a:	0007c783          	lbu	a5,0(a5)
    115e:	02b5583b          	divuw	a6,a0,a1
    1162:	00f10ba3          	sb	a5,23(sp)
    1166:	1ab56663          	bltu	a0,a1,1312 <printint.constprop.0+0x1da>
    116a:	02e8763b          	remuw	a2,a6,a4
    116e:	1602                	sll	a2,a2,0x20
    1170:	9201                	srl	a2,a2,0x20
    1172:	9636                	add	a2,a2,a3
    1174:	00064603          	lbu	a2,0(a2)
    1178:	02e855bb          	divuw	a1,a6,a4
    117c:	00c10b23          	sb	a2,22(sp)
    1180:	12e86c63          	bltu	a6,a4,12b8 <printint.constprop.0+0x180>
    1184:	02e5f63b          	remuw	a2,a1,a4
    1188:	1602                	sll	a2,a2,0x20
    118a:	9201                	srl	a2,a2,0x20
    118c:	9636                	add	a2,a2,a3
    118e:	00064603          	lbu	a2,0(a2)
    1192:	02e5d83b          	divuw	a6,a1,a4
    1196:	00c10aa3          	sb	a2,21(sp)
    119a:	12e5e863          	bltu	a1,a4,12ca <printint.constprop.0+0x192>
    119e:	02e8763b          	remuw	a2,a6,a4
    11a2:	1602                	sll	a2,a2,0x20
    11a4:	9201                	srl	a2,a2,0x20
    11a6:	9636                	add	a2,a2,a3
    11a8:	00064603          	lbu	a2,0(a2)
    11ac:	02e855bb          	divuw	a1,a6,a4
    11b0:	00c10a23          	sb	a2,20(sp)
    11b4:	12e86463          	bltu	a6,a4,12dc <printint.constprop.0+0x1a4>
    11b8:	02e5f63b          	remuw	a2,a1,a4
    11bc:	1602                	sll	a2,a2,0x20
    11be:	9201                	srl	a2,a2,0x20
    11c0:	9636                	add	a2,a2,a3
    11c2:	00064603          	lbu	a2,0(a2)
    11c6:	02e5d83b          	divuw	a6,a1,a4
    11ca:	00c109a3          	sb	a2,19(sp)
    11ce:	12e5e063          	bltu	a1,a4,12ee <printint.constprop.0+0x1b6>
    11d2:	02e8763b          	remuw	a2,a6,a4
    11d6:	1602                	sll	a2,a2,0x20
    11d8:	9201                	srl	a2,a2,0x20
    11da:	9636                	add	a2,a2,a3
    11dc:	00064603          	lbu	a2,0(a2)
    11e0:	02e855bb          	divuw	a1,a6,a4
    11e4:	00c10923          	sb	a2,18(sp)
    11e8:	0ae86f63          	bltu	a6,a4,12a6 <printint.constprop.0+0x16e>
    11ec:	02e5f63b          	remuw	a2,a1,a4
    11f0:	1602                	sll	a2,a2,0x20
    11f2:	9201                	srl	a2,a2,0x20
    11f4:	9636                	add	a2,a2,a3
    11f6:	00064603          	lbu	a2,0(a2)
    11fa:	02e5d83b          	divuw	a6,a1,a4
    11fe:	00c108a3          	sb	a2,17(sp)
    1202:	0ee5ef63          	bltu	a1,a4,1300 <printint.constprop.0+0x1c8>
    1206:	02e8763b          	remuw	a2,a6,a4
    120a:	1602                	sll	a2,a2,0x20
    120c:	9201                	srl	a2,a2,0x20
    120e:	9636                	add	a2,a2,a3
    1210:	00064603          	lbu	a2,0(a2)
    1214:	02e855bb          	divuw	a1,a6,a4
    1218:	00c10823          	sb	a2,16(sp)
    121c:	0ee86d63          	bltu	a6,a4,1316 <printint.constprop.0+0x1de>
    1220:	02e5f63b          	remuw	a2,a1,a4
    1224:	1602                	sll	a2,a2,0x20
    1226:	9201                	srl	a2,a2,0x20
    1228:	9636                	add	a2,a2,a3
    122a:	00064603          	lbu	a2,0(a2)
    122e:	02e5d7bb          	divuw	a5,a1,a4
    1232:	00c107a3          	sb	a2,15(sp)
    1236:	0ee5e963          	bltu	a1,a4,1328 <printint.constprop.0+0x1f0>
    123a:	1782                	sll	a5,a5,0x20
    123c:	9381                	srl	a5,a5,0x20
    123e:	96be                	add	a3,a3,a5
    1240:	0006c783          	lbu	a5,0(a3)
    1244:	4599                	li	a1,6
    1246:	00f10723          	sb	a5,14(sp)
    124a:	00055763          	bgez	a0,1258 <printint.constprop.0+0x120>
    124e:	02d00793          	li	a5,45
    1252:	00f106a3          	sb	a5,13(sp)
    1256:	4595                	li	a1,5
    1258:	003c                	add	a5,sp,8
    125a:	4641                	li	a2,16
    125c:	9e0d                	subw	a2,a2,a1
    125e:	4505                	li	a0,1
    1260:	95be                	add	a1,a1,a5
    1262:	26d000ef          	jal	1cce <write>
    1266:	70a2                	ld	ra,40(sp)
    1268:	6145                	add	sp,sp,48
    126a:	8082                	ret
    126c:	40a0063b          	negw	a2,a0
    1270:	02b677bb          	remuw	a5,a2,a1
    1274:	00001697          	auipc	a3,0x1
    1278:	e2c68693          	add	a3,a3,-468 # 20a0 <digits>
    127c:	00010c23          	sb	zero,24(sp)
    1280:	0005871b          	sext.w	a4,a1
    1284:	1782                	sll	a5,a5,0x20
    1286:	9381                	srl	a5,a5,0x20
    1288:	97b6                	add	a5,a5,a3
    128a:	0007c783          	lbu	a5,0(a5)
    128e:	02b6583b          	divuw	a6,a2,a1
    1292:	00f10ba3          	sb	a5,23(sp)
    1296:	ecb67ae3          	bgeu	a2,a1,116a <printint.constprop.0+0x32>
    129a:	02d00793          	li	a5,45
    129e:	00f10b23          	sb	a5,22(sp)
    12a2:	45b9                	li	a1,14
    12a4:	bf55                	j	1258 <printint.constprop.0+0x120>
    12a6:	45a9                	li	a1,10
    12a8:	fa0558e3          	bgez	a0,1258 <printint.constprop.0+0x120>
    12ac:	02d00793          	li	a5,45
    12b0:	00f108a3          	sb	a5,17(sp)
    12b4:	45a5                	li	a1,9
    12b6:	b74d                	j	1258 <printint.constprop.0+0x120>
    12b8:	45b9                	li	a1,14
    12ba:	f8055fe3          	bgez	a0,1258 <printint.constprop.0+0x120>
    12be:	02d00793          	li	a5,45
    12c2:	00f10aa3          	sb	a5,21(sp)
    12c6:	45b5                	li	a1,13
    12c8:	bf41                	j	1258 <printint.constprop.0+0x120>
    12ca:	45b5                	li	a1,13
    12cc:	f80556e3          	bgez	a0,1258 <printint.constprop.0+0x120>
    12d0:	02d00793          	li	a5,45
    12d4:	00f10a23          	sb	a5,20(sp)
    12d8:	45b1                	li	a1,12
    12da:	bfbd                	j	1258 <printint.constprop.0+0x120>
    12dc:	45b1                	li	a1,12
    12de:	f6055de3          	bgez	a0,1258 <printint.constprop.0+0x120>
    12e2:	02d00793          	li	a5,45
    12e6:	00f109a3          	sb	a5,19(sp)
    12ea:	45ad                	li	a1,11
    12ec:	b7b5                	j	1258 <printint.constprop.0+0x120>
    12ee:	45ad                	li	a1,11
    12f0:	f60554e3          	bgez	a0,1258 <printint.constprop.0+0x120>
    12f4:	02d00793          	li	a5,45
    12f8:	00f10923          	sb	a5,18(sp)
    12fc:	45a9                	li	a1,10
    12fe:	bfa9                	j	1258 <printint.constprop.0+0x120>
    1300:	45a5                	li	a1,9
    1302:	f4055be3          	bgez	a0,1258 <printint.constprop.0+0x120>
    1306:	02d00793          	li	a5,45
    130a:	00f10823          	sb	a5,16(sp)
    130e:	45a1                	li	a1,8
    1310:	b7a1                	j	1258 <printint.constprop.0+0x120>
    1312:	45bd                	li	a1,15
    1314:	b791                	j	1258 <printint.constprop.0+0x120>
    1316:	45a1                	li	a1,8
    1318:	f40550e3          	bgez	a0,1258 <printint.constprop.0+0x120>
    131c:	02d00793          	li	a5,45
    1320:	00f107a3          	sb	a5,15(sp)
    1324:	459d                	li	a1,7
    1326:	bf0d                	j	1258 <printint.constprop.0+0x120>
    1328:	459d                	li	a1,7
    132a:	f20557e3          	bgez	a0,1258 <printint.constprop.0+0x120>
    132e:	02d00793          	li	a5,45
    1332:	00f10723          	sb	a5,14(sp)
    1336:	4599                	li	a1,6
    1338:	b705                	j	1258 <printint.constprop.0+0x120>

000000000000133a <getchar>:
    133a:	1101                	add	sp,sp,-32
    133c:	00f10593          	add	a1,sp,15
    1340:	4605                	li	a2,1
    1342:	4501                	li	a0,0
    1344:	ec06                	sd	ra,24(sp)
    1346:	000107a3          	sb	zero,15(sp)
    134a:	17b000ef          	jal	1cc4 <read>
    134e:	60e2                	ld	ra,24(sp)
    1350:	00f14503          	lbu	a0,15(sp)
    1354:	6105                	add	sp,sp,32
    1356:	8082                	ret

0000000000001358 <putchar>:
    1358:	1101                	add	sp,sp,-32
    135a:	87aa                	mv	a5,a0
    135c:	00f10593          	add	a1,sp,15
    1360:	4605                	li	a2,1
    1362:	4505                	li	a0,1
    1364:	ec06                	sd	ra,24(sp)
    1366:	00f107a3          	sb	a5,15(sp)
    136a:	165000ef          	jal	1cce <write>
    136e:	60e2                	ld	ra,24(sp)
    1370:	2501                	sext.w	a0,a0
    1372:	6105                	add	sp,sp,32
    1374:	8082                	ret

0000000000001376 <puts>:
    1376:	1141                	add	sp,sp,-16
    1378:	e406                	sd	ra,8(sp)
    137a:	e022                	sd	s0,0(sp)
    137c:	842a                	mv	s0,a0
    137e:	574000ef          	jal	18f2 <strlen>
    1382:	862a                	mv	a2,a0
    1384:	85a2                	mv	a1,s0
    1386:	4505                	li	a0,1
    1388:	147000ef          	jal	1cce <write>
    138c:	60a2                	ld	ra,8(sp)
    138e:	6402                	ld	s0,0(sp)
    1390:	957d                	sra	a0,a0,0x3f
    1392:	2501                	sext.w	a0,a0
    1394:	0141                	add	sp,sp,16
    1396:	8082                	ret

0000000000001398 <printf>:
    1398:	7171                	add	sp,sp,-176
    139a:	f85a                	sd	s6,48(sp)
    139c:	ed3e                	sd	a5,152(sp)
    139e:	7b61                	lui	s6,0xffff8
    13a0:	18bc                	add	a5,sp,120
    13a2:	e8ca                	sd	s2,80(sp)
    13a4:	e4ce                	sd	s3,72(sp)
    13a6:	e0d2                	sd	s4,64(sp)
    13a8:	fc56                	sd	s5,56(sp)
    13aa:	f486                	sd	ra,104(sp)
    13ac:	f0a2                	sd	s0,96(sp)
    13ae:	eca6                	sd	s1,88(sp)
    13b0:	fcae                	sd	a1,120(sp)
    13b2:	e132                	sd	a2,128(sp)
    13b4:	e536                	sd	a3,136(sp)
    13b6:	e93a                	sd	a4,144(sp)
    13b8:	f142                	sd	a6,160(sp)
    13ba:	f546                	sd	a7,168(sp)
    13bc:	e03e                	sd	a5,0(sp)
    13be:	02500913          	li	s2,37
    13c2:	07300a13          	li	s4,115
    13c6:	07800a93          	li	s5,120
    13ca:	830b4b13          	xor	s6,s6,-2000
    13ce:	00001997          	auipc	s3,0x1
    13d2:	cd298993          	add	s3,s3,-814 # 20a0 <digits>
    13d6:	00054783          	lbu	a5,0(a0)
    13da:	16078a63          	beqz	a5,154e <printf+0x1b6>
    13de:	862a                	mv	a2,a0
    13e0:	19278d63          	beq	a5,s2,157a <printf+0x1e2>
    13e4:	00164783          	lbu	a5,1(a2)
    13e8:	0605                	add	a2,a2,1
    13ea:	fbfd                	bnez	a5,13e0 <printf+0x48>
    13ec:	84b2                	mv	s1,a2
    13ee:	40a6043b          	subw	s0,a2,a0
    13f2:	85aa                	mv	a1,a0
    13f4:	8622                	mv	a2,s0
    13f6:	4505                	li	a0,1
    13f8:	0d7000ef          	jal	1cce <write>
    13fc:	1a041463          	bnez	s0,15a4 <printf+0x20c>
    1400:	0014c783          	lbu	a5,1(s1)
    1404:	14078563          	beqz	a5,154e <printf+0x1b6>
    1408:	1b478063          	beq	a5,s4,15a8 <printf+0x210>
    140c:	14fa6b63          	bltu	s4,a5,1562 <printf+0x1ca>
    1410:	06400713          	li	a4,100
    1414:	1ee78063          	beq	a5,a4,15f4 <printf+0x25c>
    1418:	07000713          	li	a4,112
    141c:	1ae79963          	bne	a5,a4,15ce <printf+0x236>
    1420:	6702                	ld	a4,0(sp)
    1422:	01611423          	sh	s6,8(sp)
    1426:	4649                	li	a2,18
    1428:	631c                	ld	a5,0(a4)
    142a:	0721                	add	a4,a4,8
    142c:	e03a                	sd	a4,0(sp)
    142e:	00479293          	sll	t0,a5,0x4
    1432:	00879f93          	sll	t6,a5,0x8
    1436:	00c79f13          	sll	t5,a5,0xc
    143a:	01079e93          	sll	t4,a5,0x10
    143e:	01479e13          	sll	t3,a5,0x14
    1442:	01879313          	sll	t1,a5,0x18
    1446:	01c79893          	sll	a7,a5,0x1c
    144a:	02479813          	sll	a6,a5,0x24
    144e:	02879513          	sll	a0,a5,0x28
    1452:	02c79593          	sll	a1,a5,0x2c
    1456:	03079693          	sll	a3,a5,0x30
    145a:	03479713          	sll	a4,a5,0x34
    145e:	03c7d413          	srl	s0,a5,0x3c
    1462:	01c7d39b          	srlw	t2,a5,0x1c
    1466:	03c2d293          	srl	t0,t0,0x3c
    146a:	03cfdf93          	srl	t6,t6,0x3c
    146e:	03cf5f13          	srl	t5,t5,0x3c
    1472:	03cede93          	srl	t4,t4,0x3c
    1476:	03ce5e13          	srl	t3,t3,0x3c
    147a:	03c35313          	srl	t1,t1,0x3c
    147e:	03c8d893          	srl	a7,a7,0x3c
    1482:	03c85813          	srl	a6,a6,0x3c
    1486:	9171                	srl	a0,a0,0x3c
    1488:	91f1                	srl	a1,a1,0x3c
    148a:	92f1                	srl	a3,a3,0x3c
    148c:	9371                	srl	a4,a4,0x3c
    148e:	96ce                	add	a3,a3,s3
    1490:	974e                	add	a4,a4,s3
    1492:	944e                	add	s0,s0,s3
    1494:	92ce                	add	t0,t0,s3
    1496:	9fce                	add	t6,t6,s3
    1498:	9f4e                	add	t5,t5,s3
    149a:	9ece                	add	t4,t4,s3
    149c:	9e4e                	add	t3,t3,s3
    149e:	934e                	add	t1,t1,s3
    14a0:	98ce                	add	a7,a7,s3
    14a2:	93ce                	add	t2,t2,s3
    14a4:	984e                	add	a6,a6,s3
    14a6:	954e                	add	a0,a0,s3
    14a8:	95ce                	add	a1,a1,s3
    14aa:	0006c083          	lbu	ra,0(a3)
    14ae:	0002c283          	lbu	t0,0(t0)
    14b2:	00074683          	lbu	a3,0(a4)
    14b6:	000fcf83          	lbu	t6,0(t6)
    14ba:	000f4f03          	lbu	t5,0(t5)
    14be:	000ece83          	lbu	t4,0(t4)
    14c2:	000e4e03          	lbu	t3,0(t3)
    14c6:	00034303          	lbu	t1,0(t1)
    14ca:	0008c883          	lbu	a7,0(a7)
    14ce:	0003c383          	lbu	t2,0(t2)
    14d2:	00084803          	lbu	a6,0(a6)
    14d6:	00054503          	lbu	a0,0(a0)
    14da:	0005c583          	lbu	a1,0(a1)
    14de:	00044403          	lbu	s0,0(s0)
    14e2:	03879713          	sll	a4,a5,0x38
    14e6:	9371                	srl	a4,a4,0x3c
    14e8:	8bbd                	and	a5,a5,15
    14ea:	974e                	add	a4,a4,s3
    14ec:	97ce                	add	a5,a5,s3
    14ee:	005105a3          	sb	t0,11(sp)
    14f2:	01f10623          	sb	t6,12(sp)
    14f6:	01e106a3          	sb	t5,13(sp)
    14fa:	01d10723          	sb	t4,14(sp)
    14fe:	01c107a3          	sb	t3,15(sp)
    1502:	00610823          	sb	t1,16(sp)
    1506:	011108a3          	sb	a7,17(sp)
    150a:	00710923          	sb	t2,18(sp)
    150e:	010109a3          	sb	a6,19(sp)
    1512:	00a10a23          	sb	a0,20(sp)
    1516:	00b10aa3          	sb	a1,21(sp)
    151a:	00110b23          	sb	ra,22(sp)
    151e:	00d10ba3          	sb	a3,23(sp)
    1522:	00810523          	sb	s0,10(sp)
    1526:	00074703          	lbu	a4,0(a4)
    152a:	0007c783          	lbu	a5,0(a5)
    152e:	002c                	add	a1,sp,8
    1530:	4505                	li	a0,1
    1532:	00e10c23          	sb	a4,24(sp)
    1536:	00f10ca3          	sb	a5,25(sp)
    153a:	00010d23          	sb	zero,26(sp)
    153e:	790000ef          	jal	1cce <write>
    1542:	00248513          	add	a0,s1,2
    1546:	00054783          	lbu	a5,0(a0)
    154a:	e8079ae3          	bnez	a5,13de <printf+0x46>
    154e:	70a6                	ld	ra,104(sp)
    1550:	7406                	ld	s0,96(sp)
    1552:	64e6                	ld	s1,88(sp)
    1554:	6946                	ld	s2,80(sp)
    1556:	69a6                	ld	s3,72(sp)
    1558:	6a06                	ld	s4,64(sp)
    155a:	7ae2                	ld	s5,56(sp)
    155c:	7b42                	ld	s6,48(sp)
    155e:	614d                	add	sp,sp,176
    1560:	8082                	ret
    1562:	07579663          	bne	a5,s5,15ce <printf+0x236>
    1566:	6782                	ld	a5,0(sp)
    1568:	45c1                	li	a1,16
    156a:	4388                	lw	a0,0(a5)
    156c:	07a1                	add	a5,a5,8
    156e:	e03e                	sd	a5,0(sp)
    1570:	bc9ff0ef          	jal	1138 <printint.constprop.0>
    1574:	00248513          	add	a0,s1,2
    1578:	b7f9                	j	1546 <printf+0x1ae>
    157a:	84b2                	mv	s1,a2
    157c:	a039                	j	158a <printf+0x1f2>
    157e:	0024c783          	lbu	a5,2(s1)
    1582:	0605                	add	a2,a2,1
    1584:	0489                	add	s1,s1,2
    1586:	e72794e3          	bne	a5,s2,13ee <printf+0x56>
    158a:	0014c783          	lbu	a5,1(s1)
    158e:	ff2788e3          	beq	a5,s2,157e <printf+0x1e6>
    1592:	40a6043b          	subw	s0,a2,a0
    1596:	85aa                	mv	a1,a0
    1598:	8622                	mv	a2,s0
    159a:	4505                	li	a0,1
    159c:	732000ef          	jal	1cce <write>
    15a0:	e60400e3          	beqz	s0,1400 <printf+0x68>
    15a4:	8526                	mv	a0,s1
    15a6:	bd05                	j	13d6 <printf+0x3e>
    15a8:	6782                	ld	a5,0(sp)
    15aa:	6380                	ld	s0,0(a5)
    15ac:	07a1                	add	a5,a5,8
    15ae:	e03e                	sd	a5,0(sp)
    15b0:	cc21                	beqz	s0,1608 <printf+0x270>
    15b2:	0c800593          	li	a1,200
    15b6:	8522                	mv	a0,s0
    15b8:	424000ef          	jal	19dc <strnlen>
    15bc:	0005061b          	sext.w	a2,a0
    15c0:	85a2                	mv	a1,s0
    15c2:	4505                	li	a0,1
    15c4:	70a000ef          	jal	1cce <write>
    15c8:	00248513          	add	a0,s1,2
    15cc:	bfad                	j	1546 <printf+0x1ae>
    15ce:	4605                	li	a2,1
    15d0:	002c                	add	a1,sp,8
    15d2:	4505                	li	a0,1
    15d4:	01210423          	sb	s2,8(sp)
    15d8:	6f6000ef          	jal	1cce <write>
    15dc:	0014c783          	lbu	a5,1(s1)
    15e0:	4605                	li	a2,1
    15e2:	002c                	add	a1,sp,8
    15e4:	4505                	li	a0,1
    15e6:	00f10423          	sb	a5,8(sp)
    15ea:	6e4000ef          	jal	1cce <write>
    15ee:	00248513          	add	a0,s1,2
    15f2:	bf91                	j	1546 <printf+0x1ae>
    15f4:	6782                	ld	a5,0(sp)
    15f6:	45a9                	li	a1,10
    15f8:	4388                	lw	a0,0(a5)
    15fa:	07a1                	add	a5,a5,8
    15fc:	e03e                	sd	a5,0(sp)
    15fe:	b3bff0ef          	jal	1138 <printint.constprop.0>
    1602:	00248513          	add	a0,s1,2
    1606:	b781                	j	1546 <printf+0x1ae>
    1608:	00001417          	auipc	s0,0x1
    160c:	9e840413          	add	s0,s0,-1560 # 1ff0 <__clone+0xea>
    1610:	b74d                	j	15b2 <printf+0x21a>

0000000000001612 <panic>:
    1612:	1141                	add	sp,sp,-16
    1614:	e406                	sd	ra,8(sp)
    1616:	d61ff0ef          	jal	1376 <puts>
    161a:	60a2                	ld	ra,8(sp)
    161c:	f9c00513          	li	a0,-100
    1620:	0141                	add	sp,sp,16
    1622:	adf5                	j	1d1e <exit>

0000000000001624 <isspace>:
    1624:	02000793          	li	a5,32
    1628:	00f50663          	beq	a0,a5,1634 <isspace+0x10>
    162c:	355d                	addw	a0,a0,-9
    162e:	00553513          	sltiu	a0,a0,5
    1632:	8082                	ret
    1634:	4505                	li	a0,1
    1636:	8082                	ret

0000000000001638 <isdigit>:
    1638:	fd05051b          	addw	a0,a0,-48
    163c:	00a53513          	sltiu	a0,a0,10
    1640:	8082                	ret

0000000000001642 <atoi>:
    1642:	02000693          	li	a3,32
    1646:	4591                	li	a1,4
    1648:	00054783          	lbu	a5,0(a0)
    164c:	ff77871b          	addw	a4,a5,-9
    1650:	04d78c63          	beq	a5,a3,16a8 <atoi+0x66>
    1654:	0007861b          	sext.w	a2,a5
    1658:	04e5f863          	bgeu	a1,a4,16a8 <atoi+0x66>
    165c:	02b00713          	li	a4,43
    1660:	04e78963          	beq	a5,a4,16b2 <atoi+0x70>
    1664:	02d00713          	li	a4,45
    1668:	06e78263          	beq	a5,a4,16cc <atoi+0x8a>
    166c:	fd06069b          	addw	a3,a2,-48
    1670:	47a5                	li	a5,9
    1672:	872a                	mv	a4,a0
    1674:	4301                	li	t1,0
    1676:	04d7e963          	bltu	a5,a3,16c8 <atoi+0x86>
    167a:	4501                	li	a0,0
    167c:	48a5                	li	a7,9
    167e:	00174683          	lbu	a3,1(a4)
    1682:	0025179b          	sllw	a5,a0,0x2
    1686:	9fa9                	addw	a5,a5,a0
    1688:	fd06059b          	addw	a1,a2,-48
    168c:	0017979b          	sllw	a5,a5,0x1
    1690:	fd06881b          	addw	a6,a3,-48
    1694:	0705                	add	a4,a4,1
    1696:	40b7853b          	subw	a0,a5,a1
    169a:	0006861b          	sext.w	a2,a3
    169e:	ff08f0e3          	bgeu	a7,a6,167e <atoi+0x3c>
    16a2:	00030563          	beqz	t1,16ac <atoi+0x6a>
    16a6:	8082                	ret
    16a8:	0505                	add	a0,a0,1
    16aa:	bf79                	j	1648 <atoi+0x6>
    16ac:	40f5853b          	subw	a0,a1,a5
    16b0:	8082                	ret
    16b2:	00154603          	lbu	a2,1(a0)
    16b6:	47a5                	li	a5,9
    16b8:	00150713          	add	a4,a0,1
    16bc:	fd06069b          	addw	a3,a2,-48
    16c0:	4301                	li	t1,0
    16c2:	2601                	sext.w	a2,a2
    16c4:	fad7fbe3          	bgeu	a5,a3,167a <atoi+0x38>
    16c8:	4501                	li	a0,0
    16ca:	8082                	ret
    16cc:	00154603          	lbu	a2,1(a0)
    16d0:	47a5                	li	a5,9
    16d2:	00150713          	add	a4,a0,1
    16d6:	fd06069b          	addw	a3,a2,-48
    16da:	2601                	sext.w	a2,a2
    16dc:	fed7e6e3          	bltu	a5,a3,16c8 <atoi+0x86>
    16e0:	4305                	li	t1,1
    16e2:	bf61                	j	167a <atoi+0x38>

00000000000016e4 <memset>:
    16e4:	18060163          	beqz	a2,1866 <memset+0x182>
    16e8:	40a006b3          	neg	a3,a0
    16ec:	0076f793          	and	a5,a3,7
    16f0:	00778813          	add	a6,a5,7
    16f4:	48ad                	li	a7,11
    16f6:	0ff5f713          	zext.b	a4,a1
    16fa:	fff60593          	add	a1,a2,-1
    16fe:	17186563          	bltu	a6,a7,1868 <memset+0x184>
    1702:	1705ed63          	bltu	a1,a6,187c <memset+0x198>
    1706:	16078363          	beqz	a5,186c <memset+0x188>
    170a:	00e50023          	sb	a4,0(a0)
    170e:	0066f593          	and	a1,a3,6
    1712:	16058063          	beqz	a1,1872 <memset+0x18e>
    1716:	00e500a3          	sb	a4,1(a0)
    171a:	4589                	li	a1,2
    171c:	16f5f363          	bgeu	a1,a5,1882 <memset+0x19e>
    1720:	00e50123          	sb	a4,2(a0)
    1724:	8a91                	and	a3,a3,4
    1726:	00350593          	add	a1,a0,3
    172a:	4e0d                	li	t3,3
    172c:	ce9d                	beqz	a3,176a <memset+0x86>
    172e:	00e501a3          	sb	a4,3(a0)
    1732:	4691                	li	a3,4
    1734:	00450593          	add	a1,a0,4
    1738:	4e11                	li	t3,4
    173a:	02f6f863          	bgeu	a3,a5,176a <memset+0x86>
    173e:	00e50223          	sb	a4,4(a0)
    1742:	4695                	li	a3,5
    1744:	00550593          	add	a1,a0,5
    1748:	4e15                	li	t3,5
    174a:	02d78063          	beq	a5,a3,176a <memset+0x86>
    174e:	fff50693          	add	a3,a0,-1
    1752:	00e502a3          	sb	a4,5(a0)
    1756:	8a9d                	and	a3,a3,7
    1758:	00650593          	add	a1,a0,6
    175c:	4e19                	li	t3,6
    175e:	e691                	bnez	a3,176a <memset+0x86>
    1760:	00750593          	add	a1,a0,7
    1764:	00e50323          	sb	a4,6(a0)
    1768:	4e1d                	li	t3,7
    176a:	00871693          	sll	a3,a4,0x8
    176e:	01071813          	sll	a6,a4,0x10
    1772:	8ed9                	or	a3,a3,a4
    1774:	01871893          	sll	a7,a4,0x18
    1778:	0106e6b3          	or	a3,a3,a6
    177c:	0116e6b3          	or	a3,a3,a7
    1780:	02071813          	sll	a6,a4,0x20
    1784:	02871313          	sll	t1,a4,0x28
    1788:	0106e6b3          	or	a3,a3,a6
    178c:	40f608b3          	sub	a7,a2,a5
    1790:	03071813          	sll	a6,a4,0x30
    1794:	0066e6b3          	or	a3,a3,t1
    1798:	0106e6b3          	or	a3,a3,a6
    179c:	03871313          	sll	t1,a4,0x38
    17a0:	97aa                	add	a5,a5,a0
    17a2:	ff88f813          	and	a6,a7,-8
    17a6:	0066e6b3          	or	a3,a3,t1
    17aa:	983e                	add	a6,a6,a5
    17ac:	e394                	sd	a3,0(a5)
    17ae:	07a1                	add	a5,a5,8
    17b0:	ff079ee3          	bne	a5,a6,17ac <memset+0xc8>
    17b4:	ff88f793          	and	a5,a7,-8
    17b8:	0078f893          	and	a7,a7,7
    17bc:	00f586b3          	add	a3,a1,a5
    17c0:	01c787bb          	addw	a5,a5,t3
    17c4:	0a088b63          	beqz	a7,187a <memset+0x196>
    17c8:	00e68023          	sb	a4,0(a3)
    17cc:	0017859b          	addw	a1,a5,1
    17d0:	08c5fb63          	bgeu	a1,a2,1866 <memset+0x182>
    17d4:	00e680a3          	sb	a4,1(a3)
    17d8:	0027859b          	addw	a1,a5,2
    17dc:	08c5f563          	bgeu	a1,a2,1866 <memset+0x182>
    17e0:	00e68123          	sb	a4,2(a3)
    17e4:	0037859b          	addw	a1,a5,3
    17e8:	06c5ff63          	bgeu	a1,a2,1866 <memset+0x182>
    17ec:	00e681a3          	sb	a4,3(a3)
    17f0:	0047859b          	addw	a1,a5,4
    17f4:	06c5f963          	bgeu	a1,a2,1866 <memset+0x182>
    17f8:	00e68223          	sb	a4,4(a3)
    17fc:	0057859b          	addw	a1,a5,5
    1800:	06c5f363          	bgeu	a1,a2,1866 <memset+0x182>
    1804:	00e682a3          	sb	a4,5(a3)
    1808:	0067859b          	addw	a1,a5,6
    180c:	04c5fd63          	bgeu	a1,a2,1866 <memset+0x182>
    1810:	00e68323          	sb	a4,6(a3)
    1814:	0077859b          	addw	a1,a5,7
    1818:	04c5f763          	bgeu	a1,a2,1866 <memset+0x182>
    181c:	00e683a3          	sb	a4,7(a3)
    1820:	0087859b          	addw	a1,a5,8
    1824:	04c5f163          	bgeu	a1,a2,1866 <memset+0x182>
    1828:	00e68423          	sb	a4,8(a3)
    182c:	0097859b          	addw	a1,a5,9
    1830:	02c5fb63          	bgeu	a1,a2,1866 <memset+0x182>
    1834:	00e684a3          	sb	a4,9(a3)
    1838:	00a7859b          	addw	a1,a5,10
    183c:	02c5f563          	bgeu	a1,a2,1866 <memset+0x182>
    1840:	00e68523          	sb	a4,10(a3)
    1844:	00b7859b          	addw	a1,a5,11
    1848:	00c5ff63          	bgeu	a1,a2,1866 <memset+0x182>
    184c:	00e685a3          	sb	a4,11(a3)
    1850:	00c7859b          	addw	a1,a5,12
    1854:	00c5f963          	bgeu	a1,a2,1866 <memset+0x182>
    1858:	00e68623          	sb	a4,12(a3)
    185c:	27b5                	addw	a5,a5,13
    185e:	00c7f463          	bgeu	a5,a2,1866 <memset+0x182>
    1862:	00e686a3          	sb	a4,13(a3)
    1866:	8082                	ret
    1868:	482d                	li	a6,11
    186a:	bd61                	j	1702 <memset+0x1e>
    186c:	85aa                	mv	a1,a0
    186e:	4e01                	li	t3,0
    1870:	bded                	j	176a <memset+0x86>
    1872:	00150593          	add	a1,a0,1
    1876:	4e05                	li	t3,1
    1878:	bdcd                	j	176a <memset+0x86>
    187a:	8082                	ret
    187c:	86aa                	mv	a3,a0
    187e:	4781                	li	a5,0
    1880:	b7a1                	j	17c8 <memset+0xe4>
    1882:	00250593          	add	a1,a0,2
    1886:	4e09                	li	t3,2
    1888:	b5cd                	j	176a <memset+0x86>

000000000000188a <strcmp>:
    188a:	00054783          	lbu	a5,0(a0)
    188e:	0005c703          	lbu	a4,0(a1)
    1892:	00e79863          	bne	a5,a4,18a2 <strcmp+0x18>
    1896:	0505                	add	a0,a0,1
    1898:	0585                	add	a1,a1,1
    189a:	fbe5                	bnez	a5,188a <strcmp>
    189c:	4501                	li	a0,0
    189e:	9d19                	subw	a0,a0,a4
    18a0:	8082                	ret
    18a2:	0007851b          	sext.w	a0,a5
    18a6:	bfe5                	j	189e <strcmp+0x14>

00000000000018a8 <strncmp>:
    18a8:	ca15                	beqz	a2,18dc <strncmp+0x34>
    18aa:	00054783          	lbu	a5,0(a0)
    18ae:	167d                	add	a2,a2,-1
    18b0:	00c506b3          	add	a3,a0,a2
    18b4:	eb99                	bnez	a5,18ca <strncmp+0x22>
    18b6:	a815                	j	18ea <strncmp+0x42>
    18b8:	00a68e63          	beq	a3,a0,18d4 <strncmp+0x2c>
    18bc:	0505                	add	a0,a0,1
    18be:	00f71b63          	bne	a4,a5,18d4 <strncmp+0x2c>
    18c2:	00054783          	lbu	a5,0(a0)
    18c6:	cf89                	beqz	a5,18e0 <strncmp+0x38>
    18c8:	85b2                	mv	a1,a2
    18ca:	0005c703          	lbu	a4,0(a1)
    18ce:	00158613          	add	a2,a1,1
    18d2:	f37d                	bnez	a4,18b8 <strncmp+0x10>
    18d4:	0007851b          	sext.w	a0,a5
    18d8:	9d19                	subw	a0,a0,a4
    18da:	8082                	ret
    18dc:	4501                	li	a0,0
    18de:	8082                	ret
    18e0:	0015c703          	lbu	a4,1(a1)
    18e4:	4501                	li	a0,0
    18e6:	9d19                	subw	a0,a0,a4
    18e8:	8082                	ret
    18ea:	0005c703          	lbu	a4,0(a1)
    18ee:	4501                	li	a0,0
    18f0:	b7e5                	j	18d8 <strncmp+0x30>

00000000000018f2 <strlen>:
    18f2:	00757793          	and	a5,a0,7
    18f6:	cf89                	beqz	a5,1910 <strlen+0x1e>
    18f8:	87aa                	mv	a5,a0
    18fa:	a029                	j	1904 <strlen+0x12>
    18fc:	0785                	add	a5,a5,1
    18fe:	0077f713          	and	a4,a5,7
    1902:	cb01                	beqz	a4,1912 <strlen+0x20>
    1904:	0007c703          	lbu	a4,0(a5)
    1908:	fb75                	bnez	a4,18fc <strlen+0xa>
    190a:	40a78533          	sub	a0,a5,a0
    190e:	8082                	ret
    1910:	87aa                	mv	a5,a0
    1912:	6394                	ld	a3,0(a5)
    1914:	00000597          	auipc	a1,0x0
    1918:	6e45b583          	ld	a1,1764(a1) # 1ff8 <__clone+0xf2>
    191c:	00000617          	auipc	a2,0x0
    1920:	6e463603          	ld	a2,1764(a2) # 2000 <__clone+0xfa>
    1924:	a019                	j	192a <strlen+0x38>
    1926:	6794                	ld	a3,8(a5)
    1928:	07a1                	add	a5,a5,8
    192a:	00b68733          	add	a4,a3,a1
    192e:	fff6c693          	not	a3,a3
    1932:	8f75                	and	a4,a4,a3
    1934:	8f71                	and	a4,a4,a2
    1936:	db65                	beqz	a4,1926 <strlen+0x34>
    1938:	0007c703          	lbu	a4,0(a5)
    193c:	d779                	beqz	a4,190a <strlen+0x18>
    193e:	0017c703          	lbu	a4,1(a5)
    1942:	0785                	add	a5,a5,1
    1944:	d379                	beqz	a4,190a <strlen+0x18>
    1946:	0017c703          	lbu	a4,1(a5)
    194a:	0785                	add	a5,a5,1
    194c:	fb6d                	bnez	a4,193e <strlen+0x4c>
    194e:	bf75                	j	190a <strlen+0x18>

0000000000001950 <memchr>:
    1950:	00757713          	and	a4,a0,7
    1954:	87aa                	mv	a5,a0
    1956:	0ff5f593          	zext.b	a1,a1
    195a:	cb19                	beqz	a4,1970 <memchr+0x20>
    195c:	ce25                	beqz	a2,19d4 <memchr+0x84>
    195e:	0007c703          	lbu	a4,0(a5)
    1962:	00b70763          	beq	a4,a1,1970 <memchr+0x20>
    1966:	0785                	add	a5,a5,1
    1968:	0077f713          	and	a4,a5,7
    196c:	167d                	add	a2,a2,-1
    196e:	f77d                	bnez	a4,195c <memchr+0xc>
    1970:	4501                	li	a0,0
    1972:	c235                	beqz	a2,19d6 <memchr+0x86>
    1974:	0007c703          	lbu	a4,0(a5)
    1978:	06b70063          	beq	a4,a1,19d8 <memchr+0x88>
    197c:	00000517          	auipc	a0,0x0
    1980:	68c53503          	ld	a0,1676(a0) # 2008 <__clone+0x102>
    1984:	471d                	li	a4,7
    1986:	02a58533          	mul	a0,a1,a0
    198a:	04c77763          	bgeu	a4,a2,19d8 <memchr+0x88>
    198e:	00000897          	auipc	a7,0x0
    1992:	66a8b883          	ld	a7,1642(a7) # 1ff8 <__clone+0xf2>
    1996:	00000817          	auipc	a6,0x0
    199a:	66a83803          	ld	a6,1642(a6) # 2000 <__clone+0xfa>
    199e:	431d                	li	t1,7
    19a0:	a029                	j	19aa <memchr+0x5a>
    19a2:	1661                	add	a2,a2,-8
    19a4:	07a1                	add	a5,a5,8
    19a6:	00c37c63          	bgeu	t1,a2,19be <memchr+0x6e>
    19aa:	6398                	ld	a4,0(a5)
    19ac:	8f29                	xor	a4,a4,a0
    19ae:	011706b3          	add	a3,a4,a7
    19b2:	fff74713          	not	a4,a4
    19b6:	8f75                	and	a4,a4,a3
    19b8:	01077733          	and	a4,a4,a6
    19bc:	d37d                	beqz	a4,19a2 <memchr+0x52>
    19be:	853e                	mv	a0,a5
    19c0:	e601                	bnez	a2,19c8 <memchr+0x78>
    19c2:	a809                	j	19d4 <memchr+0x84>
    19c4:	0505                	add	a0,a0,1
    19c6:	c619                	beqz	a2,19d4 <memchr+0x84>
    19c8:	00054783          	lbu	a5,0(a0)
    19cc:	167d                	add	a2,a2,-1
    19ce:	feb79be3          	bne	a5,a1,19c4 <memchr+0x74>
    19d2:	8082                	ret
    19d4:	4501                	li	a0,0
    19d6:	8082                	ret
    19d8:	853e                	mv	a0,a5
    19da:	b7fd                	j	19c8 <memchr+0x78>

00000000000019dc <strnlen>:
    19dc:	1101                	add	sp,sp,-32
    19de:	e822                	sd	s0,16(sp)
    19e0:	862e                	mv	a2,a1
    19e2:	842e                	mv	s0,a1
    19e4:	4581                	li	a1,0
    19e6:	e426                	sd	s1,8(sp)
    19e8:	ec06                	sd	ra,24(sp)
    19ea:	84aa                	mv	s1,a0
    19ec:	f65ff0ef          	jal	1950 <memchr>
    19f0:	c519                	beqz	a0,19fe <strnlen+0x22>
    19f2:	60e2                	ld	ra,24(sp)
    19f4:	6442                	ld	s0,16(sp)
    19f6:	8d05                	sub	a0,a0,s1
    19f8:	64a2                	ld	s1,8(sp)
    19fa:	6105                	add	sp,sp,32
    19fc:	8082                	ret
    19fe:	60e2                	ld	ra,24(sp)
    1a00:	8522                	mv	a0,s0
    1a02:	6442                	ld	s0,16(sp)
    1a04:	64a2                	ld	s1,8(sp)
    1a06:	6105                	add	sp,sp,32
    1a08:	8082                	ret

0000000000001a0a <strcpy>:
    1a0a:	00a5c7b3          	xor	a5,a1,a0
    1a0e:	8b9d                	and	a5,a5,7
    1a10:	eb95                	bnez	a5,1a44 <strcpy+0x3a>
    1a12:	0075f793          	and	a5,a1,7
    1a16:	e7b1                	bnez	a5,1a62 <strcpy+0x58>
    1a18:	6198                	ld	a4,0(a1)
    1a1a:	00000617          	auipc	a2,0x0
    1a1e:	5de63603          	ld	a2,1502(a2) # 1ff8 <__clone+0xf2>
    1a22:	00000817          	auipc	a6,0x0
    1a26:	5de83803          	ld	a6,1502(a6) # 2000 <__clone+0xfa>
    1a2a:	a029                	j	1a34 <strcpy+0x2a>
    1a2c:	05a1                	add	a1,a1,8
    1a2e:	e118                	sd	a4,0(a0)
    1a30:	6198                	ld	a4,0(a1)
    1a32:	0521                	add	a0,a0,8
    1a34:	00c707b3          	add	a5,a4,a2
    1a38:	fff74693          	not	a3,a4
    1a3c:	8ff5                	and	a5,a5,a3
    1a3e:	0107f7b3          	and	a5,a5,a6
    1a42:	d7ed                	beqz	a5,1a2c <strcpy+0x22>
    1a44:	0005c783          	lbu	a5,0(a1)
    1a48:	00f50023          	sb	a5,0(a0)
    1a4c:	c785                	beqz	a5,1a74 <strcpy+0x6a>
    1a4e:	0015c783          	lbu	a5,1(a1)
    1a52:	0505                	add	a0,a0,1
    1a54:	0585                	add	a1,a1,1
    1a56:	00f50023          	sb	a5,0(a0)
    1a5a:	fbf5                	bnez	a5,1a4e <strcpy+0x44>
    1a5c:	8082                	ret
    1a5e:	0505                	add	a0,a0,1
    1a60:	df45                	beqz	a4,1a18 <strcpy+0xe>
    1a62:	0005c783          	lbu	a5,0(a1)
    1a66:	0585                	add	a1,a1,1
    1a68:	0075f713          	and	a4,a1,7
    1a6c:	00f50023          	sb	a5,0(a0)
    1a70:	f7fd                	bnez	a5,1a5e <strcpy+0x54>
    1a72:	8082                	ret
    1a74:	8082                	ret

0000000000001a76 <strncpy>:
    1a76:	00a5c7b3          	xor	a5,a1,a0
    1a7a:	8b9d                	and	a5,a5,7
    1a7c:	e3b5                	bnez	a5,1ae0 <strncpy+0x6a>
    1a7e:	0075f793          	and	a5,a1,7
    1a82:	cf99                	beqz	a5,1aa0 <strncpy+0x2a>
    1a84:	ea09                	bnez	a2,1a96 <strncpy+0x20>
    1a86:	a421                	j	1c8e <strncpy+0x218>
    1a88:	0585                	add	a1,a1,1
    1a8a:	0075f793          	and	a5,a1,7
    1a8e:	167d                	add	a2,a2,-1
    1a90:	0505                	add	a0,a0,1
    1a92:	c799                	beqz	a5,1aa0 <strncpy+0x2a>
    1a94:	c225                	beqz	a2,1af4 <strncpy+0x7e>
    1a96:	0005c783          	lbu	a5,0(a1)
    1a9a:	00f50023          	sb	a5,0(a0)
    1a9e:	f7ed                	bnez	a5,1a88 <strncpy+0x12>
    1aa0:	ca31                	beqz	a2,1af4 <strncpy+0x7e>
    1aa2:	0005c783          	lbu	a5,0(a1)
    1aa6:	cba1                	beqz	a5,1af6 <strncpy+0x80>
    1aa8:	479d                	li	a5,7
    1aaa:	02c7fc63          	bgeu	a5,a2,1ae2 <strncpy+0x6c>
    1aae:	00000897          	auipc	a7,0x0
    1ab2:	54a8b883          	ld	a7,1354(a7) # 1ff8 <__clone+0xf2>
    1ab6:	00000817          	auipc	a6,0x0
    1aba:	54a83803          	ld	a6,1354(a6) # 2000 <__clone+0xfa>
    1abe:	431d                	li	t1,7
    1ac0:	a039                	j	1ace <strncpy+0x58>
    1ac2:	e118                	sd	a4,0(a0)
    1ac4:	1661                	add	a2,a2,-8
    1ac6:	05a1                	add	a1,a1,8
    1ac8:	0521                	add	a0,a0,8
    1aca:	00c37b63          	bgeu	t1,a2,1ae0 <strncpy+0x6a>
    1ace:	6198                	ld	a4,0(a1)
    1ad0:	011707b3          	add	a5,a4,a7
    1ad4:	fff74693          	not	a3,a4
    1ad8:	8ff5                	and	a5,a5,a3
    1ada:	0107f7b3          	and	a5,a5,a6
    1ade:	d3f5                	beqz	a5,1ac2 <strncpy+0x4c>
    1ae0:	ca11                	beqz	a2,1af4 <strncpy+0x7e>
    1ae2:	0005c783          	lbu	a5,0(a1)
    1ae6:	0585                	add	a1,a1,1
    1ae8:	00f50023          	sb	a5,0(a0)
    1aec:	c789                	beqz	a5,1af6 <strncpy+0x80>
    1aee:	167d                	add	a2,a2,-1
    1af0:	0505                	add	a0,a0,1
    1af2:	fa65                	bnez	a2,1ae2 <strncpy+0x6c>
    1af4:	8082                	ret
    1af6:	4805                	li	a6,1
    1af8:	14061b63          	bnez	a2,1c4e <strncpy+0x1d8>
    1afc:	40a00733          	neg	a4,a0
    1b00:	00777793          	and	a5,a4,7
    1b04:	4581                	li	a1,0
    1b06:	12061c63          	bnez	a2,1c3e <strncpy+0x1c8>
    1b0a:	00778693          	add	a3,a5,7
    1b0e:	48ad                	li	a7,11
    1b10:	1316e563          	bltu	a3,a7,1c3a <strncpy+0x1c4>
    1b14:	16d5e263          	bltu	a1,a3,1c78 <strncpy+0x202>
    1b18:	14078c63          	beqz	a5,1c70 <strncpy+0x1fa>
    1b1c:	00050023          	sb	zero,0(a0)
    1b20:	00677693          	and	a3,a4,6
    1b24:	14068263          	beqz	a3,1c68 <strncpy+0x1f2>
    1b28:	000500a3          	sb	zero,1(a0)
    1b2c:	4689                	li	a3,2
    1b2e:	14f6f863          	bgeu	a3,a5,1c7e <strncpy+0x208>
    1b32:	00050123          	sb	zero,2(a0)
    1b36:	8b11                	and	a4,a4,4
    1b38:	12070463          	beqz	a4,1c60 <strncpy+0x1ea>
    1b3c:	000501a3          	sb	zero,3(a0)
    1b40:	4711                	li	a4,4
    1b42:	00450693          	add	a3,a0,4
    1b46:	02f77563          	bgeu	a4,a5,1b70 <strncpy+0xfa>
    1b4a:	00050223          	sb	zero,4(a0)
    1b4e:	4715                	li	a4,5
    1b50:	00550693          	add	a3,a0,5
    1b54:	00e78e63          	beq	a5,a4,1b70 <strncpy+0xfa>
    1b58:	fff50713          	add	a4,a0,-1
    1b5c:	000502a3          	sb	zero,5(a0)
    1b60:	8b1d                	and	a4,a4,7
    1b62:	12071263          	bnez	a4,1c86 <strncpy+0x210>
    1b66:	00750693          	add	a3,a0,7
    1b6a:	00050323          	sb	zero,6(a0)
    1b6e:	471d                	li	a4,7
    1b70:	40f80833          	sub	a6,a6,a5
    1b74:	ff887593          	and	a1,a6,-8
    1b78:	97aa                	add	a5,a5,a0
    1b7a:	95be                	add	a1,a1,a5
    1b7c:	0007b023          	sd	zero,0(a5)
    1b80:	07a1                	add	a5,a5,8
    1b82:	feb79de3          	bne	a5,a1,1b7c <strncpy+0x106>
    1b86:	ff887593          	and	a1,a6,-8
    1b8a:	00787813          	and	a6,a6,7
    1b8e:	00e587bb          	addw	a5,a1,a4
    1b92:	00b68733          	add	a4,a3,a1
    1b96:	0e080063          	beqz	a6,1c76 <strncpy+0x200>
    1b9a:	00070023          	sb	zero,0(a4)
    1b9e:	0017869b          	addw	a3,a5,1
    1ba2:	f4c6f9e3          	bgeu	a3,a2,1af4 <strncpy+0x7e>
    1ba6:	000700a3          	sb	zero,1(a4)
    1baa:	0027869b          	addw	a3,a5,2
    1bae:	f4c6f3e3          	bgeu	a3,a2,1af4 <strncpy+0x7e>
    1bb2:	00070123          	sb	zero,2(a4)
    1bb6:	0037869b          	addw	a3,a5,3
    1bba:	f2c6fde3          	bgeu	a3,a2,1af4 <strncpy+0x7e>
    1bbe:	000701a3          	sb	zero,3(a4)
    1bc2:	0047869b          	addw	a3,a5,4
    1bc6:	f2c6f7e3          	bgeu	a3,a2,1af4 <strncpy+0x7e>
    1bca:	00070223          	sb	zero,4(a4)
    1bce:	0057869b          	addw	a3,a5,5
    1bd2:	f2c6f1e3          	bgeu	a3,a2,1af4 <strncpy+0x7e>
    1bd6:	000702a3          	sb	zero,5(a4)
    1bda:	0067869b          	addw	a3,a5,6
    1bde:	f0c6fbe3          	bgeu	a3,a2,1af4 <strncpy+0x7e>
    1be2:	00070323          	sb	zero,6(a4)
    1be6:	0077869b          	addw	a3,a5,7
    1bea:	f0c6f5e3          	bgeu	a3,a2,1af4 <strncpy+0x7e>
    1bee:	000703a3          	sb	zero,7(a4)
    1bf2:	0087869b          	addw	a3,a5,8
    1bf6:	eec6ffe3          	bgeu	a3,a2,1af4 <strncpy+0x7e>
    1bfa:	00070423          	sb	zero,8(a4)
    1bfe:	0097869b          	addw	a3,a5,9
    1c02:	eec6f9e3          	bgeu	a3,a2,1af4 <strncpy+0x7e>
    1c06:	000704a3          	sb	zero,9(a4)
    1c0a:	00a7869b          	addw	a3,a5,10
    1c0e:	eec6f3e3          	bgeu	a3,a2,1af4 <strncpy+0x7e>
    1c12:	00070523          	sb	zero,10(a4)
    1c16:	00b7869b          	addw	a3,a5,11
    1c1a:	ecc6fde3          	bgeu	a3,a2,1af4 <strncpy+0x7e>
    1c1e:	000705a3          	sb	zero,11(a4)
    1c22:	00c7869b          	addw	a3,a5,12
    1c26:	ecc6f7e3          	bgeu	a3,a2,1af4 <strncpy+0x7e>
    1c2a:	00070623          	sb	zero,12(a4)
    1c2e:	27b5                	addw	a5,a5,13
    1c30:	ecc7f2e3          	bgeu	a5,a2,1af4 <strncpy+0x7e>
    1c34:	000706a3          	sb	zero,13(a4)
    1c38:	8082                	ret
    1c3a:	46ad                	li	a3,11
    1c3c:	bde1                	j	1b14 <strncpy+0x9e>
    1c3e:	00778693          	add	a3,a5,7
    1c42:	48ad                	li	a7,11
    1c44:	fff60593          	add	a1,a2,-1
    1c48:	ed16f6e3          	bgeu	a3,a7,1b14 <strncpy+0x9e>
    1c4c:	b7fd                	j	1c3a <strncpy+0x1c4>
    1c4e:	40a00733          	neg	a4,a0
    1c52:	8832                	mv	a6,a2
    1c54:	00777793          	and	a5,a4,7
    1c58:	4581                	li	a1,0
    1c5a:	ea0608e3          	beqz	a2,1b0a <strncpy+0x94>
    1c5e:	b7c5                	j	1c3e <strncpy+0x1c8>
    1c60:	00350693          	add	a3,a0,3
    1c64:	470d                	li	a4,3
    1c66:	b729                	j	1b70 <strncpy+0xfa>
    1c68:	00150693          	add	a3,a0,1
    1c6c:	4705                	li	a4,1
    1c6e:	b709                	j	1b70 <strncpy+0xfa>
    1c70:	86aa                	mv	a3,a0
    1c72:	4701                	li	a4,0
    1c74:	bdf5                	j	1b70 <strncpy+0xfa>
    1c76:	8082                	ret
    1c78:	872a                	mv	a4,a0
    1c7a:	4781                	li	a5,0
    1c7c:	bf39                	j	1b9a <strncpy+0x124>
    1c7e:	00250693          	add	a3,a0,2
    1c82:	4709                	li	a4,2
    1c84:	b5f5                	j	1b70 <strncpy+0xfa>
    1c86:	00650693          	add	a3,a0,6
    1c8a:	4719                	li	a4,6
    1c8c:	b5d5                	j	1b70 <strncpy+0xfa>
    1c8e:	8082                	ret

0000000000001c90 <open>:
    1c90:	87aa                	mv	a5,a0
    1c92:	862e                	mv	a2,a1
    1c94:	03800893          	li	a7,56
    1c98:	f9c00513          	li	a0,-100
    1c9c:	85be                	mv	a1,a5
    1c9e:	4689                	li	a3,2
    1ca0:	00000073          	ecall
    1ca4:	2501                	sext.w	a0,a0
    1ca6:	8082                	ret

0000000000001ca8 <openat>:
    1ca8:	03800893          	li	a7,56
    1cac:	18000693          	li	a3,384
    1cb0:	00000073          	ecall
    1cb4:	2501                	sext.w	a0,a0
    1cb6:	8082                	ret

0000000000001cb8 <close>:
    1cb8:	03900893          	li	a7,57
    1cbc:	00000073          	ecall
    1cc0:	2501                	sext.w	a0,a0
    1cc2:	8082                	ret

0000000000001cc4 <read>:
    1cc4:	03f00893          	li	a7,63
    1cc8:	00000073          	ecall
    1ccc:	8082                	ret

0000000000001cce <write>:
    1cce:	04000893          	li	a7,64
    1cd2:	00000073          	ecall
    1cd6:	8082                	ret

0000000000001cd8 <getpid>:
    1cd8:	0ac00893          	li	a7,172
    1cdc:	00000073          	ecall
    1ce0:	2501                	sext.w	a0,a0
    1ce2:	8082                	ret

0000000000001ce4 <getppid>:
    1ce4:	0ad00893          	li	a7,173
    1ce8:	00000073          	ecall
    1cec:	2501                	sext.w	a0,a0
    1cee:	8082                	ret

0000000000001cf0 <sched_yield>:
    1cf0:	07c00893          	li	a7,124
    1cf4:	00000073          	ecall
    1cf8:	2501                	sext.w	a0,a0
    1cfa:	8082                	ret

0000000000001cfc <fork>:
    1cfc:	0dc00893          	li	a7,220
    1d00:	4545                	li	a0,17
    1d02:	4581                	li	a1,0
    1d04:	00000073          	ecall
    1d08:	2501                	sext.w	a0,a0
    1d0a:	8082                	ret

0000000000001d0c <clone>:
    1d0c:	85b2                	mv	a1,a2
    1d0e:	863a                	mv	a2,a4
    1d10:	c191                	beqz	a1,1d14 <clone+0x8>
    1d12:	95b6                	add	a1,a1,a3
    1d14:	4781                	li	a5,0
    1d16:	4701                	li	a4,0
    1d18:	4681                	li	a3,0
    1d1a:	2601                	sext.w	a2,a2
    1d1c:	a2ed                	j	1f06 <__clone>

0000000000001d1e <exit>:
    1d1e:	05d00893          	li	a7,93
    1d22:	00000073          	ecall
    1d26:	8082                	ret

0000000000001d28 <waitpid>:
    1d28:	10400893          	li	a7,260
    1d2c:	4681                	li	a3,0
    1d2e:	00000073          	ecall
    1d32:	2501                	sext.w	a0,a0
    1d34:	8082                	ret

0000000000001d36 <exec>:
    1d36:	0dd00893          	li	a7,221
    1d3a:	00000073          	ecall
    1d3e:	2501                	sext.w	a0,a0
    1d40:	8082                	ret

0000000000001d42 <execve>:
    1d42:	0dd00893          	li	a7,221
    1d46:	00000073          	ecall
    1d4a:	2501                	sext.w	a0,a0
    1d4c:	8082                	ret

0000000000001d4e <times>:
    1d4e:	09900893          	li	a7,153
    1d52:	00000073          	ecall
    1d56:	2501                	sext.w	a0,a0
    1d58:	8082                	ret

0000000000001d5a <get_time>:
    1d5a:	1141                	add	sp,sp,-16
    1d5c:	0a900893          	li	a7,169
    1d60:	850a                	mv	a0,sp
    1d62:	4581                	li	a1,0
    1d64:	00000073          	ecall
    1d68:	2501                	sext.w	a0,a0
    1d6a:	ed09                	bnez	a0,1d84 <get_time+0x2a>
    1d6c:	67a2                	ld	a5,8(sp)
    1d6e:	3e800713          	li	a4,1000
    1d72:	00015503          	lhu	a0,0(sp)
    1d76:	02e7d7b3          	divu	a5,a5,a4
    1d7a:	02e50533          	mul	a0,a0,a4
    1d7e:	953e                	add	a0,a0,a5
    1d80:	0141                	add	sp,sp,16
    1d82:	8082                	ret
    1d84:	557d                	li	a0,-1
    1d86:	bfed                	j	1d80 <get_time+0x26>

0000000000001d88 <sys_get_time>:
    1d88:	0a900893          	li	a7,169
    1d8c:	00000073          	ecall
    1d90:	2501                	sext.w	a0,a0
    1d92:	8082                	ret

0000000000001d94 <time>:
    1d94:	42600893          	li	a7,1062
    1d98:	00000073          	ecall
    1d9c:	2501                	sext.w	a0,a0
    1d9e:	8082                	ret

0000000000001da0 <sleep>:
    1da0:	1141                	add	sp,sp,-16
    1da2:	e02a                	sd	a0,0(sp)
    1da4:	850a                	mv	a0,sp
    1da6:	e402                	sd	zero,8(sp)
    1da8:	06500893          	li	a7,101
    1dac:	85aa                	mv	a1,a0
    1dae:	00000073          	ecall
    1db2:	e501                	bnez	a0,1dba <sleep+0x1a>
    1db4:	4501                	li	a0,0
    1db6:	0141                	add	sp,sp,16
    1db8:	8082                	ret
    1dba:	4502                	lw	a0,0(sp)
    1dbc:	0141                	add	sp,sp,16
    1dbe:	8082                	ret

0000000000001dc0 <set_priority>:
    1dc0:	08c00893          	li	a7,140
    1dc4:	00000073          	ecall
    1dc8:	2501                	sext.w	a0,a0
    1dca:	8082                	ret

0000000000001dcc <mmap>:
    1dcc:	0de00893          	li	a7,222
    1dd0:	00000073          	ecall
    1dd4:	8082                	ret

0000000000001dd6 <munmap>:
    1dd6:	0d700893          	li	a7,215
    1dda:	00000073          	ecall
    1dde:	2501                	sext.w	a0,a0
    1de0:	8082                	ret

0000000000001de2 <wait>:
    1de2:	85aa                	mv	a1,a0
    1de4:	10400893          	li	a7,260
    1de8:	557d                	li	a0,-1
    1dea:	4601                	li	a2,0
    1dec:	4681                	li	a3,0
    1dee:	00000073          	ecall
    1df2:	2501                	sext.w	a0,a0
    1df4:	8082                	ret

0000000000001df6 <spawn>:
    1df6:	19000893          	li	a7,400
    1dfa:	00000073          	ecall
    1dfe:	2501                	sext.w	a0,a0
    1e00:	8082                	ret

0000000000001e02 <mailread>:
    1e02:	19100893          	li	a7,401
    1e06:	00000073          	ecall
    1e0a:	2501                	sext.w	a0,a0
    1e0c:	8082                	ret

0000000000001e0e <mailwrite>:
    1e0e:	19200893          	li	a7,402
    1e12:	00000073          	ecall
    1e16:	2501                	sext.w	a0,a0
    1e18:	8082                	ret

0000000000001e1a <fstat>:
    1e1a:	05000893          	li	a7,80
    1e1e:	00000073          	ecall
    1e22:	2501                	sext.w	a0,a0
    1e24:	8082                	ret

0000000000001e26 <sys_linkat>:
    1e26:	1702                	sll	a4,a4,0x20
    1e28:	02500893          	li	a7,37
    1e2c:	9301                	srl	a4,a4,0x20
    1e2e:	00000073          	ecall
    1e32:	2501                	sext.w	a0,a0
    1e34:	8082                	ret

0000000000001e36 <sys_unlinkat>:
    1e36:	1602                	sll	a2,a2,0x20
    1e38:	02300893          	li	a7,35
    1e3c:	9201                	srl	a2,a2,0x20
    1e3e:	00000073          	ecall
    1e42:	2501                	sext.w	a0,a0
    1e44:	8082                	ret

0000000000001e46 <link>:
    1e46:	87aa                	mv	a5,a0
    1e48:	86ae                	mv	a3,a1
    1e4a:	02500893          	li	a7,37
    1e4e:	f9c00513          	li	a0,-100
    1e52:	85be                	mv	a1,a5
    1e54:	f9c00613          	li	a2,-100
    1e58:	4701                	li	a4,0
    1e5a:	00000073          	ecall
    1e5e:	2501                	sext.w	a0,a0
    1e60:	8082                	ret

0000000000001e62 <unlink>:
    1e62:	85aa                	mv	a1,a0
    1e64:	02300893          	li	a7,35
    1e68:	f9c00513          	li	a0,-100
    1e6c:	4601                	li	a2,0
    1e6e:	00000073          	ecall
    1e72:	2501                	sext.w	a0,a0
    1e74:	8082                	ret

0000000000001e76 <uname>:
    1e76:	0a000893          	li	a7,160
    1e7a:	00000073          	ecall
    1e7e:	2501                	sext.w	a0,a0
    1e80:	8082                	ret

0000000000001e82 <brk>:
    1e82:	0d600893          	li	a7,214
    1e86:	00000073          	ecall
    1e8a:	2501                	sext.w	a0,a0
    1e8c:	8082                	ret

0000000000001e8e <getcwd>:
    1e8e:	48c5                	li	a7,17
    1e90:	00000073          	ecall
    1e94:	8082                	ret

0000000000001e96 <chdir>:
    1e96:	03100893          	li	a7,49
    1e9a:	00000073          	ecall
    1e9e:	2501                	sext.w	a0,a0
    1ea0:	8082                	ret

0000000000001ea2 <mkdir>:
    1ea2:	862e                	mv	a2,a1
    1ea4:	87aa                	mv	a5,a0
    1ea6:	1602                	sll	a2,a2,0x20
    1ea8:	02200893          	li	a7,34
    1eac:	f9c00513          	li	a0,-100
    1eb0:	85be                	mv	a1,a5
    1eb2:	9201                	srl	a2,a2,0x20
    1eb4:	00000073          	ecall
    1eb8:	2501                	sext.w	a0,a0
    1eba:	8082                	ret

0000000000001ebc <getdents>:
    1ebc:	03d00893          	li	a7,61
    1ec0:	00000073          	ecall
    1ec4:	2501                	sext.w	a0,a0
    1ec6:	8082                	ret

0000000000001ec8 <pipe>:
    1ec8:	03b00893          	li	a7,59
    1ecc:	4581                	li	a1,0
    1ece:	00000073          	ecall
    1ed2:	2501                	sext.w	a0,a0
    1ed4:	8082                	ret

0000000000001ed6 <dup>:
    1ed6:	48dd                	li	a7,23
    1ed8:	00000073          	ecall
    1edc:	2501                	sext.w	a0,a0
    1ede:	8082                	ret

0000000000001ee0 <dup2>:
    1ee0:	48e1                	li	a7,24
    1ee2:	4601                	li	a2,0
    1ee4:	00000073          	ecall
    1ee8:	2501                	sext.w	a0,a0
    1eea:	8082                	ret

0000000000001eec <mount>:
    1eec:	02800893          	li	a7,40
    1ef0:	00000073          	ecall
    1ef4:	2501                	sext.w	a0,a0
    1ef6:	8082                	ret

0000000000001ef8 <umount>:
    1ef8:	02700893          	li	a7,39
    1efc:	4581                	li	a1,0
    1efe:	00000073          	ecall
    1f02:	2501                	sext.w	a0,a0
    1f04:	8082                	ret

0000000000001f06 <__clone>:
    1f06:	15c1                	add	a1,a1,-16
    1f08:	e188                	sd	a0,0(a1)
    1f0a:	e594                	sd	a3,8(a1)
    1f0c:	8532                	mv	a0,a2
    1f0e:	863a                	mv	a2,a4
    1f10:	86be                	mv	a3,a5
    1f12:	8742                	mv	a4,a6
    1f14:	0dc00893          	li	a7,220
    1f18:	00000073          	ecall
    1f1c:	c111                	beqz	a0,1f20 <__clone+0x1a>
    1f1e:	8082                	ret
    1f20:	6582                	ld	a1,0(sp)
    1f22:	6522                	ld	a0,8(sp)
    1f24:	9582                	jalr	a1
    1f26:	05d00893          	li	a7,93
    1f2a:	00000073          	ecall
