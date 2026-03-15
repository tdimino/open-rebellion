
void * __thiscall FUN_00495460(void *this,int param_1,undefined4 param_2)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006360ae;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4b50(this,param_2);
  local_4 = 0;
  FUN_005f2f50((undefined4 *)((int)this + 0x68));
  local_4._0_1_ = 1;
  FUN_004ece30((undefined4 *)((int)this + 0x74));
  local_4 = CONCAT31(local_4._1_3_,2);
  *(undefined ***)this = &PTR_FUN_0065b798;
  *(undefined4 *)((int)this + 100) = 0;
  uVar1 = FUN_0048af30();
  *(undefined4 *)((int)this + 0x34) = uVar1;
  *(undefined4 *)((int)this + 0x40) = 0;
  FUN_00495bf0(this,param_1);
  ExceptionList = local_c;
  return this;
}

