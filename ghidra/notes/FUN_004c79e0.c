
undefined4 __fastcall FUN_004c79e0(int param_1)

{
  void *this;
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  undefined4 local_4;
  
  this = *(void **)(param_1 + 0x28);
  *(undefined4 *)(param_1 + 0x3c) = 0;
  local_4 = 0;
  iVar4 = *(int *)((int)this + 0x80) - *(int *)((int)this + 0x84);
  if (0 < iVar4) {
    iVar1 = FUN_004c7cd0(this,(undefined4 *)(param_1 + 0x40));
    if (iVar1 == 0) {
      iVar1 = 0;
      uVar2 = 0;
    }
    else if ((*(byte *)(iVar1 + 0x24) & 1) == 0) {
      iVar1 = 0x2000;
      uVar2 = 1;
    }
    else {
      iVar1 = 1;
      uVar2 = 2;
    }
    *(undefined4 *)(param_1 + 0x3c) = 1;
    iVar1 = FUN_0049dad0(*(void **)(param_1 + 0x28),(uint *)(param_1 + 0x48),(void *)0x29,0x2000,
                         iVar1,uVar2);
    if ((((*(uint *)(param_1 + 0x48) & 0xff000000) != 0) && (iVar1 != 0)) &&
       (iVar3 = iVar1 * *(int *)(param_1 + 0x3c), iVar3 - iVar4 != 0 && iVar4 <= iVar3)) {
      *(int *)(param_1 + 0x3c) = iVar4 / iVar1;
    }
    if (0 < *(int *)(param_1 + 0x3c)) {
      local_4 = 1;
    }
  }
  return local_4;
}

