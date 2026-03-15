
void __fastcall FUN_00468f20(int param_1)

{
  uint uVar1;
  bool bVar2;
  int iVar3;
  
  bVar2 = false;
  for (iVar3 = *(int *)(*(int *)(param_1 + 0x16c) + 8); iVar3 != 0; iVar3 = *(int *)(iVar3 + 8)) {
    if (*(int *)(iVar3 + 0x68) != 0) {
      uVar1 = *(uint *)(*(int *)(iVar3 + 0x68) + 0x34);
      *(undefined4 *)(iVar3 + 0x6c) = 0;
      if (((*(uint *)(param_1 + 0x158) & uVar1) == 0) &&
         ((*(int *)(param_1 + 0x11c) != 0x79 || (uVar1 == 0x200)))) {
        *(undefined4 *)(iVar3 + 0x6c) = 0;
        *(uint *)(iVar3 + 0x3c) = *(uint *)(iVar3 + 0x3c) & 0xfffffffe;
      }
      else {
        *(undefined4 *)(iVar3 + 0x6c) = 1;
        if ((bVar2) || ((*(byte *)(iVar3 + 0x3c) & 1) != 0)) {
          bVar2 = true;
        }
        else {
          bVar2 = false;
        }
      }
    }
  }
  iVar3 = (**(code **)(**(int **)(param_1 + 0x16c) + 8))();
  if ((iVar3 != 0) && (!bVar2)) {
    *(uint *)(iVar3 + 0x3c) = *(uint *)(iVar3 + 0x3c) | 1;
  }
  return;
}

