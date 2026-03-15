
void __thiscall FUN_00600970(void *this,void *param_1,char *param_2)

{
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_c;
  undefined4 local_8;
  
  if (param_1 != (void *)0x0) {
    FUN_00600860(param_1,param_2);
    local_20 = *(undefined4 *)((int)param_1 + 0x18);
    *(undefined4 *)((int)param_1 + 0x90) = *(undefined4 *)((int)this + 0x18);
    local_24 = *(undefined4 *)((int)this + 0x18);
    local_c = *(undefined4 *)((int)this + 0x1c);
    local_8 = *(undefined4 *)((int)param_1 + 0x84);
    local_2c = 0x2c;
    local_28 = 1;
    if (DAT_006be5a4 != (HWND)0x0) {
      SendMessageA(DAT_006be5a4,0x404,0,(LPARAM)&local_2c);
    }
  }
  return;
}

