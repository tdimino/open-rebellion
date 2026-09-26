
void * __thiscall
FUN_00452090(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,int param_6,undefined4 param_7,undefined4 param_8)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062fcb8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00606380(this,param_1,param_2,param_3,param_4,param_5,param_6,2,7,0,param_7,0x56000000,0,0,0);
  *(uint *)((int)this + 0x50) = *(uint *)((int)this + 0x50) & 0xefffffff;
  *(undefined ***)this = &PTR_FUN_00659de0;
  *(uint *)((int)this + 0xb8) = *(uint *)((int)this + 0xb8) | 4;
  *(undefined4 *)((int)this + 0x114) = param_8;
  *(int *)((int)this + 0x11c) = param_6;
  local_4 = 0;
  FUN_006071a0(this);
  *(undefined4 *)((int)this + 0x144) = 0;
  *(undefined4 *)((int)this + 0x148) = 0;
  *(undefined4 *)((int)this + 0x140) = 0;
  ExceptionList = local_c;
  return this;
}

