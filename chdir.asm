
chdir:     file format elf64-littleriscv


Disassembly of section .text:

0000000000001000 <_start>:
    1000:	850a                	mv	a0,sp
    1002:	a0c9                	j	10c4 <__start_main>

0000000000001004 <test_chdir>:
    1004:	1141                	add	sp,sp,-16
    1006:	00001517          	auipc	a0,0x1
    100a:	ed250513          	add	a0,a0,-302 # 1ed8 <__clone+0x2c>
    100e:	e406                	sd	ra,8(sp)
    1010:	e022                	sd	s0,0(sp)
    1012:	30a000ef          	jal	131c <puts>
    1016:	00001517          	auipc	a0,0x1
    101a:	f9a50513          	add	a0,a0,-102 # 1fb0 <__func__.0>
    101e:	2fe000ef          	jal	131c <puts>
    1022:	00001517          	auipc	a0,0x1
    1026:	ece50513          	add	a0,a0,-306 # 1ef0 <__clone+0x44>
    102a:	2f2000ef          	jal	131c <puts>
    102e:	1b600593          	li	a1,438
    1032:	00001517          	auipc	a0,0x1
    1036:	ece50513          	add	a0,a0,-306 # 1f00 <__clone+0x54>
    103a:	60f000ef          	jal	1e48 <mkdir>
    103e:	00001517          	auipc	a0,0x1
    1042:	ec250513          	add	a0,a0,-318 # 1f00 <__clone+0x54>
    1046:	5f7000ef          	jal	1e3c <chdir>
    104a:	842a                	mv	s0,a0
    104c:	85aa                	mv	a1,a0
    104e:	00001517          	auipc	a0,0x1
    1052:	ec250513          	add	a0,a0,-318 # 1f10 <__clone+0x64>
    1056:	2e8000ef          	jal	133e <printf>
    105a:	e431                	bnez	s0,10a6 <test_chdir+0xa2>
    105c:	45f9                	li	a1,30
    105e:	00001517          	auipc	a0,0x1
    1062:	f3250513          	add	a0,a0,-206 # 1f90 <buffer>
    1066:	5cf000ef          	jal	1e34 <getcwd>
    106a:	00001597          	auipc	a1,0x1
    106e:	f2658593          	add	a1,a1,-218 # 1f90 <buffer>
    1072:	00001517          	auipc	a0,0x1
    1076:	ece50513          	add	a0,a0,-306 # 1f40 <__clone+0x94>
    107a:	2c4000ef          	jal	133e <printf>
    107e:	00001517          	auipc	a0,0x1
    1082:	ee250513          	add	a0,a0,-286 # 1f60 <__clone+0xb4>
    1086:	296000ef          	jal	131c <puts>
    108a:	00001517          	auipc	a0,0x1
    108e:	f2650513          	add	a0,a0,-218 # 1fb0 <__func__.0>
    1092:	28a000ef          	jal	131c <puts>
    1096:	6402                	ld	s0,0(sp)
    1098:	60a2                	ld	ra,8(sp)
    109a:	00001517          	auipc	a0,0x1
    109e:	e5650513          	add	a0,a0,-426 # 1ef0 <__clone+0x44>
    10a2:	0141                	add	sp,sp,16
    10a4:	aca5                	j	131c <puts>
    10a6:	00001517          	auipc	a0,0x1
    10aa:	e7a50513          	add	a0,a0,-390 # 1f20 <__clone+0x74>
    10ae:	50a000ef          	jal	15b8 <panic>
    10b2:	b76d                	j	105c <test_chdir+0x58>

00000000000010b4 <main>:
    10b4:	1141                	add	sp,sp,-16
    10b6:	e406                	sd	ra,8(sp)
    10b8:	f4dff0ef          	jal	1004 <test_chdir>
    10bc:	60a2                	ld	ra,8(sp)
    10be:	4501                	li	a0,0
    10c0:	0141                	add	sp,sp,16
    10c2:	8082                	ret

00000000000010c4 <__start_main>:
    10c4:	85aa                	mv	a1,a0
    10c6:	4108                	lw	a0,0(a0)
    10c8:	1141                	add	sp,sp,-16
    10ca:	05a1                	add	a1,a1,8
    10cc:	e406                	sd	ra,8(sp)
    10ce:	fe7ff0ef          	jal	10b4 <main>
    10d2:	3f3000ef          	jal	1cc4 <exit>
    10d6:	60a2                	ld	ra,8(sp)
    10d8:	4501                	li	a0,0
    10da:	0141                	add	sp,sp,16
    10dc:	8082                	ret

