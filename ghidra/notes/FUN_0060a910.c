
undefined4 __cdecl FUN_0060a910(int param_1,int param_2,int param_3,uint param_4)

{
  int iVar1;
  HDC hdc;
  int iVar2;
  int iVar3;
  int iVar4;
  uint uVar5;
  
  iVar1 = GetSystemMetrics(0x13);
  if (iVar1 == 0) {
    FUN_0060a9e0(s_No_mouse_was_detected__A_mouse_i_006ac8cc,(LPCSTR)0x0);
    return 0;
  }
  hdc = CreateICA(s_DISPLAY_006ac8c4,(LPCSTR)0x0,(LPCSTR)0x0,(DEVMODEA *)0x0);
  iVar1 = GetDeviceCaps(hdc,8);
  iVar2 = GetDeviceCaps(hdc,10);
  iVar3 = GetDeviceCaps(hdc,0xe);
  iVar4 = GetDeviceCaps(hdc,0xc);
  uVar5 = 1 << ((char)iVar3 * (char)iVar4 & 0x1fU);
  DeleteDC(hdc);
  if (uVar5 == 1) {
    uVar5 = 0x1000000;
  }
  if (((param_2 <= iVar1) && (param_3 <= iVar2)) && (param_4 <= uVar5)) {
    if (param_1 != 0) {
      FUN_0060a9e0(&DAT_006ac868,(LPCSTR)0x0);
      return 0;
    }
    return 1;
  }
  FUN_0060a9e0(s_Display_resolution_must_be_a_min_006ac824,(LPCSTR)0x0);
  return 0;
}

