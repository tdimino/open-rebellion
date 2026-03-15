
void __thiscall FUN_0048a210(void *this,uint param_1,undefined4 param_2)

{
  void *this_00;
  void *this_01;
  void *local_c;
  undefined1 *puStack_8;
  void *local_4;
  
  local_4 = (void *)0xffffffff;
  puStack_8 = &LAB_00634e5b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this_00 = (void *)FUN_005f5500((void *)((int)this + 0x44),param_1);
  if (this_00 == (void *)0x0) {
    this_01 = (void *)FUN_00618b70(0x20);
    if (this_01 == (void *)0x0) {
      this_00 = (void *)0x0;
    }
    else {
      local_4 = this_00;
      this_00 = FUN_005f5f10(this_01,0);
    }
    local_4 = (void *)0xffffffff;
    FUN_005f5600(this_00,param_1);
    FUN_005f5440((void *)((int)this + 0x44),this_00);
  }
  *(undefined4 *)((int)this_00 + 0x1c) = param_2;
  ExceptionList = local_c;
  return;
}