00000000000010de <printint.constprop.0>:
    10de:	7179                	add	sp,sp,-48
    10e0:	f406                	sd	ra,40(sp)
    10e2:	12054863          	bltz	a0,1212 <printint.constprop.0+0x134>
    10e6:	02b577bb          	remuw	a5,a0,a1
    10ea:	00001697          	auipc	a3,0x1
    10ee:	ed668693          	add	a3,a3,-298 # 1fc0 <digits>
    10f2:	00010c23          	sb	zero,24(sp)
    10f6:	0005871b          	sext.w	a4,a1
    10fa:	1782                	sll	a5,a5,0x20
    10fc:	9381                	srl	a5,a5,0x20
    10fe:	97b6                	add	a5,a5,a3
    1100:	0007c783          	lbu	a5,0(a5)
    1104:	02b5583b          	divuw	a6,a0,a1
    1108:	00f10ba3          	sb	a5,23(sp)
    110c:	1ab56663          	bltu	a0,a1,12b8 <printint.constprop.0+0x1da>
    1110:	02e8763b          	remuw	a2,a6,a4
    1114:	1602                	sll	a2,a2,0x20
    1116:	9201                	srl	a2,a2,0x20
    1118:	9636                	add	a2,a2,a3
    111a:	00064603          	lbu	a2,0(a2)
    111e:	02e855bb          	divuw	a1,a6,a4
    1122:	00c10b23          	sb	a2,22(sp)
    1126:	12e86c63          	bltu	a6,a4,125e <printint.constprop.0+0x180>
    112a:	02e5f63b          	remuw	a2,a1,a4
    112e:	1602                	sll	a2,a2,0x20
    1130:	9201                	srl	a2,a2,0x20
    1132:	9636                	add	a2,a2,a3
    1134:	00064603          	lbu	a2,0(a2)
    1138:	02e5d83b          	divuw	a6,a1,a4
    113c:	00c10aa3          	sb	a2,21(sp)
    1140:	12e5e863          	bltu	a1,a4,1270 <printint.constprop.0+0x192>
    1144:	02e8763b          	remuw	a2,a6,a4
    1148:	1602                	sll	a2,a2,0x20
    114a:	9201                	srl	a2,a2,0x20
    114c:	9636                	add	a2,a2,a3
    114e:	00064603          	lbu	a2,0(a2)
    1152:	02e855bb          	divuw	a1,a6,a4
    1156:	00c10a23          	sb	a2,20(sp)
    115a:	12e86463          	bltu	a6,a4,1282 <printint.constprop.0+0x1a4>
    115e:	02e5f63b          	remuw	a2,a1,a4
    1162:	1602                	sll	a2,a2,0x20
    1164:	9201                	srl	a2,a2,0x20
    1166:	9636                	add	a2,a2,a3
    1168:	00064603          	lbu	a2,0(a2)
    116c:	02e5d83b          	divuw	a6,a1,a4
    1170:	00c109a3          	sb	a2,19(sp)
    1174:	12e5e063          	bltu	a1,a4,1294 <printint.constprop.0+0x1b6>
    1178:	02e8763b          	remuw	a2,a6,a4
    117c:	1602                	sll	a2,a2,0x20
    117e:	9201                	srl	a2,a2,0x20
    1180:	9636                	add	a2,a2,a3
    1182:	00064603          	lbu	a2,0(a2)
    1186:	02e855bb          	divuw	a1,a6,a4
    118a:	00c10923          	sb	a2,18(sp)
    118e:	0ae86f63          	bltu	a6,a4,124c <printint.constprop.0+0x16e>
    1192:	02e5f63b          	remuw	a2,a1,a4
    1196:	1602                	sll	a2,a2,0x20
    1198:	9201                	srl	a2,a2,0x20
    119a:	9636                	add	a2,a2,a3
    119c:	00064603          	lbu	a2,0(a2)
    11a0:	02e5d83b          	divuw	a6,a1,a4
    11a4:	00c108a3          	sb	a2,17(sp)
    11a8:	0ee5ef63          	bltu	a1,a4,12a6 <printint.constprop.0+0x1c8>
    11ac:	02e8763b          	remuw	a2,a6,a4
    11b0:	1602                	sll	a2,a2,0x20
    11b2:	9201                	srl	a2,a2,0x20
    11b4:	9636                	add	a2,a2,a3
    11b6:	00064603          	lbu	a2,0(a2)
    11ba:	02e855bb          	divuw	a1,a6,a4
    11be:	00c10823          	sb	a2,16(sp)
    11c2:	0ee86d63          	bltu	a6,a4,12bc <printint.constprop.0+0x1de>
    11c6:	02e5f63b          	remuw	a2,a1,a4
    11ca:	1602                	sll	a2,a2,0x20
    11cc:	9201                	srl	a2,a2,0x20
    11ce:	9636                	add	a2,a2,a3
    11d0:	00064603          	lbu	a2,0(a2)
    11d4:	02e5d7bb          	divuw	a5,a1,a4
    11d8:	00c107a3          	sb	a2,15(sp)
    11dc:	0ee5e963          	bltu	a1,a4,12ce <printint.constprop.0+0x1f0>
    11e0:	1782                	sll	a5,a5,0x20
    11e2:	9381                	srl	a5,a5,0x20
    11e4:	96be                	add	a3,a3,a5
    11e6:	0006c783          	lbu	a5,0(a3)
    11ea:	4599                	li	a1,6
    11ec:	00f10723          	sb	a5,14(sp)
    11f0:	00055763          	bgez	a0,11fe <printint.constprop.0+0x120>
    11f4:	02d00793          	li	a5,45
    11f8:	00f106a3          	sb	a5,13(sp)
    11fc:	4595                	li	a1,5
    11fe:	003c                	add	a5,sp,8
    1200:	4641                	li	a2,16
    1202:	9e0d                	subw	a2,a2,a1
    1204:	4505                	li	a0,1
    1206:	95be                	add	a1,a1,a5
    1208:	26d000ef          	jal	1c74 <write>
    120c:	70a2                	ld	ra,40(sp)
    120e:	6145                	add	sp,sp,48
    1210:	8082                	ret
    1212:	40a0063b          	negw	a2,a0
    1216:	02b677bb          	remuw	a5,a2,a1
    121a:	00001697          	auipc	a3,0x1
    121e:	da668693          	add	a3,a3,-602 # 1fc0 <digits>
    1222:	00010c23          	sb	zero,24(sp)
    1226:	0005871b          	sext.w	a4,a1
    122a:	1782                	sll	a5,a5,0x20
    122c:	9381                	srl	a5,a5,0x20
    122e:	97b6                	add	a5,a5,a3
    1230:	0007c783          	lbu	a5,0(a5)
    1234:	02b6583b          	divuw	a6,a2,a1
    1238:	00f10ba3          	sb	a5,23(sp)
    123c:	ecb67ae3          	bgeu	a2,a1,1110 <printint.constprop.0+0x32>
    1240:	02d00793          	li	a5,45
    1244:	00f10b23          	sb	a5,22(sp)
    1248:	45b9                	li	a1,14
    124a:	bf55                	j	11fe <printint.constprop.0+0x120>
    124c:	45a9                	li	a1,10
    124e:	fa0558e3          	bgez	a0,11fe <printint.constprop.0+0x120>
    1252:	02d00793          	li	a5,45
    1256:	00f108a3          	sb	a5,17(sp)
    125a:	45a5                	li	a1,9
    125c:	b74d                	j	11fe <printint.constprop.0+0x120>
    125e:	45b9                	li	a1,14
    1260:	f8055fe3          	bgez	a0,11fe <printint.constprop.0+0x120>
    1264:	02d00793          	li	a5,45
    1268:	00f10aa3          	sb	a5,21(sp)
    126c:	45b5                	li	a1,13
    126e:	bf41                	j	11fe <printint.constprop.0+0x120>
    1270:	45b5                	li	a1,13
    1272:	f80556e3          	bgez	a0,11fe <printint.constprop.0+0x120>
    1276:	02d00793          	li	a5,45
    127a:	00f10a23          	sb	a5,20(sp)
    127e:	45b1                	li	a1,12
    1280:	bfbd                	j	11fe <printint.constprop.0+0x120>
    1282:	45b1                	li	a1,12
    1284:	f6055de3          	bgez	a0,11fe <printint.constprop.0+0x120>
    1288:	02d00793          	li	a5,45
    128c:	00f109a3          	sb	a5,19(sp)
    1290:	45ad                	li	a1,11
    1292:	b7b5                	j	11fe <printint.constprop.0+0x120>
    1294:	45ad                	li	a1,11
    1296:	f60554e3          	bgez	a0,11fe <printint.constprop.0+0x120>
    129a:	02d00793          	li	a5,45
    129e:	00f10923          	sb	a5,18(sp)
    12a2:	45a9                	li	a1,10
    12a4:	bfa9                	j	11fe <printint.constprop.0+0x120>
    12a6:	45a5                	li	a1,9
    12a8:	f4055be3          	bgez	a0,11fe <printint.constprop.0+0x120>
    12ac:	02d00793          	li	a5,45
    12b0:	00f10823          	sb	a5,16(sp)
    12b4:	45a1                	li	a1,8
    12b6:	b7a1                	j	11fe <printint.constprop.0+0x120>
    12b8:	45bd                	li	a1,15
    12ba:	b791                	j	11fe <printint.constprop.0+0x120>
    12bc:	45a1                	li	a1,8
    12be:	f40550e3          	bgez	a0,11fe <printint.constprop.0+0x120>
    12c2:	02d00793          	li	a5,45
    12c6:	00f107a3          	sb	a5,15(sp)
    12ca:	459d                	li	a1,7
    12cc:	bf0d                	j	11fe <printint.constprop.0+0x120>
    12ce:	459d                	li	a1,7
    12d0:	f20557e3          	bgez	a0,11fe <printint.constprop.0+0x120>
    12d4:	02d00793          	li	a5,45
    12d8:	00f10723          	sb	a5,14(sp)
    12dc:	4599                	li	a1,6
    12de:	b705                	j	11fe <printint.constprop.0+0x120>

00000000000012e0 <getchar>:
    12e0:	1101                	add	sp,sp,-32
    12e2:	00f10593          	add	a1,sp,15
    12e6:	4605                	li	a2,1
    12e8:	4501                	li	a0,0
    12ea:	ec06                	sd	ra,24(sp)
    12ec:	000107a3          	sb	zero,15(sp)
    12f0:	17b000ef          	jal	1c6a <read>
    12f4:	60e2                	ld	ra,24(sp)
    12f6:	00f14503          	lbu	a0,15(sp)
    12fa:	6105                	add	sp,sp,32
    12fc:	8082                	ret

00000000000012fe <putchar>:
    12fe:	1101                	add	sp,sp,-32
    1300:	87aa                	mv	a5,a0
    1302:	00f10593          	add	a1,sp,15
    1306:	4605                	li	a2,1
    1308:	4505                	li	a0,1
    130a:	ec06                	sd	ra,24(sp)
    130c:	00f107a3          	sb	a5,15(sp)
    1310:	165000ef          	jal	1c74 <write>
    1314:	60e2                	ld	ra,24(sp)
    1316:	2501                	sext.w	a0,a0
    1318:	6105                	add	sp,sp,32
    131a:	8082                	ret

000000000000131c <puts>:
    131c:	1141                	add	sp,sp,-16
    131e:	e406                	sd	ra,8(sp)
    1320:	e022                	sd	s0,0(sp)
    1322:	842a                	mv	s0,a0
    1324:	574000ef          	jal	1898 <strlen>
    1328:	862a                	mv	a2,a0
    132a:	85a2                	mv	a1,s0
    132c:	4505                	li	a0,1
    132e:	147000ef          	jal	1c74 <write>
    1332:	60a2                	ld	ra,8(sp)
    1334:	6402                	ld	s0,0(sp)
    1336:	957d                	sra	a0,a0,0x3f
    1338:	2501                	sext.w	a0,a0
    133a:	0141                	add	sp,sp,16
    133c:	8082                	ret

