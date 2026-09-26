
int __cdecl FUN_00559700(int param_1,int param_2)

{
  int iVar1;
  
  iVar1 = 0;
  if (param_1 == 1) {
    if (param_2 == 0) {
      iVar1 = FUN_0053e290(DAT_006bb464);
      return iVar1 + DAT_006bb3d4;
    }
    iVar1 = FUN_0053e290(DAT_006bb4a4);
    iVar1 = iVar1 + DAT_006bb444;
  }
  else {
    if (param_1 == 2) {
      if (param_2 == 0) {
        iVar1 = FUN_0053e290(DAT_006bb42c);
        return 100 - (iVar1 + DAT_006bb3e8);
      }
      iVar1 = FUN_0053e290(DAT_006bb468);
      return 100 - (iVar1 + DAT_006bb4c4);
    }
    if (param_1 == 3) {
      iVar1 = FUN_0053e290(DAT_006bb3b8);
      return iVar1 + ((DAT_00661a84 + DAT_00661a88) / 2 - DAT_006bb3b8 / 2);
    }
  }
  return iVar1;
}

