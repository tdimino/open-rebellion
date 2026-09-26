
uint __fastcall FUN_004fc680(int param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006401cb;
  local_c = ExceptionList;
  uVar2 = 1;
  if (*(int *)(param_1 + 0x34) == 0) {
    ExceptionList = &local_c;
    puVar1 = (undefined4 *)FUN_00618b70(0xc);
    local_4 = 0;
    if (puVar1 == (undefined4 *)0x0) {
      puVar1 = (undefined4 *)0x0;
    }
    else {
      puVar1 = (undefined4 *)FUN_005f2f50(puVar1);
    }
    local_4 = 0xffffffff;
    *(undefined4 **)(param_1 + 0x34) = puVar1;
    uVar2 = 0;
    if ((puVar1 != (undefined4 *)0x0) && (uVar2 = puVar1[2], uVar2 == 0)) {
      if (puVar1 != (undefined4 *)0x0) {
        (**(code **)*puVar1)(1);
      }
      *(undefined4 *)(param_1 + 0x34) = 0;
    }
  }
  ExceptionList = local_c;
  return uVar2;
}

