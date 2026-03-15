
void FUN_005a0240(void)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  int *piVar4;
  void *pvVar5;
  uint uVar6;
  
  iVar1 = FUN_005aaf90();
  iVar1 = *(int *)(iVar1 + 0x9ec);
  iVar2 = FUN_005aaf90();
  piVar3 = (int *)(**(code **)(*(int *)(*(int *)(iVar2 + 8 + iVar1 * 4) + 0x58) + 0x10))();
  if (piVar3 != (int *)0x0) {
    while( true ) {
      piVar4 = (int *)(**(code **)(*piVar3 + 0x10))();
      iVar2 = FUN_005b1b70(piVar4);
      if (iVar2 == 0) break;
      piVar3 = (int *)(**(code **)(*piVar3 + 8))();
      if (piVar3 == (int *)0x0) {
        return;
      }
    }
    iVar2 = 1;
    if (iVar1 == 0) {
      uVar6 = 0x95;
      pvVar5 = (void *)FUN_005ba8b0();
      FUN_005ba980(pvVar5,uVar6,iVar2);
      return;
    }
    uVar6 = 0x10e;
    pvVar5 = (void *)FUN_005ba8b0();
    FUN_005ba980(pvVar5,uVar6,iVar2);
  }
  return;
}

