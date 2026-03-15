
undefined4 __cdecl FUN_00611440(int param_1)

{
  int iVar1;
  HMODULE pHVar2;
  int iVar3;
  undefined4 uVar4;
  
  iVar1 = param_1;
  uVar4 = 1;
  param_1 = FUN_00612bd0(&DAT_006be5d0,
                         *(int *)(param_1 + 8) * 0x10000 + (*(uint *)(param_1 + 4) & 0xffff));
  if (param_1 == 0) {
    pHVar2 = (HMODULE)FUN_005fefd0(*(int *)(iVar1 + 8));
    iVar3 = FUN_00611890(iVar1,pHVar2,&param_1);
    if ((iVar3 == 0) && (param_1 != 0)) {
      FUN_00612b30(&DAT_006be5d0,param_1);
      if (((byte)*(undefined4 *)(iVar1 + 0x10) & 0x10) == 0x10) {
        *(undefined4 *)(param_1 + 0x54) = 1;
      }
      return 0;
    }
  }
  else {
    uVar4 = 0xf;
  }
  return uVar4;
}

