
void __thiscall FUN_00458410(void *this,RECT *param_1)

{
  HWND hWnd;
  int iVar1;
  uint *puVar2;
  
  hWnd = *(HWND *)((int)this + 0x18);
  if (param_1 != (RECT *)0x0) {
    param_1[3].left = param_1[3].left & 0xfffffffe;
    InvalidateRect(hWnd,param_1,0);
    UpdateWindow(hWnd);
    return;
  }
  puVar2 = (uint *)((int)this + 0x1d0);
  iVar1 = 3;
  do {
    if ((*puVar2 & 1) != 0) {
      *puVar2 = *puVar2 & 0xfffffffe;
      InvalidateRect(hWnd,(RECT *)(puVar2 + -0xc),0);
    }
    puVar2 = puVar2 + 0xe;
    iVar1 = iVar1 + -1;
  } while (iVar1 != 0);
  UpdateWindow(hWnd);
  return;
}

