
bool __thiscall FUN_006069d0(void *this,int param_1)

{
  HDC hdc;
  HGDIOBJ pvVar1;
  HDC pHVar2;
  HGDIOBJ pvVar3;
  bool bVar4;
  
  hdc = GetDC(*(HWND *)((int)this + 0x18));
  pvVar1 = (HGDIOBJ)FUN_006002b0(4);
  if (param_1 == 0) {
    *(undefined4 *)((int)this + 0x108) = *(undefined4 *)((int)this + 0xa4);
  }
  else {
    *(int *)((int)this + 0x108) = param_1;
  }
  bVar4 = *(int *)((int)this + 0x108) != 0;
  if (bVar4) {
    pHVar2 = CreateCompatibleDC(hdc);
    *(HDC *)((int)this + 0xd0) = pHVar2;
    pvVar3 = (HGDIOBJ)FUN_005fc130(*(int *)((int)this + 0x108));
    pvVar3 = SelectObject(*(HDC *)((int)this + 0xd0),pvVar3);
    *(HGDIOBJ *)((int)this + 0xd4) = pvVar3;
    pvVar3 = SelectObject(*(HDC *)((int)this + 0xd0),DAT_006be5a0);
    *(HGDIOBJ *)((int)this + 0xd8) = pvVar3;
    ReleaseDC(*(HWND *)((int)this + 0x18),hdc);
    pvVar1 = SelectObject(*(HDC *)((int)this + 0xd0),pvVar1);
    *(HGDIOBJ *)((int)this + 0xdc) = pvVar1;
    SetTextColor(*(HDC *)((int)this + 0xd0),0x2ffffff);
    SetBkMode(*(HDC *)((int)this + 0xd0),1);
  }
  return bVar4;
}

