
void __thiscall FUN_00601ce0(void *this,HDC param_1)

{
  int iVar1;
  int iVar2;
  COLORREF color;
  LPCSTR lpchText;
  uint format;
  LPRECT lprc;
  HGDIOBJ local_4;
  
  if (*(int *)((int)this + 0x30) != 0) {
    return;
  }
  if (*(int *)((int)this + 8) == 0) {
    return;
  }
  iVar2 = FUN_00583c40(*(int *)((int)this + 8));
  if (iVar2 == 0) {
    return;
  }
  iVar2 = *(int *)((int)this + 0x1c);
  local_4 = (HGDIOBJ)0x0;
  if (iVar2 != *(int *)((int)this + 0x14)) {
    iVar1 = *(int *)((int)this + 0x20);
    if ((((iVar1 != *(int *)((int)this + 0x18)) && (iVar2 != 0)) && (iVar1 != 0)) &&
       ((*(int *)((int)this + 0x14) <= iVar2 && (*(int *)((int)this + 0x18) <= iVar1))))
    goto LAB_00601d42;
  }
  FUN_00601b80(this,param_1);
LAB_00601d42:
  if (*(HGDIOBJ *)((int)this + 0x28) != (HGDIOBJ)0x0) {
    local_4 = SelectObject(param_1,*(HGDIOBJ *)((int)this + 0x28));
  }
  SetBkMode(param_1,1);
  color = SetTextColor(param_1,*(COLORREF *)((int)this + 0xc));
  format = (~*(uint *)((int)this + 0x2c) & 0x10) << 4 | *(uint *)((int)this + 0x10);
  lprc = (LPRECT)((int)this + 0x14);
  iVar2 = -1;
  lpchText = (LPCSTR)FUN_00583c40(*(int *)((int)this + 8));
  DrawTextA(param_1,lpchText,iVar2,lprc,format);
  SetTextColor(param_1,color);
  if (*(int *)((int)this + 0x28) != 0) {
    SelectObject(param_1,local_4);
  }
  return;
}

