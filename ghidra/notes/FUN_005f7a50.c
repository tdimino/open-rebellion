
void * __thiscall FUN_005f7a50(void *this,int param_1)

{
  char *pcVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655883;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590((undefined4 *)this);
  local_4 = 0;
  FUN_005f2f50((undefined4 *)((int)this + 0x2c));
  *(undefined ***)this = &PTR_FUN_0066de1c;
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined4 *)((int)this + 0x1c) = *(undefined4 *)(param_1 + 0x1c);
  pcVar1 = (char *)FUN_00583c40(param_1 + 0x2c);
  FUN_005f35e0((undefined4 *)((int)this + 0x2c),pcVar1);
  *(undefined4 *)((int)this + 0x20) = *(undefined4 *)(param_1 + 0x20);
  *(undefined4 *)((int)this + 0x24) = *(undefined4 *)(param_1 + 0x24);
  *(undefined4 *)((int)this + 0x28) = *(undefined4 *)(param_1 + 0x28);
  ExceptionList = local_c;
  return this;
}

