
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __fastcall FUN_005ba270(int *param_1)

{
  int iVar1;
  void *pvVar2;
  float10 fVar3;
  
  fVar3 = (float10)(**(code **)(*param_1 + 0x90))();
  if (fVar3 == (float10)_DAT_0066c6d0) {
    fVar3 = (float10)_DAT_0066c6e0;
  }
  else {
    iVar1 = FUN_00597530();
    if (*(int *)(iVar1 + 0x668) == 1) {
      iVar1 = param_1[0x5a];
      pvVar2 = (void *)FUN_00596f30();
      FUN_005afc00(pvVar2,iVar1);
    }
    else {
      iVar1 = param_1[0x5a];
      pvVar2 = (void *)FUN_005aaf90();
      FUN_005ad7e0(pvVar2,iVar1);
    }
    fVar3 = (float10)(float)param_1[0xff] - (float10)_DAT_0066c6cc;
    if ((float10)_DAT_0066c700 <= fVar3) {
      return (float10)_DAT_0066c700;
    }
  }
  return fVar3;
}

