
void __fastcall FUN_005d6ef0(int *param_1)

{
  HDC hdc;
  int iVar1;
  
  hdc = GetDC((HWND)param_1[6]);
  iVar1 = GetDeviceCaps(hdc,0xc);
  param_1[0x3f] = iVar1;
  ReleaseDC((HWND)param_1[6],hdc);
  FUN_005d68f0((int)param_1);
  FUN_005d6a40((int)param_1);
  FUN_005d6b40((int)param_1);
  FUN_005d6930((int)param_1);
  FUN_005d6bb0((int)param_1);
  if (param_1[0x48] != 0) {
    FUN_00619730();
    FUN_005d6e10((int)param_1);
    FUN_005d6ec0((int)param_1);
    return;
  }
  FUN_005d6c10(param_1);
  FUN_005d6e10((int)param_1);
  FUN_005d6ec0((int)param_1);
  return;
}

