
void __fastcall FUN_005da9b0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00654b88;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066d538;
  local_4 = 0;
  if (param_1[0x27] != 0) {
    FUN_00600280(param_1[0x27]);
    if ((undefined4 *)param_1[0x27] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x27])(1);
    }
    param_1[0x27] = 0;
  }
  local_4 = 0xffffffff;
  FUN_005ef0e0(param_1);
  ExceptionList = pvStack_c;
  return;
}

