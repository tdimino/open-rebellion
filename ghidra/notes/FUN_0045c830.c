
void * __thiscall FUN_0045c830(void *this,void *param_1,LONG param_2,LONG param_3)

{
  int *this_00;
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630b67;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this_00 = FUN_0045c660(this,param_2,param_3);
  FUN_004ece30(&param_2);
  local_4 = 1;
  if (this_00 != (int *)0x0) {
    puVar1 = FUN_0042d170(this_00,&param_3);
    local_4._0_1_ = 2;
    FUN_004f26d0(&param_2,puVar1);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
  }
  FUN_004f26d0(param_1,&param_2);
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

