
uint __fastcall FUN_005562e0(int param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649642;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (*(undefined4 **)(param_1 + 0x68) != (undefined4 *)0x0) {
    ExceptionList = &local_c;
    (**(code **)**(undefined4 **)(param_1 + 0x68))(1);
  }
  puVar1 = (undefined4 *)FUN_00618b70(0x27424);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_0058af80(puVar1);
  }
  local_4 = 0xffffffff;
  *(undefined4 **)(param_1 + 0x68) = puVar1;
  uVar2 = 0;
  if (puVar1 != (undefined4 *)0x0) {
    uVar2 = FUN_0058b060((int)puVar1);
    if (uVar2 == 0) {
      if (*(undefined4 **)(param_1 + 0x68) != (undefined4 *)0x0) {
        (**(code **)**(undefined4 **)(param_1 + 0x68))(1);
      }
      *(undefined4 *)(param_1 + 0x68) = 0;
    }
  }
  ExceptionList = local_c;
  return uVar2;
}

