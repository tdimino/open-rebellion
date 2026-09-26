
void __thiscall FUN_005c7c00(void *this,undefined4 *param_1)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653dfb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar2 = (undefined4 *)FUN_00618b70(0x30);
  local_4 = 0;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    uVar1 = param_1[4];
    FUN_005f5c10(puVar2);
    puVar2[6] = uVar1;
    puVar2[7] = &PTR_LAB_0066c2c0;
    if (puVar2 + 7 != param_1) {
      puVar2[8] = param_1[1];
      puVar2[9] = param_1[2];
      puVar2[10] = param_1[3];
      puVar2[0xb] = param_1[4];
    }
    *puVar2 = &PTR_FUN_0066cbe0;
  }
  local_4 = 0xffffffff;
  FUN_005f4f10((void *)((int)this + 0x88),(int)puVar2);
  ExceptionList = local_c;
  return;
}

