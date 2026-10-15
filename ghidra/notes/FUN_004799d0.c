
void __fastcall FUN_004799d0(int param_1)

{
  if ((*(uint *)(param_1 + 0x60) & 0x2000000) != 0) {
    if (*(int **)(param_1 + 0x5c) != (int *)0x0) {
      (**(code **)(**(int **)(param_1 + 0x5c) + 0x10))(param_1 + 0x3c);
    }
    return;
  }
  FUN_004ece80((undefined4 *)(param_1 + 0x3c));
  return;
}

