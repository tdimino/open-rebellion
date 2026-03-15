
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __fastcall FUN_005b0be0(int *param_1)

{
  int iVar1;
  void *pvVar2;
  float10 fVar3;
  
  fVar3 = (float10)(**(code **)(*param_1 + 0x90))();
  if (fVar3 == (float10)_DAT_0066c484) {
    fVar3 = (float10)_DAT_0066c488;
  }
  else {
    iVar1 = FUN_00597530();
    if (*(int *)(iVar1 + 0x668) == 0) {
      iVar1 = param_1[0x5a];
      pvVar2 = (void *)FUN_005aaf90();
      fVar3 = FUN_005ad750(pvVar2,iVar1);
    }
    else {
      iVar1 = param_1[0x5a];
      pvVar2 = (void *)FUN_00596f30();
      fVar3 = FUN_005afb70(pvVar2,iVar1);
    }
    fVar3 = fVar3 + (float10)(float)param_1[0xff];
    if ((float10)_DAT_0066c4b8 <= fVar3) {
      return (float10)_DAT_0066c4b8;
    }
  }
  return fVar3;
}

