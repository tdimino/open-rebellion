
void __fastcall FUN_00485de0(int param_1)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x41c) + 1;
  *(int *)(param_1 + 0x41c) = iVar1;
  if (*(int *)(param_1 + 0x424) <= iVar1) {
    *(undefined4 *)(param_1 + 0x41c) = 0;
    *(undefined4 *)(param_1 + 0x420) = 0;
  }
  return;
}

