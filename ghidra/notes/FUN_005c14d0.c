
void * __thiscall
FUN_005c14d0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,undefined4 param_6,uint param_7,undefined4 param_8,undefined4 param_9,
            undefined4 param_10)

{
  void *pvVar1;
  int iVar2;
  uint uVar3;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653995;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005d6700(this,param_1,param_2,param_3,param_4,param_5,param_6,0x280,0x1e0,8,param_7,param_8,
               param_10,param_9);
  local_4 = 0;
  FUN_005f4ed0((undefined4 *)((int)this + 0x16c));
  *(undefined4 *)((int)this + 0x16c) = &PTR_FUN_0066ca10;
  *(undefined4 *)((int)this + 0x168) = &PTR_FUN_0066ca08;
  local_4._0_1_ = 1;
  FUN_005f4ed0((undefined4 *)((int)this + 0x178));
  *(undefined4 *)((int)this + 0x178) = &PTR_FUN_0066c9f8;
  *(undefined ***)((int)this + 0x174) = &PTR_FUN_0066c9f0;
  *(undefined4 *)((int)this + 0x168) = &PTR_FUN_0066c9e8;
  local_4._0_1_ = 2;
  FUN_005f4ed0((undefined4 *)((int)this + 0x184));
  *(undefined4 *)((int)this + 0x184) = &PTR_FUN_0066c9d8;
  *(undefined4 *)((int)this + 0x180) = &PTR_FUN_0066c9d0;
  local_4._0_1_ = 3;
  FUN_005f4ed0((undefined4 *)((int)this + 400));
  *(undefined4 *)((int)this + 400) = &PTR_FUN_0066c9c0;
  *(undefined ***)((int)this + 0x18c) = &PTR_FUN_0066c9b8;
  *(undefined4 *)((int)this + 0x180) = &PTR_FUN_0066c9b0;
  local_4._0_1_ = 4;
  FUN_005f4ed0((undefined4 *)((int)this + 0x19c));
  *(undefined4 *)((int)this + 0x19c) = &PTR_FUN_0066c9a0;
  *(undefined4 *)((int)this + 0x198) = &PTR_FUN_0066c998;
  local_4._0_1_ = 5;
  FUN_005f4ed0((undefined4 *)((int)this + 0x1a8));
  *(undefined ***)((int)this + 0x1a8) = &PTR_FUN_0066c9d8;
  *(undefined4 *)((int)this + 0x1a4) = &PTR_FUN_0066c9d0;
  local_4._0_1_ = 6;
  FUN_005f4ed0((undefined4 *)((int)this + 0x1b4));
  *(undefined ***)((int)this + 0x1b4) = &PTR_FUN_0066c9c0;
  *(undefined ***)((int)this + 0x1b0) = &PTR_FUN_0066c9b8;
  *(undefined4 *)((int)this + 0x1a4) = &PTR_FUN_0066c9b0;
  *(undefined4 *)((int)this + 0x198) = &PTR_FUN_0066c990;
  local_4._0_1_ = 7;
  FUN_005f2f00((undefined4 *)((int)this + 0x5cc));
  local_4._0_1_ = 8;
  FUN_005f52c0((undefined4 *)((int)this + 0x5d8));
  local_4._0_1_ = 9;
  FUN_005f5830((undefined4 *)((int)this + 0x5e4));
  local_4._0_1_ = 10;
  FUN_005f5830((undefined4 *)((int)this + 0x5f4));
  local_4._0_1_ = 0xb;
  FUN_005f5830((undefined4 *)((int)this + 0x604));
  local_4._0_1_ = 0xc;
  FUN_005f5830((undefined4 *)((int)this + 0x614));
  local_4._0_1_ = 0xd;
  FUN_005fbb20((undefined4 *)((int)this + 0x660));
  local_4._0_1_ = 0xe;
  *(undefined ***)this = &PTR_FUN_0066c960;
  DAT_006be5ac = FUN_006037f0(3);
  *(undefined4 *)((int)this + 0x5c0) = 0;
  *(undefined4 *)((int)this + 0x5c4) = 0;
  *(undefined4 *)((int)this + 0x5c8) = 0;
  pvVar1 = (void *)FUN_00618b70(0x34);
  local_4._0_1_ = 0xf;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_006017a0(pvVar1,0xddf0,0,0,param_4,0x1e,0,0);
  }
  local_4._0_1_ = 0xe;
  *(void **)((int)this + 0x62c) = pvVar1;
  FUN_00601c60(pvVar1,6);
  pvVar1 = (void *)FUN_00618b70(0x34);
  local_4._0_1_ = 0x10;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_006017a0(pvVar1,0xddf1,0,0,param_4,0x1e,0,0);
  }
  local_4 = CONCAT31(local_4._1_3_,0xe);
  *(void **)((int)this + 0x630) = pvVar1;
  FUN_00601c60(pvVar1,7);
  iVar2 = FUN_00595c40();
  if (iVar2 == 0) {
    FUN_00601c90(*(void **)((int)this + 0x62c),0x20000ff,(HWND)0x0);
    iVar2 = 0x200ff00;
  }
  else {
    FUN_00601c90(*(void **)((int)this + 0x62c),0x200ff00,(HWND)0x0);
    iVar2 = 0x20000ff;
  }
  FUN_00601c90(*(void **)((int)this + 0x630),iVar2,(HWND)0x0);
  pvVar1 = (void *)FUN_00618b70(0x34);
  local_4._0_1_ = 0x11;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_006017a0(pvVar1,0xddf2,0,0,param_4,0x1e,2,0);
  }
  local_4 = CONCAT31(local_4._1_3_,0xe);
  *(void **)((int)this + 0x634) = pvVar1;
  FUN_00601c60(pvVar1,6);
  FUN_00601c90(*(void **)((int)this + 0x634),0x2ffff00,(HWND)0x0);
  FUN_005c1060(this);
  *(undefined4 *)((int)this + 0x628) = 0;
  *(undefined4 *)((int)this + 0x63c) = 0;
  *(undefined4 *)((int)this + 0x640) = 0;
  *(undefined4 *)((int)this + 0x644) = 0;
  *(undefined4 *)((int)this + 0x638) = 0;
  *(undefined4 *)((int)this + 0x658) = 0;
  *(undefined4 *)((int)this + 0x678) = 0;
  *(undefined4 *)((int)this + 0x648) = 0;
  *(undefined4 *)((int)this + 0x650) = 0;
  *(undefined4 *)((int)this + 0x64c) = 0;
  *(undefined4 *)((int)this + 0x654) = 0;
  uVar3 = FUN_00401a00();
  *(undefined4 *)((int)this + 0x67c) = 0;
  if ((uVar3 & 0x10000000) != 0) {
    *(undefined4 *)((int)this + 0x67c) = 1;
  }
  *(undefined4 *)((int)this + 0x680) = 0;
  if ((uVar3 & 0x1000000) != 0) {
    *(undefined4 *)((int)this + 0x680) = 1;
  }
  *(undefined4 *)((int)this + 0x684) = 0;
  if ((uVar3 & 0x100000) != 0) {
    *(undefined4 *)((int)this + 0x684) = 1;
  }
  *(undefined4 *)((int)this + 0x688) = 0;
  if ((uVar3 & 0x10000) != 0) {
    *(undefined4 *)((int)this + 0x688) = 1;
  }
  FUN_005c2e60((int)this);
  ExceptionList = pvStack_c;
  return this;
}

