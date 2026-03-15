
void __fastcall FUN_004862b0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0063490e;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065ad70;
  local_4 = 2;
  if ((undefined4 *)param_1[0x30] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x30])(1);
  }
  local_4._0_1_ = 1;
  FUN_00488990((int)(param_1 + 0x1b));
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004bee90(param_1 + 0x17);
  local_4 = 0xffffffff;
  FUN_00488580(param_1);
  ExceptionList = pvStack_c;
  return;
}

