
void __fastcall FUN_005c4e60(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00653b28;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0066ca20;
  local_4 = 0;
  if ((undefined4 *)param_1[0x27] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x27])(1);
  }
  if ((undefined4 *)param_1[0x28] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x28])(1);
  }
  local_4 = 0xffffffff;
  FUN_005ff250(param_1);
  ExceptionList = local_c;
  return;
}

