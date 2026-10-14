
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __thiscall FUN_005ed370(int *param_1,int *param_2)

{
  int iVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  int *unaff_ESI;
  float10 fVar6;
  int iStack_20;
  float fVar7;
  undefined4 auStack_c [2];
  int *piStack_4;
  
  iVar1 = param_2[0x19];
  if (((iVar1 != 3) && (iVar1 != 6)) && (iVar1 != 2)) {
    iStack_20 = 0x5ed3aa;
    piStack_4 = param_1;
    iVar1 = (**(code **)(*param_2 + 4))();
    if (iVar1 == 4) {
      iStack_20 = 0x5ed3b8;
      for (piVar2 = (int *)(**(code **)(param_2[7] + 0x10))(); piVar2 != (int *)0x0;
          piVar2 = (int *)(**(code **)(*piVar2 + 8))()) {
        iStack_20 = 0x5ed3c5;
        iVar1 = (**(code **)(*piVar2 + 0x10))();
        if (*(int *)(iVar1 + 0x170) == 1) {
          return 1;
        }
        iStack_20 = 0x5ed3d9;
        iVar1 = (**(code **)(*piVar2 + 0x10))();
        if (*(int *)(iVar1 + 0x170) == 3) {
          return 1;
        }
        iStack_20 = 0x5ed3ec;
      }
    }
    iStack_20 = 0x5ed3fd;
    iVar1 = FUN_005d2190(param_2 + 0x23);
    if ((iVar1 == 0) || (iStack_20 = iVar1, iVar3 = (**(code **)(*param_1 + 0x10))(), iVar3 == 0)) {
      return 0;
    }
    auStack_c[0] = 0;
    fVar6 = (float10)(**(code **)(*param_1 + 4))(iVar1,param_2,auStack_c,&stack0xfffffff0);
    fVar7 = (float)fVar6;
    if (((float10)_DAT_0066da84 <= fVar6) || (fVar7 == _DAT_0066da9c)) {
      iVar3 = param_2[0x14];
      iVar4 = FUN_005aaf90();
      for (piVar2 = (int *)(**(code **)(*(int *)(*(int *)(iVar4 + 8 + iVar3 * 4) + 0x70) + 0xc))();
          piVar2 != (int *)0x0; piVar2 = (int *)(**(code **)(*piVar2 + 4))()) {
        if (piVar2[6] != param_2[6]) {
          iVar3 = (**(code **)(*piVar2 + 0xc))();
          iVar3 = FUN_005d2190((int *)(iVar3 + 0x8c));
          if (iVar3 == iVar1) {
            iVar3 = *param_1;
            uVar5 = (**(code **)(*piVar2 + 0xc))(&stack0xffffffe4,&iStack_20);
            fVar6 = (float10)(**(code **)(iVar3 + 4))(iVar1,uVar5);
            fVar7 = (float)fVar6;
            param_1 = unaff_ESI;
          }
        }
      }
      if (_DAT_0066da84 <= fVar7) {
        return 0;
      }
      if (fVar7 == _DAT_0066da9c) {
        return 0;
      }
    }
  }
  return 1;
}

