
bool __fastcall FUN_005ad930(int param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  uint uVar4;
  bool bVar5;
  uint local_4;
  
  bVar5 = false;
  local_4 = 0;
  uVar4 = 0;
  iVar1 = FUN_005a9330();
  if (iVar1 != 0) {
    iVar1 = FUN_005ba450(iVar1);
    if ((iVar1 != 0) && (iVar1 = FUN_005d0ae0(), iVar1 == 0)) {
      for (piVar2 = (int *)(**(code **)(*(int *)(param_1 + 0x988) + 0x10))(); piVar2 != (int *)0x0;
          piVar2 = (int *)(**(code **)(*piVar2 + 8))()) {
        piVar3 = (int *)(**(code **)(*piVar2 + 0x10))();
        iVar1 = (**(code **)(*piVar3 + 0x20))();
        if (iVar1 == 1) {
          iVar1 = (**(code **)(*piVar2 + 0x10))();
          if (*(int *)(iVar1 + 0x168) == 0) {
            local_4 = local_4 + 1;
          }
          else {
            uVar4 = uVar4 + 1;
          }
        }
      }
      bVar5 = uVar4 < local_4;
    }
    if ((bVar5) &&
       (piVar2 = (int *)(**(code **)(*(int *)(*(int *)(param_1 + 8) + 0x70) + 0xc))(),
       piVar2 != (int *)0x0)) {
      while ((iVar1 = (**(code **)(*piVar2 + 0xc))(), *(int *)(iVar1 + 0x68) != 6 &&
             (iVar1 = (**(code **)(*piVar2 + 0xc))(), *(int *)(iVar1 + 100) != 6))) {
        piVar2 = (int *)(**(code **)(*piVar2 + 4))();
        if (piVar2 == (int *)0x0) {
          return true;
        }
      }
      bVar5 = false;
    }
  }
  return bVar5;
}

