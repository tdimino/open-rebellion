
undefined4 __thiscall FUN_005a9d40(int param_1,undefined4 param_2)

{
  int *piVar1;
  int *piVar2;
  
  piVar1 = (int *)(**(code **)(*(int *)(*(int *)(param_1 + 8) + 0x70) + 0xc))();
  while (piVar1 != (int *)0x0) {
    piVar2 = (int *)(**(code **)(*piVar1 + 0xc))();
    piVar1 = (int *)(**(code **)(*piVar1 + 4))();
    if (piVar2 != (int *)0x0) {
      (**(code **)(*piVar2 + 0xc))(param_2);
      (**(code **)(*(int *)piVar2[0x15] + 0x40))();
      (**(code **)(*piVar2 + 0x10))(param_2);
    }
  }
  piVar1 = (int *)(**(code **)(*(int *)(*(int *)(param_1 + 0xc) + 0x70) + 0xc))();
  while (piVar1 != (int *)0x0) {
    piVar2 = (int *)(**(code **)(*piVar1 + 0xc))();
    piVar1 = (int *)(**(code **)(*piVar1 + 4))();
    if (piVar2 != (int *)0x0) {
      (**(code **)(*piVar2 + 0xc))(param_2);
      (**(code **)(*(int *)piVar2[0x15] + 0x40))();
      (**(code **)(*piVar2 + 0x10))(param_2);
    }
  }
  return 1;
}

