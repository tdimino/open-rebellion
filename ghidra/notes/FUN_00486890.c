
void __thiscall FUN_00486890(int param_1,uint param_2)

{
  if (((param_2 & 0x10000000) != 0) && (*(int **)(param_1 + 0xc0) != (int *)0x0)) {
    (**(code **)(**(int **)(param_1 + 0xc0) + 0x10))(param_2);
  }
  return;
}

