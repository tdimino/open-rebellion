
void * __thiscall FUN_004c4d90(void *this,undefined4 param_1)

{
  uint *this_00;
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a7d3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590(this);
  this_00 = (uint *)((int)this + 0x24);
  local_4 = 0;
  FUN_005f4950(this_00,0);
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined ***)this = &PTR_FUN_0065c538;
  *(undefined4 *)((int)this + 0x1c) = 0;
  *(undefined4 *)((int)this + 0x20) = param_1;
  uVar1 = thunk_FUN_004fcee0();
  *(undefined4 *)((int)this + 0x38) = uVar1;
  *(undefined4 *)((int)this + 0x28) = 0;
  *(undefined4 *)((int)this + 0x40) = 1;
  *(undefined2 *)((int)this + 0x2c) = 0;
  *(undefined2 *)((int)this + 0x2e) = 0;
  *(undefined2 *)((int)this + 0x30) = 0;
  *(undefined2 *)((int)this + 0x32) = 0;
  uVar1 = FUN_006158b0();
  *(undefined4 *)((int)this + 0x34) = uVar1;
  *this_00 = *this_00 | 0x10;
  ExceptionList = local_c;
  return this;
}

