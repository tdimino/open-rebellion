
void FUN_00406520(void)

{
  int iVar1;
  HDC hDC;
  undefined4 uVar2;
  undefined4 uVar3;
  
  if (DAT_006b06f4 != (int *)0x0) {
    iVar1 = FUN_005ff440();
    if (iVar1 != 0) {
      hDC = GetDC((HWND)DAT_006b06f4[6]);
      FUN_005ff440();
      uVar2 = FUN_005fc3f0();
      FUN_00600010(hDC,uVar2);
      ReleaseDC((HWND)DAT_006b06f4[6],hDC);
    }
    InvalidateRect((HWND)DAT_006b06f4[6],(RECT *)0x0,0);
    (**(code **)(*DAT_006b06f4 + 4))(5);
    DAT_006b06f4[0x41] = 0;
    uVar2 = *(undefined4 *)(DAT_006b06f4[8] + 0x18);
    uVar3 = FUN_00401a80();
    FUN_00401a60(uVar3);
    FUN_00610be0(uVar2,1,0);
    DAT_006b06f4[0x40] = 0;
    FUN_004173c0();
    uVar2 = FUN_0041d3b0();
    FUN_00417520(uVar2);
  }
  return;
}

