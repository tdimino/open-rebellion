
void __fastcall FUN_0048a750(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_00634f46;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065b070;
  local_4 = 1;
  if ((undefined4 *)param_1[8] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[8])(1);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00536ef0(param_1 + 1);
  ExceptionList = pvStack_c;
  return;
}

