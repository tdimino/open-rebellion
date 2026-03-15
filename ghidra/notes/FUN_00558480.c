
void * __thiscall FUN_00558480(void *this,undefined4 param_1,undefined4 param_2)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006498d0;
  local_c = ExceptionList;
  local_14 = 0x10;
  local_10 = 0x20;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00513050(this,param_1,&local_14,param_2);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_00619730();
  *(undefined ***)this = &PTR_FUN_00662730;
  ExceptionList = local_c;
  return this;
}

