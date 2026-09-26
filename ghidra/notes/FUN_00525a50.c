
void __fastcall FUN_00525a50(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  if (*(int *)(param_1 + 0x1c) != 0) {
    if (*(int *)(param_1 + 0x10) != 0) {
      FUN_00525b00((void *)(*(int *)(param_1 + 4) + 0x84),(undefined4 *)(param_1 + 0x28));
    }
    iVar1 = FUN_004ece60((uint *)(param_1 + 0x28));
    *(int *)(param_1 + 0x1c) = iVar1;
    *(uint *)(param_1 + 0x20) = (uint)(iVar1 == 0);
  }
  if (*(int *)(param_1 + 0x20) != 0) {
    if (*(int *)(param_1 + 0x14) != 0) {
      FUN_00525b00((void *)(*(int *)(param_1 + 4) + 0x8c),(undefined4 *)(param_1 + 0x28));
    }
    iVar1 = FUN_004ece60((uint *)(param_1 + 0x28));
    *(int *)(param_1 + 0x20) = iVar1;
    *(uint *)(param_1 + 0x24) = (uint)(iVar1 == 0);
  }
  if (*(int *)(param_1 + 0x24) != 0) {
    if (*(int *)(param_1 + 0x18) != 0) {
      FUN_00525b00((void *)(*(int *)(param_1 + 4) + 0x94),(undefined4 *)(param_1 + 0x28));
    }
    uVar2 = FUN_004ece60((uint *)(param_1 + 0x28));
    *(undefined4 *)(param_1 + 0x24) = uVar2;
  }
  return;
}

