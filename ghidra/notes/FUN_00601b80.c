
void __thiscall FUN_00601b80(void *this,HDC param_1)

{
  LPRECT ptVar1;
  HDC hdc;
  int iVar2;
  LPCSTR lpchText;
  uint format;
  int iVar3;
  LPRECT lprc;
  
  hdc = param_1;
  if (*(int *)((int)this + 8) != 0) {
    iVar2 = FUN_00583c40(*(int *)((int)this + 8));
    if (iVar2 != 0) {
      if (*(HGDIOBJ *)((int)this + 0x28) != (HGDIOBJ)0x0) {
        param_1 = SelectObject(param_1,*(HGDIOBJ *)((int)this + 0x28));
      }
      ptVar1 = (LPRECT)((int)this + 0x14);
      format = *(uint *)((int)this + 0x10) & 0xffffff7f | 0x400;
      iVar2 = *(int *)((int)this + 0x1c);
      iVar3 = -1;
      lprc = ptVar1;
      lpchText = (LPCSTR)FUN_00583c40(*(int *)((int)this + 8));
      DrawTextA(hdc,lpchText,iVar3,lprc,format);
      if (((*(uint *)((int)this + 0x10) & 1) != 0) && ((*(byte *)((int)this + 0x2c) & 1) == 0)) {
        *(uint *)((int)this + 0x10) = *(uint *)((int)this + 0x10) & 0xfffffffe;
        iVar3 = *(int *)((int)this + 0x1c);
        if (iVar3 < iVar2) {
          *(int *)((int)this + 0x1c) = iVar2;
        }
        else {
          ptVar1->left = ptVar1->left + -1;
          *(int *)((int)this + 0x1c) = iVar3 + 1;
        }
      }
      if ((*(uint *)((int)this + 0x2c) & 1) != 0) {
        *(int *)((int)this + 0x1c) = *(int *)this + ptVar1->left;
      }
      if ((*(uint *)((int)this + 0x2c) & 2) != 0) {
        *(int *)((int)this + 0x20) = *(int *)((int)this + 0x18) + *(int *)((int)this + 4);
      }
      if (*(int *)((int)this + 0x28) != 0) {
        SelectObject(hdc,param_1);
      }
    }
  }
  return;
}

