
void * __thiscall FUN_004ffef0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00640740;
  local_c = ExceptionList;
  local_14 = 8;
  local_10 = 0x10;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00513090(this,param_1,param_3,&local_14,param_2);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_00619730();
  *(undefined ***)this = &PTR_FUN_0065d648;
  ExceptionList = local_c;
  return this;
}

