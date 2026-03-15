
void * __thiscall FUN_004da950(void *this,undefined4 param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063d07a;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004e4fb0(this,1,param_1);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x40));
  local_4._0_1_ = 1;
  FUN_004ece30((undefined4 *)((int)this + 0x44));
  local_4._0_1_ = 2;
  FUN_0042dbe0((undefined4 *)((int)this + 0x48));
  local_4._0_1_ = 3;
  FUN_004f5510((int *)((int)this + 0x58));
  local_4._0_1_ = 4;
  FUN_004f5510((int *)((int)this + 0x60));
  local_4._0_1_ = 5;
  FUN_004355d0((undefined4 *)((int)this + 0x68));
  local_4 = CONCAT31(local_4._1_3_,6);
  *(undefined ***)this = &PTR_FUN_0065c7f8;
  *(undefined4 *)((int)this + 0x38) = 0;
  *(undefined4 *)((int)this + 0x3c) = 0;
  *(undefined4 *)((int)this + 0x4c) = 0;
  *(undefined4 *)((int)this + 0x50) = 0;
  *(undefined4 *)((int)this + 0x54) = 0;
  (**(code **)(*(int *)((int)this + 0x58) + 4))();
  (**(code **)(*(int *)((int)this + 0x60) + 4))();
  *(undefined4 *)((int)this + 0x24) = 4;
  ExceptionList = pvStack_c;
  return this;
}