000000000000133e <printf>:
    133e:	7171                	add	sp,sp,-176
    1340:	f85a                	sd	s6,48(sp)
    1342:	ed3e                	sd	a5,152(sp)
    1344:	7b61                	lui	s6,0xffff8
    1346:	18bc                	add	a5,sp,120
    1348:	e8ca                	sd	s2,80(sp)
    134a:	e4ce                	sd	s3,72(sp)
    134c:	e0d2                	sd	s4,64(sp)
    134e:	fc56                	sd	s5,56(sp)
    1350:	f486                	sd	ra,104(sp)
    1352:	f0a2                	sd	s0,96(sp)
    1354:	eca6                	sd	s1,88(sp)
    1356:	fcae                	sd	a1,120(sp)
    1358:	e132                	sd	a2,128(sp)
    135a:	e536                	sd	a3,136(sp)
    135c:	e93a                	sd	a4,144(sp)
    135e:	f142                	sd	a6,160(sp)
    1360:	f546                	sd	a7,168(sp)
    1362:	e03e                	sd	a5,0(sp)
    1364:	02500913          	li	s2,37
    1368:	07300a13          	li	s4,115
    136c:	07800a93          	li	s5,120
    1370:	830b4b13          	xor	s6,s6,-2000
    1374:	00001997          	auipc	s3,0x1
    1378:	c4c98993          	add	s3,s3,-948 # 1fc0 <digits>
    137c:	00054783          	lbu	a5,0(a0)
    1380:	16078a63          	beqz	a5,14f4 <printf+0x1b6>
    1384:	862a                	mv	a2,a0
    1386:	19278d63          	beq	a5,s2,1520 <printf+0x1e2>
    138a:	00164783          	lbu	a5,1(a2)
    138e:	0605                	add	a2,a2,1
    1390:	fbfd                	bnez	a5,1386 <printf+0x48>
    1392:	84b2                	mv	s1,a2
    1394:	40a6043b          	subw	s0,a2,a0
    1398:	85aa                	mv	a1,a0
    139a:	8622                	mv	a2,s0
    139c:	4505                	li	a0,1
    139e:	0d7000ef          	jal	1c74 <write>
    13a2:	1a041463          	bnez	s0,154a <printf+0x20c>
    13a6:	0014c783          	lbu	a5,1(s1)
    13aa:	14078563          	beqz	a5,14f4 <printf+0x1b6>
    13ae:	1b478063          	beq	a5,s4,154e <printf+0x210>
    13b2:	14fa6b63          	bltu	s4,a5,1508 <printf+0x1ca>
    13b6:	06400713          	li	a4,100
    13ba:	1ee78063          	beq	a5,a4,159a <printf+0x25c>
    13be:	07000713          	li	a4,112
    13c2:	1ae79963          	bne	a5,a4,1574 <printf+0x236>
    13c6:	6702                	ld	a4,0(sp)
    13c8:	01611423          	sh	s6,8(sp)
    13cc:	4649                	li	a2,18
    13ce:	631c                	ld	a5,0(a4)
    13d0:	0721                	add	a4,a4,8
    13d2:	e03a                	sd	a4,0(sp)
    13d4:	00479293          	sll	t0,a5,0x4
    13d8:	00879f93          	sll	t6,a5,0x8
    13dc:	00c79f13          	sll	t5,a5,0xc
    13e0:	01079e93          	sll	t4,a5,0x10
    13e4:	01479e13          	sll	t3,a5,0x14
    13e8:	01879313          	sll	t1,a5,0x18
    13ec:	01c79893          	sll	a7,a5,0x1c
    13f0:	02479813          	sll	a6,a5,0x24
    13f4:	02879513          	sll	a0,a5,0x28
    13f8:	02c79593          	sll	a1,a5,0x2c
    13fc:	03079693          	sll	a3,a5,0x30
    1400:	03479713          	sll	a4,a5,0x34
    1404:	03c7d413          	srl	s0,a5,0x3c
    1408:	01c7d39b          	srlw	t2,a5,0x1c
    140c:	03c2d293          	srl	t0,t0,0x3c
    1410:	03cfdf93          	srl	t6,t6,0x3c
    1414:	03cf5f13          	srl	t5,t5,0x3c
    1418:	03cede93          	srl	t4,t4,0x3c
    141c:	03ce5e13          	srl	t3,t3,0x3c
    1420:	03c35313          	srl	t1,t1,0x3c
    1424:	03c8d893          	srl	a7,a7,0x3c
    1428:	03c85813          	srl	a6,a6,0x3c
    142c:	9171                	srl	a0,a0,0x3c
    142e:	91f1                	srl	a1,a1,0x3c
    1430:	92f1                	srl	a3,a3,0x3c
    1432:	9371                	srl	a4,a4,0x3c
    1434:	96ce                	add	a3,a3,s3
    1436:	974e                	add	a4,a4,s3
    1438:	944e                	add	s0,s0,s3
    143a:	92ce                	add	t0,t0,s3
    143c:	9fce                	add	t6,t6,s3
    143e:	9f4e                	add	t5,t5,s3
    1440:	9ece                	add	t4,t4,s3
    1442:	9e4e                	add	t3,t3,s3
    1444:	934e                	add	t1,t1,s3
    1446:	98ce                	add	a7,a7,s3
    1448:	93ce                	add	t2,t2,s3
    144a:	984e                	add	a6,a6,s3
    144c:	954e                	add	a0,a0,s3
    144e:	95ce                	add	a1,a1,s3
    1450:	0006c083          	lbu	ra,0(a3)
    1454:	0002c283          	lbu	t0,0(t0)
    1458:	00074683          	lbu	a3,0(a4)
    145c:	000fcf83          	lbu	t6,0(t6)
    1460:	000f4f03          	lbu	t5,0(t5)
    1464:	000ece83          	lbu	t4,0(t4)
    1468:	000e4e03          	lbu	t3,0(t3)
    146c:	00034303          	lbu	t1,0(t1)
    1470:	0008c883          	lbu	a7,0(a7)
    1474:	0003c383          	lbu	t2,0(t2)
    1478:	00084803          	lbu	a6,0(a6)
    147c:	00054503          	lbu	a0,0(a0)
    1480:	0005c583          	lbu	a1,0(a1)
    1484:	00044403          	lbu	s0,0(s0)
    1488:	03879713          	sll	a4,a5,0x38
    148c:	9371                	srl	a4,a4,0x3c
    148e:	8bbd                	and	a5,a5,15
    1490:	974e                	add	a4,a4,s3
    1492:	97ce                	add	a5,a5,s3
    1494:	005105a3          	sb	t0,11(sp)
    1498:	01f10623          	sb	t6,12(sp)
    149c:	01e106a3          	sb	t5,13(sp)
    14a0:	01d10723          	sb	t4,14(sp)
    14a4:	01c107a3          	sb	t3,15(sp)
    14a8:	00610823          	sb	t1,16(sp)
    14ac:	011108a3          	sb	a7,17(sp)
    14b0:	00710923          	sb	t2,18(sp)
    14b4:	010109a3          	sb	a6,19(sp)
    14b8:	00a10a23          	sb	a0,20(sp)
    14bc:	00b10aa3          	sb	a1,21(sp)
    14c0:	00110b23          	sb	ra,22(sp)
    14c4:	00d10ba3          	sb	a3,23(sp)
    14c8:	00810523          	sb	s0,10(sp)
    14cc:	00074703          	lbu	a4,0(a4)
    14d0:	0007c783          	lbu	a5,0(a5)
    14d4:	002c                	add	a1,sp,8
    14d6:	4505                	li	a0,1
    14d8:	00e10c23          	sb	a4,24(sp)
    14dc:	00f10ca3          	sb	a5,25(sp)
    14e0:	00010d23          	sb	zero,26(sp)
    14e4:	790000ef          	jal	1c74 <write>
    14e8:	00248513          	add	a0,s1,2
    14ec:	00054783          	lbu	a5,0(a0)
    14f0:	e8079ae3          	bnez	a5,1384 <printf+0x46>
    14f4:	70a6                	ld	ra,104(sp)
    14f6:	7406                	ld	s0,96(sp)
    14f8:	64e6                	ld	s1,88(sp)
    14fa:	6946                	ld	s2,80(sp)
    14fc:	69a6                	ld	s3,72(sp)
    14fe:	6a06                	ld	s4,64(sp)
    1500:	7ae2                	ld	s5,56(sp)
    1502:	7b42                	ld	s6,48(sp)
    1504:	614d                	add	sp,sp,176
    1506:	8082                	ret
    1508:	07579663          	bne	a5,s5,1574 <printf+0x236>
    150c:	6782                	ld	a5,0(sp)
    150e:	45c1                	li	a1,16
    1510:	4388                	lw	a0,0(a5)
    1512:	07a1                	add	a5,a5,8
    1514:	e03e                	sd	a5,0(sp)
    1516:	bc9ff0ef          	jal	10de <printint.constprop.0>
    151a:	00248513          	add	a0,s1,2
    151e:	b7f9                	j	14ec <printf+0x1ae>
    1520:	84b2                	mv	s1,a2
    1522:	a039                	j	1530 <printf+0x1f2>
    1524:	0024c783          	lbu	a5,2(s1)
    1528:	0605                	add	a2,a2,1
    152a:	0489                	add	s1,s1,2
    152c:	e72794e3          	bne	a5,s2,1394 <printf+0x56>
    1530:	0014c783          	lbu	a5,1(s1)
    1534:	ff2788e3          	beq	a5,s2,1524 <printf+0x1e6>
    1538:	40a6043b          	subw	s0,a2,a0
    153c:	85aa                	mv	a1,a0
    153e:	8622                	mv	a2,s0
    1540:	4505                	li	a0,1
    1542:	732000ef          	jal	1c74 <write>
    1546:	e60400e3          	beqz	s0,13a6 <printf+0x68>
    154a:	8526                	mv	a0,s1
    154c:	bd05                	j	137c <printf+0x3e>
    154e:	6782                	ld	a5,0(sp)
    1550:	6380                	ld	s0,0(a5)
    1552:	07a1                	add	a5,a5,8
    1554:	e03e                	sd	a5,0(sp)
    1556:	cc21                	beqz	s0,15ae <printf+0x270>
    1558:	0c800593          	li	a1,200
    155c:	8522                	mv	a0,s0
    155e:	424000ef          	jal	1982 <strnlen>
    1562:	0005061b          	sext.w	a2,a0
    1566:	85a2                	mv	a1,s0
    1568:	4505                	li	a0,1
    156a:	70a000ef          	jal	1c74 <write>
    156e:	00248513          	add	a0,s1,2
    1572:	bfad                	j	14ec <printf+0x1ae>
    1574:	4605                	li	a2,1
    1576:	002c                	add	a1,sp,8
    1578:	4505                	li	a0,1
    157a:	01210423          	sb	s2,8(sp)
    157e:	6f6000ef          	jal	1c74 <write>
    1582:	0014c783          	lbu	a5,1(s1)
    1586:	4605                	li	a2,1
    1588:	002c                	add	a1,sp,8
    158a:	4505                	li	a0,1
    158c:	00f10423          	sb	a5,8(sp)
    1590:	6e4000ef          	jal	1c74 <write>
    1594:	00248513          	add	a0,s1,2
    1598:	bf91                	j	14ec <printf+0x1ae>
    159a:	6782                	ld	a5,0(sp)
    159c:	45a9                	li	a1,10
    159e:	4388                	lw	a0,0(a5)
    15a0:	07a1                	add	a5,a5,8
    15a2:	e03e                	sd	a5,0(sp)
    15a4:	b3bff0ef          	jal	10de <printint.constprop.0>
    15a8:	00248513          	add	a0,s1,2
    15ac:	b781                	j	14ec <printf+0x1ae>
    15ae:	00001417          	auipc	s0,0x1
    15b2:	9c240413          	add	s0,s0,-1598 # 1f70 <__clone+0xc4>
    15b6:	b74d                	j	1558 <printf+0x21a>

