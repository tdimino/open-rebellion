
int __cdecl FUN_00611580(int param_1)

{
  void *this;
  int iVar1;
  
  this = (void *)FUN_00612bd0(&DAT_006be5d0,
                              (*(uint *)(param_1 + 4) & 0xffff) + *(int *)(param_1 + 8) * 0x10000);
  if (this == (void *)0x0) {
    iVar1 = 0x10;
  }
  else {
    iVar1 = FUN_00612630(this,(uint *)(param_1 + 0xc),*(uint *)(param_1 + 0x10) >> 3 & 1);
    if (iVar1 == 0) {
      *(uint *)(param_1 + 0x10) = *(uint *)(param_1 + 0x10) | 0x20;
      FUN_00611230();
      return 0;
    }
  }
  FUN_00611230();
  return iVar1;
}

