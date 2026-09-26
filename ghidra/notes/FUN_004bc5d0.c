
void __fastcall FUN_004bc5d0(int param_1)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639c18;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) & 0xfffffffd;
  iVar2 = FUN_005f5500((void *)(*(int *)(param_1 + 0x44) + 0xd8),*(uint *)(param_1 + 0x54));
  if (iVar2 == 0) {
    ExceptionList = local_c;
    return;
  }
  FUN_004f26d0((uint *)(param_1 + 0x48),(undefined4 *)(iVar2 + 0x3c));
  FUN_004f26d0((uint *)(param_1 + 0x28),(undefined4 *)(iVar2 + 0x38));
  uVar3 = *(uint *)(param_1 + 0x28) >> 0x18;
  local_4 = 0;
  if ((0x13 < uVar3) && (uVar3 < 0x1c)) {
    uVar3 = *(uint *)(param_1 + 0x48) >> 0x18;
    if ((uVar3 < 0x90) || (0x97 < uVar3)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      bVar1 = true;
      goto LAB_004bc696;
    }
  }
  bVar1 = false;
LAB_004bc696:
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 2;
  }
  ExceptionList = local_c;
  return;
}

