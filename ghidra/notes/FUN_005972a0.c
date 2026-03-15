
void FUN_005972a0(void)

{
  int iVar1;
  undefined1 auStack_20 [8];
  undefined4 uStack_18;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00650f88;
  local_c = ExceptionList;
  local_4 = 0;
  uStack_18 = 0x5972c4;
  ExceptionList = &local_c;
  iVar1 = FUN_005f7cc0();
  if (iVar1 != 0) {
    FUN_005f2f90(auStack_20,(int)&stack0x00000004);
    FUN_005974e0(iVar1);
  }
  local_4 = 0xffffffff;
  uStack_18 = 0x5972f5;
  FUN_005f2ff0((undefined4 *)&stack0x00000004);
  ExceptionList = local_c;
  return;
}

