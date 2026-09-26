
void __fastcall FUN_00410dd0(int *param_1)

{
  int iVar1;
  
  iVar1 = FUN_00597270(param_1[99]);
  if (iVar1 != 0) {
    FUN_00401150(0x484,0x1208,0x1207,0x190f);
    FUN_00410b10(param_1);
    return;
  }
  FUN_0040f850((int)param_1);
  param_1[0x4b] = 0x2776;
  FUN_006075e0(param_1,0x2776);
  if ((int *)param_1[0x50] != (int *)0x0) {
    (**(code **)(*(int *)param_1[0x50] + 4))(5);
  }
  if ((int *)param_1[0x5c] != (int *)0x0) {
    (**(code **)(*(int *)param_1[0x5c] + 4))(5);
  }
  if (param_1[0x5c] != 0) {
    SetFocus(*(HWND *)(param_1[0x5c] + 0x18));
  }
  if ((int *)param_1[0x5d] != (int *)0x0) {
    (**(code **)(*(int *)param_1[0x5d] + 4))(5);
  }
  if (param_1[0x5d] != 0) {
    SetFocus(*(HWND *)(param_1[0x5d] + 0x18));
  }
  return;
}

