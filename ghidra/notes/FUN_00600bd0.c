
void __thiscall FUN_00600bd0(void *this)

{
  undefined4 in_stack_00000018;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  tagRECT local_1c;
  undefined4 uStack_c;
  undefined4 uStack_8;
  
  local_24 = *(undefined4 *)((int)this + 0x18);
  local_2c = 0x2c;
  local_28 = 0;
  CopyRect(&local_1c,(RECT *)&stack0x00000004);
  uStack_c = *(undefined4 *)((int)this + 0x1c);
  uStack_8 = in_stack_00000018;
  if (DAT_006be5a4 != (HWND)0x0) {
    SendMessageA(DAT_006be5a4,0x404,0,(LPARAM)&local_2c);
  }
  return;
}

