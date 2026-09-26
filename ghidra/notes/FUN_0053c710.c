
bool __fastcall FUN_0053c710(int param_1)

{
  undefined4 *puVar1;
  int iVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064649a;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x10);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_00536eb0(puVar1);
  }
  local_4 = 0xffffffff;
  *(undefined4 **)(param_1 + 0x44) = puVar1;
  if (puVar1 != (undefined4 *)0x0) {
    puVar1 = (undefined4 *)FUN_00618b70(8);
    local_4 = 1;
    if (puVar1 == (undefined4 *)0x0) {
      iVar2 = 0;
    }
    else {
      iVar2 = FUN_00520570(puVar1);
    }
    *(int *)(param_1 + 0x48) = iVar2;
    if (iVar2 != 0) {
      ExceptionList = local_c;
      return true;
    }
  }
  local_4 = 0xffffffff;
  FUN_0053c7d0(param_1);
  ExceptionList = local_c;
  return false;
}

