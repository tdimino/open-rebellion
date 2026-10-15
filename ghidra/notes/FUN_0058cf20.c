
void __thiscall FUN_0058cf20(int param_1,undefined4 param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(*(int *)(param_1 + 8) + 0x5c);
  if (iVar1 != 0) {
    *(undefined4 *)(param_1 + 0x20) = 1;
    FUN_00509cc0(iVar1 + -1,param_2);
  }
  return;
}

