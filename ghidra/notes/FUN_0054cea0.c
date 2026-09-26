
void * __thiscall FUN_0054cea0(void *this,int param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006481d9;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0054f380(this,param_1);
  local_4 = 0;
  *(undefined4 *)((int)this + 0x50) = *(undefined4 *)(param_1 + 0x50);
  *(undefined4 *)((int)this + 0x54) = *(undefined4 *)(param_1 + 0x54);
  *(undefined4 *)((int)this + 0x58) = *(undefined4 *)(param_1 + 0x58);
  FUN_004f26d0((void *)((int)this + 0x5c),(undefined4 *)(param_1 + 0x5c));
  local_4._0_1_ = 1;
  FUN_004f26d0((void *)((int)this + 0x60),(undefined4 *)(param_1 + 0x60));
  local_4._0_1_ = 2;
  FUN_004f4390((void *)((int)this + 100),param_1 + 100);
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_004f4390((void *)((int)this + 0x6c),param_1 + 0x6c);
  *(undefined ***)this = &PTR_FUN_00661e28;
  ExceptionList = local_c;
  return this;
}