00000000000015b8 <panic>:
    15b8:	1141                	add	sp,sp,-16
    15ba:	e406                	sd	ra,8(sp)
    15bc:	d61ff0ef          	jal	131c <puts>
    15c0:	60a2                	ld	ra,8(sp)
    15c2:	f9c00513          	li	a0,-100
    15c6:	0141                	add	sp,sp,16
    15c8:	adf5                	j	1cc4 <exit>

00000000000015ca <isspace>:
    15ca:	02000793          	li	a5,32
    15ce:	00f50663          	beq	a0,a5,15da <isspace+0x10>
    15d2:	355d                	addw	a0,a0,-9
    15d4:	00553513          	sltiu	a0,a0,5
    15d8:	8082                	ret
    15da:	4505                	li	a0,1
    15dc:	8082                	ret

00000000000015de <isdigit>:
    15de:	fd05051b          	addw	a0,a0,-48
    15e2:	00a53513          	sltiu	a0,a0,10
    15e6:	8082                	ret

00000000000015e8 <atoi>:
    15e8:	02000693          	li	a3,32
    15ec:	4591                	li	a1,4
    15ee:	00054783          	lbu	a5,0(a0)
    15f2:	ff77871b          	addw	a4,a5,-9
    15f6:	04d78c63          	beq	a5,a3,164e <atoi+0x66>
    15fa:	0007861b          	sext.w	a2,a5
    15fe:	04e5f863          	bgeu	a1,a4,164e <atoi+0x66>
    1602:	02b00713          	li	a4,43
    1606:	04e78963          	beq	a5,a4,1658 <atoi+0x70>
    160a:	02d00713          	li	a4,45
    160e:	06e78263          	beq	a5,a4,1672 <atoi+0x8a>
    1612:	fd06069b          	addw	a3,a2,-48
    1616:	47a5                	li	a5,9
    1618:	872a                	mv	a4,a0
    161a:	4301                	li	t1,0
    161c:	04d7e963          	bltu	a5,a3,166e <atoi+0x86>
    1620:	4501                	li	a0,0
    1622:	48a5                	li	a7,9
    1624:	00174683          	lbu	a3,1(a4)
    1628:	0025179b          	sllw	a5,a0,0x2
    162c:	9fa9                	addw	a5,a5,a0
    162e:	fd06059b          	addw	a1,a2,-48
    1632:	0017979b          	sllw	a5,a5,0x1
    1636:	fd06881b          	addw	a6,a3,-48
    163a:	0705                	add	a4,a4,1
    163c:	40b7853b          	subw	a0,a5,a1
    1640:	0006861b          	sext.w	a2,a3
    1644:	ff08f0e3          	bgeu	a7,a6,1624 <atoi+0x3c>
    1648:	00030563          	beqz	t1,1652 <atoi+0x6a>
    164c:	8082                	ret
    164e:	0505                	add	a0,a0,1
    1650:	bf79                	j	15ee <atoi+0x6>
    1652:	40f5853b          	subw	a0,a1,a5
    1656:	8082                	ret
    1658:	00154603          	lbu	a2,1(a0)
    165c:	47a5                	li	a5,9
    165e:	00150713          	add	a4,a0,1
    1662:	fd06069b          	addw	a3,a2,-48
    1666:	4301                	li	t1,0
    1668:	2601                	sext.w	a2,a2
    166a:	fad7fbe3          	bgeu	a5,a3,1620 <atoi+0x38>
    166e:	4501                	li	a0,0
    1670:	8082                	ret
    1672:	00154603          	lbu	a2,1(a0)
    1676:	47a5                	li	a5,9
    1678:	00150713          	add	a4,a0,1
    167c:	fd06069b          	addw	a3,a2,-48
    1680:	2601                	sext.w	a2,a2
    1682:	fed7e6e3          	bltu	a5,a3,166e <atoi+0x86>
    1686:	4305                	li	t1,1
    1688:	bf61                	j	1620 <atoi+0x38>

000000000000168a <memset>:
    168a:	18060163          	beqz	a2,180c <memset+0x182>
    168e:	40a006b3          	neg	a3,a0
    1692:	0076f793          	and	a5,a3,7
    1696:	00778813          	add	a6,a5,7
    169a:	48ad                	li	a7,11
    169c:	0ff5f713          	zext.b	a4,a1
    16a0:	fff60593          	add	a1,a2,-1
    16a4:	17186563          	bltu	a6,a7,180e <memset+0x184>
    16a8:	1705ed63          	bltu	a1,a6,1822 <memset+0x198>
    16ac:	16078363          	beqz	a5,1812 <memset+0x188>
    16b0:	00e50023          	sb	a4,0(a0)
    16b4:	0066f593          	and	a1,a3,6
    16b8:	16058063          	beqz	a1,1818 <memset+0x18e>
    16bc:	00e500a3          	sb	a4,1(a0)
    16c0:	4589                	li	a1,2
    16c2:	16f5f363          	bgeu	a1,a5,1828 <memset+0x19e>
    16c6:	00e50123          	sb	a4,2(a0)
    16ca:	8a91                	and	a3,a3,4
    16cc:	00350593          	add	a1,a0,3
    16d0:	4e0d                	li	t3,3
    16d2:	ce9d                	beqz	a3,1710 <memset+0x86>
    16d4:	00e501a3          	sb	a4,3(a0)
    16d8:	4691                	li	a3,4
    16da:	00450593          	add	a1,a0,4
    16de:	4e11                	li	t3,4
    16e0:	02f6f863          	bgeu	a3,a5,1710 <memset+0x86>
    16e4:	00e50223          	sb	a4,4(a0)
    16e8:	4695                	li	a3,5
    16ea:	00550593          	add	a1,a0,5
    16ee:	4e15                	li	t3,5
    16f0:	02d78063          	beq	a5,a3,1710 <memset+0x86>
    16f4:	fff50693          	add	a3,a0,-1
    16f8:	00e502a3          	sb	a4,5(a0)
    16fc:	8a9d                	and	a3,a3,7
    16fe:	00650593          	add	a1,a0,6
    1702:	4e19                	li	t3,6
    1704:	e691                	bnez	a3,1710 <memset+0x86>
    1706:	00750593          	add	a1,a0,7
    170a:	00e50323          	sb	a4,6(a0)
    170e:	4e1d                	li	t3,7
    1710:	00871693          	sll	a3,a4,0x8
    1714:	01071813          	sll	a6,a4,0x10
    1718:	8ed9                	or	a3,a3,a4
    171a:	01871893          	sll	a7,a4,0x18
    171e:	0106e6b3          	or	a3,a3,a6
    1722:	0116e6b3          	or	a3,a3,a7
    1726:	02071813          	sll	a6,a4,0x20
    172a:	02871313          	sll	t1,a4,0x28
    172e:	0106e6b3          	or	a3,a3,a6
    1732:	40f608b3          	sub	a7,a2,a5
    1736:	03071813          	sll	a6,a4,0x30
    173a:	0066e6b3          	or	a3,a3,t1
    173e:	0106e6b3          	or	a3,a3,a6
    1742:	03871313          	sll	t1,a4,0x38
    1746:	97aa                	add	a5,a5,a0
    1748:	ff88f813          	and	a6,a7,-8
    174c:	0066e6b3          	or	a3,a3,t1
    1750:	983e                	add	a6,a6,a5
    1752:	e394                	sd	a3,0(a5)
    1754:	07a1                	add	a5,a5,8
    1756:	ff079ee3          	bne	a5,a6,1752 <memset+0xc8>
    175a:	ff88f793          	and	a5,a7,-8
    175e:	0078f893          	and	a7,a7,7
    1762:	00f586b3          	add	a3,a1,a5
    1766:	01c787bb          	addw	a5,a5,t3
    176a:	0a088b63          	beqz	a7,1820 <memset+0x196>
    176e:	00e68023          	sb	a4,0(a3)
    1772:	0017859b          	addw	a1,a5,1
    1776:	08c5fb63          	bgeu	a1,a2,180c <memset+0x182>
    177a:	00e680a3          	sb	a4,1(a3)
    177e:	0027859b          	addw	a1,a5,2
    1782:	08c5f563          	bgeu	a1,a2,180c <memset+0x182>
    1786:	00e68123          	sb	a4,2(a3)
    178a:	0037859b          	addw	a1,a5,3
    178e:	06c5ff63          	bgeu	a1,a2,180c <memset+0x182>
    1792:	00e681a3          	sb	a4,3(a3)
    1796:	0047859b          	addw	a1,a5,4
    179a:	06c5f963          	bgeu	a1,a2,180c <memset+0x182>
    179e:	00e68223          	sb	a4,4(a3)
    17a2:	0057859b          	addw	a1,a5,5
    17a6:	06c5f363          	bgeu	a1,a2,180c <memset+0x182>
    17aa:	00e682a3          	sb	a4,5(a3)
    17ae:	0067859b          	addw	a1,a5,6
    17b2:	04c5fd63          	bgeu	a1,a2,180c <memset+0x182>
    17b6:	00e68323          	sb	a4,6(a3)
    17ba:	0077859b          	addw	a1,a5,7
    17be:	04c5f763          	bgeu	a1,a2,180c <memset+0x182>
    17c2:	00e683a3          	sb	a4,7(a3)
    17c6:	0087859b          	addw	a1,a5,8
    17ca:	04c5f163          	bgeu	a1,a2,180c <memset+0x182>
    17ce:	00e68423          	sb	a4,8(a3)
    17d2:	0097859b          	addw	a1,a5,9
    17d6:	02c5fb63          	bgeu	a1,a2,180c <memset+0x182>
    17da:	00e684a3          	sb	a4,9(a3)
    17de:	00a7859b          	addw	a1,a5,10
    17e2:	02c5f563          	bgeu	a1,a2,180c <memset+0x182>
    17e6:	00e68523          	sb	a4,10(a3)
    17ea:	00b7859b          	addw	a1,a5,11
    17ee:	00c5ff63          	bgeu	a1,a2,180c <memset+0x182>
    17f2:	00e685a3          	sb	a4,11(a3)
    17f6:	00c7859b          	addw	a1,a5,12
    17fa:	00c5f963          	bgeu	a1,a2,180c <memset+0x182>
    17fe:	00e68623          	sb	a4,12(a3)
    1802:	27b5                	addw	a5,a5,13
    1804:	00c7f463          	bgeu	a5,a2,180c <memset+0x182>
    1808:	00e686a3          	sb	a4,13(a3)
    180c:	8082                	ret
    180e:	482d                	li	a6,11
    1810:	bd61                	j	16a8 <memset+0x1e>
    1812:	85aa                	mv	a1,a0
    1814:	4e01                	li	t3,0
    1816:	bded                	j	1710 <memset+0x86>
    1818:	00150593          	add	a1,a0,1
    181c:	4e05                	li	t3,1
    181e:	bdcd                	j	1710 <memset+0x86>
    1820:	8082                	ret
    1822:	86aa                	mv	a3,a0
    1824:	4781                	li	a5,0
    1826:	b7a1                	j	176e <memset+0xe4>
    1828:	00250593          	add	a1,a0,2
    182c:	4e09                	li	t3,2
    182e:	b5cd                	j	1710 <memset+0x86>

