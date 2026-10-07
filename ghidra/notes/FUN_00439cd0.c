
bool __fastcall FUN_00439cd0(int param_1)

{
  int iVar1;
  bool bVar2;
  
  bVar2 = false;
  if (*(int *)(param_1 + 0x184) != 0) {
    iVar1 = FUN_005f4960((undefined4 *)(*(int *)(param_1 + 0x184) + 0x1c));
    bVar2 = iVar1 == 2;
  }
  return bVar2;
}

