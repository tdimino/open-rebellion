
void __thiscall FUN_0048be60(int param_1,int *param_2,uint *param_3,int param_4)

{
  bool bVar1;
  int *piVar2;
  int iVar3;
  void *this;
  uint *puVar4;
  uint uVar5;
  int *piVar6;
  undefined4 *puVar7;
  uint uStack_5c;
  uint uStack_58;
  uint uStack_54;
  uint uStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined4 uStack_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 auStack_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint uStack_4;
  
  puVar4 = param_3;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006352e0;
  pvStack_c = ExceptionList;
  if (param_2 != (int *)0x160) {
    return;
  }
  ExceptionList = &pvStack_c;
  piVar2 = FUN_004f2d10(*(int *)(param_1 + 0x20),param_3);
  piVar6 = (int *)0x0;
  param_2 = (int *)0x0;
  param_3 = (uint *)0x0;
  uStack_54 = 0x2a;
  uStack_50 = 0x2b;
  uStack_4 = 0;
  if (piVar2 != (int *)0x0) {
    uStack_34 = 0xa0;
    uStack_30 = 0xb0;
    if ((*puVar4 >> 0x18 < 0xa0) || (bVar1 = true, 0xaf < *puVar4 >> 0x18)) {
      bVar1 = false;
    }
    FUN_00619730();
    if (bVar1) {
      FUN_004f26d0((void *)(param_1 + 0x6c),puVar4);
      piVar6 = piVar2;
    }
    param_3 = (uint *)piVar2[7];
  }
  FUN_004f26d0((void *)(param_1 + 0x5c),puVar4);
  if (piVar6 == (int *)0x0) goto LAB_0048c214;
  if (piVar6[0x16] == 0) {
    *(undefined4 *)(param_1 + 0x40) = 1;
  }
  else {
    *(undefined4 *)(param_1 + 0x40) = 0;
  }
  if (*(int *)(param_1 + 0x20) == 1) {
    if (((byte)piVar2[9] & 0xc0) != 0x40) goto LAB_0048bf5d;
  }
  else if ((*(int *)(param_1 + 0x20) != 2) || (((byte)piVar2[9] & 0xc0) != 0x80)) {
LAB_0048bf5d:
    *(undefined4 *)(param_1 + 0x40) = 0;
  }
  if (param_4 != 0) {
    *(undefined4 *)(param_1 + 0x40) = 0;
  }
  iVar3 = (**(code **)(*piVar6 + 0x1ec))();
  if (iVar3 == 0) {
    *(undefined4 *)(param_1 + 100) = 3;
    uStack_3c = 0x28;
    uStack_38 = 0x29;
    uStack_4._0_1_ = 2;
    FUN_00520580(&uStack_54,&uStack_3c);
    uStack_4 = (uint)uStack_4._1_3_ << 8;
LAB_0048c022:
    FUN_00619730();
  }
  else {
    if (iVar3 == 1) {
      *(undefined4 *)(param_1 + 100) = 1;
      uStack_44 = 0x2a;
      uStack_40 = 0x2b;
      uStack_4._0_1_ = 1;
      FUN_00520580(&uStack_54,&uStack_44);
      uStack_4 = (uint)uStack_4._1_3_ << 8;
      goto LAB_0048c022;
    }
    if (iVar3 == 2) {
      *(undefined4 *)(param_1 + 100) = 2;
      uStack_4c = 0x29;
      uStack_48 = 0x2a;
      uStack_4._0_1_ = 3;
      FUN_00520580(&uStack_54,&uStack_4c);
      uStack_4 = (uint)uStack_4._1_3_ << 8;
      goto LAB_0048c022;
    }
    *(undefined4 *)(param_1 + 0x40) = 0;
  }
  if (*(int *)(param_1 + 0x40) != 0) {
    FUN_0052ce40(auStack_2c,param_3,1);
    uStack_4 = CONCAT31(uStack_4._1_3_,4);
    FUN_00513120((int)auStack_2c);
    iVar3 = FUN_0052bed0((int)auStack_2c);
    while (iVar3 != 0) {
      bVar1 = false;
      if (param_2 != (int *)0x0) goto LAB_0048c0cd;
      puVar4 = &uStack_5c;
      this = (void *)FUN_0052bed0((int)auStack_2c);
      puVar4 = FUN_004025b0(this,puVar4);
      if ((uStack_54 <= *puVar4 >> 0x18) && (*puVar4 >> 0x18 < uStack_50)) {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        param_2 = (int *)FUN_0052bed0((int)auStack_2c);
      }
      FUN_005130d0((int)auStack_2c);
      iVar3 = FUN_0052bed0((int)auStack_2c);
    }
    if (param_2 == (int *)0x0) {
      *(undefined4 *)(param_1 + 0x40) = 0;
    }
LAB_0048c0cd:
    uStack_4 = uStack_4 & 0xffffff00;
    FUN_0052cec0(auStack_2c);
  }
  if (*(int *)(param_1 + 0x40) == 0) goto LAB_0048c214;
  iVar3 = *(int *)(param_1 + 100);
  if (iVar3 == 1) {
    if (param_3 == (uint *)0x0) {
      puVar7 = (undefined4 *)0x0;
    }
    else {
      puVar7 = (undefined4 *)((int)param_3 + 0x30);
    }
    FUN_0060b9d0((void *)(param_1 + 0x44),0x71f8,puVar7,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    if (param_3 == (uint *)0x0) {
      puVar7 = (undefined4 *)0x0;
    }
    else {
      puVar7 = (undefined4 *)((int)param_3 + 0x30);
    }
    uVar5 = 0x71f9;
LAB_0048c1cb:
    FUN_0060b9d0((void *)(param_1 + 0x50),uVar5,puVar7,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
  }
  else {
    if (iVar3 == 2) {
      if (param_3 == (uint *)0x0) {
        puVar7 = (undefined4 *)0x0;
      }
      else {
        puVar7 = (undefined4 *)((int)param_3 + 0x30);
      }
      FUN_0060b9d0((void *)(param_1 + 0x44),0x71fa,puVar7,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (param_3 == (uint *)0x0) {
        puVar7 = (undefined4 *)0x0;
        uVar5 = 0x71fb;
      }
      else {
        puVar7 = (undefined4 *)((int)param_3 + 0x30);
        uVar5 = 0x71fb;
      }
      goto LAB_0048c1cb;
    }
    if (iVar3 == 3) {
      if (param_3 == (uint *)0x0) {
        puVar7 = (undefined4 *)0x0;
      }
      else {
        puVar7 = (undefined4 *)((int)param_3 + 0x30);
      }
      FUN_0060b9d0((void *)(param_1 + 0x44),0x71fc,puVar7,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (param_3 == (uint *)0x0) {
        puVar7 = (undefined4 *)0x0;
        uVar5 = 0x71fd;
      }
      else {
        puVar7 = (undefined4 *)((int)param_3 + 0x30);
        uVar5 = 0x71fd;
      }
      goto LAB_0048c1cb;
    }
    *(undefined4 *)(param_1 + 0x40) = 0;
  }
  puVar4 = FUN_004025b0(param_2,&uStack_58);
  uStack_4._0_1_ = 5;
  FUN_004f26d0((void *)(param_1 + 0x5c),puVar4);
  uStack_4 = (uint)uStack_4._1_3_ << 8;
  FUN_00619730();
  uVar5 = FUN_004c5000(param_2);
  *(short *)(param_1 + 0x2e) = (short)uVar5;
  *(undefined4 *)(param_1 + 0x28) = 3;
  *(undefined2 *)(param_1 + 0x30) = 0x44d;
LAB_0048c214:
  uStack_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

