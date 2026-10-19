
void __fastcall FUN_004e54f0(int param_1)

{
  int iVar1;
  
  for (iVar1 = thunk_FUN_005f5060(*(int *)(param_1 + 0xc) + 0xd8); iVar1 != 0;
      iVar1 = *(int *)(iVar1 + 0x10)) {
    *(uint *)(iVar1 + 0x60) = *(uint *)(iVar1 + 0x60) & 0xf7ffffff;
  }
  FUN_004fcdd0((undefined4 *)(param_1 + 0x10));
  return;
}

