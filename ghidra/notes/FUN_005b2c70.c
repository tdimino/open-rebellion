
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl FUN_005b2c70(int param_1)

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
    if (iVar5 != 0) {
      FUN_005aaf90();
      iVar4 = FUN_005ad870(iVar4);
      if (iVar4 != 0) {
        (**(code **)(*piVar3 + 200))();
        if (_DAT_0066c4e4 <= (float)puVar1[2]) {
          FUN_005c12d0(puVar1[1],*puVar1,3,0x3e4ccccd,3);
          return;
        }
        FUN_005c12d0(puVar1[1],*puVar1,3,0x3e4ccccd,1);
      }
    }
  }
  return;
}

