
int __thiscall FUN_00511930(void *param_1,int param_2,void *param_3)

{
  bool bVar1;
  void *pvVar2;
  int iVar3;
  void *pvVar4;
  int iVar5;
  uint uVar6;
  void *pvVar7;
  int iStack_54;
  int iStack_50;
  undefined4 auStack_4c [7];
  int iStack_30;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00642140;
  pvStack_c = ExceptionList;
  iVar5 = 1;
  if (param_2 != 0) {
    iStack_50 = *(int *)((int)param_1 + 100);
    iStack_54 = *(int *)((int)param_1 + 0x5c);
    ExceptionList = &pvStack_c;
    iVar5 = FUN_00559e10(&iStack_50,&iStack_54);
    pvVar2 = param_3;
    iVar3 = FUN_00509cc0(param_1,iStack_54,param_3);
    if ((iVar3 == 0) || (iVar5 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar5 = FUN_00509dc0(param_1,iStack_50,pvVar2);
    if ((iVar5 == 0) || (!bVar1)) {
      iVar5 = 0;
    }
    else {
      iVar5 = 1;
    }
    param_2 = 0;
    FUN_0053b6e0(auStack_2c,param_1,3);
    uStack_4 = 0;
    FUN_00513120((int)auStack_2c);
    while (iStack_10 != 0) {
      iVar3 = FUN_0052bed0((int)auStack_2c);
      param_3 = (void *)(*(uint *)(iVar3 + 0x50) >> 4 & 1);
      if (param_3 == (void *)0x0) {
        param_2 = 0;
        iVar3 = FUN_00559df0(&param_2);
        if ((iVar3 == 0) || (iVar5 == 0)) {
          iVar5 = 0;
        }
        else {
          iVar5 = 1;
        }
        if (param_2 != 0) {
          uVar6 = 0xb;
          pvVar7 = pvVar2;
          pvVar4 = (void *)FUN_0052bed0((int)auStack_2c);
          iVar3 = FUN_004f71d0(pvVar4,uVar6,pvVar7);
          if ((iVar3 == 0) || (iVar5 == 0)) {
            iVar5 = 0;
          }
          else {
            iVar5 = 1;
          }
        }
      }
      FUN_005130d0((int)auStack_2c);
    }
    FUN_00526fd0(auStack_4c,param_1,3);
    uStack_4 = CONCAT31(uStack_4._1_3_,1);
    FUN_00513120((int)auStack_4c);
    while (iStack_30 != 0) {
      iVar3 = FUN_0052bed0((int)auStack_4c);
      param_3 = (void *)(*(uint *)(iVar3 + 0x50) >> 4 & 1);
      if (param_3 == (void *)0x0) {
        param_2 = 0;
        iVar3 = FUN_00559df0(&param_2);
        if ((iVar3 == 0) || (iVar5 == 0)) {
          iVar5 = 0;
        }
        else {
          iVar5 = 1;
        }
        if (param_2 != 0) {
          uVar6 = 0xb;
          pvVar7 = pvVar2;
          pvVar4 = (void *)FUN_0052bed0((int)auStack_4c);
          iVar3 = FUN_004f71d0(pvVar4,uVar6,pvVar7);
          if ((iVar3 == 0) || (iVar5 == 0)) {
            iVar5 = 0;
          }
          else {
            iVar5 = 1;
          }
        }
      }
      FUN_005130d0((int)auStack_4c);
    }
    uStack_4 = uStack_4 & 0xffffff00;
    FUN_005270d0(auStack_4c);
    uStack_4 = 0xffffffff;
    FUN_0053b7e0(auStack_2c);
  }
  ExceptionList = pvStack_c;
  return iVar5;
}

