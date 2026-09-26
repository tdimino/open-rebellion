
void __thiscall
FUN_00543410(void *this,undefined4 param_1,uint *param_2,undefined4 param_3,undefined4 *param_4,
            void *param_5)

{
  bool bVar1;
  undefined3 extraout_var;
  undefined4 local_1c;
  undefined4 local_18;
  code *local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00647258;
  local_c = ExceptionList;
  local_1c = 1;
  local_18 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_5,&local_1c);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_4 = 0;
  bVar1 = FUN_0054c720(this,param_2,local_14);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    (*local_14[0])(param_1,param_3,param_4,param_5);
  }
  ExceptionList = local_c;
  return;
}

