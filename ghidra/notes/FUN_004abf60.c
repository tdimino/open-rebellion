
int __fastcall FUN_004abf60(int *param_1)

{
  int iVar1;
  int iVar2;
  
  iVar2 = 0;
  if ((int *)param_1[5] != (int *)0x0) {
    iVar2 = FUN_005fc0e0((int *)param_1[5]);
    return iVar2;
  }
  if ((param_1[0x17] != 0) && ((int *)param_1[8] != (int *)0x0)) {
    iVar2 = FUN_005fc0e0((int *)param_1[8]);
    return iVar2;
  }
  iVar1 = (**(code **)(*param_1 + 0xc))();
  if (iVar1 != 0) {
    iVar2 = 0x14;
  }
  return iVar2;
}

