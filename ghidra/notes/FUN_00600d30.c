
void __thiscall FUN_00600d30(void *this,int param_1)

{
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  tagRECT local_1c;
  undefined4 uStack_c;
  undefined4 uStack_8;
  
  local_24 = *(undefined4 *)(param_1 + 0x90);
  local_20 = *(undefined4 *)(param_1 + 0x18);
  local_2c = 0x2c;
  local_28 = 1;
  SetRectEmpty(&local_1c);
  uStack_c = *(undefined4 *)((int)this + 0x1c);
  uStack_8 = 0;
  if (DAT_006be5a4 != (HWND)0x0) {
    SendMessageA(DAT_006be5a4,0x405,0,(LPARAM)&local_2c);
  }
  if (*(undefined **)(param_1 + 0x84) != (undefined *)0x0) {
    FUN_00618b60(*(undefined **)(param_1 + 0x84));
    *(undefined4 *)(param_1 + 0x84) = 0;
  }
  return;
}

