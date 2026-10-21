
short __fastcall FUN_005f50e0(int param_1)

{
  int iVar1;
  short sVar2;
  
  sVar2 = 0;
  for (iVar1 = FUN_005f5060(param_1); iVar1 != 0; iVar1 = *(int *)(iVar1 + 0x10)) {
    sVar2 = sVar2 + 1;
  }
  return sVar2;
}

