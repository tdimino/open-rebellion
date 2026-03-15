
uint __fastcall FUN_0058b8e0(int param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fe1f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0058b8c0(param_1);
  puVar1 = (undefined4 *)FUN_00618b70(0x10);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_00595230(puVar1);
  }
  *(undefined4 **)(param_1 + 8) = puVar1;
  uVar2 = (uint)(puVar1 != (undefined4 *)0x0);
  if (uVar2 != 0) {
    uVar2 = puVar1[3];
  }
  ExceptionList = local_c;
  return uVar2;
}

