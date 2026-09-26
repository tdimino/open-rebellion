
void * __thiscall FUN_00596c40(void *this,undefined4 param_1,undefined4 param_2,undefined2 param_3)

{
  undefined4 in_stack_0000001c;
  undefined4 in_stack_00000020;
  undefined4 in_stack_00000024;
  undefined4 in_stack_00000028;
  undefined4 in_stack_0000002c;
  undefined4 in_stack_00000030;
  undefined4 in_stack_00000034;
  undefined4 in_stack_00000038;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00650eb1;
  local_c = ExceptionList;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_005a6210(this);
  *(undefined ***)((int)this + 0x1c) = &PTR_FUN_0066bacc;
  *(undefined4 *)((int)this + 0x20) = 0;
  *(undefined ***)((int)this + 0x24) = &PTR_FUN_0066bac8;
  *(undefined4 *)((int)this + 0x28) = 0;
  local_4._0_1_ = 3;
  FUN_005f2f50((undefined4 *)((int)this + 0x38));
  *(undefined4 *)((int)this + 0x2c) = param_1;
  *(undefined4 *)((int)this + 0x30) = param_2;
  local_4 = CONCAT31(local_4._1_3_,4);
  *(undefined ***)this = &PTR_FUN_0066bac0;
  *(undefined2 *)((int)this + 0x34) = param_3;
  FUN_005f3090((undefined4 *)((int)this + 0x38),(int)&stack0x00000010);
  *(undefined4 *)((int)this + 0x44) = in_stack_0000001c;
  *(undefined4 *)((int)this + 0x48) = in_stack_00000020;
  *(undefined4 *)((int)this + 0x58) = in_stack_00000028;
  *(undefined4 *)((int)this + 0x54) = in_stack_00000024;
  *(undefined4 *)((int)this + 0x4c) = in_stack_0000002c;
  *(undefined4 *)((int)this + 0x5c) = in_stack_00000034;
  *(undefined4 *)((int)this + 0x50) = in_stack_00000030;
  *(undefined4 *)((int)this + 0x60) = in_stack_00000038;
  local_4 = 0xffffffff;
  FUN_005f2ff0((undefined4 *)&stack0x00000010);
  ExceptionList = local_c;
  return this;
}

