
void * __thiscall FUN_004117c0(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a6d3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2f50((undefined4 *)((int)this + 4));
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_006587e0;
  pvVar1 = FUN_00408d90(param_1,local_18);
  local_4._0_1_ = 1;
  FUN_005f3090((undefined4 *)((int)this + 4),(int)pvVar1);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(local_18);
  *(undefined4 *)((int)this + 0x10) = *(undefined4 *)((int)param_1 + 0x10);
  *(undefined4 *)((int)this + 0x14) = *(undefined4 *)((int)param_1 + 0x14);
  *(undefined4 *)((int)this + 0x18) = *(undefined4 *)((int)param_1 + 0x18);
  *(undefined4 *)((int)this + 0x1c) = *(undefined4 *)((int)param_1 + 0x1c);
  *(undefined4 *)((int)this + 0x20) = *(undefined4 *)((int)param_1 + 0x20);
  *(undefined4 *)((int)this + 0x24) = *(undefined4 *)((int)param_1 + 0x24);
  ExceptionList = local_c;
  return this;
}

