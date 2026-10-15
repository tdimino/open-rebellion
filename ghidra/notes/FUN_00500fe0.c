
int __fastcall FUN_00500fe0(undefined4 param_1)

{
  int iVar1;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640938;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005039d0(local_2c,param_1,3);
  local_4 = 0;
  iVar1 = FUN_00513180((int)local_2c);
  local_4 = 0xffffffff;
  FUN_00503ad0(local_2c);
  ExceptionList = pvStack_c;
  return iVar1;
}

