
uint __thiscall FUN_0058dc10(void *this,undefined4 param_1)

{
  void *pvVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  uint uVar5;
  int *local_74;
  int *local_70;
  int aiStack_6c [2];
  int local_64 [2];
  int iStack_5c;
  int local_58;
  void *local_54;
  undefined4 local_4c [7];
  int iStack_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006500c0;
  pvStack_c = ExceptionList;
  iVar4 = 1;
  ExceptionList = &pvStack_c;
  FUN_00527550(local_2c,*(undefined4 *)((int)this + 0x10),1);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  do {
    if (local_10 == 0) break;
    FUN_0058d230(local_64,(void *)((int)this + 8));
    local_4 = CONCAT31(local_4._1_3_,1);
    local_58 = FUN_0053e290(*(int *)((int)this + 0x1c) + -1);
    if ((((iVar4 != 0) &&
         (iVar4 = FUN_0058ca70((void *)((int)this + 8),local_64,param_1), pvVar1 = local_54,
         iVar4 != 0)) && (iVar4 != 0)) && (local_54 != (void *)0x0)) {
      iVar4 = FUN_0052bed0((int)local_2c);
      iVar2 = FUN_00520b70(iVar4);
      iVar3 = 0;
      local_70 = (int *)0x0;
      local_74 = (int *)0x0;
      iVar4 = FUN_00509330(*(void **)((int)this + 0x10),3,(int *)&local_74);
      if (iVar4 != 0) {
        if (local_74 != (int *)0x0) {
          iVar3 = (**(code **)(*local_74 + 500))();
        }
        if ((iVar4 != 0) && (iVar4 = FUN_0055e930(iVar2,iVar3,(int *)&local_70), iVar4 != 0)) {
          iVar2 = FUN_005006e0((int)pvVar1);
          FUN_00501ba0(pvVar1,iVar2 + (int)local_70);
        }
      }
    }
    local_4 = local_4 & 0xffffff00;
    FUN_0058d280(local_64);
    FUN_005130d0((int)local_2c);
  } while (iVar4 != 0);
  FUN_005272e0(local_4c,*(undefined4 *)((int)this + 0x10),1);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_00513120((int)local_4c);
  while ((iVar4 != 0 && (iStack_30 != 0))) {
    FUN_0058d230(local_64,(void *)((int)this + 8));
    local_4 = CONCAT31(local_4._1_3_,3);
    local_58 = FUN_0053e290(*(int *)((int)this + 0x1c) + -1);
    if ((iVar4 != 0) &&
       (((iVar4 = FUN_0058ca70((void *)((int)this + 8),local_64,param_1), pvVar1 = local_54,
         iVar4 != 0 && (iVar4 != 0)) && (local_54 != (void *)0x0)))) {
      iVar4 = FUN_0052bed0((int)local_4c);
      iVar2 = FUN_00520b70(iVar4);
      iVar3 = 0;
      local_74 = (int *)0x0;
      local_70 = (int *)0x0;
      iVar4 = FUN_00509330(*(void **)((int)this + 0x10),3,(int *)&local_70);
      if (iVar4 != 0) {
        if (local_70 != (int *)0x0) {
          iVar3 = (**(code **)(*local_70 + 500))();
        }
        if ((iVar4 != 0) && (iVar4 = FUN_0055e930(iVar2,iVar3,(int *)&local_74), iVar4 != 0)) {
          iVar2 = FUN_0053a6e0((int)pvVar1);
          FUN_00501bc0(pvVar1,iVar2 + (int)local_74);
        }
      }
    }
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_0058d280(local_64);
    FUN_005130d0((int)local_4c);
  }
  pvVar1 = (void *)((int)this + 8);
  FUN_0058d4f0(aiStack_6c,pvVar1);
  local_4._0_1_ = 4;
  uVar5 = 0;
  if (iVar4 != 0) {
    iVar4 = FUN_0058ca70(pvVar1,aiStack_6c,param_1);
    uVar5 = 0;
    if (iVar4 != 0) {
      FUN_0058d150(local_64,pvVar1);
      local_4._0_1_ = 5;
      iVar4 = FUN_0058ca70(pvVar1,local_64,param_1);
      uVar5 = (uint)(iVar4 != 0);
      if (iStack_5c == 0) {
        *(undefined4 *)((int)this + 0x30) = 1;
      }
      local_4._0_1_ = 4;
      FUN_0058d190(local_64);
    }
  }
  local_4._0_1_ = 2;
  FUN_0058d530(aiStack_6c);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00527360(local_4c);
  local_4 = 0xffffffff;
  FUN_00527650(local_2c);
  ExceptionList = pvStack_c;
  return uVar5;
}

