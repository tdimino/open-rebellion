
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 * __fastcall FUN_00436a40(undefined4 *param_1)

{
  int iVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d5a8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f52c0(param_1);
  *param_1 = &PTR_FUN_00658da0;
  local_4 = 0;
  if (DAT_006a8704 != 0) {
    puVar2 = &DAT_006b1f40;
    for (iVar1 = 0x2e; iVar1 != 0; iVar1 = iVar1 + -1) {
      *puVar2 = 0;
      puVar2 = puVar2 + 1;
    }
    if ((DAT_006b24ec & 1) == 0) {
      DAT_006b24ec = DAT_006b24ec | 1;
      FUN_0049bda0((undefined4 *)&DAT_006b24f0);
      FUN_00618c20(0x437660);
    }
    _DAT_006b1f48 = &DAT_006b24f0;
    if ((DAT_006b24ec & 2) == 0) {
      DAT_006b24ec = DAT_006b24ec | 2;
      FUN_0049aad0((undefined4 *)&DAT_006b2358);
      FUN_00618c20(0x437650);
    }
    _DAT_006b1f44 = &DAT_006b2358;
    if ((DAT_006b24ec & 4) == 0) {
      DAT_006b24ec = DAT_006b24ec | 4;
      FUN_0049a120((undefined4 *)&DAT_006b1b68);
      FUN_00618c20(0x437640);
    }
    iVar1 = FUN_0048c2a0();
    (&DAT_006b1f40)[iVar1] = &DAT_006b1b68;
    if ((DAT_006b24ec & 8) == 0) {
      DAT_006b24ec = DAT_006b24ec | 8;
      FUN_00499ce0((undefined4 *)&DAT_006b16a0);
      FUN_00618c20(0x437630);
    }
    _DAT_006b1f58 = &DAT_006b16a0;
    if ((DAT_006b24ec & 0x10) == 0) {
      DAT_006b24ec = DAT_006b24ec | 0x10;
      FUN_004999c0((undefined4 *)&DAT_006b1bc8);
      FUN_00618c20(0x437620);
    }
    _DAT_006b1f54 = &DAT_006b1bc8;
    if ((DAT_006b24ec & 0x20) == 0) {
      DAT_006b24ec = DAT_006b24ec | 0x20;
      FUN_00499680((undefined4 *)&DAT_006b2060);
      FUN_00618c20(0x437610);
    }
    _DAT_006b1f50 = &DAT_006b2060;
    if ((DAT_006b24ec & 0x40) == 0) {
      DAT_006b24ec = DAT_006b24ec | 0x40;
      FUN_00499360((undefined4 *)&DAT_006b21b0);
      FUN_00618c20(0x437600);
    }
    _DAT_006b1f4c = &DAT_006b21b0;
    if ((DAT_006b24ec & 0x80) == 0) {
      DAT_006b24ec = DAT_006b24ec | 0x80;
      FUN_00499030((undefined4 *)&DAT_006b2830);
      FUN_00618c20(0x4375f0);
    }
    _DAT_006b1f64 = &DAT_006b2830;
    if ((DAT_006b169c & 1) == 0) {
      DAT_006b169c = DAT_006b169c | 1;
      FUN_00498e20((undefined4 *)&DAT_006b2570);
      FUN_00618c20(0x4375e0);
    }
    _DAT_006b1f68 = &DAT_006b2570;
    if ((DAT_006b169c & 2) == 0) {
      DAT_006b169c = DAT_006b169c | 2;
      FUN_00498bb0((undefined4 *)&DAT_006b1e70);
      FUN_00618c20(0x4375d0);
    }
    _DAT_006b1f6c = &DAT_006b1e70;
    if ((DAT_006b169c & 4) == 0) {
      DAT_006b169c = DAT_006b169c | 4;
      FUN_00498870((undefined4 *)&DAT_006b1878);
      FUN_00618c20(0x4375c0);
    }
    _DAT_006b1f70 = &DAT_006b1878;
    if ((DAT_006b169c & 8) == 0) {
      DAT_006b169c = DAT_006b169c | 8;
      FUN_004980c0((undefined4 *)&DAT_006b2218);
      FUN_00618c20(0x4375b0);
    }
    _DAT_006b1f74 = &DAT_006b2218;
    if ((DAT_006b169c & 0x10) == 0) {
      DAT_006b169c = DAT_006b169c | 0x10;
      FUN_00497810((undefined4 *)&DAT_006b1770);
      FUN_00618c20(0x4375a0);
    }
    _DAT_006b1f78 = &DAT_006b1770;
    if ((DAT_006b169c & 0x20) == 0) {
      DAT_006b169c = DAT_006b169c | 0x20;
      FUN_00497560((undefined4 *)&DAT_006b14e8);
      FUN_00618c20(0x437590);
    }
    _DAT_006b1f80 = &DAT_006b14e8;
    if ((DAT_006b169c & 0x40) == 0) {
      DAT_006b169c = DAT_006b169c | 0x40;
      FUN_00496380((undefined4 *)&DAT_006b2788);
      FUN_00618c20(0x437580);
    }
    _DAT_006b1f7c = &DAT_006b2788;
    if ((DAT_006b169c & 0x80) == 0) {
      DAT_006b169c = DAT_006b169c | 0x80;
      FUN_00496010((undefined4 *)&DAT_006b1950);
      FUN_00618c20(0x437570);
    }
    _DAT_006b1f5c = &DAT_006b1950;
    if ((DAT_006b1af8 & 1) == 0) {
      DAT_006b1af8 = DAT_006b1af8 | 1;
      FUN_00495d50((undefined4 *)&DAT_006b1a20);
      FUN_00618c20(0x437560);
    }
    _DAT_006b1f8c = &DAT_006b1a20;
    if ((DAT_006b1af8 & 2) == 0) {
      DAT_006b1af8 = DAT_006b1af8 | 2;
      FUN_004953c0((undefined4 *)&DAT_006b2130);
      FUN_00618c20(0x437550);
    }
    _DAT_006b1f90 = &DAT_006b2130;
    if ((DAT_006b1af8 & 4) == 0) {
      DAT_006b1af8 = DAT_006b1af8 | 4;
      FUN_00494ef0((undefined4 *)&DAT_006b1d98);
      FUN_00618c20(0x437540);
    }
    _DAT_006b1f94 = &DAT_006b1d98;
    if ((DAT_006b1af8 & 8) == 0) {
      DAT_006b1af8 = DAT_006b1af8 | 8;
      FUN_004923a0((undefined4 *)&DAT_006b1c30);
      FUN_00618c20(0x437530);
    }
    iVar1 = FUN_00492380(0x6b1c30);
    (&DAT_006b1f40)[iVar1] = &DAT_006b1c30;
    if ((DAT_006b1af8 & 0x10) == 0) {
      DAT_006b1af8 = DAT_006b1af8 | 0x10;
      FUN_00491190((undefined4 *)&DAT_006b17e0);
      FUN_00618c20(0x437520);
    }
    iVar1 = FUN_00492380(0x6b17e0);
    (&DAT_006b1f40)[iVar1] = &DAT_006b17e0;
    if ((DAT_006b1af8 & 0x20) == 0) {
      DAT_006b1af8 = DAT_006b1af8 | 0x20;
      FUN_00490f80((undefined4 *)&DAT_006b2720);
      FUN_00618c20(0x437510);
    }
    _DAT_006b1fa0 = &DAT_006b2720;
    if ((DAT_006b1af8 & 0x40) == 0) {
      DAT_006b1af8 = DAT_006b1af8 | 0x40;
      FUN_00490250((undefined4 *)&DAT_006b1708);
      FUN_00618c20(0x437500);
    }
    _DAT_006b1fa4 = &DAT_006b1708;
    if ((DAT_006b1af8 & 0x80) == 0) {
      DAT_006b1af8 = DAT_006b1af8 | 0x80;
      FUN_0048fd00((undefined4 *)&DAT_006b2488);
      FUN_00618c20(0x4374f0);
    }
    _DAT_006b1fa8 = &DAT_006b2488;
    if ((DAT_006b1c2c & 1) == 0) {
      DAT_006b1c2c = DAT_006b1c2c | 1;
      FUN_0048f910((undefined4 *)&DAT_006b26b8);
      FUN_00618c20(0x4374e0);
    }
    _DAT_006b1fa8 = &DAT_006b26b8;
    if ((DAT_006b1c2c & 2) == 0) {
      DAT_006b1c2c = DAT_006b1c2c | 2;
      FUN_0048f690((undefined4 *)&DAT_006b20c8);
      FUN_00618c20(0x4374d0);
    }
    _DAT_006b1fb8 = &DAT_006b20c8;
    if ((DAT_006b1c2c & 4) == 0) {
      DAT_006b1c2c = DAT_006b1c2c | 4;
      FUN_0048f0e0((undefined4 *)&DAT_006b1e08);
      FUN_00618c20(0x4374c0);
    }
    _DAT_006b1fb0 = &DAT_006b1e08;
    if ((DAT_006b1c2c & 8) == 0) {
      DAT_006b1c2c = DAT_006b1c2c | 8;
      FUN_0048ec80((undefined4 *)&DAT_006b1b00);
      FUN_00618c20(0x4374b0);
    }
    _DAT_006b1fb4 = &DAT_006b1b00;
    if ((DAT_006b1c2c & 0x10) == 0) {
      DAT_006b1c2c = DAT_006b1c2c | 0x10;
      FUN_0048e690((undefined4 *)&DAT_006b25d8);
      FUN_00618c20(0x4374a0);
    }
    _DAT_006b1fc0 = &DAT_006b25d8;
    if ((DAT_006b1c2c & 0x20) == 0) {
      DAT_006b1c2c = DAT_006b1c2c | 0x20;
      FUN_0048e490((undefined4 *)&DAT_006b1a90);
      FUN_00618c20(0x437490);
    }
    _DAT_006b1fbc = &DAT_006b1a90;
    if ((DAT_006b1c2c & 0x40) == 0) {
      DAT_006b1c2c = DAT_006b1c2c | 0x40;
      FUN_0048d650((undefined4 *)&DAT_006b18e0);
      FUN_00618c20(0x437480);
    }
    _DAT_006b1fd0 = &DAT_006b18e0;
    if ((DAT_006b1c2c & 0x80) == 0) {
      DAT_006b1c2c = DAT_006b1c2c | 0x80;
      FUN_0048d370((undefined4 *)&DAT_006b1d30);
      FUN_00618c20(0x437470);
    }
    _DAT_006b1fcc = &DAT_006b1d30;
    if ((DAT_006b22ec & 1) == 0) {
      DAT_006b22ec = DAT_006b22ec | 1;
      FUN_0048cff0((undefined4 *)&DAT_006b2650);
      FUN_00618c20(0x437460);
    }
    _DAT_006b1fd4 = &DAT_006b2650;
    if ((DAT_006b22ec & 2) == 0) {
      DAT_006b22ec = DAT_006b22ec | 2;
      FUN_0048cd80((undefined4 *)&DAT_006b19b8);
      FUN_00618c20(0x437450);
    }
    _DAT_006b1fd8 = &DAT_006b19b8;
    if ((DAT_006b22ec & 4) == 0) {
      DAT_006b22ec = DAT_006b22ec | 4;
      FUN_0048c8c0((undefined4 *)&DAT_006b2288);
      FUN_00618c20(0x437440);
    }
    _DAT_006b1fdc = &DAT_006b2288;
    if ((DAT_006b22ec & 8) == 0) {
      DAT_006b22ec = DAT_006b22ec | 8;
      FUN_0048c320((undefined4 *)&DAT_006b1630);
      FUN_00618c20(0x437430);
    }
    _DAT_006b1fe0 = &DAT_006b1630;
    if ((DAT_006b22ec & 0x10) == 0) {
      DAT_006b22ec = DAT_006b22ec | 0x10;
      FUN_0048bc80((undefined4 *)&DAT_006b15c0);
      FUN_00618c20(0x437420);
    }
    _DAT_006b1fc4 = &DAT_006b15c0;
    if ((DAT_006b22ec & 0x20) == 0) {
      DAT_006b22ec = DAT_006b22ec | 0x20;
      FUN_0048b860((undefined4 *)&DAT_006b1cc8);
      FUN_00618c20(0x437410);
    }
    _DAT_006b1fc8 = &DAT_006b1cc8;
    if ((DAT_006b22ec & 0x40) == 0) {
      DAT_006b22ec = DAT_006b22ec | 0x40;
      FUN_0048b5e0((undefined4 *)&DAT_006b2420);
      FUN_00618c20(0x437400);
    }
    _DAT_006b1fe4 = &DAT_006b2420;
    if ((DAT_006b22ec & 0x80) == 0) {
      DAT_006b22ec = DAT_006b22ec | 0x80;
      FUN_0048b270((undefined4 *)&DAT_006b22f0);
      FUN_00618c20(0x4373f0);
    }
    _DAT_006b1fe8 = &DAT_006b22f0;
    if ((DAT_006b1ed4 & 1) == 0) {
      DAT_006b1ed4 = DAT_006b1ed4 | 1;
      FUN_0048af50((undefined4 *)&DAT_006b1ff8);
      FUN_00618c20(0x4373e0);
    }
    _DAT_006b1fec = &DAT_006b1ff8;
    if ((DAT_006b1ed4 & 2) == 0) {
      DAT_006b1ed4 = DAT_006b1ed4 | 2;
      FUN_0048ad40((undefined4 *)&DAT_006b1ed8);
      FUN_00618c20(0x4373d0);
    }
    _DAT_006b1ff4 = &DAT_006b1ed8;
    if ((DAT_006b1ed4 & 4) == 0) {
      DAT_006b1ed4 = DAT_006b1ed4 | 4;
      FUN_0048aac0((undefined4 *)&DAT_006b1558);
      FUN_00618c20(0x4373c0);
    }
    _DAT_006b1ff0 = &DAT_006b1558;
    DAT_006a8704 = 0;
  }
  ExceptionList = local_c;
  return param_1;
}

