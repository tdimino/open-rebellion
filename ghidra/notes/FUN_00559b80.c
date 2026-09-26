
bool __cdecl FUN_00559b80(int param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  bool bVar4;
  
  bVar4 = true;
  iVar1 = FUN_00513180(param_1);
  if (iVar1 != 0) {
    iVar2 = FUN_0053e290(iVar1 + -1);
    FUN_00513120(param_1);
    iVar1 = *(int *)(param_1 + 0x1c);
    for (iVar3 = 0; (iVar1 != 0 && (iVar3 < iVar2)); iVar3 = iVar3 + 1) {
      FUN_005130d0(param_1);
      iVar1 = *(int *)(param_1 + 0x1c);
    }
    iVar1 = FUN_0052bed0(param_1);
    bVar4 = iVar1 != 0;
  }
  return bVar4;
}