0000000000001830 <strcmp>:
    1830:	00054783          	lbu	a5,0(a0)
    1834:	0005c703          	lbu	a4,0(a1)
    1838:	00e79863          	bne	a5,a4,1848 <strcmp+0x18>
    183c:	0505                	add	a0,a0,1
    183e:	0585                	add	a1,a1,1
    1840:	fbe5                	bnez	a5,1830 <strcmp>
    1842:	4501                	li	a0,0
    1844:	9d19                	subw	a0,a0,a4
    1846:	8082                	ret
    1848:	0007851b          	sext.w	a0,a5
    184c:	bfe5                	j	1844 <strcmp+0x14>

000000000000184e <strncmp>:
    184e:	ca15                	beqz	a2,1882 <strncmp+0x34>
    1850:	00054783          	lbu	a5,0(a0)
    1854:	167d                	add	a2,a2,-1
    1856:	00c506b3          	add	a3,a0,a2
    185a:	eb99                	bnez	a5,1870 <strncmp+0x22>
    185c:	a815                	j	1890 <strncmp+0x42>
    185e:	00a68e63          	beq	a3,a0,187a <strncmp+0x2c>
    1862:	0505                	add	a0,a0,1
    1864:	00f71b63          	bne	a4,a5,187a <strncmp+0x2c>
    1868:	00054783          	lbu	a5,0(a0)
    186c:	cf89                	beqz	a5,1886 <strncmp+0x38>
    186e:	85b2                	mv	a1,a2
    1870:	0005c703          	lbu	a4,0(a1)
    1874:	00158613          	add	a2,a1,1
    1878:	f37d                	bnez	a4,185e <strncmp+0x10>
    187a:	0007851b          	sext.w	a0,a5
    187e:	9d19                	subw	a0,a0,a4
    1880:	8082                	ret
    1882:	4501                	li	a0,0
    1884:	8082                	ret
    1886:	0015c703          	lbu	a4,1(a1)
    188a:	4501                	li	a0,0
    188c:	9d19                	subw	a0,a0,a4
    188e:	8082                	ret
    1890:	0005c703          	lbu	a4,0(a1)
    1894:	4501                	li	a0,0
    1896:	b7e5                	j	187e <strncmp+0x30>

0000000000001898 <strlen>:
    1898:	00757793          	and	a5,a0,7
    189c:	cf89                	beqz	a5,18b6 <strlen+0x1e>
    189e:	87aa                	mv	a5,a0
    18a0:	a029                	j	18aa <strlen+0x12>
    18a2:	0785                	add	a5,a5,1
    18a4:	0077f713          	and	a4,a5,7
    18a8:	cb01                	beqz	a4,18b8 <strlen+0x20>
    18aa:	0007c703          	lbu	a4,0(a5)
    18ae:	fb75                	bnez	a4,18a2 <strlen+0xa>
    18b0:	40a78533          	sub	a0,a5,a0
    18b4:	8082                	ret
    18b6:	87aa                	mv	a5,a0
    18b8:	6394                	ld	a3,0(a5)
    18ba:	00000597          	auipc	a1,0x0
    18be:	6be5b583          	ld	a1,1726(a1) # 1f78 <__clone+0xcc>
    18c2:	00000617          	auipc	a2,0x0
    18c6:	6be63603          	ld	a2,1726(a2) # 1f80 <__clone+0xd4>
    18ca:	a019                	j	18d0 <strlen+0x38>
    18cc:	6794                	ld	a3,8(a5)
    18ce:	07a1                	add	a5,a5,8
    18d0:	00b68733          	add	a4,a3,a1
    18d4:	fff6c693          	not	a3,a3
    18d8:	8f75                	and	a4,a4,a3
    18da:	8f71                	and	a4,a4,a2
    18dc:	db65                	beqz	a4,18cc <strlen+0x34>
    18de:	0007c703          	lbu	a4,0(a5)
    18e2:	d779                	beqz	a4,18b0 <strlen+0x18>
    18e4:	0017c703          	lbu	a4,1(a5)
    18e8:	0785                	add	a5,a5,1
    18ea:	d379                	beqz	a4,18b0 <strlen+0x18>
    18ec:	0017c703          	lbu	a4,1(a5)
    18f0:	0785                	add	a5,a5,1
    18f2:	fb6d                	bnez	a4,18e4 <strlen+0x4c>
    18f4:	bf75                	j	18b0 <strlen+0x18>

00000000000018f6 <memchr>:
    18f6:	00757713          	and	a4,a0,7
    18fa:	87aa                	mv	a5,a0
    18fc:	0ff5f593          	zext.b	a1,a1
    1900:	cb19                	beqz	a4,1916 <memchr+0x20>
    1902:	ce25                	beqz	a2,197a <memchr+0x84>
    1904:	0007c703          	lbu	a4,0(a5)
    1908:	00b70763          	beq	a4,a1,1916 <memchr+0x20>
    190c:	0785                	add	a5,a5,1
    190e:	0077f713          	and	a4,a5,7
    1912:	167d                	add	a2,a2,-1
    1914:	f77d                	bnez	a4,1902 <memchr+0xc>
    1916:	4501                	li	a0,0
    1918:	c235                	beqz	a2,197c <memchr+0x86>
    191a:	0007c703          	lbu	a4,0(a5)
    191e:	06b70063          	beq	a4,a1,197e <memchr+0x88>
    1922:	00000517          	auipc	a0,0x0
    1926:	66653503          	ld	a0,1638(a0) # 1f88 <__clone+0xdc>
    192a:	471d                	li	a4,7
    192c:	02a58533          	mul	a0,a1,a0
    1930:	04c77763          	bgeu	a4,a2,197e <memchr+0x88>
    1934:	00000897          	auipc	a7,0x0
    1938:	6448b883          	ld	a7,1604(a7) # 1f78 <__clone+0xcc>
    193c:	00000817          	auipc	a6,0x0
    1940:	64483803          	ld	a6,1604(a6) # 1f80 <__clone+0xd4>
    1944:	431d                	li	t1,7
    1946:	a029                	j	1950 <memchr+0x5a>
    1948:	1661                	add	a2,a2,-8
    194a:	07a1                	add	a5,a5,8
    194c:	00c37c63          	bgeu	t1,a2,1964 <memchr+0x6e>
    1950:	6398                	ld	a4,0(a5)
    1952:	8f29                	xor	a4,a4,a0
    1954:	011706b3          	add	a3,a4,a7
    1958:	fff74713          	not	a4,a4
    195c:	8f75                	and	a4,a4,a3
    195e:	01077733          	and	a4,a4,a6
    1962:	d37d                	beqz	a4,1948 <memchr+0x52>
    1964:	853e                	mv	a0,a5
    1966:	e601                	bnez	a2,196e <memchr+0x78>
    1968:	a809                	j	197a <memchr+0x84>
    196a:	0505                	add	a0,a0,1
    196c:	c619                	beqz	a2,197a <memchr+0x84>
    196e:	00054783          	lbu	a5,0(a0)
    1972:	167d                	add	a2,a2,-1
    1974:	feb79be3          	bne	a5,a1,196a <memchr+0x74>
    1978:	8082                	ret
    197a:	4501                	li	a0,0
    197c:	8082                	ret
    197e:	853e                	mv	a0,a5
    1980:	b7fd                	j	196e <memchr+0x78>

