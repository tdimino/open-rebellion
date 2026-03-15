
void __fastcall FUN_004084b0(void *param_1)

{
  void *pvVar1;
  LPSTR *ppCVar2;
  char *pcVar3;
  uint uVar4;
  undefined4 local_34;
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629b2f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  DAT_006be5ac = FUN_006037f0(1);
  FUN_005ff6b0(param_1,0x1028);
  FUN_005ff6b0(param_1,0x1026);
  FUN_005ff6b0(param_1,0x1027);
  local_34 = (void *)FUN_00618b70(0x34);
  local_4 = 0;
  if (local_34 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601620(local_34,0x1025,0x178,0x4f,0xb7,0x21,
                          (-(uint)((*(uint *)((int)param_1 + 0xa0) & 0x100) != 0) & 0x7f00) +
                          0x2008000,7,0x24,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0xe8) = pvVar1;
  local_34 = (void *)FUN_00618b70(0x34);
  local_4 = 1;
  if (local_34 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601620(local_34,0x101f,0x18a,0x139,0xb7,0x10,
                          (-(uint)((*(uint *)((int)param_1 + 0xa0) & 0x10000000) != 0) & 0x7f00) +
                          0x2008000,7,0x10,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0xec) = pvVar1;
  local_34 = (void *)FUN_00618b70(0x34);
  local_4 = 2;
  if (local_34 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601620(local_34,0x1020,0x18a,0x154,0xb7,0x10,
                          (-(uint)((*(uint *)((int)param_1 + 0xa0) & 0x1000000) != 0) & 0x7f00) +
                          0x2008000,7,0x10,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0xf0) = pvVar1;
  local_34 = (void *)FUN_00618b70(0x34);
  local_4 = 3;
  if (local_34 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601620(local_34,0x1021,0x18a,0x16f,0xb7,0x10,
                          (-(uint)((*(uint *)((int)param_1 + 0xa0) & 0x100000) != 0) & 0x7f00) +
                          0x2008000,7,0x10,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0xf4) = pvVar1;
  local_34 = (void *)FUN_00618b70(0x34);
  local_4 = 4;
  if (local_34 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601620(local_34,0x1022,0x18a,0x18a,0xb7,0x10,
                          (-(uint)((*(uint *)((int)param_1 + 0xa0) & 0x10000) != 0) & 0x7f00) +
                          0x2008000,7,0x10,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0xf8) = pvVar1;
  local_34 = (void *)FUN_00618b70(0x34);
  local_4 = 5;
  if (local_34 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601620(local_34,0x1024,0x18a,0x1a5,0xb7,0x10,
                          (-(uint)((*(uint *)((int)param_1 + 0xa0) & 0x2000000) != 0) & 0x7f00) +
                          0x2008000,7,0x10,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0xfc) = pvVar1;
  local_34 = (void *)FUN_00618b70(0x34);
  local_4 = 6;
  if (local_34 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    uVar4 = *(uint *)((int)param_1 + 0xa0) & 0x100;
    pvVar1 = FUN_00601620(local_34,0x1032 - (uVar4 != 0),0x245,0x4f,0x1f,0x21,
                          (-(uint)(uVar4 != 0) & 0x7f00) + 0x2008000,7,0x24,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x100) = pvVar1;
  local_34 = (void *)FUN_00618b70(0x34);
  local_4 = 7;
  if (local_34 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    uVar4 = *(uint *)((int)param_1 + 0xa0) & 0x10000000;
    pvVar1 = FUN_00601620(local_34,0x1032 - (uVar4 != 0),0x241,0x139,0x1f,0x10,
                          (-(uint)(uVar4 != 0) & 0x7f00) + 0x2008000,7,0x10,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x104) = pvVar1;
  local_34 = (void *)FUN_00618b70(0x34);
  local_4 = 8;
  if (local_34 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    uVar4 = *(uint *)((int)param_1 + 0xa0) & 0x1000000;
    pvVar1 = FUN_00601620(local_34,0x1032 - (uVar4 != 0),0x241,0x154,0x1f,0x10,
                          (-(uint)(uVar4 != 0) & 0x7f00) + 0x2008000,7,0x10,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x108) = pvVar1;
  local_34 = (void *)FUN_00618b70(0x34);
  local_4 = 9;
  if (local_34 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    uVar4 = *(uint *)((int)param_1 + 0xa0) & 0x100000;
    pvVar1 = FUN_00601620(local_34,0x1032 - (uVar4 != 0),0x241,0x16f,0x1f,0x10,
                          (-(uint)(uVar4 != 0) & 0x7f00) + 0x2008000,7,0x10,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x10c) = pvVar1;
  local_34 = (void *)FUN_00618b70(0x34);
  local_4 = 10;
  if (local_34 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    uVar4 = *(uint *)((int)param_1 + 0xa0) & 0x10000;
    pvVar1 = FUN_00601620(local_34,0x1032 - (uVar4 != 0),0x241,0x18a,0x1f,0x10,
                          (-(uint)(uVar4 != 0) & 0x7f00) + 0x2008000,7,0x10,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x110) = pvVar1;
  local_34 = (void *)FUN_00618b70(0x34);
  local_4 = 0xb;
  if (local_34 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    uVar4 = *(uint *)((int)param_1 + 0xa0) & 0x2000000;
    pvVar1 = FUN_00601620(local_34,0x1032 - (uVar4 != 0),0x241,0x1a5,0x1f,0x10,
                          (-(uint)(uVar4 != 0) & 0x7f00) + 0x2008000,7,0x10,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x114) = pvVar1;
  local_34 = (void *)FUN_00618b70(0x34);
  local_4 = 0xc;
  if (local_34 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601620(local_34,0,0x19,0x1b5,0x136,0x12,0x2000000,10,0x25,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x118) = pvVar1;
  if (pvVar1 != (void *)0x0) {
    local_34 = (void *)CONCAT22(DAT_0065d420,0x100c);
    FUN_005f2fc0(local_30,&local_34);
    local_4 = 0xd;
    FUN_005f35b0(local_24,s_1_01_00_006a7f44);
    local_4._0_1_ = 0xe;
    FUN_005f30d0(local_30,(int)local_24);
    local_4._0_1_ = 0xd;
    FUN_005f2ff0(local_24);
    ppCVar2 = FUN_00406840();
    FUN_005f35b0(local_18,(char *)ppCVar2);
    local_4._0_1_ = 0xf;
    FUN_005f35b0(local_24,(char *)&lpString_006a7f40);
    local_4._0_1_ = 0x10;
    FUN_005f30d0(local_30,(int)local_24);
    FUN_005f30d0(local_30,(int)local_18);
    local_4._0_1_ = 0xf;
    FUN_005f2ff0(local_24);
    local_4 = CONCAT31(local_4._1_3_,0xd);
    FUN_005f2ff0(local_18);
    pcVar3 = (char *)FUN_00583c40((int)local_30);
    FUN_00601aa0(*(void **)((int)param_1 + 0x118),pcVar3);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_30);
  }
  ExceptionList = local_c;
  return;
}

