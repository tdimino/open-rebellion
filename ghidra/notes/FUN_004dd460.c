
void * __thiscall FUN_004dd460(void *this,undefined4 param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063d598;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004e4fb0(this,6,param_1);
  local_4 = 0;
  FUN_004355d0((undefined4 *)((int)this + 0x40));
  local_4._0_1_ = 1;
  FUN_004ece30((undefined4 *)((int)this + 0x50));
  local_4._0_1_ = 2;
  FUN_004ece30((undefined4 *)((int)this + 0x54));
  local_4._0_1_ = 3;
  FUN_004ece30((undefined4 *)((int)this + 0x58));
  local_4._0_1_ = 4;
  FUN_0042dbe0((undefined4 *)((int)this + 0x5c));
  local_4._0_1_ = 5;
  FUN_004f5510((int *)((int)this + 0x7c));
  local_4._0_1_ = 6;
  FUN_004f5510((int *)((int)this + 0x84));
  local_4 = CONCAT31(local_4._1_3_,7);
  *(undefined ***)this = &PTR_FUN_0065c818;
  *(undefined4 *)((int)this + 0x38) = 0;
  *(undefined4 *)((int)this + 0x3c) = 0;
  *(undefined4 *)((int)this + 0x60) = 0;
  *(undefined4 *)((int)this + 100) = 0;
  *(undefined4 *)((int)this + 0x68) = 0;
  *(undefined4 *)((int)this + 0x78) = 1;
  *(undefined4 *)((int)this + 0x6c) = 0;
  *(undefined4 *)((int)this + 0x70) = 0;
  *(undefined4 *)((int)this + 0x74) = 0;
  (**(code **)(*(int *)((int)this + 0x7c) + 4))();
  (**(code **)(*(int *)((int)this + 0x84) + 4))();
  *(undefined4 *)((int)this + 0x24) = 0x16;
  ExceptionList = pvStack_c;
  return this;
}

