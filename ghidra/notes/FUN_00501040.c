
int __fastcall FUN_00501040(undefined4 param_1)

{
  int iVar1;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640958;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00504c40(local_2c,param_1,3);
  local_4 = 0;
  iVar1 = FUN_00513180((int)local_2c);
  local_4 = 0xffffffff;
  FUN_00504d40(local_2c);
  ExceptionList = pvStack_c;
  return iVar1;
}

