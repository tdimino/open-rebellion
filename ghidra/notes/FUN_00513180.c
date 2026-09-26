
int __fastcall FUN_00513180(int param_1)

{
  int iVar1;
  int iVar2;
  
  iVar2 = 0;
  FUN_00513120(param_1);
  iVar1 = *(int *)(param_1 + 0x1c);
  while (iVar1 != 0) {
    iVar2 = iVar2 + 1;
    FUN_005130d0(param_1);
    iVar1 = *(int *)(param_1 + 0x1c);
  }
  return iVar2;
}

