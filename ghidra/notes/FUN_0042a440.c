
void __thiscall FUN_0042a440(void *this,undefined4 *param_1)

{
  void *this_00;
  LPARAM lParam;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c53b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  PostMessageA(*(HWND *)((int)this + 0x18),0x409,0,0);
  this_00 = (void *)FUN_00618b70(4);
  local_4 = 0;
  if (this_00 == (void *)0x0) {
    lParam = 0;
  }
  else {
    lParam = FUN_004f26d0(this_00,param_1);
  }
  local_4 = 0xffffffff;
  PostMessageA(*(HWND *)((int)this + 0x18),0x468,0x1a,lParam);
  ExceptionList = local_c;
  return;
}

