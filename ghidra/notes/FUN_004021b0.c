
void __fastcall FUN_004021b0(int param_1)

{
  bool bVar1;
  uint uVar2;
  
  bVar1 = true;
  *(uint *)(param_1 + 0x30) = *(uint *)(param_1 + 0x30) | 0x80000000;
  uVar2 = *(uint *)(param_1 + 0x38) >> 0x18;
  if ((uVar2 == 0) || (0xfe < uVar2)) {
    bVar1 = false;
  }
  FUN_00619730();
  if (bVar1) {
    FUN_00418a60(*(void **)(param_1 + 0x3c),(undefined4 *)(param_1 + 0x38));
  }
  FUN_004184f0(*(int *)(param_1 + 0x3c));
  return;
}

