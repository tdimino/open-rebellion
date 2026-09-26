
void * __thiscall FUN_00432120(void *this,void *param_1,uint param_2)

{
  void *this_00;
  undefined4 *puVar1;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d057;
  local_c = ExceptionList;
  local_10 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_14);
  local_4 = 1;
  this_00 = *(void **)((int)this + (param_2 & 0xffff) * 4 + 0x48);
  if (this_00 != (void *)0x0) {
    puVar1 = FUN_00403040(this_00,&param_2);
    local_4._0_1_ = 2;
    FUN_004f26d0(&local_14,puVar1);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
  }
  FUN_004f26d0(param_1,&local_14);
  local_10 = 1;
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

