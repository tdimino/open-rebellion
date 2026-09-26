
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl FUN_005b1ea0(int param_1)

{
  uint *puVar1;
  void *pvVar2;
  int iVar3;
  int *piVar4;
  uint uVar5;
  int *piVar6;
  
  puVar1 = *(uint **)(param_1 + 4);
  uVar5 = *puVar1;
  pvVar2 = (void *)FUN_00596bd0();
  iVar3 = FUN_005a6340(pvVar2,uVar5);
  uVar5 = puVar1[1];
  pvVar2 = (void *)FUN_00596bd0();
  piVar4 = (int *)FUN_005a6340(pvVar2,uVar5);
  if ((iVar3 != 0) && (piVar4 != (int *)0x0)) {
    FUN_005aaf90();
    iVar3 = FUN_005ad870(iVar3);
    if (iVar3 != 0) {
      piVar6 = piVar4;
      FUN_005aaf90();
      iVar3 = FUN_005ad870(piVar6);
      if (iVar3 != 0) {
        (**(code **)(*piVar4 + 0xc4))();
        iVar3 = FUN_005aaf90();
        *(undefined4 *)(iVar3 + 0x88c) = 1;
        if (_DAT_0066c4d8 <= (float)puVar1[2]) {
          FUN_005c12d0(puVar1[1],*puVar1,0,0x3f000000,2);
          return;
        }
        FUN_005c12d0(puVar1[1],*puVar1,0,0x3f000000,1);
      }
    }
  }
  return;
}

