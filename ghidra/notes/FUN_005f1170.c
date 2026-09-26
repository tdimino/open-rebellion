
void __thiscall FUN_005f1170(int *param_1,int param_2)

{
  if (param_2 == 0xdd01) {
    FUN_00596a70();
  }
  else if (param_2 != 0xdd02) {
    return;
  }
  (**(code **)(*param_1 + 8))();
  PostMessageA(*(HWND *)(param_1[8] + 0x18),0x405,0,0);
  return;
}

