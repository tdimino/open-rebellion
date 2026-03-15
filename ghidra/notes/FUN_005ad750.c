
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __thiscall FUN_005ad750(void *param_1,int param_2)

{
  int iVar1;
  void *this;
  int iVar2;
  float10 fVar3;
  uint uVar4;
  
  uVar4 = *(uint *)((int)param_1 + param_2 * 4 + 0x958);
  if ((uVar4 != 0) && (iVar1 = FUN_005f5500((void *)((int)param_1 + 0xe50),uVar4), iVar1 != 0)) {
    uVar4 = *(uint *)(iVar1 + 0x5c);
    this = (void *)FUN_00596bd0();
    iVar2 = FUN_005a6340(this,uVar4);
    if ((iVar2 != 0) && (iVar2 = FUN_005ad870(iVar2), iVar2 != 0)) {
      fVar3 = (float10)_DAT_0066c2b8 - (float10)*(float *)(iVar1 + 0x4c);
      if (fVar3 <= (float10)_DAT_0066c298) {
        fVar3 = (float10)_DAT_0066c298;
      }
      if (fVar3 < (float10)_DAT_0066c2b8) {
        return fVar3;
      }
      return (float10)_DAT_0066c2b8;
    }
  }
  return (float10)_DAT_0066c298;
}

