
void * __thiscall FUN_00492470(void *this,int param_1,undefined4 param_2)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00635e17;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4b50(this,param_2);
  local_4 = 0;
  FUN_005f2f50((undefined4 *)((int)this + 0x68));
  local_4._0_1_ = 1;
  FUN_004ece30((undefined4 *)((int)this + 0x74));
  local_4._0_1_ = 2;
  FUN_004ece30((undefined4 *)((int)this + 0x78));
  local_4._0_1_ = 3;
  FUN_004f5510((undefined4 *)((int)this + 0x88));
  local_4 = CONCAT31(local_4._1_3_,4);
  *(undefined ***)this = &PTR_FUN_0065b710;
  *(undefined4 *)((int)this + 100) = 0;
  *(undefined4 *)((int)this + 0x7c) = 0x16;
  uVar1 = FUN_0048af30();
  *(undefined4 *)((int)this + 0x34) = uVar1;
  *(undefined4 *)((int)this + 0x80) = 0;
  *(undefined4 *)((int)this + 0x84) = 3;
  *(undefined4 *)((int)this + 0x90) = 0;
  *(undefined4 *)((int)this + 0x94) = 0;
  *(undefined4 *)((int)this + 0x40) = 0;
  FUN_00494d20(this,param_1);
  ExceptionList = local_c;
  return this;
}

