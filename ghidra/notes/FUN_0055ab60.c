
undefined4 __thiscall FUN_0055ab60(void *this,void *param_1)

{
  int local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649ae8;
  local_c = ExceptionList;
  if ((((*(byte *)((int)this + 0x50) & 1) == 0) || ((*(byte *)((int)this + 0x60) & 1) != 0)) ||
     (((byte)*(undefined4 *)((int)this + 0x24) & 0xc0) == 0xc0)) {
    ExceptionList = &local_c;
    FUN_0052e740(this,local_14);
    local_4 = 0;
    FUN_0051c9b0(local_14,0);
    FUN_0055a6e0(this,local_14,param_1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return 1;
}

