
/* WARNING: Removing unreachable block (ram,0x005d5150) */
/* WARNING: Removing unreachable block (ram,0x005d56dc) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void * __thiscall FUN_005d4d10(void *this,int *param_1,undefined4 param_2,undefined4 param_3)

{
  int *piVar1;
  undefined4 *puVar2;
  int iVar3;
  undefined4 uVar4;
  float unaff_EBX;
  longlong lVar5;
  int *piStack_2f0;
  undefined4 uStack_2ec;
  undefined4 uStack_2a0;
  undefined4 uStack_29c;
  short local_13c;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006547a3;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005d8980(this,param_3,param_1);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0066d278;
  *(undefined ***)((int)this + 0x1c) = &PTR_LAB_0066d268;
  piVar1 = (int *)((int)this + 0x58);
  *(undefined4 *)((int)this + 0x44) = 0;
  *(undefined4 *)((int)this + 0x48) = 0;
  *piVar1 = 0;
  iVar3 = (**(code **)(*DAT_006bcbd0 + 0x10))();
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(*(int *)*piVar1 + 0xf4))();
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(*DAT_006bcbd0 + 0x10))();
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(*(int *)*piVar1 + 0x90))();
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(*DAT_006bcbd0 + 0x30))();
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(*(int *)*piVar1 + 0x30))();
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(*DAT_006bcbd0 + 0x30))();
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(*param_1 + 0x30))();
  FUN_005d8b00(iVar3);
  if ((int *)*piVar1 != (int *)0x0) {
    (**(code **)(*(int *)*piVar1 + 8))();
  }
  local_13c = 0;
  if (*(int **)((int)this + 0x44) != (int *)0x0) {
    (**(code **)(**(int **)((int)this + 0x44) + 8))();
  }
  if (*(int **)((int)this + 0x48) != (int *)0x0) {
    (**(code **)(**(int **)((int)this + 0x48) + 8))();
  }
  puVar2 = (undefined4 *)((int)this + 0x3c);
  iVar3 = (**(code **)(*DAT_006bcbd0 + 0x18))();
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(*(int *)*puVar2 + 0x5c))();
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(*(int *)*puVar2 + 0x34))();
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(*(int *)*puVar2 + 100))();
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(*(int *)*puVar2 + 0x78))();
  FUN_005d8b00(iVar3);
  puVar2 = (undefined4 *)((int)this + 0x40);
  iVar3 = (**(code **)(*DAT_006bcbd0 + 0x18))();
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(*(int *)*puVar2 + 0x5c))();
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(*(int *)*puVar2 + 0x34))();
  uStack_29c = 0x5d526d;
  FUN_005d8b00(iVar3);
  uStack_29c = 0;
  uStack_2a0 = 0;
  iVar3 = (**(code **)(*(int *)*puVar2 + 100))();
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(*(int *)*puVar2 + 0x78))();
  FUN_005d8b00(iVar3);
  *(undefined4 *)((int)this + 0x68) = 0;
  FUN_005d58c0((int)this);
  FUN_005f5650((undefined4 *)&stack0xfffffd78);
  FUN_005fefd0(6);
  if (*(int *)(DAT_006bcbd4 + 0x67c) == 0) {
    iVar3 = (**(code **)(*param_1 + 0xc4))();
  }
  else {
    piVar1 = (int *)(DAT_006bcbd4 + 0x168);
    for (iVar3 = FUN_005f5060(DAT_006bcbd4 + 0x16c); iVar3 != 0; iVar3 = *(int *)(iVar3 + 0x10)) {
      if (*(short *)(iVar3 + 0x18) == 0x13a6) goto LAB_005d5300;
    }
    iVar3 = 0;
LAB_005d5300:
    if (iVar3 == 0) {
      uVar4 = (**(code **)(*piVar1 + 4))();
      *(undefined4 *)((int)this + 0x60) = uVar4;
    }
    else {
      *(undefined4 *)((int)this + 0x60) = *(undefined4 *)(iVar3 + 0x1c);
    }
    iVar3 = (**(code **)(**(int **)((int)this + 0x60) + 0x3c))();
    FUN_005d8b00(iVar3);
    iVar3 = (**(code **)(**(int **)((int)this + 0x60) + 0x40))();
    FUN_005d8b00(iVar3);
    iVar3 = (**(code **)(*param_1 + 0xcc))();
  }
  FUN_005d8b00(iVar3);
  if (*(int *)(DAT_006bcbd4 + 0x680) != 0) {
    iVar3 = (**(code **)(*DAT_006bcbd0 + 0x10))();
    FUN_005d8b00(iVar3);
    lVar5 = __ftol();
    FUN_005f5700(&stack0xfffffd54,(int)lVar5);
    iVar3 = **(int **)((int)this + 0x54);
    lVar5 = __ftol();
    FUN_005f5700(&stack0xfffffd54,(int)lVar5);
    iVar3 = (**(code **)(iVar3 + 0xf4))();
    uStack_2ec = 0x5d541e;
    FUN_005d8b00(iVar3);
    piVar1 = (int *)(DAT_006bcbd4 + 0x168);
    for (iVar3 = FUN_005f5060(DAT_006bcbd4 + 0x16c); iVar3 != 0; iVar3 = *(int *)(iVar3 + 0x10)) {
      if (*(short *)(iVar3 + 0x18) == local_13c) goto LAB_005d5457;
    }
    iVar3 = 0;
LAB_005d5457:
    if (iVar3 == 0) {
      uVar4 = (**(code **)(*piVar1 + 4))();
      *(undefined4 *)((int)this + 0x5c) = uVar4;
    }
    else {
      *(undefined4 *)((int)this + 0x5c) = *(undefined4 *)(iVar3 + 0x1c);
    }
    piStack_2f0 = *(int **)((int)this + 0x5c);
    uStack_2ec = 0x80;
    iVar3 = (**(code **)(*piStack_2f0 + 0x48))();
    FUN_005d8b00(iVar3);
    piStack_2f0 = (int *)0x0;
    iVar3 = (**(code **)(**(int **)((int)this + 0x5c) + 0x50))(*(int **)((int)this + 0x5c),1);
    FUN_005d8b00(iVar3);
    iVar3 = (**(code **)(**(int **)((int)this + 0x5c) + 0x3c))(*(int **)((int)this + 0x5c),0x80,0);
    FUN_005d8b00(iVar3);
    iVar3 = (**(code **)(**(int **)((int)this + 0x5c) + 0x40))(*(int **)((int)this + 0x5c),1,0);
    FUN_005d8b00(iVar3);
    iVar3 = (**(code **)(**(int **)((int)this + 0x5c) + 0x44))
                      (*(int **)((int)this + 0x5c),0xbf000000,0xbf000000,0);
    FUN_005d8b00(iVar3);
    *(undefined4 *)((int)this + 0x6c) = 0;
    FUN_005d5910((int)this);
  }
  iVar3 = (**(code **)(*DAT_006bcbd0 + 0x10))();
  FUN_005d8b00(iVar3);
  lVar5 = __ftol();
  FUN_005f5700(&stack0xfffffd54,(int)lVar5);
  iVar3 = **(int **)((int)this + 0x50);
  lVar5 = __ftol();
  FUN_005f5700(&stack0xfffffd54,(int)lVar5);
  iVar3 = (**(code **)(iVar3 + 0xf4))();
  uStack_2ec = 0x5d55a9;
  FUN_005d8b00(iVar3);
  uStack_2ec = *(undefined4 *)((int)this + 0x50);
  piStack_2f0 = DAT_006bcbd0;
  iVar3 = (**(code **)(*DAT_006bcbd0 + 0x10))();
  FUN_005d8b00(iVar3);
  piStack_2f0 = (int *)-((float)_DAT_0066d250 * -0.5);
  (**(code **)(**(int **)((int)this + 0x4c) + 0xf4))
            (*(int **)((int)this + 0x4c),*(undefined4 *)((int)this + 0x50),0,
             (float)_DAT_0066d258 * -0.5);
  piStack_2f0 = (int *)0xbf000000;
  piVar1 = (int *)(DAT_006bcbd4 + 0x168);
  for (iVar3 = FUN_005f5060(DAT_006bcbd4 + 0x16c); iVar3 != 0; iVar3 = *(int *)(iVar3 + 0x10)) {
    if (*(short *)(iVar3 + 0x18) == (short)piStack_2f0) goto LAB_005d5632;
  }
  iVar3 = 0;
LAB_005d5632:
  if (iVar3 == 0) {
    uVar4 = (**(code **)(*piVar1 + 4))(&piStack_2f0);
    *(undefined4 *)((int)this + 100) = uVar4;
  }
  else {
    *(undefined4 *)((int)this + 100) = *(undefined4 *)(iVar3 + 0x1c);
  }
  iVar3 = (**(code **)(**(int **)((int)this + 100) + 0x48))(*(int **)((int)this + 100),0x40,0x40,0);
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(**(int **)((int)this + 100) + 0x50))(*(int **)((int)this + 100),1,0);
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(**(int **)((int)this + 100) + 0x3c))(*(int **)((int)this + 100),0x80,0);
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(**(int **)((int)this + 100) + 0x40))(*(int **)((int)this + 100),1,0);
  FUN_005d8b00(iVar3);
  iVar3 = (**(code **)(**(int **)((int)this + 100) + 0x44))
                    (*(int **)((int)this + 100),unaff_EBX * (float)_DAT_0066d260,
                     unaff_EBX * (float)_DAT_0066d260,0);
  FUN_005d8b00(iVar3);
  *(undefined4 *)((int)this + 0x38) = 0;
  FUN_005d5970(this);
  *(undefined4 *)((int)this + 0x34) = 0;
  FUN_005f56a0(&uStack_2a0);
  ExceptionList = (void *)0x3f000000;
  return this;
}

