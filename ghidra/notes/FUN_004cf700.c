
void __fastcall FUN_004cf700(int param_1)

{
  bool bVar1;
  int iVar2;
  
  bVar1 = false;
  iVar2 = *(int *)(param_1 + 0x2c);
  if (*(int *)(iVar2 + 0x204) < 0) {
    for (iVar2 = thunk_FUN_005f5060(iVar2 + 0xd8); iVar2 != 0; iVar2 = *(int *)(iVar2 + 0x10)) {
      if ((*(uint *)(iVar2 + 0x60) & 0x4000000) == 0) {
        bVar1 = true;
      }
    }
    if (!bVar1) {
      FUN_0042f350((void *)(*(int *)(param_1 + 0x2c) + 0xd8));
      return;
    }
  }
  else if (0 < *(int *)(iVar2 + 0x204)) {
    FUN_0042f3d0((void *)(iVar2 + 0xd8));
  }
  return;
}

