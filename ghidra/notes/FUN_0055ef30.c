
int FUN_0055ef30(undefined4 param_1)

{
  int iVar1;
  int iVar2;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649f28;
  local_c = ExceptionList;
  iVar2 = 0;
  ExceptionList = &local_c;
  iVar1 = FUN_00506e20();
  if (iVar1 != 0) {
    FUN_0056f450(local_2c,iVar1,4,param_1);
    local_4 = 0;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      iVar1 = FUN_0052bed0((int)local_2c);
      if ((*(uint *)(iVar1 + 0x50) >> 1 & 1) == 0) {
        iVar2 = iVar2 + 1;
      }
      FUN_005130d0((int)local_2c);
    }
    local_4 = 0xffffffff;
    FUN_0056f4d0(local_2c);
  }
  ExceptionList = local_c;
  return iVar2;
}

