
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __fastcall FUN_005d19a0(int *param_1)

{
  int iVar1;
  int iVar2;
  void *pvVar3;
  int *piVar4;
  float10 fVar5;
  
  iVar1 = (**(code **)(*param_1 + 0xc))();
  if (iVar1 != 0) {
    if (param_1[0x2c] == 1) {
      return 1;
    }
    (**(code **)(*param_1 + 0x14))(0);
    if ((float)param_1[0xf] == _DAT_0066d0f8) {
      return 1;
    }
    iVar1 = FUN_00596f30();
    iVar2 = (**(code **)(*param_1 + 4))();
    if (iVar2 == 0) {
      piVar4 = *(int **)(iVar1 + 0x250);
    }
    else {
      piVar4 = *(int **)(iVar1 + 0x24c);
    }
    (**(code **)(*piVar4 + 0x14))(0);
    iVar1 = (**(code **)(*param_1 + 4))();
    pvVar3 = (void *)FUN_00596f30();
    fVar5 = FUN_005afa30(pvVar3,iVar1);
    if ((float10)_DAT_0066d0f8 < fVar5) {
      if ((float10)(float)param_1[0xf] / fVar5 < (float10)(float)param_1[9]) {
        return 1;
      }
      if ((float10)(float)param_1[0xf] / fVar5 == (float10)_DAT_0066d0f8) {
        return 1;
      }
    }
  }
  return 0;
}