0000000000001982 <strnlen>:
    1982:	1101                	add	sp,sp,-32
    1984:	e822                	sd	s0,16(sp)
    1986:	862e                	mv	a2,a1
    1988:	842e                	mv	s0,a1
    198a:	4581                	li	a1,0
    198c:	e426                	sd	s1,8(sp)
    198e:	ec06                	sd	ra,24(sp)
    1990:	84aa                	mv	s1,a0
    1992:	f65ff0ef          	jal	18f6 <memchr>
    1996:	c519                	beqz	a0,19a4 <strnlen+0x22>
    1998:	60e2                	ld	ra,24(sp)
    199a:	6442                	ld	s0,16(sp)
    199c:	8d05                	sub	a0,a0,s1
    199e:	64a2                	ld	s1,8(sp)
    19a0:	6105                	add	sp,sp,32
    19a2:	8082                	ret
    19a4:	60e2                	ld	ra,24(sp)
    19a6:	8522                	mv	a0,s0
    19a8:	6442                	ld	s0,16(sp)
    19aa:	64a2                	ld	s1,8(sp)
    19ac:	6105                	add	sp,sp,32
    19ae:	8082                	ret

00000000000019b0 <strcpy>:
    19b0:	00a5c7b3          	xor	a5,a1,a0
    19b4:	8b9d                	and	a5,a5,7
    19b6:	eb95                	bnez	a5,19ea <strcpy+0x3a>
    19b8:	0075f793          	and	a5,a1,7
    19bc:	e7b1                	bnez	a5,1a08 <strcpy+0x58>
    19be:	6198                	ld	a4,0(a1)
    19c0:	00000617          	auipc	a2,0x0
    19c4:	5b863603          	ld	a2,1464(a2) # 1f78 <__clone+0xcc>
    19c8:	00000817          	auipc	a6,0x0
    19cc:	5b883803          	ld	a6,1464(a6) # 1f80 <__clone+0xd4>
    19d0:	a029                	j	19da <strcpy+0x2a>
    19d2:	05a1                	add	a1,a1,8
    19d4:	e118                	sd	a4,0(a0)
    19d6:	6198                	ld	a4,0(a1)
    19d8:	0521                	add	a0,a0,8
    19da:	00c707b3          	add	a5,a4,a2
    19de:	fff74693          	not	a3,a4
    19e2:	8ff5                	and	a5,a5,a3
    19e4:	0107f7b3          	and	a5,a5,a6
    19e8:	d7ed                	beqz	a5,19d2 <strcpy+0x22>
    19ea:	0005c783          	lbu	a5,0(a1)
    19ee:	00f50023          	sb	a5,0(a0)
    19f2:	c785                	beqz	a5,1a1a <strcpy+0x6a>
    19f4:	0015c783          	lbu	a5,1(a1)
    19f8:	0505                	add	a0,a0,1
    19fa:	0585                	add	a1,a1,1
    19fc:	00f50023          	sb	a5,0(a0)
    1a00:	fbf5                	bnez	a5,19f4 <strcpy+0x44>
    1a02:	8082                	ret
    1a04:	0505                	add	a0,a0,1
    1a06:	df45                	beqz	a4,19be <strcpy+0xe>
    1a08:	0005c783          	lbu	a5,0(a1)
    1a0c:	0585                	add	a1,a1,1
    1a0e:	0075f713          	and	a4,a1,7
    1a12:	00f50023          	sb	a5,0(a0)
    1a16:	f7fd                	bnez	a5,1a04 <strcpy+0x54>
    1a18:	8082                	ret
    1a1a:	8082                	ret

