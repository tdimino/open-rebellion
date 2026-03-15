
void __fastcall FUN_004511e0(void *param_1)

{
  undefined4 uVar1;
  int *piVar2;
  uint uVar3;
  int iVar4;
  void *pvVar5;
  int *piVar6;
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062fc9d;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_10 = (void *)FUN_00422ca0();
  FUN_004ab2b0(param_1);
  uVar1 = *(undefined4 *)((int)param_1 + 0x1c);
  piVar2 = (int *)FUN_006037f0(7);
  FUN_006037f0(2);
  FUN_00607740(param_1,piVar2,0x2779,0x277a,0x277b,(int *)0x2778,(int *)0x2775,(int *)0x2774,0x2777,
               (int *)0x2776);
  piVar6 = (int *)0x0;
  uVar3 = (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 9) + 0x277e;
  local_14 = (void *)CONCAT22(DAT_0065d424,0x1600);
  FUN_004ab1f0(param_1,piVar2,&local_14,0x10,0x1e,10,*(int *)((int)param_1 + 0x30),0x14,uVar3,uVar3,
               0,0);
  local_14 = (void *)CONCAT22(DAT_0065d424,0x1601);
  uVar3 = (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 9) + 0x277f;
  FUN_004ab1f0(param_1,piVar2,&local_14,0x20,0x1e,0x20,*(int *)((int)param_1 + 0x30),0x14,uVar3,
               uVar3,0,0);
  local_14 = (void *)CONCAT22(DAT_0065d424,0x1602);
  uVar3 = (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 9) + 0x2780;
  FUN_004ab1f0(param_1,piVar2,&local_14,0x40,0x1e,0x36,*(int *)((int)param_1 + 0x30),0x14,uVar3,
               uVar3,0,0);
  uVar3 = (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 9) + 0x2781;
  local_14 = (void *)CONCAT22(DAT_0065d424,0x1605);
  FUN_004ab1f0(param_1,piVar2,&local_14,0x50,0x1e,0x4c,*(int *)((int)param_1 + 0x30),0x14,uVar3,
               uVar3,0,0);
  local_14 = (void *)CONCAT22(DAT_0065d424,0x160a);
  uVar3 = (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 9) + 0x2782;
  FUN_004ab1f0(param_1,piVar2,&local_14,0x60,0x1e,0x62,*(int *)((int)param_1 + 0x30),0x14,uVar3,
               uVar3,0,0);
  local_14 = (void *)CONCAT22(DAT_0065d424,0x160f);
  uVar3 = (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 9) + 0x2783;
  FUN_004ab1f0(param_1,piVar2,&local_14,0x70,0x1e,0x78,*(int *)((int)param_1 + 0x30),0x14,uVar3,
               uVar3,0,0);
  local_14 = (void *)FUN_00618b70(0x28);
  local_4 = 0;
  if (local_14 != (void *)0x0) {
    piVar6 = (int *)FUN_005fbd20(local_14,piVar2,0x2783,2);
  }
  local_4 = 0xffffffff;
  iVar4 = FUN_005fc0e0(piVar6);
  if (piVar6 != (int *)0x0) {
    FUN_005fbfa0(piVar6);
    FUN_00618b60((undefined *)piVar6);
  }
  local_14 = (void *)CONCAT22(DAT_0065d424,0x1614);
  FUN_004ab1f0(param_1,piVar2,&local_14,0x80,iVar4 + 0x1e,0x8e,*(int *)((int)param_1 + 0x30),0x32,0,
               0,0,0);
  FUN_004ab1b0(param_1,(-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xfe01) + 0x20000ff,0x2ffffff
               ,0);
  local_14 = (void *)FUN_00618b70(0x14c);
  local_4 = 1;
  if (local_14 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_004aa960(local_14,uVar1,0x86,0xed,0xf0,0x37,*(int *)((int)param_1 + 0x150),0x141,
                          *(undefined4 *)((int)param_1 + 0x14c));
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x158) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    FUN_005ffce0(pvVar5,0);
    (**(code **)(**(int **)((int)param_1 + 0x158) + 8))();
    local_14 = (void *)CONCAT22(DAT_0065d424,0x1615);
    uVar3 = (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 9) + 0x277e;
    FUN_004ab1f0(*(void **)((int)param_1 + 0x158),piVar2,&local_14,0x11,0x1e,7,
                 *(int *)((int)*(void **)((int)param_1 + 0x158) + 0x30),0x14,uVar3,uVar3,0,0);
    local_14 = (void *)CONCAT22(DAT_0065d424,0x1616);
    uVar3 = (*(int *)((int)param_1 + 0x154) != 1) + 0x2d58;
    FUN_004ab1f0(*(void **)((int)param_1 + 0x158),piVar2,&local_14,0x12,0x1e,0x1d,
                 *(int *)((int)*(void **)((int)param_1 + 0x158) + 0x30),0x14,uVar3,uVar3,0,0);
    FUN_00607740(*(void **)((int)param_1 + 0x158),piVar2,0x2779,0x277a,0x277b,(int *)0x2778,
                 (int *)0x2775,(int *)0x2774,0x2777,(int *)0x2776);
    FUN_004ab1b0(*(void **)((int)param_1 + 0x158),
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xfe01) + 0x20000ff,0x2ffffff,0);
    FUN_004ab3a0(param_1,piVar2,0x10,*(int *)((int)param_1 + 0x158),
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xb) + 0x2785,
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xb) + 0x2786);
  }
  local_14 = (void *)FUN_00618b70(0x14c);
  local_4 = 2;
  if (local_14 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_004aa960(local_14,uVar1,0x86,0x104,0xf0,0x37,*(int *)((int)param_1 + 0x150),0x142,
                          *(undefined4 *)((int)param_1 + 0x14c));
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x15c) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    FUN_005ffce0(pvVar5,0);
    (**(code **)(**(int **)((int)param_1 + 0x15c) + 8))();
    local_14 = (void *)CONCAT22(DAT_0065d424,0x1617);
    uVar3 = (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 9) + 0x277f;
    FUN_004ab1f0(*(void **)((int)param_1 + 0x15c),piVar2,&local_14,0x21,0x1e,7,
                 *(int *)(*(int *)((int)param_1 + 0x158) + 0x30),0x14,uVar3,uVar3,0,0);
    local_14 = (void *)CONCAT22(DAT_0065d424,0x1618);
    uVar3 = (*(int *)((int)param_1 + 0x154) != 1) + 0x2d5d;
    FUN_004ab1f0(*(void **)((int)param_1 + 0x15c),piVar2,&local_14,0x22,0x1e,0x1d,
                 *(int *)(*(int *)((int)param_1 + 0x158) + 0x30),0x14,uVar3,uVar3,0,0);
    FUN_00607740(*(void **)((int)param_1 + 0x15c),piVar2,0x2779,0x277a,0x277b,(int *)0x2778,
                 (int *)0x2775,(int *)0x2774,0x2777,(int *)0x2776);
    FUN_004ab1b0(*(void **)((int)param_1 + 0x15c),
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xfe01) + 0x20000ff,0x2ffffff,0);
    FUN_004ab3a0(param_1,piVar2,0x20,*(int *)((int)param_1 + 0x15c),
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xb) + 0x2785,
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xb) + 0x2786);
  }
  local_14 = (void *)FUN_00618b70(0x14c);
  local_4 = 3;
  if (local_14 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_004aa960(local_14,uVar1,0x86,0x104,0xf0,0x37,*(int *)((int)param_1 + 0x150),0x143,
                          *(undefined4 *)((int)param_1 + 0x14c));
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x160) = pvVar5;
  iVar4 = (int)local_10;
  if (pvVar5 != (void *)0x0) {
    FUN_005ffce0(pvVar5,0);
    (**(code **)(**(int **)((int)param_1 + 0x160) + 8))();
    iVar4 = (int)local_10;
    uVar3 = 0x279c;
    if (*(int *)((int)local_10 + 0x9c) != 1) {
      uVar3 = 0x278f;
    }
    local_14 = (void *)CONCAT22(DAT_0065d424,0x161e);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x160),piVar2,&local_14,0x44,0x1e,0x1d,
                 *(int *)((int)*(void **)((int)param_1 + 0x160) + 0x30),0x14,uVar3,uVar3,0,0);
    uVar3 = 0x279a;
    if (*(int *)(iVar4 + 0x9c) != 1) {
      uVar3 = 0x2789;
    }
    local_14 = (void *)CONCAT22(DAT_0065d424,0x161d);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x160),piVar2,&local_14,0x43,0x1e,7,
                 *(int *)((int)*(void **)((int)param_1 + 0x160) + 0x30),10,uVar3,uVar3,0,0);
    FUN_00607740(*(void **)((int)param_1 + 0x160),piVar2,0x2779,0x277a,0x277b,(int *)0x2778,
                 (int *)0x2775,(int *)0x2774,0x2777,(int *)0x2776);
    FUN_004ab1b0(*(void **)((int)param_1 + 0x160),
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xfe01) + 0x20000ff,0x2ffffff,0);
    FUN_004ab3a0(param_1,piVar2,0x40,*(int *)((int)param_1 + 0x160),
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xb) + 0x2785,
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xb) + 0x2786);
  }
  local_10 = (void *)FUN_00618b70(0x14c);
  local_4 = 4;
  if (local_10 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_004aa960(local_10,piVar2,0x86,0x11d,0xf0,100,*(int *)((int)param_1 + 0x150),0x144,
                          *(undefined4 *)((int)param_1 + 0x14c));
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x164) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    FUN_005ffce0(pvVar5,0);
    (**(code **)(**(int **)((int)param_1 + 0x164) + 8))();
    local_10 = (void *)CONCAT22(DAT_0065d424,0x1606);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x164),piVar2,&local_10,0x51,0x1e,7,
                 *(int *)((int)*(void **)((int)param_1 + 0x164) + 0x30),10,0x2796,0x2796,0,0);
    local_10 = (void *)CONCAT22(DAT_0065d424,0x1607);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x164),piVar2,&local_10,0x52,0x1e,0x1d,
                 *(int *)((int)*(void **)((int)param_1 + 0x164) + 0x30),0x1e,0x2797,0x2797,0,0);
    local_10 = (void *)CONCAT22(DAT_0065d424,0x1608);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x164),piVar2,&local_10,0x53,0x1e,0x33,
                 *(int *)((int)*(void **)((int)param_1 + 0x164) + 0x30),10,0x2798,0x2798,0,0);
    local_10 = (void *)CONCAT22(DAT_0065d424,0x1609);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x164),piVar2,&local_10,0x54,0x1e,0x49,
                 *(int *)((int)*(void **)((int)param_1 + 0x164) + 0x30),0x1e,0x2799,0x2799,0,0);
    FUN_004ab1b0(*(void **)((int)param_1 + 0x164),
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xfe01) + 0x20000ff,0x2ffffff,0);
    FUN_00607740(*(void **)((int)param_1 + 0x164),piVar2,0x2779,0x277a,0x277b,(int *)0x2778,
                 (int *)0x2775,(int *)0x2774,0x2777,(int *)0x2776);
    FUN_004ab3a0(param_1,piVar2,0x50,*(int *)((int)param_1 + 0x164),
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xb) + 0x2785,
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xb) + 0x2786);
  }
  local_10 = (void *)FUN_00618b70(0x14c);
  local_4 = 5;
  if (local_10 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_004aa960(local_10,uVar1,0x86,0xed,0xf0,0x8e,*(int *)((int)param_1 + 0x150),0x145,
                          *(undefined4 *)((int)param_1 + 0x14c));
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x168) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    FUN_005ffce0(pvVar5,0);
    (**(code **)(**(int **)((int)param_1 + 0x168) + 8))();
    local_10 = (void *)CONCAT22(DAT_0065d424,0x160c);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x168),piVar2,&local_10,0x62,0x1e,7,
                 *(int *)((int)*(void **)((int)param_1 + 0x168) + 0x30),0x1e,0x2793,0x2793,0,0);
    local_10 = (void *)CONCAT22(DAT_0065d424,0x161c);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x168),piVar2,&local_10,0x65,0x1e,0x1d,
                 *(int *)((int)*(void **)((int)param_1 + 0x168) + 0x30),0x1e,0x2793,0x2793,0,0);
    local_10 = (void *)CONCAT22(DAT_0065d424,0x160d);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x168),piVar2,&local_10,99,0x1e,0x33,
                 *(int *)((int)*(void **)((int)param_1 + 0x168) + 0x30),10,0x2794,0x2794,0,0);
    local_10 = (void *)CONCAT22(DAT_0065d424,0x161b);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x168),piVar2,&local_10,0x66,0x1e,0x49,
                 *(int *)((int)*(void **)((int)param_1 + 0x168) + 0x30),10,0x2794,0x2794,0,0);
    local_10 = (void *)CONCAT22(DAT_0065d424,0x160e);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x168),piVar2,&local_10,100,0x1e,0x5f,
                 *(int *)((int)*(void **)((int)param_1 + 0x168) + 0x30),0x1e,0x2795,0x2795,0,0);
    local_10 = (void *)CONCAT22(DAT_0065d424,0x161a);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x168),piVar2,&local_10,0x67,0x1e,0x75,
                 *(int *)((int)*(void **)((int)param_1 + 0x168) + 0x30),0x1e,0x2795,0x2795,0,0);
    FUN_004ab1b0(*(void **)((int)param_1 + 0x168),
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xfe01) + 0x20000ff,0x2ffffff,0);
    FUN_00607740(*(void **)((int)param_1 + 0x168),piVar2,0x2779,0x277a,0x277b,(int *)0x2778,
                 (int *)0x2775,(int *)0x2774,0x2777,(int *)0x2776);
    FUN_004ab3a0(param_1,piVar2,0x60,*(int *)((int)param_1 + 0x168),
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xb) + 0x2785,
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xb) + 0x2786);
  }
  local_10 = (void *)FUN_00618b70(0x14c);
  local_4 = 6;
  if (local_10 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_004aa960(local_10,uVar1,0x86,0x104,0xf0,0x7a,*(int *)((int)param_1 + 0x150),0x146,
                          *(undefined4 *)((int)param_1 + 0x14c));
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x16c) = pvVar5;
  if (pvVar5 != (void *)0x0) {
    FUN_005ffce0(pvVar5,0);
    (**(code **)(**(int **)((int)param_1 + 0x16c) + 8))();
    uVar3 = 0x279c;
    if (*(int *)(iVar4 + 0x9c) != 1) {
      uVar3 = 0x278f;
    }
    local_10 = (void *)CONCAT22(DAT_0065d424,0x1610);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x16c),piVar2,&local_10,0x71,0x1e,0x49,
                 *(int *)((int)*(void **)((int)param_1 + 0x16c) + 0x30),10,uVar3,uVar3,0,0);
    uVar3 = 0x279d;
    if (*(int *)(iVar4 + 0x9c) != 1) {
      uVar3 = 0x278e;
    }
    local_10 = (void *)CONCAT22(DAT_0065d424,0x1611);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x16c),piVar2,&local_10,0x72,0x1e,0x33,
                 *(int *)((int)*(void **)((int)param_1 + 0x16c) + 0x30),0x1e,uVar3,uVar3,0,0);
    local_10 = (void *)CONCAT22(DAT_0065d424,0x1612);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x16c),piVar2,&local_10,0x73,0x1e,0x5f,
                 *(int *)((int)*(void **)((int)param_1 + 0x16c) + 0x30),10,0x279e,0x279e,0,0);
    uVar3 = 0x279f;
    if (*(int *)(iVar4 + 0x9c) != 1) {
      uVar3 = 0x278c;
    }
    local_10 = (void *)CONCAT22(DAT_0065d424,0x1613);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x16c),piVar2,&local_10,0x74,0x1e,0x1d,
                 *(int *)((int)*(void **)((int)param_1 + 0x16c) + 0x30),0x1e,uVar3,uVar3,0,0);
    local_10 = (void *)CONCAT22(DAT_0065d424,0x1619);
    FUN_004ab1f0(*(void **)((int)param_1 + 0x16c),piVar2,&local_10,0x75,0x1e,7,
                 *(int *)((int)*(void **)((int)param_1 + 0x16c) + 0x30),0x1e,0x2d5a,0x2d5a,0,0);
    FUN_004ab1b0(*(void **)((int)param_1 + 0x16c),
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xfe01) + 0x20000ff,0x2ffffff,0);
    FUN_00607740(*(void **)((int)param_1 + 0x16c),piVar2,0x2779,0x277a,0x277b,(int *)0x2778,
                 (int *)0x2775,(int *)0x2774,0x2777,(int *)0x2776);
    FUN_004ab3a0(param_1,piVar2,0x70,*(int *)((int)param_1 + 0x16c),
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xb) + 0x2785,
                 (-(uint)(*(int *)((int)param_1 + 0x154) != 1) & 0xb) + 0x2786);
  }
  SetFocus(*(HWND *)((int)param_1 + 0x18));
  ExceptionList = pvStack_c;
  return;
}

