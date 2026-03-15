
bool __fastcall FUN_005afcc0(int param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  uint uVar4;
  uint uVar5;
  bool bVar6;
  
  uVar5 = 0;
  uVar4 = 0;
  bVar6 = false;
  iVar1 = FUN_00596bd0();
  iVar1 = FUN_005a66a0(iVar1);
  if ((iVar1 != 0) && (iVar1 = FUN_005ba450(iVar1), iVar1 != 0)) {
    for (piVar2 = (int *)(**(code **)(*(int *)(param_1 + 100) + 0x10))(); piVar2 != (int *)0x0;
        piVar2 = (int *)(**(code **)(*piVar2 + 8))()) {
      piVar3 = (int *)(**(code **)(*piVar2 + 0x10))();
      iVar1 = (**(code **)(*piVar3 + 0x20))();
      if (iVar1 == 1) {
        iVar1 = (**(code **)(*piVar2 + 0x10))();
        if (*(int *)(iVar1 + 0x168) == 0) {
          uVar5 = uVar5 + 1;
        }
        else {
          uVar4 = uVar4 + 1;
        }
      }
    }
    bVar6 = uVar4 < uVar5;
  }
  return bVar6;
}

