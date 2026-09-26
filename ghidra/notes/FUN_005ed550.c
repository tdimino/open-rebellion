
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 FUN_005ed550(int *param_1)

{
  int iVar1;
  int iVar2;
  float10 fVar3;
  int *piVar4;
  
  piVar4 = param_1;
  FUN_005aaf90();
  iVar1 = FUN_005ad870(piVar4);
  if (iVar1 == 1) {
    iVar1 = param_1[0x5a];
    iVar2 = FUN_005aaf90();
    if (*(int *)(*(int *)(iVar2 + 8 + (uint)(iVar1 == 0) * 4) + 0x84) == 0) {
      fVar3 = (float10)(**(code **)(*param_1 + 0x60))();
      if (fVar3 != (float10)_DAT_0066da94) {
        return 1;
      }
    }
    else {
      iVar1 = FUN_005b1b70(param_1);
      if (iVar1 != 0) {
        return 1;
      }
      fVar3 = FUN_005b1bc0((int)param_1);
      if ((fVar3 != (float10)_DAT_0066da94) && (iVar1 = FUN_005b7d90((int)param_1), iVar1 != 1)) {
        return 1;
      }
    }
  }
  return 0;
}

