
void __thiscall FUN_006002f0(void *this,HDC param_1)

{
  if (*(HGDIOBJ *)((int)this + 0x80) != (HGDIOBJ)0x0) {
    SelectObject(param_1,*(HGDIOBJ *)((int)this + 0x80));
  }
  return;
}

