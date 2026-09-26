
void * __thiscall FUN_00496460(void *this,int param_1,undefined4 param_2)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006362f6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4b50(this,param_2);
  local_4 = 0;
  FUN_004f5510((undefined4 *)((int)this + 0x70));
  local_4._0_1_ = 1;
  FUN_004f5510((undefined4 *)((int)this + 0x78));
  local_4._0_1_ = 2;
  FUN_004f5510((undefined4 *)((int)this + 0x80));
  local_4._0_1_ = 3;
  FUN_004f5510((undefined4 *)((int)this + 0x88));
  local_4._0_1_ = 4;
  FUN_004f5510((undefined4 *)((int)this + 0x90));
  local_4._0_1_ = 5;
  FUN_004f5510((undefined4 *)((int)this + 0x98));
  local_4 = CONCAT31(local_4._1_3_,6);
  *(undefined ***)this = &PTR_FUN_0065b870;
  *(undefined4 *)((int)this + 0x40) = 0;
  uVar1 = FUN_0048bc60();
  *(undefined4 *)((int)this + 0x34) = uVar1;
  *(undefined4 *)((int)this + 0x68) = 0;
  *(undefined4 *)((int)this + 0x6c) = 0;
  *(undefined4 *)((int)this + 0xa0) = 0;
  *(undefined4 *)((int)this + 100) = 0;
  FUN_004974f0(this,param_1);
  ExceptionList = local_c;
  return this;
}

