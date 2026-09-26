
int __thiscall FUN_0058ba10(void *this,int *param_1,undefined4 param_2)

{
  int iVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  int iVar4;
  int local_74;
  void *local_70;
  undefined4 local_6c [7];
  int local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fe48;
  local_c = ExceptionList;
  iVar4 = 1;
  local_74 = 0;
  if (*(int *)((int)this + 8) != 0) {
    ExceptionList = &local_c;
    local_70 = this;
    FUN_004ffef0(local_2c,*(int *)((int)this + 8),1,*(undefined4 *)((int)this + 0xc));
    local_4 = 0;
    FUN_00513120((int)local_2c);
    do {
      if ((local_10 == 0) || (local_74 != 0)) break;
      iVar1 = FUN_0052bed0((int)local_2c);
      FUN_005130d0((int)local_2c);
      if ((*(byte *)(iVar1 + 0x58) & 0x80) != 0) {
        FUN_00502e30(local_4c,iVar1,1,*(undefined4 *)((int)this + 0xc));
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00513120((int)local_4c);
        while (((iVar4 != 0 && (local_30 != 0)) && (local_74 == 0))) {
          uVar2 = FUN_0052bed0((int)local_4c);
          FUN_005130d0((int)local_4c);
          FUN_00504cc0(local_6c,uVar2,1,*(undefined4 *)((int)local_70 + 0xc));
          local_4 = CONCAT31(local_4._1_3_,2);
          FUN_00513120((int)local_6c);
          while (((iVar4 != 0 && (local_50 != 0)) && (local_74 == 0))) {
            uVar3 = FUN_0052bed0((int)local_6c);
            FUN_005130d0((int)local_6c);
            iVar4 = (**(code **)(*param_1 + 4))(uVar3,uVar2,iVar1,&local_74,param_2);
          }
          local_4._1_3_ = (undefined3)(local_4 >> 8);
          local_4 = CONCAT31(local_4._1_3_,1);
          FUN_00504d40(local_6c);
        }
        local_4 = local_4 & 0xffffff00;
        FUN_00502eb0(local_4c);
        this = local_70;
      }
    } while (iVar4 != 0);
    local_4 = 0xffffffff;
    FUN_004fff70(local_2c);
  }
  ExceptionList = local_c;
  return iVar4;
}

