
undefined4 FUN_0058cf80(int *param_1,int *param_2)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  
  *param_2 = 0;
  piVar1 = (int *)*param_1;
  if (piVar1 != (int *)0x0) {
    iVar2 = (**(code **)(*piVar1 + 0x1c4))();
    iVar3 = (**(code **)(*piVar1 + 0x1c0))();
    *param_2 = iVar3 - iVar2;
    return 1;
  }
  piVar1 = (int *)param_1[1];
  if (piVar1 != (int *)0x0) {
    iVar2 = (**(code **)(*piVar1 + 0x1e4))();
    iVar3 = (**(code **)(*piVar1 + 0x1d8))();
    *param_2 = iVar3 - iVar2;
    return 1;
  }
  if (param_1[2] != 0) {
    *param_2 = DAT_006bb760;
    return 1;
  }
  if (param_1[3] != 0) {
    *param_2 = DAT_006bb770;
  }
  return 1;
}

