
void __fastcall FUN_0059aa80(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006518b8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066bb00;
  local_4 = 0;
  if ((undefined4 *)param_1[0x43] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x43])(1);
  }
  if ((undefined4 *)param_1[0x44] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x44])(1);
  }
  if ((undefined4 *)param_1[0x42] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x42])(1);
    param_1[0x42] = 0;
  }
  DAT_006bc45c = 0;
  FUN_00604460(&DAT_006be4c8,3);
  FUN_00604460(&DAT_006be4c8,6);
  FUN_00604460(&DAT_006be4c8,0xe);
  local_4 = 0xffffffff;
  FUN_005ff250(param_1);
  ExceptionList = pvStack_c;
  return;
}

