
undefined4 __fastcall FUN_005131d0(int param_1)

{
  int *piVar1;
  uint uVar2;
  void *this;
  int iVar3;
  undefined4 uVar4;
  
  uVar4 = 1;
  if (*(int *)(param_1 + 0x1c) != 0) {
    piVar1 = (int *)FUN_0052bed0(param_1);
    uVar2 = (**(code **)(*piVar1 + 4))();
    if ((uVar2 < *(uint *)(param_1 + 8)) || (*(uint *)(param_1 + 0xc) <= uVar2)) {
      uVar4 = 0;
    }
    uVar2 = *(uint *)(param_1 + 0x10);
    this = (void *)FUN_0052bed0(param_1);
    uVar2 = FUN_004f6b90(this,uVar2);
    if (uVar2 == 0) {
      uVar4 = 0;
    }
    if (*(int *)(param_1 + 0x14) != 0) {
      iVar3 = FUN_0052bed0(param_1);
      if ((*(uint *)(iVar3 + 0x24) >> 6 & 3) != *(uint *)(param_1 + 0x18)) {
        uVar4 = 0;
      }
    }
  }
  return uVar4;
}

