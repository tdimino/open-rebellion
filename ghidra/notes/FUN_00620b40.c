
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __cdecl FUN_00620b40(double param_1,double param_2,double *param_3)

{
  bool bVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  double dVar4;
  int iVar5;
  
  dVar4 = param_1;
  if (param_1 < _DAT_0066e908) {
    param_1 = -param_1;
  }
  if (param_2 == INFINITY) {
    if (_DAT_0066e910 < param_1) {
      *(undefined4 *)param_3 = DAT_006af498;
      *(undefined4 *)((int)param_3 + 4) = DAT_006af49c;
      return 0;
    }
    if (param_1 < _DAT_0066e910) {
      *(undefined4 *)param_3 = 0;
      *(undefined4 *)((int)param_3 + 4) = 0;
      return 0;
    }
  }
  else {
    if (param_2 != -INFINITY) {
      if (dVar4 == INFINITY) {
        if (_DAT_0066e908 < param_2) {
          *(undefined4 *)param_3 = DAT_006af498;
          *(undefined4 *)((int)param_3 + 4) = DAT_006af49c;
          return 0;
        }
        bVar1 = param_2 < _DAT_0066e908;
        *(undefined4 *)param_3 = 0;
        if (bVar1) {
          *(undefined4 *)((int)param_3 + 4) = 0;
          return 0;
        }
      }
      else {
        if (dVar4 != -INFINITY) {
          return 0;
        }
        iVar5 = FUN_00620d70(param_2);
        uVar3 = DAT_006af4bc;
        uVar2 = DAT_006af49c;
        if (_DAT_0066e908 < param_2) {
          if (iVar5 != 1) {
            *(undefined4 *)param_3 = DAT_006af498;
            *(undefined4 *)((int)param_3 + 4) = uVar2;
            return 0;
          }
          *param_3 = -(double)CONCAT44(DAT_006af49c,DAT_006af498);
          return 0;
        }
        if (param_2 < _DAT_0066e908) {
          if (iVar5 != 1) {
            *(undefined4 *)param_3 = 0;
            *(undefined4 *)((int)param_3 + 4) = 0;
            return 0;
          }
          *(undefined4 *)param_3 = DAT_006af4b8;
          *(undefined4 *)((int)param_3 + 4) = uVar3;
          return 0;
        }
        *(undefined4 *)param_3 = 0;
      }
      *(undefined4 *)((int)param_3 + 4) = 0x3ff00000;
      return 0;
    }
    if (_DAT_0066e910 < param_1) {
      *(undefined4 *)param_3 = 0;
      *(undefined4 *)((int)param_3 + 4) = 0;
      return 0;
    }
    if (param_1 < _DAT_0066e910) {
      *(undefined4 *)param_3 = DAT_006af498;
      *(undefined4 *)((int)param_3 + 4) = DAT_006af49c;
      return 0;
    }
  }
  *(undefined4 *)param_3 = DAT_006af4a0;
  *(undefined4 *)((int)param_3 + 4) = DAT_006af4a4;
  return 1;
}

