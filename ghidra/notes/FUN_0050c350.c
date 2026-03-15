
int __thiscall FUN_0050c350(void *this,void *param_1)

{
  undefined4 uVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  void *pvVar6;
  undefined4 local_8c [7];
  int local_70;
  undefined4 local_6c [7];
  int local_50;
  undefined4 local_4c [7];
  int iStack_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641d13;
  pvStack_c = ExceptionList;
  iVar4 = 1;
  ExceptionList = &pvStack_c;
  FUN_004ffe70(local_2c,this,3);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    uVar5 = 3;
    uVar1 = FUN_0052bed0((int)local_2c);
    FUN_00502db0(local_8c,uVar1,uVar5);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00513120((int)local_8c);
    while (local_70 != 0) {
      pvVar6 = param_1;
      piVar2 = (int *)FUN_0052bed0((int)local_8c);
      iVar3 = FUN_0050c580(this,piVar2,pvVar6);
      if ((iVar3 == 0) || (iVar4 == 0)) {
        iVar4 = 0;
      }
      else {
        iVar4 = 1;
      }
      uVar5 = 3;
      uVar1 = FUN_0052bed0((int)local_8c);
      FUN_005039d0(local_6c,uVar1,uVar5);
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00513120((int)local_6c);
      while (local_50 != 0) {
        pvVar6 = param_1;
        piVar2 = (int *)FUN_0052bed0((int)local_6c);
        iVar3 = FUN_0050c580(this,piVar2,pvVar6);
        if ((iVar3 == 0) || (iVar4 == 0)) {
          iVar4 = 0;
        }
        else {
          iVar4 = 1;
        }
        FUN_005130d0((int)local_6c);
      }
      local_4._1_3_ = (undefined3)(local_4 >> 8);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00503ad0(local_6c);
      FUN_005130d0((int)local_8c);
    }
    local_4 = local_4 & 0xffffff00;
    FUN_00502eb0(local_8c);
    FUN_005130d0((int)local_2c);
  }
  FUN_005039d0(local_4c,this,3);
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_00513120((int)local_4c);
  while (iStack_30 != 0) {
    pvVar6 = param_1;
    piVar2 = (int *)FUN_0052bed0((int)local_4c);
    iVar3 = FUN_0050c580(this,piVar2,pvVar6);
    if ((iVar3 == 0) || (iVar4 == 0)) {
      iVar4 = 0;
    }
    else {
      iVar4 = 1;
    }
    FUN_005130d0((int)local_4c);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00503ad0(local_4c);
  local_4 = 0xffffffff;
  FUN_004fff70(local_2c);
  ExceptionList = pvStack_c;
  return iVar4;
}

