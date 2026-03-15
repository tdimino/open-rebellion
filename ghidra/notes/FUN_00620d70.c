
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __cdecl FUN_00620d70(double param_1)

{
  double dVar1;
  uint uVar2;
  float10 fVar3;
  
  uVar2 = FUN_00626720(param_1._0_4_,param_1._4_4_);
  if ((uVar2 & 0x90) == 0) {
    fVar3 = FUN_0061f410(param_1);
    if ((double)fVar3 == param_1) {
      dVar1 = param_1 / _DAT_0066e918;
      fVar3 = FUN_0061f410(dVar1);
      if (fVar3 == (float10)dVar1) {
        return 2;
      }
      return 1;
    }
  }
  return 0;
}

