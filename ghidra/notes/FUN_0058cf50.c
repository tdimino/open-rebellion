
void __thiscall FUN_0058cf50(int param_1,undefined4 param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(*(int *)(param_1 + 8) + 100);
  if (iVar1 != 0) {
    *(undefined4 *)(param_1 + 0x20) = 1;
    FUN_00509dc0(iVar1 + -1,param_2);
  }
  return;
}

