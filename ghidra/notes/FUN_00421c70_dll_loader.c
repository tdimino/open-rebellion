
void * __thiscall
FUN_00421c70(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,undefined4 param_6,undefined4 param_7,undefined4 param_8)

{
  undefined4 *puVar1;
  LPBYTE pBVar2;
  HINSTANCE hInstance;
  HACCEL pHVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int yBottom;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062bb7c;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005ff1c0(this,param_1,param_2,param_3,param_4,param_5,param_6,0,0);
  local_4 = 0;
  FUN_00601880((void *)((int)this + 0xe0),(char *)0x0,0,0,0,0,0,0);
  local_4._0_1_ = 1;
  FUN_00601880((void *)((int)this + 0x178),(char *)0x0,0,0,0,0,0,0);
  local_4._0_1_ = 2;
  FUN_00601880((void *)((int)this + 0x1ac),(char *)0x0,0,0,0,0,0,0);
  local_4._0_1_ = 3;
  FUN_00601880((void *)((int)this + 0x1e0),(char *)0x0,0,0,0,0,0,0);
  local_4._0_1_ = 4;
  FUN_00601880((void *)((int)this + 0x214),(char *)0x0,0,0,0,0,0,0);
  local_4._0_1_ = 5;
  FUN_0060a790((void *)((int)this + 0x24c),1);
  local_4._0_1_ = 6;
  FUN_00601880((void *)((int)this + 0x260),(char *)0x0,0,0,0,0,0,0);
  local_4._0_1_ = 7;
  FUN_005f52c0((undefined4 *)((int)this + 0x2cc));
  local_4._0_1_ = 8;
  FUN_00618c80((undefined4 *)((int)this + 0x2d8),0x1c,0xc,FUN_0042dbe0);
  local_4._0_1_ = 9;
  FUN_005f52c0((undefined4 *)((int)this + 0x478));
  local_4._0_1_ = 10;
  FUN_0060a790((void *)((int)this + 0x484),1);
  *(undefined2 *)((int)this + 0x494) = 0;
  *(undefined2 *)((int)this + 0x496) = 0;
  *(undefined ***)this = &PTR_FUN_00658bf8;
  *(undefined4 *)((int)this + 0xdc) = 0;
  local_4 = CONCAT31(local_4._1_3_,0xb);
  *(uint *)((int)this + 0x50) = *(uint *)((int)this + 0x50) | 0x6000000;
  *(undefined4 *)((int)this + 100) = 0;
  *(undefined4 *)((int)this + 0x9c) = param_7;
  *(undefined4 *)((int)this + 0xb0) = param_8;
  *(undefined4 *)((int)this + 0xb4) = 1;
  *(undefined4 *)((int)this + 0xa8) = 0;
  *(undefined2 *)((int)this + 0x114) = 0xffff;
  *(undefined4 *)((int)this + 0xa0) = 0;
  *(undefined4 *)((int)this + 0x134) = 0;
  *(undefined4 *)((int)this + 0x138) = 0;
  *(undefined4 *)((int)this + 0x13c) = 0;
  *(undefined4 *)((int)this + 0x140) = 0;
  *(undefined4 *)((int)this + 0x144) = 0;
  *(undefined4 *)((int)this + 0x148) = 0;
  *(undefined4 *)((int)this + 0x14c) = 0;
  *(undefined4 *)((int)this + 0x498) = 0;
  *(undefined4 *)((int)this + 0x154) = 0;
  *(undefined4 *)((int)this + 0x158) = 0;
  *(undefined4 *)((int)this + 0x15c) = 0;
  *(undefined4 *)((int)this + 0x160) = 0;
  *(undefined4 *)((int)this + 0x164) = 0;
  *(undefined4 *)((int)this + 0x168) = 0;
  *(undefined4 *)((int)this + 0x16c) = 0;
  *(undefined4 *)((int)this + 0x170) = 0;
  *(undefined4 *)((int)this + 0x174) = 0;
  *(undefined4 *)((int)this + 0xa4) = 0;
  puVar1 = (undefined4 *)((int)this + 0x2a4);
  iVar4 = 4;
  do {
    puVar1[-4] = 0;
    *puVar1 = 0;
    puVar1[4] = 0;
    puVar1 = puVar1 + 1;
    iVar4 = iVar4 + -1;
  } while (iVar4 != 0);
  *(undefined4 *)((int)this + 0x2c4) = 0;
  *(undefined4 *)((int)this + 0x2c8) = 0;
  *(undefined4 *)((int)this + 0x494) = 0;
  *(undefined4 *)((int)this + 0x118) = 0;
  *(undefined4 *)((int)this + 0x248) = 0;
  *(undefined4 *)((int)this + 0x124) = 0;
  *(undefined4 *)((int)this + 0x11c) = 0x96;
  *(undefined4 *)((int)this + 0x120) = 0x78;
  *(undefined4 *)((int)this + 0x150) = 0;
  *(undefined4 *)((int)this + 0x25c) = 0;
  if (*(int *)((int)this + 0x9c) == 1) {
    iVar6 = 0x186;
    iVar5 = 0x21c;
    iVar4 = 0x37;
  }
  else {
    iVar6 = 0x18b;
    iVar5 = 600;
    iVar4 = 0x78;
  }
  SetRect((LPRECT)((int)this + 0xcc),iVar4,0x28,iVar5,iVar6);
  *(undefined4 *)((int)this + 0x458) = 0;
  puVar1 = (undefined4 *)((int)this + 0x2d8);
  for (iVar4 = 0x54; iVar4 != 0; iVar4 = iVar4 + -1) {
    *puVar1 = 0;
    puVar1 = puVar1 + 1;
  }
  if (*(int *)((int)this + 0x9c) == 1) {
    SetRect((LPRECT)((int)this + 0x2e0),0x220,0x3d,0x25e,0x4f);
    SetRect((LPRECT)((int)this + 0x2fc),0x220,0x53,0x25e,0x65);
    SetRect((LPRECT)((int)this + 0x318),0x220,0x69,0x25e,0x7b);
    SetRect((LPRECT)((int)this + 0x334),0x220,0x7f,0x25e,0x91);
    SetRect((LPRECT)((int)this + 0x350),0x220,0x95,0x25e,0xa7);
    SetRect((LPRECT)((int)this + 0x36c),0x220,0xab,0x25e,0xbd);
    SetRect((LPRECT)((int)this + 0x388),0x220,0xc1,0x25e,0xd3);
    SetRect((LPRECT)((int)this + 0x3a4),0x220,0xd6,0x25e,0xe8);
    SetRect((LPRECT)((int)this + 0x3c0),0x220,0xec,0x25e,0xfe);
    SetRect((LPRECT)((int)this + 0x3dc),0x220,0x102,0x25e,0x114);
    SetRect((LPRECT)((int)this + 0x3f8),0x220,0x117,0x25e,0x129);
    yBottom = 0x13f;
    iVar6 = 0x25e;
    iVar5 = 0x12d;
    iVar4 = 0x220;
  }
  else {
    SetRect((LPRECT)((int)this + 0x2e0),0x15,0x30,0x4b,0x42);
    SetRect((LPRECT)((int)this + 0x2fc),0x15,0x48,0x4b,0x5a);
    SetRect((LPRECT)((int)this + 0x318),0x15,0x62,0x4b,0x74);
    SetRect((LPRECT)((int)this + 0x334),0x15,0x7a,0x4b,0x8c);
    SetRect((LPRECT)((int)this + 0x350),0x15,0x93,0x4b,0xa5);
    SetRect((LPRECT)((int)this + 0x36c),0x15,0xac,0x4b,0xbe);
    SetRect((LPRECT)((int)this + 0x388),0x15,0xc4,0x4b,0xd6);
    SetRect((LPRECT)((int)this + 0x3a4),0x15,0xdd,0x4b,0xef);
    SetRect((LPRECT)((int)this + 0x3c0),0x15,0xf5,0x4b,0x107);
    SetRect((LPRECT)((int)this + 0x3dc),0x15,0x10e,0x4b,0x120);
    SetRect((LPRECT)((int)this + 0x3f8),0x15,0x127,0x4b,0x139);
    yBottom = 0x151;
    iVar6 = 0x4b;
    iVar5 = 0x13f;
    iVar4 = 0x15;
  }
  SetRect((LPRECT)((int)this + 0x414),iVar4,iVar5,iVar6,yBottom);
  DAT_006b14cc = this;
  *(undefined4 *)((int)this + 0xb8) = 0;
  *(undefined4 *)((int)this + 0xbc) = 0;
  *(undefined4 *)((int)this + 0xc0) = 1;
  *(undefined4 *)((int)this + 0xc4) = 0;
  *(undefined4 *)((int)this + 0xac) = 0;
  *(undefined4 *)((int)this + 0x46c) = 0;
  *(undefined4 *)((int)this + 0x468) = 0;
  *(undefined4 *)((int)this + 0x460) = 0;
  *(undefined4 *)((int)this + 0x128) = 0;
  *(undefined4 *)((int)this + 300) = 0;
  *(undefined4 *)((int)this + 0x130) = 0;
  *(undefined4 *)((int)this + 0x474) = 0;
  *(undefined4 *)((int)this + 200) = 0x80;
  *(undefined4 *)((int)this + 0x470) = 0;
  pBVar2 = FUN_00401440();
  FUN_005ff020((char *)pBVar2,s_strategy_dll_006a86a8,7);
  hInstance = (HINSTANCE)FUN_005ff020((char *)pBVar2,s_textcomm_dll_006a7edc,1);
  pHVar3 = LoadAcceleratorsA(hInstance,(LPCSTR)0xb);
  if ((pHVar3 != (HACCEL)0x0) && (DAT_006be3b8 != 0)) {
    *(HACCEL *)(DAT_006be3b8 + 0x98) = pHVar3;
  }
  if (pBVar2 != (LPBYTE)0x0) {
    FUN_00618b60(pBVar2);
  }
  ExceptionList = pvStack_c;
  return this;
}

