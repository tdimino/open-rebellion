
void __fastcall FUN_005cf940(int param_1)

{
  int *piVar1;
  int iVar2;
  
  *(undefined4 *)(param_1 + 4) = 0;
  for (piVar1 = (int *)(**(code **)(*(int *)(*(int *)(param_1 + 8) + 0x1c) + 0x10))();
      piVar1 != (int *)0x0; piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
    iVar2 = (**(code **)(*piVar1 + 0x10))();
    *(undefined4 *)(iVar2 + 0x35c) = 0;
  }
  return;
}

