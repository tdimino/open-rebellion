
void __thiscall FUN_004edb70(void *this,undefined4 param_1,void *param_2)

{
  void *local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0063ee58;
  pvStack_c = ExceptionList;
  local_14 = (void *)0x1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
                    /* WARNING: Load size is inaccurate */
  (**(code **)(*this + 0x2d8))(param_1,param_2);
  ExceptionList = local_14;
  return;
}

