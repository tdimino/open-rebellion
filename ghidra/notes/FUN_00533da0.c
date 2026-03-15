
void __thiscall FUN_00533da0(void *this,undefined4 param_1,undefined4 param_2,void *param_3)

{
  void *local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00645978;
  pvStack_c = ExceptionList;
  local_14 = (void *)0x1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
                    /* WARNING: Load size is inaccurate */
  (**(code **)(*this + 0x1c8))(param_1,param_3);
  ExceptionList = local_14;
  return;
}

