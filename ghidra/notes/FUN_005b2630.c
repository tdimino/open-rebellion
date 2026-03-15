
void __cdecl FUN_005b2630(int param_1)

{
  uint *puVar1;
  void *pvVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  uint uVar6;
  int *piVar7;
  
  puVar1 = *(uint **)(param_1 + 4);
  uVar6 = *puVar1;
  pvVar2 = (void *)FUN_00596bd0();
  piVar3 = (int *)FUN_005a6340(pvVar2,uVar6);
  uVar6 = puVar1[1];
  pvVar2 = (void *)FUN_00596bd0();
  iVar4 = FUN_005a6340(pvVar2,uVar6);
  if ((piVar3 != (int *)0x0) && (iVar4 != 0)) {
    piVar7 = piVar3;
    FUN_005aaf90();
    iVar5 = FUN_005ad870(piVar7);
    if (iVar5 == 1) {
      FUN_005aaf90();
      iVar4 = FUN_005ad870(iVar4);
      if (iVar4 == 1) {
        (**(code **)(*piVar3 + 0x40))(puVar1[1]);
      }
    }
  }
  return;
}

