
int __fastcall FUN_004fda10(int param_1)

{
  int iVar1;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640358;
  pvStack_c = ExceptionList;
  iVar1 = 0;
  ExceptionList = &pvStack_c;
  FUN_00502e30(local_2c,param_1,3,*(uint *)(param_1 + 0x24) >> 6 & 3);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while ((local_10 != 0 && (iVar1 == 0))) {
    iVar1 = FUN_0052bed0((int)local_2c);
    if (((*(uint *)(iVar1 + 0x50) >> 2 & 1) == 0) ||
       (iVar1 = FUN_0052bed0((int)local_2c), (*(uint *)(iVar1 + 0x50) >> 5 & 1) != 0)) {
      iVar1 = 0;
    }
    else {
      iVar1 = 1;
    }
    FUN_005130d0((int)local_2c);
  }
  local_4 = 0xffffffff;
  FUN_00502eb0(local_2c);
  ExceptionList = pvStack_c;
  return iVar1;
}

