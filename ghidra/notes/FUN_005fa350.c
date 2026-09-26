
bool __thiscall FUN_005fa350(void *this,int param_1)

{
  bool bVar1;
  undefined4 local_80;
  int local_7c;
  int local_78;
  LPARAM local_74;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655cd8;
  local_c = ExceptionList;
  bVar1 = false;
  if (param_1 != 0) {
    ExceptionList = &local_c;
    FUN_005fa920(&local_80,*(char **)(param_1 + 4));
    bVar1 = local_7c == -0x54322346;
    local_4 = 0;
    if ((bVar1) && (local_78 == 4)) {
      FUN_005f8600(this);
      FUN_005ae1e0();
      PostMessageA(*(HWND *)((int)this + 0x2c),0x40c,4,local_74);
    }
    local_4 = 0xffffffff;
    FUN_005faad0(&local_80);
  }
  ExceptionList = local_c;
  return bVar1;
}

