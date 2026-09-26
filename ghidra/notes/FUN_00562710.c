
bool FUN_00562710(uint param_1,int *param_2,uint param_3)

{
  void *pvVar1;
  int *this;
  void *pvVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  uint uVar6;
  int local_60;
  uint local_5c;
  undefined4 local_58;
  uint uStack_54;
  undefined4 uStack_50;
  undefined4 auStack_4c [7];
  int iStack_30;
  undefined4 auStack_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a400;
  pvStack_c = ExceptionList;
  uVar6 = 0;
  ExceptionList = &pvStack_c;
  *param_2 = 0;
  local_5c = 0;
  local_60 = 0;
  pvVar1 = (void *)FUN_00505ef0();
  while (pvVar1 != (void *)0x0) {
    if (((*(byte *)((int)pvVar1 + 0x50) & 1) != 0) &&
       ((*(uint *)((int)pvVar1 + 0x24) >> 6 & 3) == param_1)) {
      uVar6 = uVar6 + 1;
    }
    local_5c = 0x90;
    local_58 = 0x98;
    local_4 = 0;
    pvVar1 = (void *)FUN_004f6010(pvVar1,&local_5c,1);
    local_4 = 0xffffffff;
    FUN_00619730();
    local_5c = uVar6;
  }
  this = (int *)FUN_00506410();
  while (this != (int *)0x0) {
    (**(code **)(*this + 0x84))(param_3);
    if (((*(byte *)(this + 0x14) & 1) != 0) && (((uint)this[9] >> 6 & 3) == param_1)) {
      local_60 = local_60 + 1;
    }
    uStack_54 = 8;
    uStack_50 = 0x10;
    local_4 = 1;
    this = (int *)FUN_004f6010(this,&uStack_54,1);
    local_4 = 0xffffffff;
    FUN_00619730();
    uVar6 = local_5c;
  }
  uVar6 = FUN_0053e290(local_60 + -1 + uVar6);
  if ((int)uVar6 < (int)local_5c) {
    param_3 = 0;
    pvVar1 = (void *)FUN_00505ef0();
    while ((pvVar1 != (void *)0x0 && (*param_2 == 0))) {
      if (((*(byte *)((int)pvVar1 + 0x50) & 1) != 0) &&
         ((*(uint *)((int)pvVar1 + 0x24) >> 6 & 3) == param_1)) {
        if (param_3 == uVar6) {
          *param_2 = (int)pvVar1;
        }
        param_3 = param_3 + 1;
      }
      uStack_54 = 0x90;
      uStack_50 = 0x98;
      local_4 = 2;
      pvVar1 = (void *)FUN_004f6010(pvVar1,&uStack_54,1);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  else {
    local_5c = uVar6 - local_5c;
    pvVar1 = (void *)0x0;
    param_3 = 0;
    pvVar2 = (void *)FUN_00506410();
    while ((pvVar2 != (void *)0x0 && (pvVar1 == (void *)0x0))) {
      if (((*(byte *)((int)pvVar2 + 0x50) & 1) != 0) &&
         ((*(uint *)((int)pvVar2 + 0x24) >> 6 & 3) == param_1)) {
        if (param_3 == local_5c) {
          pvVar1 = pvVar2;
        }
        param_3 = param_3 + 1;
      }
      uStack_54 = 8;
      uStack_50 = 0x10;
      local_4 = 3;
      pvVar2 = (void *)FUN_004f6010(pvVar2,&uStack_54,1);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    pvVar2 = FUN_00502e30(auStack_2c,pvVar1,1,param_1);
    local_4 = 4;
    iVar3 = FUN_00513180((int)pvVar2);
    local_4 = 0xffffffff;
    FUN_00502eb0(auStack_2c);
    iVar4 = FUN_0053e290(iVar3 + -1);
    FUN_00502e30(auStack_4c,pvVar1,1,param_1);
    local_4 = 5;
    iVar3 = 0;
    FUN_00513120((int)auStack_4c);
    while ((iStack_30 != 0 && (*param_2 == 0))) {
      if (iVar3 == iVar4) {
        iVar5 = FUN_0052bed0((int)auStack_4c);
        *param_2 = iVar5;
      }
      iVar3 = iVar3 + 1;
      FUN_005130d0((int)auStack_4c);
    }
    local_4 = 0xffffffff;
    FUN_00502eb0(auStack_4c);
  }
  ExceptionList = pvStack_c;
  return *param_2 != 0;
}

