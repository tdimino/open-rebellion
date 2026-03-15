
void __thiscall FUN_0059b690(void *this,int *param_1)

{
  int *piVar1;
  int iVar2;
  int *piVar3;
  undefined4 *puVar4;
  
  piVar3 = param_1;
  puVar4 = (undefined4 *)((int)this + 0xf8);
  param_1 = (int *)0x4;
  do {
    piVar1 = (int *)puVar4[-0x14];
    if (*piVar3 == 1) {
      iVar2 = FUN_005ffea0((int)piVar1);
      if (iVar2 == 0) {
        FUN_00600c40(this,piVar1,*puVar4);
        (**(code **)(*piVar1 + 0xc))();
      }
    }
    if (*piVar3 == 0) {
      iVar2 = FUN_005ffea0((int)piVar1);
      if (iVar2 == 1) {
        FUN_00600c40(this,piVar1,&DAT_006bc2b4);
        (**(code **)(*piVar1 + 0x10))();
      }
    }
    puVar4 = puVar4 + 1;
    piVar3 = piVar3 + 1;
    param_1 = (int *)((int)param_1 + -1);
  } while (param_1 != (int *)0x0);
  return;
}

