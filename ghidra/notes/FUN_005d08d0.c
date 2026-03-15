
void __fastcall FUN_005d08d0(int param_1)

{
  int *piVar1;
  int *piVar2;
  void *this;
  int *piVar3;
  
  for (piVar1 = (int *)(**(code **)(*(int *)(*(int *)(param_1 + 8) + 0x1c) + 0x10))();
      piVar1 != (int *)0x0; piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
    piVar2 = (int *)(**(code **)(*piVar1 + 0x10))();
    piVar3 = piVar2;
    this = (void *)FUN_005aaf90();
    FUN_005a93b0(this,piVar3);
    piVar2[0xf4] = piVar2[0xf3];
    piVar2[0xf1] = piVar2[0xf0];
    piVar2[0x5c] = 10;
    piVar2[0x42] = 1;
    FUN_0059ed70();
    FUN_0059f4d0((int)piVar2);
  }
  return;
}

