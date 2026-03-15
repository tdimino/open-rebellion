
void * __thiscall FUN_00526090(void *this,undefined4 param_1)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00643f20;
  local_c = ExceptionList;
  local_14 = 0x30;
  local_10 = 0x3c;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00525870(this,param_1,&local_14,1,0,0);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_00619730();
  *(undefined ***)this = &PTR_FUN_0065f090;
  ExceptionList = local_c;
  return this;
}

