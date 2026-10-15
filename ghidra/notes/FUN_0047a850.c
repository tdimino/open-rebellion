
void __fastcall FUN_0047a850(int param_1)

{
  int iVar1;
  
  iVar1 = FUN_0047aa90(*(void **)(param_1 + 0x44),(undefined4 *)(param_1 + 0x28));
  if (iVar1 != 0) {
    *(uint *)(iVar1 + 0x30) = *(uint *)(iVar1 + 0x30) & 0xdfffffff;
  }
  return;
}