0000000000001a1c <strncpy>:
    1a1c:	00a5c7b3          	xor	a5,a1,a0
    1a20:	8b9d                	and	a5,a5,7
    1a22:	e3b5                	bnez	a5,1a86 <strncpy+0x6a>
    1a24:	0075f793          	and	a5,a1,7
    1a28:	cf99                	beqz	a5,1a46 <strncpy+0x2a>
    1a2a:	ea09                	bnez	a2,1a3c <strncpy+0x20>
    1a2c:	a421                	j	1c34 <strncpy+0x218>
    1a2e:	0585                	add	a1,a1,1
    1a30:	0075f793          	and	a5,a1,7
    1a34:	167d                	add	a2,a2,-1
    1a36:	0505                	add	a0,a0,1
    1a38:	c799                	beqz	a5,1a46 <strncpy+0x2a>
    1a3a:	c225                	beqz	a2,1a9a <strncpy+0x7e>
    1a3c:	0005c783          	lbu	a5,0(a1)
    1a40:	00f50023          	sb	a5,0(a0)
    1a44:	f7ed                	bnez	a5,1a2e <strncpy+0x12>
    1a46:	ca31                	beqz	a2,1a9a <strncpy+0x7e>
    1a48:	0005c783          	lbu	a5,0(a1)
    1a4c:	cba1                	beqz	a5,1a9c <strncpy+0x80>
    1a4e:	479d                	li	a5,7
    1a50:	02c7fc63          	bgeu	a5,a2,1a88 <strncpy+0x6c>
    1a54:	00000897          	auipc	a7,0x0
    1a58:	5248b883          	ld	a7,1316(a7) # 1f78 <__clone+0xcc>
    1a5c:	00000817          	auipc	a6,0x0
    1a60:	52483803          	ld	a6,1316(a6) # 1f80 <__clone+0xd4>
    1a64:	431d                	li	t1,7
    1a66:	a039                	j	1a74 <strncpy+0x58>
    1a68:	e118                	sd	a4,0(a0)
    1a6a:	1661                	add	a2,a2,-8
    1a6c:	05a1                	add	a1,a1,8
    1a6e:	0521                	add	a0,a0,8
    1a70:	00c37b63          	bgeu	t1,a2,1a86 <strncpy+0x6a>
    1a74:	6198                	ld	a4,0(a1)
    1a76:	011707b3          	add	a5,a4,a7
    1a7a:	fff74693          	not	a3,a4
    1a7e:	8ff5                	and	a5,a5,a3
    1a80:	0107f7b3          	and	a5,a5,a6
    1a84:	d3f5                	beqz	a5,1a68 <strncpy+0x4c>
    1a86:	ca11                	beqz	a2,1a9a <strncpy+0x7e>
    1a88:	0005c783          	lbu	a5,0(a1)
    1a8c:	0585                	add	a1,a1,1
    1a8e:	00f50023          	sb	a5,0(a0)
    1a92:	c789                	beqz	a5,1a9c <strncpy+0x80>
    1a94:	167d                	add	a2,a2,-1
    1a96:	0505                	add	a0,a0,1
    1a98:	fa65                	bnez	a2,1a88 <strncpy+0x6c>
    1a9a:	8082                	ret
    1a9c:	4805                	li	a6,1
    1a9e:	14061b63          	bnez	a2,1bf4 <strncpy+0x1d8>
    1aa2:	40a00733          	neg	a4,a0
    1aa6:	00777793          	and	a5,a4,7
    1aaa:	4581                	li	a1,0
    1aac:	12061c63          	bnez	a2,1be4 <strncpy+0x1c8>
    1ab0:	00778693          	add	a3,a5,7
    1ab4:	48ad                	li	a7,11
    1ab6:	1316e563          	bltu	a3,a7,1be0 <strncpy+0x1c4>
    1aba:	16d5e263          	bltu	a1,a3,1c1e <strncpy+0x202>
    1abe:	14078c63          	beqz	a5,1c16 <strncpy+0x1fa>
    1ac2:	00050023          	sb	zero,0(a0)
    1ac6:	00677693          	and	a3,a4,6
    1aca:	14068263          	beqz	a3,1c0e <strncpy+0x1f2>
    1ace:	000500a3          	sb	zero,1(a0)
    1ad2:	4689                	li	a3,2
    1ad4:	14f6f863          	bgeu	a3,a5,1c24 <strncpy+0x208>
    1ad8:	00050123          	sb	zero,2(a0)
    1adc:	8b11                	and	a4,a4,4
    1ade:	12070463          	beqz	a4,1c06 <strncpy+0x1ea>
    1ae2:	000501a3          	sb	zero,3(a0)
    1ae6:	4711                	li	a4,4
    1ae8:	00450693          	add	a3,a0,4
    1aec:	02f77563          	bgeu	a4,a5,1b16 <strncpy+0xfa>
    1af0:	00050223          	sb	zero,4(a0)
    1af4:	4715                	li	a4,5
    1af6:	00550693          	add	a3,a0,5
    1afa:	00e78e63          	beq	a5,a4,1b16 <strncpy+0xfa>
    1afe:	fff50713          	add	a4,a0,-1
    1b02:	000502a3          	sb	zero,5(a0)
    1b06:	8b1d                	and	a4,a4,7
    1b08:	12071263          	bnez	a4,1c2c <strncpy+0x210>
    1b0c:	00750693          	add	a3,a0,7
    1b10:	00050323          	sb	zero,6(a0)
    1b14:	471d                	li	a4,7
    1b16:	40f80833          	sub	a6,a6,a5
    1b1a:	ff887593          	and	a1,a6,-8
    1b1e:	97aa                	add	a5,a5,a0
    1b20:	95be                	add	a1,a1,a5
    1b22:	0007b023          	sd	zero,0(a5)
    1b26:	07a1                	add	a5,a5,8
    1b28:	feb79de3          	bne	a5,a1,1b22 <strncpy+0x106>
    1b2c:	ff887593          	and	a1,a6,-8
    1b30:	00787813          	and	a6,a6,7
    1b34:	00e587bb          	addw	a5,a1,a4
    1b38:	00b68733          	add	a4,a3,a1
    1b3c:	0e080063          	beqz	a6,1c1c <strncpy+0x200>
    1b40:	00070023          	sb	zero,0(a4)
    1b44:	0017869b          	addw	a3,a5,1
    1b48:	f4c6f9e3          	bgeu	a3,a2,1a9a <strncpy+0x7e>
    1b4c:	000700a3          	sb	zero,1(a4)
    1b50:	0027869b          	addw	a3,a5,2
    1b54:	f4c6f3e3          	bgeu	a3,a2,1a9a <strncpy+0x7e>
    1b58:	00070123          	sb	zero,2(a4)
    1b5c:	0037869b          	addw	a3,a5,3
    1b60:	f2c6fde3          	bgeu	a3,a2,1a9a <strncpy+0x7e>
    1b64:	000701a3          	sb	zero,3(a4)
    1b68:	0047869b          	addw	a3,a5,4
    1b6c:	f2c6f7e3          	bgeu	a3,a2,1a9a <strncpy+0x7e>
    1b70:	00070223          	sb	zero,4(a4)
    1b74:	0057869b          	addw	a3,a5,5
    1b78:	f2c6f1e3          	bgeu	a3,a2,1a9a <strncpy+0x7e>
    1b7c:	000702a3          	sb	zero,5(a4)
    1b80:	0067869b          	addw	a3,a5,6
    1b84:	f0c6fbe3          	bgeu	a3,a2,1a9a <strncpy+0x7e>
    1b88:	00070323          	sb	zero,6(a4)
    1b8c:	0077869b          	addw	a3,a5,7
    1b90:	f0c6f5e3          	bgeu	a3,a2,1a9a <strncpy+0x7e>
    1b94:	000703a3          	sb	zero,7(a4)
    1b98:	0087869b          	addw	a3,a5,8
    1b9c:	eec6ffe3          	bgeu	a3,a2,1a9a <strncpy+0x7e>
    1ba0:	00070423          	sb	zero,8(a4)
    1ba4:	0097869b          	addw	a3,a5,9
    1ba8:	eec6f9e3          	bgeu	a3,a2,1a9a <strncpy+0x7e>
    1bac:	000704a3          	sb	zero,9(a4)
    1bb0:	00a7869b          	addw	a3,a5,10
    1bb4:	eec6f3e3          	bgeu	a3,a2,1a9a <strncpy+0x7e>
    1bb8:	00070523          	sb	zero,10(a4)
    1bbc:	00b7869b          	addw	a3,a5,11
    1bc0:	ecc6fde3          	bgeu	a3,a2,1a9a <strncpy+0x7e>
    1bc4:	000705a3          	sb	zero,11(a4)
    1bc8:	00c7869b          	addw	a3,a5,12
    1bcc:	ecc6f7e3          	bgeu	a3,a2,1a9a <strncpy+0x7e>
    1bd0:	00070623          	sb	zero,12(a4)
    1bd4:	27b5                	addw	a5,a5,13
    1bd6:	ecc7f2e3          	bgeu	a5,a2,1a9a <strncpy+0x7e>
    1bda:	000706a3          	sb	zero,13(a4)
    1bde:	8082                	ret
    1be0:	46ad                	li	a3,11
    1be2:	bde1                	j	1aba <strncpy+0x9e>
    1be4:	00778693          	add	a3,a5,7
    1be8:	48ad                	li	a7,11
    1bea:	fff60593          	add	a1,a2,-1
    1bee:	ed16f6e3          	bgeu	a3,a7,1aba <strncpy+0x9e>
    1bf2:	b7fd                	j	1be0 <strncpy+0x1c4>
    1bf4:	40a00733          	neg	a4,a0
    1bf8:	8832                	mv	a6,a2
    1bfa:	00777793          	and	a5,a4,7
    1bfe:	4581                	li	a1,0
    1c00:	ea0608e3          	beqz	a2,1ab0 <strncpy+0x94>
    1c04:	b7c5                	j	1be4 <strncpy+0x1c8>
    1c06:	00350693          	add	a3,a0,3
    1c0a:	470d                	li	a4,3
    1c0c:	b729                	j	1b16 <strncpy+0xfa>
    1c0e:	00150693          	add	a3,a0,1
    1c12:	4705                	li	a4,1
    1c14:	b709                	j	1b16 <strncpy+0xfa>
    1c16:	86aa                	mv	a3,a0
    1c18:	4701                	li	a4,0
    1c1a:	bdf5                	j	1b16 <strncpy+0xfa>
    1c1c:	8082                	ret
    1c1e:	872a                	mv	a4,a0
    1c20:	4781                	li	a5,0
    1c22:	bf39                	j	1b40 <strncpy+0x124>
    1c24:	00250693          	add	a3,a0,2
    1c28:	4709                	li	a4,2
    1c2a:	b5f5                	j	1b16 <strncpy+0xfa>
    1c2c:	00650693          	add	a3,a0,6
    1c30:	4719                	li	a4,6
    1c32:	b5d5                	j	1b16 <strncpy+0xfa>
    1c34:	8082                	ret

0000000000001c36 <open>:
    1c36:	87aa                	mv	a5,a0
    1c38:	862e                	mv	a2,a1
    1c3a:	03800893          	li	a7,56
    1c3e:	f9c00513          	li	a0,-100
    1c42:	85be                	mv	a1,a5
    1c44:	4689                	li	a3,2
    1c46:	00000073          	ecall
    1c4a:	2501                	sext.w	a0,a0
    1c4c:	8082                	ret

0000000000001c4e <openat>:
    1c4e:	03800893          	li	a7,56
    1c52:	18000693          	li	a3,384
    1c56:	00000073          	ecall
    1c5a:	2501                	sext.w	a0,a0
    1c5c:	8082                	ret

0000000000001c5e <close>:
    1c5e:	03900893          	li	a7,57
    1c62:	00000073          	ecall
    1c66:	2501                	sext.w	a0,a0
    1c68:	8082                	ret

0000000000001c6a <read>:
    1c6a:	03f00893          	li	a7,63
    1c6e:	00000073          	ecall
    1c72:	8082                	ret

0000000000001c74 <write>:
    1c74:	04000893          	li	a7,64
    1c78:	00000073          	ecall
    1c7c:	8082                	ret

0000000000001c7e <getpid>:
    1c7e:	0ac00893          	li	a7,172
    1c82:	00000073          	ecall
    1c86:	2501                	sext.w	a0,a0
    1c88:	8082                	ret

0000000000001c8a <getppid>:
    1c8a:	0ad00893          	li	a7,173
    1c8e:	00000073          	ecall
    1c92:	2501                	sext.w	a0,a0
    1c94:	8082                	ret

0000000000001c96 <sched_yield>:
    1c96:	07c00893          	li	a7,124
    1c9a:	00000073          	ecall
    1c9e:	2501                	sext.w	a0,a0
    1ca0:	8082                	ret

0000000000001ca2 <fork>:
    1ca2:	0dc00893          	li	a7,220
    1ca6:	4545                	li	a0,17
    1ca8:	4581                	li	a1,0
    1caa:	00000073          	ecall
    1cae:	2501                	sext.w	a0,a0
    1cb0:	8082                	ret

0000000000001cb2 <clone>:
    1cb2:	85b2                	mv	a1,a2
    1cb4:	863a                	mv	a2,a4
    1cb6:	c191                	beqz	a1,1cba <clone+0x8>
    1cb8:	95b6                	add	a1,a1,a3
    1cba:	4781                	li	a5,0
    1cbc:	4701                	li	a4,0
    1cbe:	4681                	li	a3,0
    1cc0:	2601                	sext.w	a2,a2
    1cc2:	a2ed                	j	1eac <__clone>

0000000000001cc4 <exit>:
    1cc4:	05d00893          	li	a7,93
    1cc8:	00000073          	ecall
    1ccc:	8082                	ret

0000000000001cce <waitpid>:
    1cce:	10400893          	li	a7,260
    1cd2:	4681                	li	a3,0
    1cd4:	00000073          	ecall
    1cd8:	2501                	sext.w	a0,a0
    1cda:	8082                	ret

0000000000001cdc <exec>:
    1cdc:	0dd00893          	li	a7,221
    1ce0:	00000073          	ecall
    1ce4:	2501                	sext.w	a0,a0
    1ce6:	8082                	ret

