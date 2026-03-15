
void * __thiscall
FUN_004aa960(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,int param_6,undefined4 param_7,undefined4 param_8)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638348;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00606380(this,param_1,param_2,param_3,param_4,param_5,param_6,0,0,0,param_7,0x52000000,0,0,0);
  local_4 = 0;
  FUN_004c7f50((undefined4 *)((int)this + 0x138));
  *(undefined4 *)((int)this + 0x114) = 0;
  *(undefined4 *)((int)this + 0x118) = 0;
  *(uint *)((int)this + 0x50) = *(uint *)((int)this + 0x50) & 0xefffffff | 0x6000000;
  *(undefined4 *)((int)this + 0x11c) = 0;
  *(undefined4 *)((int)this + 0x120) = 0;
  *(undefined4 *)((int)this + 0x148) = 0;
  *(undefined4 *)((int)this + 0x124) = 0;
  *(undefined4 *)((int)this + 0x128) = 0;
  *(undefined4 *)((int)this + 0x134) = 0;
  *(undefined ***)this = &PTR_FUN_0065bef0;
  *(undefined4 *)((int)this + 300) = param_8;
  ExceptionList = local_c;
  return this;
}

