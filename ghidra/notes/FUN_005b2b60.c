
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl FUN_005b2b60(int param_1)

{
  uint *puVar1;
  void *pvVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  uint uVar6;
  uint uVar7;
  int *piVar8;
  
  puVar1 = *(uint **)(param_1 + 4);
  uVar7 = *puVar1;
  pvVar2 = (void *)FUN_00596bd0();
  piVar3 = (int *)FUN_005a6340(pvVar2,uVar7);
  uVar7 = puVar1[1];
  pvVar2 = (void *)FUN_00596bd0();
  iVar4 = FUN_005a6340(pvVar2,uVar7);
  if (iVar4 == 0) {
    FUN_005aaf90();
    iVar5 = FUN_005a9330();
    if ((iVar5 != 0) && (*(uint *)(iVar5 + 0x18) == puVar1[1])) {
      piVar8 = piVar3;
      FUN_005aaf90();
      iVar4 = FUN_005ad870(piVar8);
      if (iVar4 == 0) {
        return;
      }
      (**(code **)(*piVar3 + 0xd0))();
      uVar7 = piVar3[0x3b];
      iVar4 = (**(code **)(*piVar3 + 0xb8))();
      FUN_005c13d0(uVar7,4,iVar4);
      (**(code **)(*piVar3 + 0x1c))();
      return;
    }
  }
  if (piVar3 != (int *)0x0) {
    piVar8 = piVar3;
    FUN_005aaf90();
    iVar5 = FUN_005ad870(piVar8);
    if ((iVar5 != 0) && ((float)piVar3[0xf3] < _DAT_0066c4f0)) {
      (**(code **)(*piVar3 + 0xd0))();
      uVar7 = piVar3[0x3b];
      iVar5 = (**(code **)(*piVar3 + 0xb8))();
      FUN_005c13d0(uVar7,4,iVar5);
      if (iVar4 != 0) {
        uVar7 = puVar1[1];
        uVar6 = *puVar1;
        pvVar2 = (void *)FUN_0059ed70();
        FUN_005a5cf0(pvVar2,uVar6,uVar7);
      }
      (**(code **)(*piVar3 + 0x1c))();
    }
  }
  return;
}

