
void FUN_005aafc0(uint param_1,int *param_2,int *param_3)

{
  void *pvVar1;
  uint uVar2;
  int *piVar3;
  int iVar4;
  
  if (param_1 < 8) {
    pvVar1 = (void *)FUN_0059ed70();
    uVar2 = FUN_0059f5d0(pvVar1,param_1);
    pvVar1 = (void *)FUN_00596bd0();
    piVar3 = (int *)FUN_005a6460(pvVar1,uVar2);
    if (piVar3 != (int *)0x0) {
      FUN_0059ed70();
      iVar4 = FUN_005a5fb0();
      if ((0 < iVar4) && (iVar4 < 9)) {
        iVar4 = FUN_0059ed70();
        iVar4 = FUN_00583c40(iVar4);
        if (iVar4 != 0) {
          iVar4 = FUN_0059ed70();
          iVar4 = FUN_00583c40(iVar4);
          *param_2 = *(int *)(iVar4 + 0xec);
          iVar4 = (**(code **)(*piVar3 + 0x44))();
          *param_3 = iVar4;
          return;
        }
      }
      *param_2 = piVar3[0x1b];
      iVar4 = (**(code **)(*piVar3 + 0x44))();
      *param_3 = iVar4;
      return;
    }
  }
  else {
    uVar2 = param_1 - 8;
    pvVar1 = (void *)FUN_0059ed70();
    uVar2 = FUN_0059f5f0(pvVar1,uVar2);
    pvVar1 = (void *)FUN_00596bd0();
    piVar3 = (int *)FUN_005a6470(pvVar1,uVar2);
    if (piVar3 == (int *)0x0) {
      *param_2 = 0;
      *param_3 = 0;
    }
    else {
      *param_2 = piVar3[0x1b];
      iVar4 = (**(code **)(*piVar3 + 0x44))();
      *param_3 = iVar4;
      iVar4 = FUN_005c6070(piVar3 + 7);
      if (((iVar4 != 0) && (*param_3 == 0)) && (piVar3[0x1a] == 2)) {
        *param_3 = *(int *)(iVar4 + 0x164);
        return;
      }
    }
  }
  return;
}

