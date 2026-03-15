
void * __thiscall FUN_0052c170(void *this,undefined4 param_1,undefined4 param_2)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00644bf0;
  local_c = ExceptionList;
  local_14 = 0xa0;
  local_10 = 0xb0;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00513050(this,param_1,&local_14,param_2);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_00619730();
  *(undefined ***)this = &PTR_FUN_0065fe54;
  ExceptionList = local_c;
  return this;
}

