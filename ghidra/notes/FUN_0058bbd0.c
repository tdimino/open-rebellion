
int __thiscall FUN_0058bbd0(void *this,int *param_1,undefined4 param_2)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fe68;
  local_c = ExceptionList;
  iVar1 = *(int *)((int)this + 8);
  iVar3 = 1;
  local_30 = 0;
  if (iVar1 != 0) {
    ExceptionList = &local_c;
    FUN_00504cc0(local_2c,iVar1,1,*(uint *)(iVar1 + 0x24) >> 6 & 3);
    local_4 = 0;
    FUN_00513120((int)local_2c);
    do {
      if ((local_10 == 0) || (local_30 != 0)) break;
      uVar2 = FUN_0052bed0((int)local_2c);
      FUN_005130d0((int)local_2c);
      iVar3 = (**(code **)(*param_1 + 4))(uVar2,0,0,&local_30,param_2);
    } while (iVar3 != 0);
    local_4 = 0xffffffff;
    FUN_00504d40(local_2c);
  }
  ExceptionList = local_c;
  return iVar3;
}