0000000000001ce8 <execve>:
    1ce8:	0dd00893          	li	a7,221
    1cec:	00000073          	ecall
    1cf0:	2501                	sext.w	a0,a0
    1cf2:	8082                	ret

0000000000001cf4 <times>:
    1cf4:	09900893          	li	a7,153
    1cf8:	00000073          	ecall
    1cfc:	2501                	sext.w	a0,a0
    1cfe:	8082                	ret

0000000000001d00 <get_time>:
    1d00:	1141                	add	sp,sp,-16
    1d02:	0a900893          	li	a7,169
    1d06:	850a                	mv	a0,sp
    1d08:	4581                	li	a1,0
    1d0a:	00000073          	ecall
    1d0e:	2501                	sext.w	a0,a0
    1d10:	ed09                	bnez	a0,1d2a <get_time+0x2a>
    1d12:	67a2                	ld	a5,8(sp)
    1d14:	3e800713          	li	a4,1000
    1d18:	00015503          	lhu	a0,0(sp)
    1d1c:	02e7d7b3          	divu	a5,a5,a4
    1d20:	02e50533          	mul	a0,a0,a4
    1d24:	953e                	add	a0,a0,a5
    1d26:	0141                	add	sp,sp,16
    1d28:	8082                	ret
    1d2a:	557d                	li	a0,-1
    1d2c:	bfed                	j	1d26 <get_time+0x26>

0000000000001d2e <sys_get_time>:
    1d2e:	0a900893          	li	a7,169
    1d32:	00000073          	ecall
    1d36:	2501                	sext.w	a0,a0
    1d38:	8082                	ret

0000000000001d3a <time>:
    1d3a:	42600893          	li	a7,1062
    1d3e:	00000073          	ecall
    1d42:	2501                	sext.w	a0,a0
    1d44:	8082                	ret

0000000000001d46 <sleep>:
    1d46:	1141                	add	sp,sp,-16
    1d48:	e02a                	sd	a0,0(sp)
    1d4a:	850a                	mv	a0,sp
    1d4c:	e402                	sd	zero,8(sp)
    1d4e:	06500893          	li	a7,101
    1d52:	85aa                	mv	a1,a0
    1d54:	00000073          	ecall
    1d58:	e501                	bnez	a0,1d60 <sleep+0x1a>
    1d5a:	4501                	li	a0,0
    1d5c:	0141                	add	sp,sp,16
    1d5e:	8082                	ret
    1d60:	4502                	lw	a0,0(sp)
    1d62:	0141                	add	sp,sp,16
    1d64:	8082                	ret

0000000000001d66 <set_priority>:
    1d66:	08c00893          	li	a7,140
    1d6a:	00000073          	ecall
    1d6e:	2501                	sext.w	a0,a0
    1d70:	8082                	ret

0000000000001d72 <mmap>:
    1d72:	0de00893          	li	a7,222
    1d76:	00000073          	ecall
    1d7a:	8082                	ret

0000000000001d7c <munmap>:
    1d7c:	0d700893          	li	a7,215
    1d80:	00000073          	ecall
    1d84:	2501                	sext.w	a0,a0
    1d86:	8082                	ret

0000000000001d88 <wait>:
    1d88:	85aa                	mv	a1,a0
    1d8a:	10400893          	li	a7,260
    1d8e:	557d                	li	a0,-1
    1d90:	4601                	li	a2,0
    1d92:	4681                	li	a3,0
    1d94:	00000073          	ecall
    1d98:	2501                	sext.w	a0,a0
    1d9a:	8082                	ret

0000000000001d9c <spawn>:
    1d9c:	19000893          	li	a7,400
    1da0:	00000073          	ecall
    1da4:	2501                	sext.w	a0,a0
    1da6:	8082                	ret

0000000000001da8 <mailread>:
    1da8:	19100893          	li	a7,401
    1dac:	00000073          	ecall
    1db0:	2501                	sext.w	a0,a0
    1db2:	8082                	ret

0000000000001db4 <mailwrite>:
    1db4:	19200893          	li	a7,402
    1db8:	00000073          	ecall
    1dbc:	2501                	sext.w	a0,a0
    1dbe:	8082                	ret

0000000000001dc0 <fstat>:
    1dc0:	05000893          	li	a7,80
    1dc4:	00000073          	ecall
    1dc8:	2501                	sext.w	a0,a0
    1dca:	8082                	ret

0000000000001dcc <sys_linkat>:
    1dcc:	1702                	sll	a4,a4,0x20
    1dce:	02500893          	li	a7,37
    1dd2:	9301                	srl	a4,a4,0x20
    1dd4:	00000073          	ecall
    1dd8:	2501                	sext.w	a0,a0
    1dda:	8082                	ret

0000000000001ddc <sys_unlinkat>:
    1ddc:	1602                	sll	a2,a2,0x20
    1dde:	02300893          	li	a7,35
    1de2:	9201                	srl	a2,a2,0x20
    1de4:	00000073          	ecall
    1de8:	2501                	sext.w	a0,a0
    1dea:	8082                	ret

0000000000001dec <link>:
    1dec:	87aa                	mv	a5,a0
    1dee:	86ae                	mv	a3,a1
    1df0:	02500893          	li	a7,37
    1df4:	f9c00513          	li	a0,-100
    1df8:	85be                	mv	a1,a5
    1dfa:	f9c00613          	li	a2,-100
    1dfe:	4701                	li	a4,0
    1e00:	00000073          	ecall
    1e04:	2501                	sext.w	a0,a0
    1e06:	8082                	ret

0000000000001e08 <unlink>:
    1e08:	85aa                	mv	a1,a0
    1e0a:	02300893          	li	a7,35
    1e0e:	f9c00513          	li	a0,-100
    1e12:	4601                	li	a2,0
    1e14:	00000073          	ecall
    1e18:	2501                	sext.w	a0,a0
    1e1a:	8082                	ret

0000000000001e1c <uname>:
    1e1c:	0a000893          	li	a7,160
    1e20:	00000073          	ecall
    1e24:	2501                	sext.w	a0,a0
    1e26:	8082                	ret

0000000000001e28 <brk>:
    1e28:	0d600893          	li	a7,214
    1e2c:	00000073          	ecall
    1e30:	2501                	sext.w	a0,a0
    1e32:	8082                	ret

0000000000001e34 <getcwd>:
    1e34:	48c5                	li	a7,17
    1e36:	00000073          	ecall
    1e3a:	8082                	ret

0000000000001e3c <chdir>:
    1e3c:	03100893          	li	a7,49
    1e40:	00000073          	ecall
    1e44:	2501                	sext.w	a0,a0
    1e46:	8082                	ret

0000000000001e48 <mkdir>:
    1e48:	862e                	mv	a2,a1
    1e4a:	87aa                	mv	a5,a0
    1e4c:	1602                	sll	a2,a2,0x20
    1e4e:	02200893          	li	a7,34
    1e52:	f9c00513          	li	a0,-100
    1e56:	85be                	mv	a1,a5
    1e58:	9201                	srl	a2,a2,0x20
    1e5a:	00000073          	ecall
    1e5e:	2501                	sext.w	a0,a0
    1e60:	8082                	ret

0000000000001e62 <getdents>:
    1e62:	03d00893          	li	a7,61
    1e66:	00000073          	ecall
    1e6a:	2501                	sext.w	a0,a0
    1e6c:	8082                	ret

0000000000001e6e <pipe>:
    1e6e:	03b00893          	li	a7,59
    1e72:	4581                	li	a1,0
    1e74:	00000073          	ecall
    1e78:	2501                	sext.w	a0,a0
    1e7a:	8082                	ret

0000000000001e7c <dup>:
    1e7c:	48dd                	li	a7,23
    1e7e:	00000073          	ecall
    1e82:	2501                	sext.w	a0,a0
    1e84:	8082                	ret

0000000000001e86 <dup2>:
    1e86:	48e1                	li	a7,24
    1e88:	4601                	li	a2,0
    1e8a:	00000073          	ecall
    1e8e:	2501                	sext.w	a0,a0
    1e90:	8082                	ret

0000000000001e92 <mount>:
    1e92:	02800893          	li	a7,40
    1e96:	00000073          	ecall
    1e9a:	2501                	sext.w	a0,a0
    1e9c:	8082                	ret

0000000000001e9e <umount>:
    1e9e:	02700893          	li	a7,39
    1ea2:	4581                	li	a1,0
    1ea4:	00000073          	ecall
    1ea8:	2501                	sext.w	a0,a0
    1eaa:	8082                	ret

0000000000001eac <__clone>:
    1eac:	15c1                	add	a1,a1,-16
    1eae:	e188                	sd	a0,0(a1)
    1eb0:	e594                	sd	a3,8(a1)
    1eb2:	8532                	mv	a0,a2
    1eb4:	863a                	mv	a2,a4
    1eb6:	86be                	mv	a3,a5
    1eb8:	8742                	mv	a4,a6
    1eba:	0dc00893          	li	a7,220
    1ebe:	00000073          	ecall
    1ec2:	c111                	beqz	a0,1ec6 <__clone+0x1a>
    1ec4:	8082                	ret
    1ec6:	6582                	ld	a1,0(sp)
    1ec8:	6522                	ld	a0,8(sp)
    1eca:	9582                	jalr	a1
    1ecc:	05d00893          	li	a7,93
    1ed0:	00000073          	ecall
