
bool __fastcall FUN_0041e2d0(int param_1)

{
  int iVar1;
  bool bVar2;
  
  iVar1 = FUN_0041e1b0();
  bVar2 = *(int *)(param_1 + 0x10) == iVar1;
  if (bVar2) {
    *(undefined4 *)(param_1 + 0x10) = 0;
    FUN_00401950();
  }
  return bVar2;
}

