
int __thiscall FUN_0058ca70(void *this,int *param_1,undefined4 param_2)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  int local_54;
  void *local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064ffa0;
  local_c = ExceptionList;
  iVar3 = 1;
  local_54 = 0;
  if (*(int *)((int)this + 8) != 0) {
    ExceptionList = &local_c;
    local_50 = this;
    FUN_004ffef0(local_2c,*(int *)((int)this + 8),1,*(undefined4 *)((int)this + 0xc));
    local_4 = 0;
    FUN_00513120((int)local_2c);
    do {
      if ((local_10 == 0) || (local_54 != 0)) break;
      iVar1 = FUN_0052bed0((int)local_2c);
      FUN_005130d0((int)local_2c);
      if ((*(byte *)(iVar1 + 0x58) & 0x40) != 0) {
        FUN_00502e30(local_4c,iVar1,1,*(undefined4 *)((int)local_50 + 0xc));
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00513120((int)local_4c);
        while (((iVar3 != 0 && (local_30 != 0)) && (local_54 == 0))) {
          uVar2 = FUN_0052bed0((int)local_4c);
          FUN_005130d0((int)local_4c);
          iVar3 = (**(code **)(*param_1 + 4))(uVar2,iVar1,&local_54,param_2);
        }
        local_4 = local_4 & 0xffffff00;
        FUN_00502eb0(local_4c);
      }
    } while (iVar3 != 0);
    local_4 = 0xffffffff;
    FUN_004fff70(local_2c);
  }
  ExceptionList = local_c;
  return iVar3;
}

