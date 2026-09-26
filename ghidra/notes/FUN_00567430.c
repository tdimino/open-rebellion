
void FUN_00567430(void)

{
  undefined4 *puVar1;
  int iVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064b1ff;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005674c0();
  puVar1 = (undefined4 *)FUN_00618b70(0xc);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    DAT_006bb7a4 = (undefined4 *)0x0;
  }
  else {
    DAT_006bb7a4 = FUN_005673d0(puVar1);
  }
  local_4 = 0xffffffff;
  if (((DAT_006bb7a4 != (undefined4 *)0x0) && (DAT_006bb7a4[1] != 0)) &&
     (iVar2 = FUN_005674e0(DAT_006bb7a4), iVar2 != 0)) {
    ExceptionList = local_c;
    return;
  }
  FUN_005674c0();
  ExceptionList = local_c;
  return;
}

