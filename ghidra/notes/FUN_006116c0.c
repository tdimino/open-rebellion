
undefined4 __cdecl FUN_006116c0(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  uVar2 = 5;
  iVar1 = FUN_00612bd0(&DAT_006be5d0,
                       (*(uint *)(param_1 + 4) & 0xffff) + *(int *)(param_1 + 8) * 0x10000);
  if (iVar1 != 0) {
    uVar2 = 0;
    do {
      FUN_006120d0(&DAT_006be5d0,(*(uint *)(param_1 + 4) & 0xffff) + *(int *)(param_1 + 8) * 0x10000
                  );
      iVar1 = FUN_00612bd0(&DAT_006be5d0,
                           (*(uint *)(param_1 + 4) & 0xffff) + *(int *)(param_1 + 8) * 0x10000);
    } while (iVar1 != 0);
  }
  return uVar2;
}

