
void FUN_0058b370(void)

{
  undefined4 *puVar1;
  int iVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fddf;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0058b400();
  puVar1 = (undefined4 *)FUN_00618b70(0xc);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    DAT_006bbe50 = (undefined4 *)0x0;
  }
  else {
    DAT_006bbe50 = FUN_0058b310(puVar1);
  }
  local_4 = 0xffffffff;
  if (((DAT_006bbe50 != (undefined4 *)0x0) && (DAT_006bbe50[1] != 0)) &&
     (iVar2 = FUN_0058b420(DAT_006bbe50), iVar2 != 0)) {
    ExceptionList = local_c;
    return;
  }
  FUN_0058b400();
  ExceptionList = local_c;
  return;
}

