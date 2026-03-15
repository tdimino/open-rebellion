
void __thiscall FUN_005a0ad0(void *this,undefined4 param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00651d9b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x10);
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5270(puVar1);
    *puVar1 = &PTR_FUN_0066bd48;
    puVar1[3] = param_1;
  }
  local_4 = 0xffffffff;
  FUN_005f5e40((void *)((int)this + 0xec),(int)puVar1);
  ExceptionList = local_c;
  return;
}

