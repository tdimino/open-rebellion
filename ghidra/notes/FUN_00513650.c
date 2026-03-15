
undefined4 __fastcall FUN_00513650(int param_1)

{
  void *this;
  int iVar1;
  undefined4 local_20 [5];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642488;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this = (void *)FUN_0051c740(local_20);
  local_4 = 0;
  iVar1 = FUN_0051c7c0(this,param_1 + 0x58);
  local_4 = 0xffffffff;
  FUN_0051c780(local_20);
  if (iVar1 != 0) {
    *(undefined4 *)(param_1 + 0x5c) = 1;
  }
  FUN_0051c010(2);
  ExceptionList = local_c;
  return 1;
}

