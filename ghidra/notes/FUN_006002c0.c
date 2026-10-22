
void __thiscall FUN_006002c0(void *this,HDC param_1)

{
  HGDIOBJ pvVar1;
  
  if (*(HGDIOBJ *)((int)this + 0x7c) != (HGDIOBJ)0x0) {
    pvVar1 = SelectObject(param_1,*(HGDIOBJ *)((int)this + 0x7c));
    *(HGDIOBJ *)((int)this + 0x80) = pvVar1;
    return;
  }
  *(undefined4 *)((int)this + 0x80) = 0;
  return;
}

