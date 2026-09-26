
int __thiscall FUN_0058cba0(void *this,int *param_1,undefined4 param_2)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  int local_74;
  void *local_70;
  undefined4 auStack_6c [7];
  int iStack_50;
  undefined4 local_4c [7];
  int iStack_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064ffd0;
  local_c = ExceptionList;
  iVar1 = *(int *)((int)this + 8);
  iVar3 = 1;
  local_74 = 0;
  if (iVar1 != 0) {
    ExceptionList = &local_c;
    local_70 = this;
    if (*(int *)((int)this + 0x2c) != 0) {
      ExceptionList = &local_c;
      FUN_00504cc0(local_2c,iVar1,1,*(uint *)(iVar1 + 0x24) >> 6 & 3);
      local_4 = 0;
      FUN_00513120((int)local_2c);
      do {
        if ((local_10 == 0) || (local_74 != 0)) break;
        uVar2 = FUN_0052bed0((int)local_2c);
        FUN_005130d0((int)local_2c);
        iVar3 = (**(code **)(*param_1 + 4))(0,uVar2,0,0,&local_74,param_2);
      } while (iVar3 != 0);
      FUN_00527050(local_4c,*(int *)((int)local_70 + 8),1,
                   *(uint *)(*(int *)((int)local_70 + 8) + 0x24) >> 6 & 3);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00513120((int)local_4c);
      while (((iVar3 != 0 && (iStack_30 != 0)) && (local_74 == 0))) {
        uVar2 = FUN_0052bed0((int)local_4c);
        FUN_005130d0((int)local_4c);
        iVar3 = (**(code **)(*param_1 + 4))(uVar2,0,0,0,&local_74,param_2);
      }
      FUN_00526d70(auStack_6c,*(int *)((int)local_70 + 8),1,
                   *(uint *)(*(int *)((int)local_70 + 8) + 0x24) >> 6 & 3);
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00513120((int)auStack_6c);
      while (((iVar3 != 0 && (iStack_50 != 0)) && (local_74 == 0))) {
        uVar2 = FUN_0052bed0((int)auStack_6c);
        FUN_005130d0((int)auStack_6c);
        iVar3 = (**(code **)(*param_1 + 4))(uVar2,0,0,0,&local_74,param_2);
      }
      local_4._1_3_ = (uint3)((uint)local_4 >> 8);
      local_4._0_1_ = 1;
      FUN_00526df0(auStack_6c);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_005270d0(local_4c);
      local_4 = 0xffffffff;
      FUN_00504d40(local_2c);
    }
    if (*(int *)((int)local_70 + 0x30) != 0) {
      FUN_0053b760(local_2c,*(int *)((int)local_70 + 8),1,
                   *(uint *)(*(int *)((int)local_70 + 8) + 0x24) >> 6 & 3);
      local_4 = 3;
      FUN_00513120((int)local_2c);
      while (((iVar3 != 0 && (local_10 != 0)) && (local_74 == 0))) {
        uVar2 = FUN_0052bed0((int)local_2c);
        FUN_005130d0((int)local_2c);
        iVar3 = (**(code **)(*param_1 + 4))(uVar2,0,0,0,&local_74,param_2);
      }
      local_4 = 0xffffffff;
      FUN_0053b7e0(local_2c);
    }
    if ((*(int *)((int)local_70 + 0x34) != 0) && (iVar3 != 0)) {
      if ((*(int *)(*(int *)((int)local_70 + 8) + 0x5c) != 0) && (local_74 == 0)) {
        iVar3 = (**(code **)(*param_1 + 4))(0,0,1,0,&local_74,param_2);
      }
      if (((iVar3 != 0) && (*(int *)(*(int *)((int)local_70 + 8) + 100) != 0)) && (local_74 == 0)) {
        iVar3 = (**(code **)(*param_1 + 4))(0,0,0,1,&local_74,param_2);
      }
    }
  }
  ExceptionList = local_c;
  return iVar3;
}

