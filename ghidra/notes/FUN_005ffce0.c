
bool __thiscall FUN_005ffce0(void *this,int param_1)

{
  BOOL BVar1;
  HWND pHVar2;
  HMENU hMenu;
  DWORD dwExStyle;
  LPCSTR lpClassName;
  LPCSTR lpWindowName;
  HINSTANCE hInstance;
  HWND pHStack_5c;
  tagWNDCLASSA local_58;
  void *pvStack_30;
  undefined4 uStack_2c;
  undefined4 uStack_28;
  undefined4 uStack_24;
  int iStack_20;
  int iStack_1c;
  int iStack_18;
  int iStack_14;
  DWORD DStack_10;
  undefined4 uStack_c;
  undefined4 uStack_8;
  int iStack_4;
  
  BVar1 = GetClassInfoA(*(HINSTANCE *)((int)this + 0x1c),*(LPCSTR *)((int)this + 0x48),&local_58);
  if (BVar1 == 0) {
    FUN_005ffc80(this,0xb);
  }
  uStack_2c = *(undefined4 *)((int)this + 0x1c);
  uStack_28 = *(undefined4 *)((int)this + 0x68);
  if (*(int *)((int)this + 0x20) == 0) {
    uStack_24 = 0;
  }
  else {
    uStack_24 = *(undefined4 *)(*(int *)((int)this + 0x20) + 0x18);
  }
  uStack_8 = *(undefined4 *)((int)this + 0x48);
  iStack_1c = *(int *)((int)this + 0x30);
  iStack_18 = *(int *)((int)this + 0x2c);
  iStack_14 = *(int *)((int)this + 0x28);
  DStack_10 = *(DWORD *)((int)this + 0x50);
  iStack_20 = *(int *)((int)this + 0x34);
  iStack_4 = *(int *)((int)this + 0x54);
  uStack_c = 0;
  if ((param_1 == 0) && (iStack_4 == 0)) {
    if (*(int *)((int)this + 0x20) == 0) {
      hMenu = *(HMENU *)((int)this + 0x68);
    }
    else {
      hMenu = *(HMENU *)((int)this + 0x24);
    }
    if (*(int *)((int)this + 0x20) == 0) {
      pHStack_5c = (HWND)0x0;
    }
    else {
      pHStack_5c = *(HWND *)(*(int *)((int)this + 0x20) + 0x18);
    }
    hInstance = *(HINSTANCE *)((int)this + 0x1c);
    lpWindowName = *(LPCSTR *)((int)this + 0x4c);
    lpClassName = *(LPCSTR *)((int)this + 0x48);
    dwExStyle = 0;
  }
  else {
    if (*(int *)((int)this + 0x20) == 0) {
      hMenu = *(HMENU *)((int)this + 0x68);
    }
    else {
      hMenu = *(HMENU *)((int)this + 0x24);
    }
    if (*(int *)((int)this + 0x20) == 0) {
      pHStack_5c = (HWND)0x0;
    }
    else {
      pHStack_5c = *(HWND *)(*(int *)((int)this + 0x20) + 0x18);
    }
    hInstance = *(HINSTANCE *)((int)this + 0x1c);
    lpClassName = *(LPCSTR *)((int)this + 0x48);
    lpWindowName = *(LPCSTR *)((int)this + 0x4c);
    dwExStyle = *(DWORD *)((int)this + 0x54);
  }
  pvStack_30 = this;
  pHVar2 = CreateWindowExA(dwExStyle,lpClassName,lpWindowName,DStack_10,iStack_14,iStack_18,
                           iStack_1c,iStack_20,pHStack_5c,hMenu,hInstance,&pvStack_30);
  *(HWND *)((int)this + 0x18) = pHVar2;
  if (pHVar2 == (HWND)0x0) {
    GetLastError();
  }
  FUN_00601260((int)this);
  return *(int *)((int)this + 0x18) != 0;
}

