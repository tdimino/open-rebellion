
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 FUN_005ed300(int *param_1)

{
  int iVar1;
  float10 fVar2;
  int *piVar3;
  
  piVar3 = param_1;
  FUN_005aaf90();
  iVar1 = FUN_005ad870(piVar3);
  if (iVar1 == 1) {
    iVar1 = FUN_005aaf90();
    iVar1 = (**(code **)(*(int *)(iVar1 + 0xe5c) + 0xc))();
    if (iVar1 != 0) {
      do {
        if (*(int *)(iVar1 + 0x48) == param_1[0x3b]) {
          return 1;
        }
        iVar1 = *(int *)(iVar1 + 0x10);
      } while (iVar1 != 0);
    }
    fVar2 = (float10)(**(code **)(*param_1 + 0x60))();
    if (fVar2 != (float10)_DAT_0066da94) {
      return 1;
    }
  }
  return 0;
}

