
void * __thiscall FUN_005fa920(void *this,char *param_1)

{
  undefined1 local_68 [12];
  undefined4 local_5c [20];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655d23;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00615bb0((void *)((int)this + 0x1c),1);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0066de30;
  *(undefined4 *)((int)this + 4) = 0;
  *(undefined4 *)((int)this + 8) = 0;
  *(undefined4 *)((int)this + 0xc) = 0;
  *(undefined4 *)((int)this + 0x10) = 0;
  *(undefined4 *)((int)this + 0x14) = 0;
  *(undefined4 *)((int)this + 0x18) = 0;
  if (param_1 != (char *)0x0) {
    FUN_006159e0(local_68,param_1,1);
    local_4._0_1_ = 1;
    FUN_005f4ce0(local_68,(undefined4 *)((int)this + 4));
    FUN_005f4ce0(local_68,(undefined4 *)((int)this + 8));
    FUN_005f4ce0(local_68,(undefined4 *)((int)this + 0xc));
    FUN_005f4ce0(local_68,(int)this + 0x10);
    FUN_005f4ce0(local_68,(int)this + 0x14);
    FUN_005f4ce0(local_68,(int)this + 0x18);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00615b90((int)local_5c);
    FUN_00615440(local_5c);
  }
  ExceptionList = local_c;
  return this;
}

