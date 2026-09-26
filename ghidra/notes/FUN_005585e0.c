
bool __cdecl FUN_005585e0(int param_1,int param_2,undefined4 *param_3)

{
  int iVar1;
  int iVar2;
  bool bVar3;
  
  bVar3 = true;
  *param_3 = 1;
  if (param_1 < 1) {
    *param_3 = 7;
    return true;
  }
  if (param_2 == 0) {
    *param_3 = 1;
    return true;
  }
  iVar1 = FUN_0053e290(DAT_006bb378);
  iVar2 = FUN_0053e1b0(param_2,param_2 + param_1);
  if (DAT_006bb380 <= iVar2 + iVar1) {
    iVar1 = FUN_0053e290(DAT_006bb384);
    iVar1 = FUN_0055bed0(5,iVar1);
    bVar3 = iVar1 != 0;
    if (bVar3) {
      *param_3 = *(undefined4 *)(iVar1 + 0x24);
    }
  }
  return bVar3;
}

