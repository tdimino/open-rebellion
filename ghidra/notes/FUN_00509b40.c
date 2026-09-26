
undefined4 __thiscall FUN_00509b40(void *this,undefined4 param_1,int *param_2)

{
  int iVar1;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641a38;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_2 = 0;
  FUN_004ffef0(local_2c,this,4,param_1);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while ((local_10 != 0 && (*param_2 == 0))) {
    iVar1 = FUN_0052bed0((int)local_2c);
    if ((*(uint *)(iVar1 + 0x50) >> 3 & 1) == 0) {
      iVar1 = FUN_0052bed0((int)local_2c);
      if ((*(uint *)(iVar1 + 0x58) >> 2 & 1) != 0) {
        iVar1 = FUN_0052bed0((int)local_2c);
        *param_2 = iVar1;
      }
    }
    FUN_005130d0((int)local_2c);
  }
  local_4 = 0xffffffff;
  FUN_004fff70(local_2c);
  ExceptionList = pvStack_c;
  return 1;
}

