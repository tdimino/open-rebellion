
bool __fastcall FUN_005f7240(int param_1)

{
  HWND hWnd;
  short sVar1;
  ushort uVar2;
  ushort uVar3;
  int iVar4;
  HDC hDC;
  bool bVar5;
  
  hWnd = *(HWND *)(param_1 + 0x18);
  iVar4 = _SmackWait_4(*(undefined4 *)(param_1 + 0x1ac));
  if (iVar4 != 0) {
    uVar2 = GetAsyncKeyState(0x1b);
    uVar3 = GetAsyncKeyState(1);
    bVar5 = ((int)(short)(uVar3 | uVar2) & 0x10000000U) != 0;
    if (bVar5) {
      SendMessageA(*(HWND *)(*(int *)(param_1 + 0x20) + 0x18),0x404,0,0);
    }
    return bVar5;
  }
  hDC = GetDC(hWnd);
  iVar4 = *(int *)(param_1 + 0x1ac);
  if (*(int *)(iVar4 + 0x68) != 0) {
    _SmackBufferNewPalette_12
              (*(undefined4 *)(param_1 + 0x1a8),iVar4 + 0x6c,
               CONCAT22((short)((uint)*(int *)(iVar4 + 0x68) >> 0x10),*(undefined2 *)(iVar4 + 0x370)
                       ));
  }
  _SmackDoFrame_4(*(undefined4 *)(param_1 + 0x1ac));
  iVar4 = _SmackToBufferRect_8
                    (*(undefined4 *)(param_1 + 0x1ac),*(undefined2 *)(*(int *)(param_1 + 0x1a8) + 4)
                    );
  while (iVar4 != 0) {
    iVar4 = *(int *)(param_1 + 0x1ac);
    sVar1 = _SmackBufferBlit_32(*(undefined4 *)(param_1 + 0x1a8),hDC,0,0,
                                *(undefined2 *)(iVar4 + 0x378),*(undefined2 *)(iVar4 + 0x37c),
                                *(undefined2 *)(iVar4 + 0x380),*(undefined2 *)(iVar4 + 900));
    if (sVar1 != 0) break;
    iVar4 = _SmackToBufferRect_8
                      (*(undefined4 *)(param_1 + 0x1ac),
                       *(undefined2 *)(*(int *)(param_1 + 0x1a8) + 4));
  }
  ReleaseDC(hWnd,hDC);
  iVar4 = *(int *)(param_1 + 0x1ac);
  if (*(int *)(iVar4 + 0x374) == *(int *)(iVar4 + 0xc) + -1) {
    bVar5 = *(int *)(param_1 + 0x1a4) != 0;
    if (bVar5) {
      _SmackGoto_8(iVar4,1);
    }
    SendMessageA(*(HWND *)(*(int *)(param_1 + 0x20) + 0x18),0x402,0,0);
    return !bVar5;
  }
  _SmackNextFrame_4(iVar4);
  return false;
}

