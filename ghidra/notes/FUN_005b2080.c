
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl FUN_005b2080(int param_1)

{
  uint *puVar1;
  void *pvVar2;
  int iVar3;
  int *piVar4;
  int iVar5;
  uint uVar6;
  int *piVar7;
  
  puVar1 = *(uint **)(param_1 + 4);
  uVar6 = *puVar1;
  pvVar2 = (void *)FUN_00596bd0();
  iVar3 = FUN_005a6340(pvVar2,uVar6);
  uVar6 = puVar1[1];
  pvVar2 = (void *)FUN_00596bd0();
  piVar4 = (int *)FUN_005a6340(pvVar2,uVar6);
  if ((iVar3 != 0) && (piVar4 != (int *)0x0)) {
    iVar5 = FUN_005aaf90();
    *(undefined4 *)(iVar5 + 0x88c) = 1;
    FUN_005aaf90();
    iVar3 = FUN_005ad870(iVar3);
    if (iVar3 != 0) {
      piVar7 = piVar4;
      FUN_005aaf90();
      iVar3 = FUN_005ad870(piVar7);
      if (iVar3 != 0) {
        (**(code **)(*piVar4 + 0xcc))();
        if (_DAT_0066c4e0 <= (float)puVar1[2]) {
          FUN_005c12d0(puVar1[1],*puVar1,2,0x3f800000,3);
          return;
        }
        FUN_005c12d0(puVar1[1],*puVar1,2,0x3ecccccd,1);
      }
    }
  }
  return;
}

