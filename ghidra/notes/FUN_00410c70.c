
void __fastcall FUN_00410c70(int *param_1)

{
  int iVar1;
  HCURSOR pHVar2;
  
  iVar1 = FUN_00597270(param_1[99]);
  if (iVar1 != 0) {
    FUN_00401150(0x484,0x1208,0x1207,0x190f);
    FUN_00410b10(param_1);
    return;
  }
  if ((undefined4 *)param_1[100] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[100])(1);
    param_1[100] = 0;
  }
  pHVar2 = LoadCursorA((HINSTANCE)0x0,(LPCSTR)0x7f02);
  SetCursor(pHVar2);
  iVar1 = FUN_0040f450(param_1);
  if (iVar1 == 0) {
    SetTimer((HWND)param_1[6],0x191,5000,(TIMERPROC)0x0);
    pHVar2 = LoadCursorA((HINSTANCE)0x0,(LPCSTR)0x7f00);
    SetCursor(pHVar2);
    FUN_0040f850((int)param_1);
    param_1[0x4b] = 0x2775;
    FUN_006075e0(param_1,0x2775);
    if (param_1[0x50] != 0) {
      iVar1 = FUN_00609eb0(param_1[0x62]);
      if (iVar1 == 0) {
        (**(code **)(*(int *)param_1[0x50] + 0x10))();
      }
      (**(code **)(*(int *)param_1[0x50] + 4))(5);
    }
    if ((int *)param_1[0x62] != (int *)0x0) {
      (**(code **)(*(int *)param_1[0x62] + 4))(5);
    }
    if ((int *)param_1[0x5e] != (int *)0x0) {
      (**(code **)(*(int *)param_1[0x5e] + 4))(5);
    }
    if (param_1[0x5e] != 0) {
      SetFocus(*(HWND *)(param_1[0x5e] + 0x18));
      return;
    }
  }
  else {
    pHVar2 = LoadCursorA((HINSTANCE)0x0,(LPCSTR)0x7f00);
    SetCursor(pHVar2);
    FUN_00410b10(param_1);
  }
  return;
}

