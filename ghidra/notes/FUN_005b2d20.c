
undefined4 __fastcall FUN_005b2d20(int *param_1)

{
  void *pvVar1;
  int *piVar2;
  int iVar3;
  uint uVar4;
  
  iVar3 = param_1[0x5c];
  if ((((iVar3 != 2) && (iVar3 != 5)) && (iVar3 != 4)) &&
     (((iVar3 != 6 && (iVar3 != 10)) && (iVar3 != 1)))) {
    param_1[0x3d] = 0;
    param_1[0x40] = 0;
    param_1[0x44] = 0;
    FUN_0059ed70();
    FUN_0059f4d0((int)param_1);
    return 0;
  }
  param_1[0xf3] = 0;
  if (param_1[0x49] == 0) {
    (**(code **)(*param_1 + 0x68))();
  }
  uVar4 = param_1[0x5d];
  pvVar1 = (void *)FUN_00596bd0();
  piVar2 = (int *)FUN_005a6480(pvVar1,uVar4);
  if (piVar2 != (int *)0x0) {
    (**(code **)(*piVar2 + 0x54))(param_1[0x3b]);
    FUN_005caef0(piVar2,param_1[0x3b]);
  }
  piVar2 = param_1;
  pvVar1 = (void *)FUN_005aaf90();
  FUN_005a9130(pvVar1,(int)piVar2);
  iVar3 = (**(code **)(*param_1 + 0x20))();
  if (iVar3 == 0) {
    FUN_005b8d80((int)(param_1 + 0x106));
  }
  param_1[0x5c] = 0xb;
  param_1[0x3d] = 0;
  param_1[0x40] = 0;
  param_1[0x44] = 0;
  FUN_0059ed70();
  FUN_0059f4d0((int)param_1);
  return 1;
}

