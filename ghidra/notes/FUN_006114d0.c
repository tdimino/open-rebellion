
undefined4 __cdecl FUN_006114d0(int param_1,LPCSTR param_2,int param_3)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  
  iVar1 = param_1;
  uVar3 = 1;
  param_1 = FUN_00612bd0(&DAT_006be5d0,
                         (*(uint *)(param_1 + 4) & 0xffff) + *(int *)(param_1 + 8) * 0x10000);
  if (param_1 == 0) {
    iVar2 = FUN_00611960(iVar1,param_2,param_3,&param_1);
    if ((iVar2 == 0) && (param_1 != 0)) {
      *(uint *)(param_1 + 0x18) = (*(uint *)(iVar1 + 4) & 0xffff) + *(int *)(iVar1 + 8) * 0x10000;
      FUN_00612b30(&DAT_006be5d0,param_1);
      if (((byte)*(undefined4 *)(iVar1 + 0x10) & 0x10) == 0x10) {
        *(undefined4 *)(param_1 + 0x54) = 1;
      }
      return 0;
    }
  }
  else {
    uVar3 = 0xf;
  }
  return uVar3;
}

