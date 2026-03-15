
undefined4 __fastcall FUN_005d1920(int param_1)

{
  undefined4 *puVar1;
  
  *(undefined4 *)(param_1 + 0x1c) = 0;
  if (*(undefined4 **)(param_1 + 0x2c) != (undefined4 *)0x0) {
    (**(code **)**(undefined4 **)(param_1 + 0x2c))(1);
    *(undefined4 *)(param_1 + 0x2c) = 0;
    *(undefined4 *)(param_1 + 0x28) = 0;
  }
  if (*(undefined4 **)(param_1 + 0x34) != (undefined4 *)0x0) {
    (**(code **)**(undefined4 **)(param_1 + 0x34))(1);
    *(undefined4 *)(param_1 + 0x34) = 0;
    *(undefined4 *)(param_1 + 0x30) = 0;
  }
  *(undefined4 *)(param_1 + 0x30) = 3;
  puVar1 = (undefined4 *)FUN_00618b70(4);
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    *puVar1 = &PTR_FUN_0066d118;
  }
  *(undefined4 **)(param_1 + 0x34) = puVar1;
  *(undefined4 *)(param_1 + 0x28) = 4;
  puVar1 = (undefined4 *)FUN_00618b70(4);
  if (puVar1 != (undefined4 *)0x0) {
    *puVar1 = &PTR_FUN_0066d100;
    *(undefined4 **)(param_1 + 0x2c) = puVar1;
    return *(undefined4 *)(param_1 + 0x28);
  }
  *(undefined4 *)(param_1 + 0x2c) = 0;
  return *(undefined4 *)(param_1 + 0x28);
}

