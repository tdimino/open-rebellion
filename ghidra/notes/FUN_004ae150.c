
undefined4 FUN_004ae150(int param_1,int param_2)

{
  uint uVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  undefined4 local_4;
  
  iVar2 = 0;
  iVar3 = 0;
  iVar4 = 0;
  local_4 = 0;
  if ((param_1 != 0) && (param_2 != 0)) {
    uVar1 = *(uint *)(param_2 + 0x30);
    if ((uVar1 & 0x200) != 0) {
      iVar2 = *(int *)(param_2 + 0xe4);
      iVar3 = *(int *)(param_2 + 0xe8);
      iVar4 = *(int *)(param_2 + 0xf4);
    }
    if ((uVar1 & 4) != 0) {
      iVar2 = iVar2 + *(int *)(param_2 + 0xb0);
      iVar3 = iVar3 + *(int *)(param_2 + 0xb4);
      iVar4 = iVar4 + *(int *)(param_2 + 0xa4);
    }
    if ((0 < iVar2) && ((iVar2 * 0xc) / 10 < *(int *)(param_1 + 0x58))) {
      if (0 < iVar4) {
        if (*(int *)(param_1 + 0x5c) <= iVar3) {
          return 0;
        }
        if (((*(byte *)(param_1 + 0x38) & 0x10) == 0) && ((uVar1 & 0x400) == 0)) {
          if (*(int *)(param_1 + 0x9c) < 1) {
            if (*(int *)(param_1 + 0x74) < iVar4) {
              return 0;
            }
            return 1;
          }
        }
        else if (*(int *)(param_1 + 0x74) <= iVar4) {
          return 0;
        }
      }
      local_4 = 1;
    }
  }
  return local_4;
}

